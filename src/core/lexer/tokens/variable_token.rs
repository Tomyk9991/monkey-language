use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;
use std::str::FromStr;

use anyhow::Context;

use crate::core::code_generator::generator::{Stack, StackLocation};
use crate::core::code_generator::target_os::TargetOS;
use crate::core::code_generator::ToASM;
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::errors::EmptyIteratorErr;
use crate::core::lexer::levenshtein_distance::{MethodCallSummarizeTransform, PatternedLevenshteinDistance, PatternedLevenshteinString, QuoteSummarizeTransform};
use crate::core::lexer::tokenizer::StaticTypeContext;
use crate::core::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::core::lexer::tokens::name_token::{NameToken, NameTokenErr};
use crate::core::lexer::TryParse;
use crate::core::lexer::type_token::{InferTypeError, TypeToken};

/// Token for a variable. Pattern is defined as: name <Assignment> assignment <Separator>
/// # Examples
/// - `name = assignment;`
/// - `name: assignment,`
#[derive(Debug, PartialEq, Clone)]
pub struct VariableToken<const ASSIGNMENT: char, const SEPARATOR: char> {
    pub name_token: NameToken,
    // flag defining if the variable is mutable or not
    pub mutability: bool,
    /// type of the variable. It's None, when the type is unknown
    pub ty: Option<TypeToken>,
    /// flag defining if the variable is a new definition or a re-assignment
    pub define: bool,
    pub assignable: AssignableToken,
    pub code_line: CodeLine
}

impl<const ASSIGNMENT: char, const SEPARATOR: char> VariableToken<ASSIGNMENT, SEPARATOR> {
    pub fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<TypeToken, InferTypeError> {
        return match &self.ty {
            // validity check. is the assignment really the type the programmer used
            // example: let a: i32 = "Hallo"; is not valid since you're assigning a string to an integer

            // if type is present. check, if the type matches the assignment
            // else infer the type with a context
            Some(ty) => {
                let inferred_type = self.assignable.infer_type_with_context(&type_context)?;

                if ty != &inferred_type {
                    return Err(InferTypeError::MismatchedTypes { expected: ty.clone(), actual: inferred_type.clone() }.into())
                }

                Ok(ty.clone())
            }
            None => {
                let ty = self.infer_with_context(&type_context)?;
                self.ty = Some(ty.clone());
                type_context.push((self.name_token.clone(), ty.clone()));

                Ok(ty)
            }
        }
    }
}

impl<const ASSIGNMENT: char, const SEPARATOR: char> Display for VariableToken<ASSIGNMENT, SEPARATOR> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:<30}:{:?} {} {}", if self.define { "let " } else { "" }, self.name_token, self.ty, ASSIGNMENT, self.assignable)
    }
}

#[derive(Debug)]
pub enum ParseVariableTokenErr {
    PatternNotMatched { target_value: String },
    NameTokenErr(NameTokenErr),
    AssignableTokenErr(AssignableTokenErr),
    InferType(InferTypeError),
    EmptyIterator(EmptyIteratorErr),
}

impl Error for ParseVariableTokenErr {}

impl From<InferTypeError> for ParseVariableTokenErr {
    fn from(value: InferTypeError) -> Self {
        ParseVariableTokenErr::InferType(value)
    }
}

impl From<NameTokenErr> for ParseVariableTokenErr {
    fn from(a: NameTokenErr) -> Self { ParseVariableTokenErr::NameTokenErr(a) }
}

impl From<anyhow::Error> for ParseVariableTokenErr {
    fn from(value: anyhow::Error) -> Self {
        let mut buffer = String::new();
        buffer += &value.to_string();
        buffer += "\n";

        if let Some(e) = value.downcast_ref::<AssignableTokenErr>() {
            buffer += &e.to_string();
        }
        ParseVariableTokenErr::PatternNotMatched { target_value: buffer }
    }
}

impl From<AssignableTokenErr> for ParseVariableTokenErr {
    fn from(a: AssignableTokenErr) -> Self { ParseVariableTokenErr::AssignableTokenErr(a) }
}

impl Display for ParseVariableTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ParseVariableTokenErr::PatternNotMatched { target_value } => format!("`{target_value}`\n\tThe pattern for a variable is defined as: name = assignment;"),
            ParseVariableTokenErr::NameTokenErr(a) => a.to_string(),
            ParseVariableTokenErr::AssignableTokenErr(a) => a.to_string(),
            ParseVariableTokenErr::EmptyIterator(e) => e.to_string(),
            ParseVariableTokenErr::InferType(err) => format!("{err}")
        })
    }
}

impl<const ASSIGNMENT: char, const SEPARATOR: char> ToASM for VariableToken<ASSIGNMENT, SEPARATOR> {
    fn to_asm(&self, stack: &mut Stack, target_os: &TargetOS) -> Result<String, crate::core::code_generator::Error> {
        let mut target = String::new();

        let mut i = 0;
        while i < stack.variables.len() {
            if stack.variables[i].name.name == self.name_token.name {
                if !self.define {
                    target.push_str(&format!("    ; Re-assign {}\n", self));
                    target.push_str(&self.assignable.to_asm(stack, target_os)?);
                    target.push_str(&stack.pop_stack("rax"));
                    target.push_str(&format!("    mov QWORD [rsp + {}], rax\n", (stack.stack_position - stack.variables[i].position - 1) * 8));

                    return Ok(target);
                }
                // else {
                //     Err(crate::core::code_generator::Error::VariableAlreadyUsed { name: self.name_token.name.clone() })
                // };
            }
            i += 1;
        }

        // at this point a line has the structure
        // a = 5
        // this is not valid, since its not a definition, its an assignment on a variable with the name
        // a but a not found in the scope
        if !self.define {
            return Err(crate::core::code_generator::Error::UnresolvedReference { name: self.name_token.name.clone() });
        }

        stack.variables.push(StackLocation { position: stack.stack_position, name: self.name_token.clone() });

        target.push_str(&format!("    ; Pushing onto stack: {}\n", self));
        target.push_str(&self.assignable.to_asm(stack, target_os)?);

        Ok(target)
    }
}

impl<const ASSIGNMENT: char, const SEPARATOR: char> TryParse for VariableToken<ASSIGNMENT, SEPARATOR> {
    type Output = VariableToken<ASSIGNMENT, SEPARATOR>;
    type Err = ParseVariableTokenErr;

    fn try_parse(code_lines_iterator: &mut Peekable<Iter<CodeLine>>) -> anyhow::Result<Self::Output, Self::Err> {
        let code_line = *code_lines_iterator.peek().ok_or(ParseVariableTokenErr::EmptyIterator(EmptyIteratorErr))?;
        VariableToken::try_parse(code_line)
    }
}

impl<const ASSIGNMENT: char, const SEPARATOR: char> VariableToken<ASSIGNMENT, SEPARATOR> {
    pub fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self, ParseVariableTokenErr> {
        let split_alloc = code_line.split(vec![' ', ';']);
        let split = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();


        let assignment = ASSIGNMENT.to_string();
        let separator = SEPARATOR.to_string();

        let let_used;
        let mut_used;

        let final_variable_name: &str;
        let assignable: AssignableToken;
        let type_token: Option<TypeToken>;

        match &split[..] {
            // [let] [mut] name[: i32] = 5;
            ["let", name, assignment_token, middle @ .., separator_token] if assignment_token == &assignment && separator_token == &separator => {
                final_variable_name = name;
                assignable = AssignableToken::from_str(middle.join(" ").as_str()).context(code_line.line.clone())?;
                // type token is not specified by the programmer, so it must be inferred
                type_token = assignable.infer_type();

                let_used = true;
                mut_used = false;
            }
            ["let", name, ":", type_str, assignment_token, middle @ .., separator_token] if assignment_token == &assignment && separator_token == &separator => {
                final_variable_name = name;
                assignable = AssignableToken::from_str(middle.join(" ").as_str()).context(code_line.line.clone())?;
                type_token = Some(TypeToken::from_str(type_str)?);

                let_used = true;
                mut_used = false;
            }
            ["let", "mut", name, assignment_token, middle @ .., separator_token] if assignment_token == &assignment && separator_token == &separator => {
                final_variable_name = name;
                assignable = AssignableToken::from_str(middle.join(" ").as_str()).context(code_line.line.clone())?;
                // type token is not specified by the programmer, so it must be inferred
                type_token = assignable.infer_type();

                let_used = true;
                mut_used = true;
            }
            ["let", "mut", name, ":", type_str, assignment_token, middle @ .., separator_token] if assignment_token == &assignment && separator_token == &separator => {
                final_variable_name = name;
                assignable = AssignableToken::from_str(middle.join(" ").as_str()).context(code_line.line.clone())?;
                type_token = Some(TypeToken::from_str(type_str)?);

                let_used = true;
                mut_used = true;
            }
            [name, assignment_token, middle @ .., separator_token] if assignment_token == &assignment && separator_token == &separator => {
                final_variable_name = name;
                assignable = AssignableToken::from_str(middle.join(" ").as_str()).context(code_line.line.clone())?;
                type_token = assignable.infer_type();

                let_used = false;
                mut_used = false;
            }
            _ => {
                return Err(ParseVariableTokenErr::PatternNotMatched { target_value: code_line.line.to_string() });
            }
        }

        Ok(VariableToken {
            name_token: NameToken::from_str(final_variable_name, false)?,
            mutability: mut_used,
            ty: type_token,
            define: let_used,
            assignable,
            code_line: code_line.clone(),
        })
    }

    pub fn infer_with_context(&self, context: &StaticTypeContext) -> Result<TypeToken, InferTypeError> {
        // context

        match &self.assignable {
            AssignableToken::MethodCallToken(method_call) => {
                if let Some((_, ty)) = context.iter().find(|(context_name, _)| {
                    context_name == &method_call.name
                }) {
                    return Ok(ty.clone());

                }
            }
            AssignableToken::Variable(variable) => {
                if let Some((_, ty)) = context.iter().rfind(|(context_name, _)| {
                    context_name == variable
                }) {
                    return Ok(ty.clone());
                }
            },
            AssignableToken::ArithmeticEquation(expression) => {
                return expression.traverse_type_resulted(&context)
            },
            AssignableToken::BooleanEquation(expression) => {
                return expression.traverse_type_resulted(&context)
            }
            a => unreachable!("{}", format!("The type {a} should have been inferred or directly parsed. Something went wrong"))
        }

        return Err(InferTypeError::TypeNotInferrable(self.assignable.to_string()))
    }
}


impl<const ASSIGNMENT: char, const SEPARATOR: char> PatternedLevenshteinDistance for VariableToken<ASSIGNMENT, SEPARATOR> {
    fn distance_from_code_line(code_line: &CodeLine) -> usize {
        let variable_pattern = PatternedLevenshteinString::default()
            .insert(PatternedLevenshteinString::ignore())
            .insert(&ASSIGNMENT.to_string())
            .insert(PatternedLevenshteinString::ignore())
            .insert(&SEPARATOR.to_string());

        <VariableToken<ASSIGNMENT, SEPARATOR> as PatternedLevenshteinDistance>::distance(
            PatternedLevenshteinString::match_to(
                &code_line.line,
                &variable_pattern,
                vec![Box::new(QuoteSummarizeTransform), Box::new(MethodCallSummarizeTransform)],
            ),
            variable_pattern,
        )
    }
}
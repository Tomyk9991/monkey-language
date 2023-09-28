use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;
use std::str::FromStr;

use anyhow::Context;

use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::generator::{Stack, StackLocation};
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::errors::EmptyIteratorErr;
use crate::core::lexer::levenshtein_distance::{MethodCallSummarizeTransform, PatternedLevenshteinDistance, PatternedLevenshteinString, QuoteSummarizeTransform};
use crate::core::lexer::tokenizer::StaticTypeContext;
use crate::core::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::core::lexer::tokens::name_token::{NameToken, NameTokenErr};
use crate::core::lexer::TryParse;
use crate::core::lexer::type_token::{InferTypeError, TypeToken};
use crate::core::type_checker::InferType;

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
    pub code_line: CodeLine,
}

impl<const ASSIGNMENT: char, const SEPARATOR: char> InferType for VariableToken<ASSIGNMENT, SEPARATOR> {
    fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<(), InferTypeError> {
        if type_context.methods.iter().filter(|a| a.name == self.name_token).count() > 0 {
            return Err(InferTypeError::NameCollision(self.name_token.name.clone(), self.code_line.clone()));
        }

        if !self.define {
            return Ok(());
        }

        match &self.ty {
            // validity check. is the assignment really the type the programmer used
            // example: let a: i32 = "Hallo"; is not valid since you're assigning a string to an integer

            // if type is present. check, if the type matches the assignment
            // else infer the type with a context
            Some(ty) => {
                let inferred_type = self.assignable.infer_type_with_context(type_context, &self.code_line)?;

                if ty != &inferred_type {
                    return Err(InferTypeError::MismatchedTypes { expected: ty.clone(), actual: inferred_type.clone(), code_line: self.code_line.clone() });
                }

                Ok(())
            }
            None => {
                let ty = self.infer_with_context(type_context, &self.code_line)?;
                self.ty = Some(ty.clone());
                type_context.push((self.name_token.clone(), ty.clone()));

                Ok(())
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
    fn to_asm(&self, stack: &mut Stack, meta: &mut MetaInfo) -> Result<String, crate::core::code_generator::ASMGenerateError> {
        let mut target = String::new();

        let mov_target = if self.define {
            let byte_size = self.assignable.byte_size(meta);

            stack.variables.push(StackLocation { position: stack.stack_position, name: self.name_token.clone(), size: byte_size });
            stack.stack_position += byte_size;

            let offset = stack.stack_position;
            format!("{}WORD [rbp - {}]", match byte_size {
                4 => "D",
                _ => "Q"
            }, offset)
        } else {
            self.name_token.to_asm(stack, meta)?
        };

        target += &ASMBuilder::ident(&ASMBuilder::comment_line(&format!("{}", self)));

        if self.define && !self.assignable.is_stack_look_up(stack, meta) {
            target += &ASMBuilder::ident_line(&format!("mov {}, {}", mov_target, self.assignable.to_asm(stack, meta)?));
        } else {
            match &self.assignable {
                AssignableToken::ArithmeticEquation(eq) => {
                    target += &eq.to_asm(stack, meta)?.to_string();
                }
                AssignableToken::BooleanEquation(eq) => {
                    target += &eq.to_asm(stack, meta)?.to_string();
                }
                _ => {
                    target += &ASMBuilder::ident_line(&format!("mov eax, {}", self.assignable.to_asm(stack, meta)?));
                }
            }

            target += &ASMBuilder::ident_line(&format!("mov {}, eax", mov_target));
        }


        Ok(target)
    }

    fn is_stack_look_up(&self, stack: &mut Stack, meta: &MetaInfo) -> bool {
        self.assignable.is_stack_look_up(stack, meta)
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        if let Some(ty) = &self.ty {
            return ty.byte_size()
        } else {
            0
        }
    }

    fn before_label(&self, stack: &mut Stack, meta: &mut MetaInfo) -> Option<Result<String, ASMGenerateError>> {
        self.assignable.before_label(stack, meta)
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
                type_token = assignable.infer_type(code_line);

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
                type_token = assignable.infer_type(code_line);

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
                type_token = assignable.infer_type(code_line);

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

    pub fn infer_with_context(&self, context: &StaticTypeContext, code_line: &CodeLine) -> Result<TypeToken, InferTypeError> {
        // context

        match &self.assignable {
            AssignableToken::MethodCallToken(method_call) => {
                if let Some((_, ty)) = context.iter().find(|(context_name, _)| {
                    context_name == &method_call.name
                }) {
                    return Ok(ty.clone());
                }
            }
            AssignableToken::NameToken(variable) => {
                if let Some((_, ty)) = context.iter().rfind(|(context_name, _)| {
                    context_name == variable
                }) {
                    return Ok(ty.clone());
                }
            }
            AssignableToken::ArithmeticEquation(expression) => {
                return expression.traverse_type_resulted(context, code_line);
            }
            AssignableToken::BooleanEquation(expression) => {
                return expression.traverse_type_resulted(context, code_line);
            }
            a => unreachable!("{}", format!("The type {a} should have been inferred or directly parsed. Something went wrong"))
        }

        Err(InferTypeError::UnresolvedReference(self.assignable.to_string(), self.code_line.clone()))
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
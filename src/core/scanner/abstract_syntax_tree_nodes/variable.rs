use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use anyhow::Context;

use crate::core::code_generator::{ASMGenerateError, MetaInfo, register_destination, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_options::identifier_present::IdentifierPresent;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::asm_result::{ASMResult, ASMResultError, ASMResultVariance};
use crate::core::code_generator::generator::{Stack, StackLocation};
use crate::core::code_generator::registers::{Bit64, ByteSize, GeneralPurposeRegister};
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token_match::{Match, MatchResult};
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::model::abstract_syntax_tree_nodes::identifier::IdentifierError;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::types::mutability::Mutability;
use crate::core::model::types::ty::Type;
use crate::core::scanner::errors::EmptyIteratorErr;
use crate::core::scanner::scope::PatternNotMatchedError;
use crate::core::scanner::static_type_context::StaticTypeContext;
use crate::core::scanner::{Lines, TryParse};
use crate::core::scanner::abstract_syntax_tree_nodes::l_value::LValueErr;
use crate::core::scanner::types::r#type::{InferTypeError};
use crate::core::semantics::type_checker::{InferType, StaticTypeCheck};
use crate::core::semantics::type_checker::static_type_checker::StaticTypeCheckError;
use crate::pattern;

impl<const ASSIGNMENT: char, const SEPARATOR: char> Parse for Variable<ASSIGNMENT, SEPARATOR> {
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, crate::core::lexer::error::Error> where Self: Sized, Self: Default {
        if let Some(MatchResult::Parse(l_value)) = pattern!(tokens, Let, @parse LValue, Equals) {
            if let Some(MatchResult::Parse(assign)) = pattern!(&tokens[l_value.consumed + 2..], @parse Assignable, SemiColon) {
                return Ok(ParseResult {
                    result: Variable {
                        l_value: l_value.result,
                        mutability: false,
                        ty: None,
                        define: true,
                        assignable: assign.result,
                        code_line: FilePosition::from_min_max(&tokens[0], &tokens[l_value.consumed + assign.consumed + 2]),
                    },
                    consumed: l_value.consumed + assign.consumed + 3,
                });
            }
        }

        if let Some(MatchResult::Parse(l_value)) = pattern!(tokens, Let, Mut, @parse LValue, Equals) {
            if let Some(MatchResult::Parse(assign)) = pattern!(&tokens[l_value.consumed + 3..], @parse Assignable, SemiColon) {
                return Ok(ParseResult {
                    result: Variable {
                        l_value: l_value.result,
                        mutability: true,
                        ty: None,
                        define: true,
                        assignable: assign.result,
                        code_line: FilePosition::from_min_max(&tokens[0], &tokens[l_value.consumed + assign.consumed + 3]),
                    },
                    consumed: l_value.consumed + assign.consumed + 4,
                });
            }
        }

        if let Some(MatchResult::Parse(l_value)) = pattern!(tokens, Let, @parse LValue, Colon) {
            if let Some(MatchResult::Parse(ty)) = pattern!(&tokens[l_value.consumed + 2..], @parse Type, Equals) {
                if let Some(MatchResult::Parse(assign)) = pattern!(&tokens[l_value.consumed + ty.consumed + 3..], @parse Assignable, SemiColon) {
                    return Ok(ParseResult {
                        result: Variable {
                            l_value: l_value.result,
                            mutability: false,
                            ty: Some(ty.result),
                            define: true,
                            assignable: assign.result,
                            code_line: FilePosition::from_min_max(&tokens[0], &tokens[l_value.consumed + ty.consumed + assign.consumed + 3]),
                        },
                        consumed: l_value.consumed + ty.consumed + assign.consumed + 4,
                    });
                }
            }
        }

        if let Some(MatchResult::Parse(l_value)) = pattern!(tokens, Let, Mut, @parse LValue, Colon) {
            if let Some(MatchResult::Parse(ty)) = pattern!(&tokens[l_value.consumed + 3..], @parse Type, Equals) {
                if let Some(MatchResult::Parse(assign)) = pattern!(&tokens[l_value.consumed + ty.consumed + 4..], @parse Assignable, SemiColon) {
                    return Ok(ParseResult {
                        result: Variable {
                            l_value: l_value.result,
                            mutability: true,
                            ty: Some(ty.result),
                            define: true,
                            assignable: assign.result,
                            code_line: FilePosition::from_min_max(&tokens[0], &tokens[l_value.consumed + ty.consumed + assign.consumed + 4]),
                        },
                        consumed: l_value.consumed + ty.consumed + assign.consumed + 5,
                    });
                }
            }
        }


        Err(crate::core::lexer::error::Error::UnexpectedToken(tokens[0].clone()))
    }
}

impl<const ASSIGNMENT: char, const SEPARATOR: char> TryFrom<Result<ParseResult<Self>, crate::core::lexer::error::Error>> for Variable<ASSIGNMENT, SEPARATOR> {
    type Error = crate::core::lexer::error::Error;

    fn try_from(value: Result<ParseResult<Self>, crate::core::lexer::error::Error>) -> Result<Self, Self::Error> {
        match value {
            Ok(value) => Ok(value.result),
            Err(e) => Err(e),
        }
    }
}


impl<const ASSIGNMENT: char, const SEPARATOR: char> InferType for Variable<ASSIGNMENT, SEPARATOR> {
    fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<(), InferTypeError> {
        if let LValue::Identifier(l_value) = &self.l_value {
            if type_context.methods.iter().filter(|a| a.identifier == *l_value).count() > 0 {
                return Err(InferTypeError::NameCollision(l_value.name.clone(), CodeLine::default()));
            }
        }

        if !self.define {
            return Ok(());
        }

        let line = CodeLine::default();
        match &self.ty {
            // validity check. is the assignment really the type the programmer used
            // example: let a: i32 = "Hallo"; is not valid since you're assigning a string to an integer

            // if type is present. check, if the type matches the assignment
            // else infer the type with a context
            Some(ty) => {
                let inferred_type = self.assignable.infer_type_with_context(type_context, &line/*&self.code_line*/)?;

                if ty < &inferred_type {
                    // let a: i64 = 5; instead of let a: i32 = 5;
                    if let Some(implicit_cast) = inferred_type.implicit_cast_to(&mut self.assignable, ty, &line)? {
                        self.ty = Some(implicit_cast);
                    } else {
                        return Err(InferTypeError::MismatchedTypes { expected: ty.clone(), actual: inferred_type.clone(), code_line: line });
                    }
                }

                Ok(())
            }
            None => {
                let ty = self.infer_with_context(type_context, &line)?;
                self.ty = Some(ty.clone());
                type_context.push(Variable {
                    l_value: self.l_value.clone(),
                    ty: Some(ty.clone()),
                    define: self.define,
                    assignable: self.assignable.clone(),
                    mutability: self.mutability,
                    code_line: self.code_line.clone(),
                });

                Ok(())
            }
        }
    }
}

impl StaticTypeCheck for Variable<'=', ';'> {
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        let line = CodeLine::default();
        if self.define {
            if let Assignable::Array(array) = &self.assignable {
                // check if all types are equal, where the first type is the expected type
                let all_types = array.values
                    .iter()
                    .map(|a| a.infer_type_with_context(type_context, &line/*&self.code_line.clone()*/))
                    .collect::<Vec<Result<Type, InferTypeError>>>();

                if !all_types.is_empty() {
                    let first_type = &all_types[0];
                    if let Ok(first_type) = first_type {
                        for (index, current_type) in all_types.iter().enumerate() {
                            if let Ok(current_type) = current_type {
                                if current_type != first_type {
                                    return Err(StaticTypeCheckError::InferredError(InferTypeError::MultipleTypesInArray {
                                        expected: first_type.clone(),
                                        unexpected_type: current_type.clone(),
                                        unexpected_type_index: index,
                                        code_line: Default::default(),
                                    }))
                                }
                            }
                        }
                    }
                }
            }

            let ty = self.assignable.infer_type_with_context(type_context, &line/*&self.code_line*/)?;
            if matches!(ty, Type::Void) {
                return Err(StaticTypeCheckError::VoidType { assignable: self.assignable.clone(), code_line: line/*self.code_line.clone()*/ });
            }


            if self.ty.is_some() {
                type_context.context.push(self.clone());
                return Ok(());
            }
        }

        if !self.define {
            if let Some(found_variable) = type_context.iter().rfind(|v| v.l_value.identifier() == self.l_value.identifier()) {
                let inferred_type = self.assignable.infer_type_with_context(type_context, &line/*&self.code_line*/)?;
                if let Some(ty) = &found_variable.ty {

                    if ty > &inferred_type {
                        return Err(InferTypeError::MismatchedTypes { expected: ty.clone(), actual: inferred_type.clone(), code_line: line/*self.code_line.clone()*/ }.into());
                    }

                    if !found_variable.mutability {
                        return Err(StaticTypeCheckError::ImmutabilityViolated {
                            name: self.l_value.clone(),
                            code_line: line/*self.code_line.clone()*/,
                        });
                    }
                } else {
                    return Err(StaticTypeCheckError::NoTypePresent { name: self.l_value.clone(), code_line: line/*self.code_line.clone()*/ });
                }
            } else {
                return Err(StaticTypeCheckError::UnresolvedReference { name: self.l_value.clone(), code_line: line/*self.code_line.clone()*/ });
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum ParseVariableErr {
    PatternNotMatched { target_value: String },
    IdentifierErr(IdentifierError),
    AssignableErr(AssignableError),
    LValue(LValueErr),
    InferType(InferTypeError),
    EmptyIterator(EmptyIteratorErr),
}

impl PatternNotMatchedError for ParseVariableErr {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, ParseVariableErr::PatternNotMatched {..})
    }
}

impl Error for ParseVariableErr {}

impl From<InferTypeError> for ParseVariableErr {
    fn from(value: InferTypeError) -> Self {
        ParseVariableErr::InferType(value)
    }
}

impl From<LValueErr> for ParseVariableErr {
    fn from(value: LValueErr) -> Self {
        ParseVariableErr::LValue(value)
    }
}

impl From<IdentifierError> for ParseVariableErr {
    fn from(a: IdentifierError) -> Self { ParseVariableErr::IdentifierErr(a) }
}

impl From<anyhow::Error> for ParseVariableErr {
    fn from(value: anyhow::Error) -> Self {
        let mut buffer = String::new();
        buffer += &value.to_string();
        buffer += "\n";

        if let Some(e) = value.downcast_ref::<AssignableError>() {
            buffer += &e.to_string();
        }
        ParseVariableErr::PatternNotMatched { target_value: buffer }
    }
}

impl From<AssignableError> for ParseVariableErr {
    fn from(a: AssignableError) -> Self { ParseVariableErr::AssignableErr(a) }
}

impl Display for ParseVariableErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ParseVariableErr::PatternNotMatched { target_value } => format!("`{target_value}`\n\tThe pattern for a variable is defined as: lvalue = assignment;"),
            ParseVariableErr::IdentifierErr(a) => a.to_string(),
            ParseVariableErr::AssignableErr(a) => a.to_string(),
            ParseVariableErr::EmptyIterator(e) => e.to_string(),
            ParseVariableErr::InferType(err) => err.to_string(),
            ParseVariableErr::LValue(err) => err.to_string(),
        })
    }
}

impl<const ASSIGNMENT: char, const SEPARATOR: char> TryParse for Variable<ASSIGNMENT, SEPARATOR> {
    type Output = Variable<ASSIGNMENT, SEPARATOR>;
    type Err = ParseVariableErr;

    fn try_parse(code_lines_iterator: &mut Lines<'_>) -> anyhow::Result<Self::Output, Self::Err> {
        let code_line = *code_lines_iterator.peek().ok_or(ParseVariableErr::EmptyIterator(EmptyIteratorErr))?;
        Variable::try_parse(code_line)
    }
}

impl<const ASSIGNMENT: char, const SEPARATOR: char> Variable<ASSIGNMENT, SEPARATOR> {
    pub fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self, ParseVariableErr> {
        let split_alloc = code_line.split(vec![' ', ';']);
        let split = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();


        let assignment = ASSIGNMENT.to_string();
        let separator = SEPARATOR.to_string();

        let binding = process_name_collapse(&split, &assignment);
        let split: Vec<&str> = binding.iter().map(|a| a.as_str()).collect();

        let let_used;
        let mut_used;

        let final_variable_name: &str;
        let assignable: Assignable;
        let ty: Option<Type>;

        match &split[..] {
            // [let] [mut] name[: i32] = 5;
            ["let", name, assignment_str, middle @ .., separator_str] if assignment_str == &assignment && separator_str == &separator => {
                final_variable_name = name;
                assignable = Assignable::from_str(middle.join(" ").as_str()).context(code_line.line.clone())?;
                // type is not specified by the programmer, so it must be inferred
                ty = assignable.infer_type(code_line);

                let_used = true;
                mut_used = false;
            }
            ["let", name, ":", type_str, assignment_str, middle @ .., separator_str] if assignment_str == &assignment && separator_str == &separator => {
                final_variable_name = name;
                assignable = Assignable::from_str(middle.join(" ").as_str()).context(code_line.line.clone())?;
                ty = Some(Type::from_str(type_str, Mutability::Immutable)?);

                let_used = true;
                mut_used = false;
            }
            ["let", name, ":", "[", type_str, ",", type_size, "]", assignment_str, middle @ .., separator_str] if assignment_str == &assignment && separator_str == &separator => {
                final_variable_name = name;
                assignable = Assignable::from_str(middle.join(" ").as_str()).context(code_line.line.clone())?;
                ty = Some(Type::from_str(&format!("[ {} , {} ]", type_str, type_size), Mutability::Immutable)?);

                let_used = true;
                mut_used = false;
            }
            ["let", "mut", name, assignment_str, middle @ .., separator_str] if assignment_str == &assignment && separator_str == &separator => {
                final_variable_name = name;
                assignable = Assignable::from_str(middle.join(" ").as_str()).context(code_line.line.clone())?;
                // type is not specified by the programmer, so it must be inferred
                ty = assignable.infer_type(code_line);

                let_used = true;
                mut_used = true;
            }
            ["let", "mut", name, ":", type_str, assignment_str, middle @ .., separator_str] if assignment_str == &assignment && separator_str == &separator => {
                final_variable_name = name;
                assignable = Assignable::from_str(middle.join(" ").as_str()).context(code_line.line.clone())?;
                ty = Some(Type::from_str(type_str, Mutability::Mutable)?);

                let_used = true;
                mut_used = true;
            }
            ["let", "mut", name, ":", "[", type_str, ",", type_size, "]", assignment_str, middle @ .., separator_str] if assignment_str == &assignment && separator_str == &separator => {
                final_variable_name = name;
                assignable = Assignable::from_str(middle.join(" ").as_str()).context(code_line.line.clone())?;
                ty = Some(Type::from_str(&format!("[ {} , {} ]", type_str, type_size), Mutability::Mutable)?);

                let_used = true;
                mut_used = true;
            }
            [name , assignment_str, middle @ .., separator_str] if assignment_str == &assignment && separator_str == &separator => {
                final_variable_name = name;
                assignable = Assignable::from_str(middle.join(" ").as_str()).context(code_line.line.clone())?;
                ty = assignable.infer_type(code_line);

                let_used = false;
                mut_used = false;
            }
            _ => {
                return Err(ParseVariableErr::PatternNotMatched { target_value: code_line.line.to_string() });
            }
        }

        Ok(Variable {
            l_value: LValue::from_str(final_variable_name)?,
            mutability: mut_used,
            ty,
            define: let_used,
            assignable,
            code_line: FilePosition::default()/*code_line.clone()*/,
        })
    }

    pub fn infer_with_context(&self, context: &mut StaticTypeContext, code_line: &CodeLine) -> Result<Type, InferTypeError> {
        match &self.assignable {
            Assignable::MethodCall(method_call) => {
                if let Some(method_def) = context.methods.iter().find(|method_def| {
                    method_def.identifier == method_call.identifier
                }) {
                    return Ok(method_def.return_type.clone());
                }
            }
            Assignable::Identifier(variable) => {
                if let Some(v) = context.iter().rfind(|v| {
                    v.l_value == LValue::Identifier(variable.clone())
                }) {
                    return if let Some(ty) = &v.ty {
                        Ok(ty.clone())
                    } else {
                        Err(InferTypeError::NoTypePresent(v.l_value.clone(), CodeLine::default()/*self.code_line.clone()*/))
                    };
                }
            }
            Assignable::ArithmeticEquation(expression) => {
                return expression.traverse_type_resulted(context, code_line);
            }
            a => unreachable!("{}", format!("The type {a} should have been inferred or directly parsed. Something went wrong"))
        }

        Err(InferTypeError::UnresolvedReference(self.assignable.to_string(), CodeLine::default()/*self.code_line.clone()*/))
    }
}

// trys to collapse everything that can belong to the l_value
fn process_name_collapse(regex_split: &[&str], assignment_str: &str) -> Vec<String> {
    if let Some(assignment_index) = regex_split.iter().position(|a| a == &assignment_str) {
        let (l_value, right_value) = regex_split.split_at(assignment_index);
        #[allow(clippy::redundant_slicing)] // slicing must happen, otherwise middle is not a slice with a length known at compile time
        let l_value = match &l_value[..] {
            [name, "[", middle@ .., "]"] => {
                let mut result = name.to_string();
                result.push_str(" [ ");
                result.extend(middle.iter().map(|a| a.to_string()));
                result.push_str(" ]");
                result
            },
            _ => return regex_split.iter().map(|a| a.to_string()).collect(),
        };

        let mut resulting_vec = vec![l_value, assignment_str.to_string()];
        resulting_vec.extend(right_value.iter().skip(1).map(|a| a.to_string()));
        return resulting_vec;
    }

    regex_split.iter().map(|a| a.to_string()).collect()
}

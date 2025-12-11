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
use crate::core::lexer::parse::{Parse};
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::model::abstract_syntax_tree_nodes::identifier::IdentifierError;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::abstract_syntax_tree_nodes::variable::{ParseVariableErr, Variable};
use crate::core::model::types::mutability::Mutability;
use crate::core::model::types::ty::Type;
use crate::core::scanner::errors::EmptyIteratorErr;
use crate::core::scanner::scope::PatternNotMatchedError;
use crate::core::scanner::static_type_context::StaticTypeContext;
use crate::core::scanner::{Lines, TryParse};
use crate::core::scanner::abstract_syntax_tree_nodes::l_value::LValueErr;
use crate::core::scanner::types::r#type::{InferTypeError};
use crate::core::semantics::type_checker::{InferType, StaticTypeCheck};



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


impl<const ASSIGNMENT: char, const SEPARATOR: char> ToASM for Variable<ASSIGNMENT, SEPARATOR> {
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        let mut target = String::new();
        target += &ASMBuilder::ident(&ASMBuilder::comment_line(&format!("{}", self)));

        let result = match &self.assignable {
            Assignable::Array(_) => {
                let i = IdentifierPresent {
                    identifier: self.l_value.clone(),
                };
                self.assignable.to_asm(stack, meta, (!self.define).then_some(i))?
            },
            _ => {
                let interim_options = Some(InterimResultOption {
                    general_purpose_register: GeneralPurposeRegister::iter_from_byte_size(self.assignable.byte_size(meta))?.current().clone(),
                });
                self.assignable.to_asm(stack, meta, interim_options)?
            },
        };

        let destination = if self.define {
            let byte_size = self.assignable.byte_size(meta);

            let elements = match &self.assignable {
                Assignable::Array(array) if array.values.len() > 1 => array.values.len(),
                _ => 1
            };

            stack.variables.push(StackLocation { position: stack.stack_position, size: byte_size, name: self.l_value.clone(), elements });

            stack.stack_position += byte_size;

            let offset = stack.stack_position;
            if !matches!(result, ASMResult::Multiline(_)) {
                format!("{} [rbp - {}]", register_destination::word_from_byte_size(byte_size), offset)
            } else {
                String::new()
            }
        } else {
            stack.register_to_use.push(GeneralPurposeRegister::Bit64(Bit64::Rdx));
            let result = match self.l_value.to_asm(stack, meta, options)? {
                ASMResult::Inline(r) => r,
                ASMResult::MultilineResulted(t, r) => {
                    target += &t;
                    r.to_string()
                }
                ASMResult::Multiline(_) => {
                    return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                        expected: vec![ASMResultVariance::MultilineResulted, ASMResultVariance::Inline],
                        actual: ASMResultVariance::Multiline,
                        ast_node: "variable node".to_string(),
                    }))
                }
            };

            stack.register_to_use.pop();

            result
        };

        match result {
            ASMResult::Inline(source) => {
                if self.assignable.is_stack_look_up(stack, meta) {
                    let destination_register = GeneralPurposeRegister::iter_from_byte_size(self.assignable.byte_size(meta))?.current();
                    target += &ASMBuilder::mov_x_ident_line(&destination_register, source, Some(destination_register.size() as usize));
                    target += &ASMBuilder::mov_ident_line(destination, &destination_register);
                } else {
                    target += &ASMBuilder::mov_ident_line(destination, source);
                }
            },
            ASMResult::MultilineResulted(source, mut register) => {
                target += &source;

                if let Assignable::ArithmeticEquation(expr) = &self.assignable {
                    let final_type = expr.traverse_type(meta).ok_or(ASMGenerateError::InternalError("Cannot infer type".to_string()))?;
                    let r = GeneralPurposeRegister::Bit64(Bit64::Rax).to_size_register(&ByteSize::try_from(final_type.byte_size())?);

                    if let GeneralPurposeRegister::Memory(memory) = &register {
                        target += &ASMBuilder::mov_x_ident_line(&r, memory, None);
                        register = r.clone();
                    }


                    if let Type::Float(s, _) = final_type {
                        target += &ASMBuilder::mov_x_ident_line(&r, register, Some(s.byte_size()));
                        register = r;
                    }
                }

                target += &ASMBuilder::mov_ident_line(destination, register);
            },
            ASMResult::Multiline(source) => {
                target += &source;
            }
        }

        Ok(ASMResult::Multiline(target))
    }


    fn is_stack_look_up(&self, stack: &mut Stack, meta: &MetaInfo) -> bool {
        self.assignable.is_stack_look_up(stack, meta)
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        self.ty.as_ref().map_or(0, |ty| ty.byte_size())
    }

    fn data_section(&self, stack: &mut Stack, meta: &mut MetaInfo) -> bool {
        self.assignable.data_section(stack, meta)
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

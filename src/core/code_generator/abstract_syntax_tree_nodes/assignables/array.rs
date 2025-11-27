use std::any::Any;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::core::code_generator::asm_result::{ASMResult};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, register_destination, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_options::identifier_present::IdentifierPresent;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::registers::{Bit64, ByteSize, GeneralPurposeRegister};
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::collect_tokens_until_scope_close::CollectTokensFromUntil;
use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::types::array::{Array, ArrayErr};
use crate::core::model::types::mutability::Mutability;
use crate::core::model::types::ty::Type;
use crate::core::scanner::static_type_context::StaticTypeContext;
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::method_call::{dyck_language, dyck_language_generic};
use crate::core::scanner::types::r#type::{InferTypeError};
use crate::pattern;

fn contains(a: &[TokenWithSpan], b: &TokenWithSpan) -> bool {
    a.iter().any(|x| x.token == b.token)
}


impl ToASM for Array {
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        let mut target = String::new();
        target += &ASMBuilder::ident(&ASMBuilder::comment_line(&format!("{}", self)));

        let initial_position = match options {
            Some(options) => {
                let any_t = &options as &dyn Any;
                if let Some(concrete_type) = any_t.downcast_ref::<IdentifierPresent>() {
                    let stack_variable = stack.variables.iter().rfind(|v| v.name.name == concrete_type.identifier.name).ok_or(ASMGenerateError::InternalError("Cannot find variable".to_string()))?;
                    stack_variable.position
                } else {
                    stack.stack_position
                }
            }
            None => {
                stack.stack_position
            }
        };

        let mut offset = if let [first, ..] = &self.values[..] {
            initial_position + first.byte_size(meta) * self.values.len()
        } else {
            initial_position
        };

        for assignable in self.values.iter() {
            let first_register = GeneralPurposeRegister::iter_from_byte_size(assignable.byte_size(meta))?.current();
            let result = assignable.to_asm(stack, meta, Some(InterimResultOption {
                general_purpose_register: first_register.clone(),
            }))?;

            let byte_size = assignable.byte_size(meta);
            let destination = format!("{} [rbp - {}]", register_destination::word_from_byte_size(byte_size), offset);

            match result {
                ASMResult::Inline(source) => {
                    if assignable.is_stack_look_up(stack, meta) {
                        target += &ASMBuilder::mov_x_ident_line(&first_register, source, Some(first_register.size() as usize));
                        target += &ASMBuilder::mov_ident_line(destination, &first_register);
                    } else {
                        target += &ASMBuilder::mov_ident_line(destination, source);
                    }
                }
                ASMResult::MultilineResulted(source, mut register) => {
                    target += &source;

                    if let Assignable::ArithmeticEquation(expr) = assignable {
                        let final_type = expr.traverse_type(meta).ok_or(ASMGenerateError::InternalError("Cannot infer type".to_string()))?;
                        let r = GeneralPurposeRegister::Bit64(Bit64::Rax).to_size_register(&ByteSize::try_from(final_type.byte_size())?);

                        if let Type::Float(s, _) = final_type {
                            target += &ASMBuilder::mov_x_ident_line(&r, register, Some(s.byte_size()));
                            register = r;
                        }
                    }

                    target += &ASMBuilder::mov_ident_line(destination, register);
                }
                ASMResult::Multiline(source) => {
                    target += &source;
                }
            }

            offset -= byte_size;
        }

        Ok(ASMResult::Multiline(target))
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, meta: &mut MetaInfo) -> usize {
        self.values.iter().map(|a| a.byte_size(meta)).sum::<usize>()
    }

    fn data_section(&self, stack: &mut Stack, meta: &mut MetaInfo) -> bool {
        let mut has_before_label_asm = false;
        let count_before = stack.label_count;

        for value in &self.values {
            if value.data_section(stack, meta) {
                has_before_label_asm = true;
                stack.label_count -= 1;
            }
        }

        stack.label_count = count_before;
        has_before_label_asm
    }
}
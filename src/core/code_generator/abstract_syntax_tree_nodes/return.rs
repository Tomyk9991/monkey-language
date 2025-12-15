use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::asm_result::{ASMResult, ASMResultError, ASMResultVariance};
use crate::core::code_generator::conventions::return_calling_convention;
use crate::core::code_generator::registers::{ByteSize, GeneralPurposeRegister};
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::model::abstract_syntax_tree_nodes::ret::{Return, ReturnError};
use crate::core::model::types::integer::{IntegerAST, IntegerType};
use crate::core::parser::errors::EmptyIteratorErr;
use crate::core::parser::scope::PatternNotMatchedError;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::{InferTypeError};
use crate::core::semantics::static_type_check::static_type_checker::StaticTypeCheckError;
use crate::core::semantics::static_type_check::static_type_check::StaticTypeCheck;

impl PatternNotMatchedError for ReturnError {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, ReturnError::PatternNotMatched {..})
    }
}



impl From<AssignableError> for ReturnError {
    fn from(value: AssignableError) -> Self {
        ReturnError::AssignableError(value)
    }
}

impl From<anyhow::Error> for ReturnError {
    fn from(value: anyhow::Error) -> Self {
        ReturnError::PatternNotMatched { target_value: value.to_string() }
    }
}

impl ToASM for Return {
    fn to_asm<T: ASMOptions>(&self, stack: &mut Stack, meta: &mut MetaInfo, _options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        let mut target = String::new();
        target += &ASMBuilder::ident(&ASMBuilder::comment_line(&format!("{}", self)));

        if let Some(assignable) = &self.assignable {
            let destination_register = return_calling_convention(stack, meta)?.to_size_register_ignore_float(
                &ByteSize::try_from(meta.static_type_information.expected_return_type.as_ref().map_or(8, |t| t.return_type.byte_size()))?
            );
            let options = InterimResultOption {
                general_purpose_register: destination_register.clone(),
            };

            let source = assignable.to_asm(stack, meta, Some(options))?;

            match source {
                ASMResult::Inline(source) => target += &ASMBuilder::mov_ident_line(destination_register, source),
                ASMResult::MultilineResulted(source, r) => {
                    target += &source;

                    if let GeneralPurposeRegister::Float(f) = r {
                        target += &ASMBuilder::mov_x_ident_line(destination_register, f, Some(assignable.byte_size(meta)));
                    }
                }
                ASMResult::Multiline(_) => return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                    expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                    actual: ASMResultVariance::Multiline,
                    ast_node: "Return".to_string(),
                }))

            }
        }

        target += &ASMBuilder::ident_line("leave");
        target += &ASMBuilder::ident_line("ret");

        Ok(ASMResult::Multiline(target))
    }

    fn is_stack_look_up(&self, stack: &mut Stack, meta: &MetaInfo) -> bool {
        if let Some(assignable) = &self.assignable {
            return assignable.is_stack_look_up(stack, meta);
        }

        false
    }

    fn byte_size(&self, meta: &mut MetaInfo) -> usize {
        if let Some(assignable) = &self.assignable {
            return assignable.byte_size(meta)
        }

        0
    }

    fn data_section(&self, stack: &mut Stack, meta: &mut MetaInfo) -> bool {
        if let Some(assignable) = &self.assignable {
            assignable.data_section(stack, meta)
        } else {
            false
        }
    }
}
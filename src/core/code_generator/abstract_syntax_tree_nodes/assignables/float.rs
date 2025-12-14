use std::any::Any;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::asm_options::prepare_register::PrepareRegisterOption;
use crate::core::code_generator::asm_result::{ASMResult, ASMResultError};
use crate::core::code_generator::registers::{ByteSize};
use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::types::float::{FloatAST, FloatType};
use crate::core::parser::abstract_syntax_tree_nodes::assignables::integer::{NumberErr};



impl ToASM for FloatAST {
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        if let Some(options) = options {
            let any_t = &options as &dyn Any;
            if let Some(concrete_type) = any_t.downcast_ref::<InterimResultOption>() {
                let value_str = if !self.value.to_string().contains('.') {
                    format!("{}.0", self.value)
                } else {
                    self.value.to_string()
                };

                return match self.ty {
                    FloatType::Float32 => Ok(ASMResult::MultilineResulted(
                        ASMBuilder::mov_ident_line(concrete_type.general_purpose_register.to_size_register(&ByteSize::_4), format!("__?float32?__({})", value_str)), concrete_type.general_purpose_register.clone())
                    ),
                    FloatType::Float64 => Ok(ASMResult::MultilineResulted(
                        ASMBuilder::mov_ident_line(concrete_type.general_purpose_register.to_size_register(&ByteSize::_8), format!("__?float64?__({})", value_str)), concrete_type.general_purpose_register.clone())
                    )
                };
            }

            if let Some(s) = any_t.downcast_ref::<PrepareRegisterOption>() {
                return s.transform(stack, meta);
            }
        }

        Err(ASMGenerateError::ASMResult(ASMResultError::NoOptionProvided("float".to_string())))
    }


    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        match self.ty {
            FloatType::Float32 => 4,
            FloatType::Float64 => 8,
        }
    }
}
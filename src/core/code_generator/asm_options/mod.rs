pub mod in_expression_method_call;
pub mod interim_result;
pub mod prepare_register;
pub mod identifier_present;

use crate::core::code_generator::asm_result::ASMResult;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo};

pub trait ASMOptions: Clone {
    fn transform(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Result<ASMResult, ASMGenerateError> {
        Ok(ASMResult::Inline(String::new()))
    }
}
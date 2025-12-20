use std::fmt::{Display, Formatter};
use std::str::{FromStr, ParseBoolError};

use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::{ASMResult};
use crate::core::code_generator::generator::Stack;
use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::types::boolean::{Boolean, BooleanError};



impl From<ParseBoolError> for BooleanError {
    fn from(value: ParseBoolError) -> Self { BooleanError::ParseBoolError(value) }
}

impl Display for BooleanError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            BooleanError::UnmatchedRegex => "Boolean must match ^(?i:true|false)$".to_string(),
            BooleanError::ParseBoolError(err) => err.to_string()
        })
    }
}

impl std::error::Error for BooleanError {}

impl ToASM for Boolean {
    fn to_asm(&self, _stack: &mut Stack, _meta: &mut MetaInfo, _options: Option<ASMOptions>) -> Result<ASMResult, ASMGenerateError> {
        Ok(ASMResult::Inline((if self.value { "1" } else { "0" }).to_string()))
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        1
    }
}
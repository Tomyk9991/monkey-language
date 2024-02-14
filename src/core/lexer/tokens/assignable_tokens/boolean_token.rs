use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::{FromStr, ParseBoolError};

use crate::core::code_generator::{ASMGenerateError, ASMOptions, ASMResult, MetaInfo, ToASM};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::registers::GeneralPurposeRegister;

#[derive(Debug, PartialEq, Clone)]
pub struct BooleanToken {
    pub value: bool,
}

#[derive(Debug)]
pub enum BooleanTokenErr {
    UnmatchedRegex,
    ParseBoolError(ParseBoolError),
}

impl From<ParseBoolError> for BooleanTokenErr {
    fn from(value: ParseBoolError) -> Self { BooleanTokenErr::ParseBoolError(value) }
}

impl Display for BooleanTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            BooleanTokenErr::UnmatchedRegex => "Boolean must match ^(?i:true|false)$".to_string(),
            BooleanTokenErr::ParseBoolError(err) => err.to_string()
        })
    }
}

impl Error for BooleanTokenErr {}

impl Display for BooleanToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.value) }
}

impl ToASM for BooleanToken {
    fn to_asm(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Result<String, ASMGenerateError> {
        Ok((if self.value { "1" } else { "0" }).to_string())
    }

    fn to_asm_new<T: ASMOptions>(&self, _stack: &mut Stack, _meta: &mut MetaInfo, _options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        Ok(ASMResult::Inline((if self.value { "1" } else { "0" }).to_string()))
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        todo!()
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        1
    }

    fn before_label(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Option<Result<String, ASMGenerateError>> {
        None
    }

    fn multi_line_asm(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Result<(bool, String, Option<GeneralPurposeRegister>), ASMGenerateError> {
        Ok((false, String::new(), None))
    }
}

impl FromStr for BooleanToken {
    type Err = BooleanTokenErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !lazy_regex::regex_is_match!("^(?i:true|false)$", s) {
            return Err(BooleanTokenErr::UnmatchedRegex);
        }

        Ok(BooleanToken {
            value: s.parse::<bool>()?
        })
    }
}
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, ASMOptions, ASMResult, MetaInfo, ToASM};
use crate::core::code_generator::registers::GeneralPurposeRegister;
use crate::core::lexer::types::integer::Integer;


#[derive(Default, Debug, Eq, PartialEq, Clone)]
pub struct IntegerToken {
    // Must be stored as a string literal, because
    // you can have a bigger value than a i64. consider every number that's between i64::MAX and u64::MAX
    pub value: String,
    pub ty: Integer
}


impl Display for IntegerToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}


#[derive(Debug)]
pub enum NumberTokenErr {
    UnmatchedRegex,
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError)
}

impl From<ParseIntError> for NumberTokenErr {
    fn from(value: ParseIntError) -> Self {
        NumberTokenErr::ParseIntError(value)
    }
}

impl From<ParseFloatError> for NumberTokenErr {
    fn from(value: ParseFloatError) -> Self { NumberTokenErr::ParseFloatError(value) }
}

impl Error for NumberTokenErr { }

impl Display for NumberTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            NumberTokenErr::UnmatchedRegex => "Integer must match ^[+-]?\\d+$".to_string(),
            NumberTokenErr::ParseIntError(err) => err.to_string(),
            NumberTokenErr::ParseFloatError(err) => err.to_string()
        })
    }
}

impl FromStr for IntegerToken {
    type Err = NumberTokenErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !lazy_regex::regex_is_match!("^[+-]?\\d+$", s) {
            return Err(NumberTokenErr::UnmatchedRegex);
        }

        let value: i128 = s.parse::<i128>()?;

        let final_type = match value {
            -2_147_483_648..=2_147_483_647 => Integer::I32,
            -9_223_372_036_854_775_808..=9_223_372_036_854_775_808 => Integer::I64,
            _ => return Err(NumberTokenErr::UnmatchedRegex)
        };

        Ok(IntegerToken {
            value: value.to_string(),
            ty: final_type,
        })
    }
}

impl ToASM for IntegerToken {
    fn to_asm(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Result<String, ASMGenerateError> {
        Ok(self.value.to_string())
    }

    fn to_asm_new<T: ASMOptions>(&self, _stack: &mut Stack, _meta: &mut MetaInfo, _options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        Ok(ASMResult::Inline(self.value.to_string()))
    }


    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        match self.ty {
            Integer::I8 | Integer::U8 => 1,
            Integer::I16 | Integer::U16 => 2,
            Integer::I32 | Integer::U32 => 4,
            Integer::I64 | Integer::U64 => 8,
        }
    }

    fn before_label(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Option<Result<String, ASMGenerateError>> {
        None
    }

    fn multi_line_asm(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Result<(bool, String, Option<GeneralPurposeRegister>), ASMGenerateError> {
        Ok((false, String::new(), None))
    }
}
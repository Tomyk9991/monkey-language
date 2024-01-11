use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::lexer::tokens::assignable_tokens::integer_token::NumberTokenErr;
use crate::core::lexer::type_token::Float;

#[derive(Debug, PartialEq, Clone)]
pub struct FloatToken {
    // there is no need to use a string literal instead of a f64 like in the integer token, because
    // you cant have a float that's bigger than the biggest value of f64. but you can have a bigger value than a i64. consider every number that's between i64::MAX and u64::MAX
    pub value: f64,
    pub ty: Float
}

impl Display for FloatToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl ToASM for FloatToken {
    fn to_asm(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Result<String, ASMGenerateError> {
        todo!()
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        todo!()
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        match self.ty {
            Float::Float32 => 4,
            Float::Float64 => 8,
        }
    }

    fn before_label(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Option<Result<String, ASMGenerateError>> {
        None
    }
}


impl FromStr for FloatToken {
    type Err = NumberTokenErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !lazy_regex::regex_is_match!("^[+-]?(\\d+\\.\\d*|\\d*\\.\\d+)$", s) {
            return Err(NumberTokenErr::UnmatchedRegex);
        }

        let value = s.parse::<f64>()?;



        let final_type = if (-1.17549435e-38..=3.40282347e+38).contains(&value) {
            Float::Float32
        } else {
            Float::Float64
        };

        Ok(FloatToken {
            value: s.parse::<f64>()?,
            ty: final_type,
        })
    }
}
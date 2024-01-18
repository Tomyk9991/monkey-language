use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::{FromStr, ParseBoolError};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;
use crate::core::lexer::types::type_token::TypeToken;

#[derive(Debug, PartialEq, Clone)]
pub struct BooleanToken {
    pub value: bool
}

#[derive(Debug)]
pub enum BooleanTokenErr {
    UnmatchedRegex,
    ParseBoolError(ParseBoolError)
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Boolean {
    True,
    False
}

impl Boolean {
    pub fn operation_matrix() -> Vec<crate::core::lexer::types::type_token::OperatorMatrixRow> {
        vec![
            (TypeToken::Bool, Operator::Add, TypeToken::Bool, TypeToken::Bool),
            (TypeToken::Bool, Operator::Sub, TypeToken::Bool, TypeToken::Bool),
            (TypeToken::Bool, Operator::Mul, TypeToken::Bool, TypeToken::Bool),
            (TypeToken::Bool, Operator::Div, TypeToken::Bool, TypeToken::Bool),
        ]
    }
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

impl Error for BooleanTokenErr { }

impl Display for BooleanToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.value) }
}

impl ToASM for BooleanToken {
    fn to_asm(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Result<String, ASMGenerateError> {
        todo!()
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        todo!()
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        4
    }

    fn before_label(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Option<Result<String, ASMGenerateError>> {
        None
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
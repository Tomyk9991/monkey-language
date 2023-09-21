use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;
use crate::core::lexer::tokens::name_token::{NameToken, NameTokenErr};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TypeToken {
    I32,
    String,
    Bool,
    Void,
    F32,
    Custom(NameToken)
}

#[derive(Debug)]
pub enum InferTypeError {
    TypesNotCalculable(TypeToken, Operator, TypeToken),
    TypeNotInferred(NameTokenErr),
    TypeNotInferrable(String),
    NameCollision(String),
    MismatchedTypes {expected: TypeToken, actual: TypeToken }
}

impl Error for InferTypeError { }

impl From<NameTokenErr> for InferTypeError {
    fn from(value: NameTokenErr) -> Self {
        return InferTypeError::TypeNotInferred(value)
    }
}

impl Display for InferTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InferTypeError::TypesNotCalculable(a, o, b) => {
                write!(f, "Cannot {} between types {} and {}", o, a, b)
            }
            InferTypeError::TypeNotInferred(s) => write!(f, "Cannot infer type: {s}"),
            InferTypeError::TypeNotInferrable(s) => write!(f, "Cannot infer type: {s}"),
            InferTypeError::MismatchedTypes { expected, actual } => write!(f, "Mismatched types: Expected `{expected}` but found `{actual}`"),
            InferTypeError::NameCollision(name) => write!(f, "A variable and a method cannot have the same name: `{name}`")
        }
    }
}

impl Display for TypeToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TypeToken::I32 => "i32".to_string(),
            TypeToken::String => "string".to_string(),
            TypeToken::Bool => "bool".to_string(),
            TypeToken::Void => "void".to_string(),
            TypeToken::F32 => "f32".to_string(),
            TypeToken::Custom(name) => name.name.clone()
        })
    }
}

impl FromStr for TypeToken {
    type Err = InferTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "i32" =>    TypeToken::I32,
            "string" => TypeToken::String,
            "bool" =>   TypeToken::Bool,
            "void" =>   TypeToken::Void,
            "f32" =>    TypeToken::F32,
            fallback => TypeToken::Custom(NameToken::from_str(fallback, false)?)
        })
    }
}
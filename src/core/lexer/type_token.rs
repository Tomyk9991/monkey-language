use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TypeToken {
    I32,
    String,
    Bool,
    Void,
    F32,
    MethodCallTODO,
    VariableTODO,
    ObjectTODO,
}

#[derive(Debug)]
pub enum InferTypeError {
    TypesNotCalculable(TypeToken, Operator, TypeToken),
    TypeNotInferred(String)
}

impl Error for InferTypeError { }

impl Display for InferTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InferTypeError::TypesNotCalculable(a, o, b) => {
                write!(f, "Cannot {} between types {} and {}", o, a, b)
            }
            InferTypeError::TypeNotInferred(s) => write!(f, "Cannot infer type: {s}")
        }
    }
}

impl Display for TypeToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TypeToken::I32 => "i32",
            TypeToken::String => "string",
            TypeToken::Bool => "bool",
            TypeToken::Void => "void",
            TypeToken::F32 => "f32",
            TypeToken::MethodCallTODO => "methodCall",
            TypeToken::VariableTODO => "variable",
            TypeToken::ObjectTODO => "object"
        })
    }
}

impl FromStr for TypeToken {
    type Err = InferTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "i32" =>    Ok(TypeToken::I32),
            "string" => Ok(TypeToken::String),
            "bool" =>   Ok(TypeToken::Bool),
            "void" =>   Ok(TypeToken::Void),
            "f32" =>    Ok(TypeToken::F32),
            _ => Err(InferTypeError::TypeNotInferred(s.to_string()))
        }
    }
}
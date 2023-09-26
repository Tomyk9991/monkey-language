use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::core::io::code_line::CodeLine;
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
    TypesNotCalculable(TypeToken, Operator, TypeToken, CodeLine),
    UnresolvedReference(String, CodeLine),
    TypeNotAllowed(NameTokenErr),
    MethodCallArgumentAmountMismatch { expected: usize, actual: usize, code_line: CodeLine },
    MethodCallArgumentTypeMismatch { expected: TypeToken, actual: TypeToken, nth_parameter: usize, code_line: CodeLine },
    NameCollision(String, CodeLine),
    MismatchedTypes {expected: TypeToken, actual: TypeToken, code_line: CodeLine }
}

impl From<NameTokenErr> for InferTypeError {
    fn from(value: NameTokenErr) -> Self {
        InferTypeError::TypeNotAllowed(value)
    }
}

impl Error for InferTypeError { }

impl Display for InferTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InferTypeError::TypesNotCalculable(a, o, b, code_line) => write!(f, "Line: {}: \tCannot {} between types {} and {}", code_line.line, o, a, b),
            InferTypeError::UnresolvedReference(s, code_line) => write!(f, "Line: {:?}: \tUnresolved reference: {s}", code_line.actual_line_number),
            InferTypeError::MismatchedTypes { expected, actual, code_line } => write!(f, "Line: {:?}: \tMismatched types: Expected `{expected}` but found `{actual}`", code_line.actual_line_number),
            InferTypeError::NameCollision(name, code_line) => write!(f, "Line: {:?}: \tA variable and a method cannot have the same name: `{name}`", code_line.actual_line_number),
            InferTypeError::TypeNotAllowed(ty) => write!(f, "This type is not allowed due to: {}", ty.to_string()),
            InferTypeError::MethodCallArgumentAmountMismatch { expected, actual, code_line } => write!(f, "Line: {:?}: \tThe method expects {} parameter, but {} are provided", code_line.actual_line_number, expected, actual),
            InferTypeError::MethodCallArgumentTypeMismatch { expected, actual, nth_parameter, code_line } => write!(f, "Line: {:?}: \t The {}. argument should be of type: `{}` but `{}` is provided", code_line.actual_line_number, nth_parameter, expected, actual)
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
            custom => TypeToken::Custom(NameToken::from_str(custom, false)?)
        })
    }
}
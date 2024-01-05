use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::core::io::code_line::CodeLine;
use crate::core::lexer::tokens::assignable_token::AssignableToken;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;
use crate::core::lexer::tokens::name_token::{NameToken, NameTokenErr};

pub mod common {
    use crate::core::lexer::tokens::name_token::NameToken;
    use crate::core::lexer::type_token::TypeToken;

    #[allow(unused)]
    pub fn string() -> TypeToken { TypeToken::Custom(NameToken { name: "*string".to_string() })}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TypeToken {
    I32,
    Bool,
    Void,
    F32,
    Custom(NameToken),
}


#[derive(Debug)]
pub enum InferTypeError {
    TypesNotCalculable(TypeToken, Operator, TypeToken, CodeLine),
    UnresolvedReference(String, CodeLine),
    TypeNotAllowed(NameTokenErr),
    IllegalDereference(AssignableToken, CodeLine),
    NoTypePresent(NameToken, CodeLine),
    MethodCallArgumentAmountMismatch { expected: usize, actual: usize, code_line: CodeLine },
    MethodCallArgumentTypeMismatch { info: Box<MethodCallArgumentTypeMismatch> },
    NameCollision(String, CodeLine),
    MismatchedTypes { expected: TypeToken, actual: TypeToken, code_line: CodeLine },
}

#[derive(Debug)]
pub struct MethodCallArgumentTypeMismatch {
    pub expected: TypeToken,
    pub actual: TypeToken,
    pub nth_parameter: usize,
    pub code_line: CodeLine,
}

impl From<NameTokenErr> for InferTypeError {
    fn from(value: NameTokenErr) -> Self {
        InferTypeError::TypeNotAllowed(value)
    }
}

impl Error for InferTypeError {}

impl Display for InferTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InferTypeError::TypesNotCalculable(a, o, b, code_line) => write!(f, "Line: {}: \tCannot {} between types {} and {}", code_line.line, o, a, b),
            InferTypeError::UnresolvedReference(s, code_line) => write!(f, "Line: {:?}: \tUnresolved reference: {s}", code_line.actual_line_number),
            InferTypeError::MismatchedTypes { expected, actual, code_line } => write!(f, "Line: {:?}: \tMismatched types: Expected `{expected}` but found `{actual}`", code_line.actual_line_number),
            InferTypeError::NameCollision(name, code_line) => write!(f, "Line: {:?}: \tA variable and a method cannot have the same name: `{name}`", code_line.actual_line_number),
            InferTypeError::TypeNotAllowed(ty) => write!(f, "This type is not allowed due to: {}", ty),
            InferTypeError::MethodCallArgumentAmountMismatch { expected, actual, code_line } => write!(f, "Line: {:?}: \tThe method expects {} parameter, but {} are provided", code_line.actual_line_number, expected, actual),
            InferTypeError::MethodCallArgumentTypeMismatch { info } => write!(f, "Line: {:?}: \t The {}. argument should be of type: `{}` but `{}` is provided", info.code_line.actual_line_number, info.nth_parameter, info.expected, info.actual),
            InferTypeError::NoTypePresent(name, code_line) => write!(f, "Line: {:?}\tType not inferred: `{name}`", code_line.actual_line_number),
            InferTypeError::IllegalDereference(assignable, code_line) => write!(f, "Line: {:?}\tType cannot be dereferenced: {assignable}", code_line.actual_line_number)
        }
    }
}

impl Display for TypeToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TypeToken::I32 => "i32".to_string(),
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
            "i32" => TypeToken::I32,
            "bool" => TypeToken::Bool,
            "void" => TypeToken::Void,
            "f32" => TypeToken::F32,
            custom => {
                if !lazy_regex::regex_is_match!(r"^[\*&]*[a-zA-Z_$][a-zA-Z_$0-9]*[\*&]*$", s) {
                    return Err(InferTypeError::TypeNotAllowed(NameTokenErr::UnmatchedRegex { target_value: String::from(custom) }));
                }

                TypeToken::Custom(NameToken { name: custom.to_string() })
            }
        })
    }
}

impl TypeToken {
    /// removes * from type
    pub fn pop_pointer(&self) -> Option<TypeToken> {
        if let TypeToken::Custom(name_token) = self {
            if name_token.name.starts_with('*') {
                let new_name_token = name_token.name.replacen('*', "", 1);

                if let Ok(ty_token) = TypeToken::from_str(&new_name_token) {
                    return Some(ty_token);
                }
            }
        }

        None
    }

    pub fn is_pointer(&self) -> bool {
        if let TypeToken::Custom(name) = self {
            return name.name.starts_with('*');
        }

        false
    }

    /// adds * from type
    pub fn push_pointer(&self) -> Self {
        match self {
            TypeToken::I32 => TypeToken::Custom(NameToken { name: format!("*{}", TypeToken::I32) }),
            TypeToken::Bool => TypeToken::Custom(NameToken { name: format!("*{}", TypeToken::Bool) }),
            TypeToken::Void => TypeToken::Custom(NameToken { name: format!("*{}", TypeToken::Void) }),
            TypeToken::F32 => TypeToken::Custom(NameToken { name: format!("*{}", TypeToken::F32) }),
            TypeToken::Custom(custom) => TypeToken::Custom(NameToken { name: format!("*{}", custom) })
        }
    }

    pub fn byte_size(&self) -> usize {
        match self {
            TypeToken::I32 => 4,
            TypeToken::Bool => 4,
            TypeToken::Void => 0,
            TypeToken::F32 => 4,
            TypeToken::Custom(_) => 8 // todo: calculate custom data types recursively
        }
    }
}
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

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub enum Integer {
    I8,
    U8,
    I16,
    U16,
    #[default]
    I32,
    U32,
    I64,
    U64
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub enum Float {
    #[default]
    Float32,
    Float64
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TypeToken {
    Integer(Integer),
    Float(Float),
    Bool,
    Void,
    Custom(NameToken),
}


type OperatorMatrixRow = (TypeToken, Operator, TypeToken, TypeToken);

impl Integer {
    pub fn from_number_str<T: FromStr>(value: &str) -> Result<T, InferTypeError> {
        value.parse().map_err(|_| InferTypeError::TypeNotAllowed(NameTokenErr::UnmatchedRegex { target_value: String::from(value) }))
    }

    pub fn _signed(&self) -> bool {
        matches!(self, Integer::I8 | Integer::I16 | Integer::I32 | Integer::I64)
    }

    pub fn operation_matrix() -> Vec<OperatorMatrixRow> {
        let mut matrix = Vec::new();
        let types = [Integer::I8, Integer::U8, Integer::I16, Integer::U16, Integer::I32, Integer::U32, Integer::I64, Integer::U64];

        for ty in &types {
            matrix.push((TypeToken::Integer(ty.clone()), Operator::Add, TypeToken::Integer(ty.clone()), TypeToken::Integer(ty.clone())));
            matrix.push((TypeToken::Integer(ty.clone()), Operator::Sub, TypeToken::Integer(ty.clone()), TypeToken::Integer(ty.clone())));
            matrix.push((TypeToken::Integer(ty.clone()), Operator::Mul, TypeToken::Integer(ty.clone()), TypeToken::Integer(ty.clone())));
            matrix.push((TypeToken::Integer(ty.clone()), Operator::Div, TypeToken::Integer(ty.clone()), TypeToken::Integer(ty.clone())));
        }

        matrix
    }


    pub fn byte_size(&self) -> usize {
        match self {
            Integer::I8 => 1,
            Integer::U8 => 1,
            Integer::I16 => 2,
            Integer::U16 => 2,
            Integer::I32 => 4,
            Integer::U32 => 4,
            Integer::I64 => 8,
            Integer::U64 => 8,
        }
    }
}

impl Float {
    pub fn operation_matrix() -> Vec<OperatorMatrixRow> {
        let mut matrix = Vec::new();
        let types = [Float::Float32, Float::Float64];

        for ty in &types {
            matrix.push((TypeToken::Float(ty.clone()), Operator::Add, TypeToken::Float(ty.clone()), TypeToken::Float(ty.clone())));
            matrix.push((TypeToken::Float(ty.clone()), Operator::Sub, TypeToken::Float(ty.clone()), TypeToken::Float(ty.clone())));
            matrix.push((TypeToken::Float(ty.clone()), Operator::Mul, TypeToken::Float(ty.clone()), TypeToken::Float(ty.clone())));
            matrix.push((TypeToken::Float(ty.clone()), Operator::Div, TypeToken::Float(ty.clone()), TypeToken::Float(ty.clone())));
        }

        matrix
    }

    pub fn byte_size(&self) -> usize {
        match self {
            Float::Float32 => 4,
            Float::Float64 => 8,
        }
    }
}


impl FromStr for Integer {
    type Err = InferTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "i8" => Integer::I8,
            "u8" => Integer::U8,
            "i16" => Integer::I16,
            "u16" => Integer::U16,
            "i32" => Integer::I32,
            "u32" => Integer::U32,
            "i64" => Integer::I64,
            "u64" => Integer::U64,
            _ => return Err(InferTypeError::TypeNotAllowed(NameTokenErr::UnmatchedRegex { target_value: String::from(s) }))
        })
    }
}

impl FromStr for Float {
    type Err = InferTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "f32" => Float::Float32,
            "f64" => Float::Float64,
            _ => return Err(InferTypeError::TypeNotAllowed(NameTokenErr::UnmatchedRegex { target_value: String::from(s) }))
        })
    }
}


impl Display for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Integer::I8 => write!(f, "i8"),
            Integer::U8 => write!(f, "u8"),
            Integer::I16 => write!(f, "i16"),
            Integer::U16 => write!(f, "u16"),
            Integer::I32 => write!(f, "i32"),
            Integer::U32 => write!(f, "u32"),
            Integer::I64 => write!(f, "i64"),
            Integer::U64 => write!(f, "u64"),
        }
    }
}

impl Display for Float {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Float::Float32 => write!(f, "f32"),
            Float::Float64 => write!(f, "f64"),
        }
    }
}


#[derive(Debug)]
pub enum InferTypeError {
    TypesNotCalculable(TypeToken, Operator, TypeToken, CodeLine),
    UnresolvedReference(String, CodeLine),
    TypeNotAllowed(NameTokenErr),
    IllegalDereference(AssignableToken, CodeLine),
    NoTypePresent(NameToken, CodeLine),
    IntegerTooSmall { ty: TypeToken, literal: String ,code_line: CodeLine },
    FloatTooSmall { ty: TypeToken, float: f64, code_line: CodeLine },
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
            InferTypeError::TypesNotCalculable(a, o, b, code_line) => write!(f, "Line: {:?}: \tCannot {} between types {} and {}", code_line.actual_line_number, o, a, b),
            InferTypeError::UnresolvedReference(s, code_line) => write!(f, "Line: {:?}: \tUnresolved reference: {s}", code_line.actual_line_number),
            InferTypeError::MismatchedTypes { expected, actual, code_line } => write!(f, "Line: {:?}: \tMismatched types: Expected `{expected}` but found `{actual}`", code_line.actual_line_number),
            InferTypeError::NameCollision(name, code_line) => write!(f, "Line: {:?}: \tA variable and a method cannot have the same name: `{name}`", code_line.actual_line_number),
            InferTypeError::TypeNotAllowed(ty) => write!(f, "This type is not allowed due to: {}", ty),
            InferTypeError::MethodCallArgumentAmountMismatch { expected, actual, code_line } => write!(f, "Line: {:?}: \tThe method expects {} parameter, but {} are provided", code_line.actual_line_number, expected, actual),
            InferTypeError::MethodCallArgumentTypeMismatch { info } => write!(f, "Line: {:?}: \t The {}. argument should be of type: `{}` but `{}` is provided", info.code_line.actual_line_number, info.nth_parameter, info.expected, info.actual),
            InferTypeError::NoTypePresent(name, code_line) => write!(f, "Line: {:?}\tType not inferred: `{name}`", code_line.actual_line_number),
            InferTypeError::IllegalDereference(assignable, code_line) => write!(f, "Line: {:?}\tType cannot be dereferenced: {assignable}", code_line.actual_line_number),
            InferTypeError::IntegerTooSmall { ty, literal: integer, code_line } => write!(f, "Line: {:?}\t`{integer}` doesn't fit into the type `{ty}`", code_line.actual_line_number),
            InferTypeError::FloatTooSmall { ty, float, code_line } => write!(f, "Line: {:?}\t`{float}` doesn't fit into the type `{ty}`", code_line.actual_line_number),
        }
    }
}

impl Display for TypeToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TypeToken::Integer(int) => int.to_string(),
            TypeToken::Float(float) => float.to_string(),
            TypeToken::Bool => "bool".to_string(),
            TypeToken::Void => "void".to_string(),
            TypeToken::Custom(name) => name.name.clone()
        })
    }
}

impl FromStr for TypeToken {
    type Err = InferTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "bool" => TypeToken::Bool,
            "void" => TypeToken::Void,
            custom => {
                if let Ok(int) = Integer::from_str(custom) {
                    return Ok(TypeToken::Integer(int));
                }

                if let Ok(float) = Float::from_str(custom) {
                    return Ok(TypeToken::Float(float));
                }

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
            TypeToken::Integer(int) => TypeToken::Custom(NameToken { name: format!("*{}", int) }),
            TypeToken::Float(float) => TypeToken::Custom(NameToken { name: format!("*{}", float) }),
            TypeToken::Bool => TypeToken::Custom(NameToken { name: format!("*{}", TypeToken::Bool) }),
            TypeToken::Void => TypeToken::Custom(NameToken { name: format!("*{}", TypeToken::Void) }),
            TypeToken::Custom(custom) => TypeToken::Custom(NameToken { name: format!("*{}", custom) })
        }
    }

    pub fn implicit_cast_to(&self, assignable_token: &AssignableToken, desired_type: &TypeToken, code_line: &CodeLine) -> Result<Option<TypeToken>, InferTypeError> {
        match (self, desired_type) {
            (TypeToken::Integer(_), TypeToken::Integer(desired)) => {
                if let AssignableToken::IntegerToken(integer_token) = assignable_token {
                    match desired {
                        Integer::I8 if Self::in_range(-127_i8, 127_i8, Integer::from_number_str(&integer_token.value)) => { return Ok(Some(desired_type.clone())) },
                        Integer::U8 => if Self::in_range(0_u8, 255_u8, Integer::from_number_str(&integer_token.value)) { return Ok(Some(desired_type.clone())) },
                        Integer::I16 => if Self::in_range(-32768_i16, 32767_i16, Integer::from_number_str(&integer_token.value)) { return Ok(Some(desired_type.clone())) },
                        Integer::U16 => if Self::in_range(0_u16, 65535_u16, Integer::from_number_str(&integer_token.value)) { return Ok(Some(desired_type.clone())) },
                        Integer::I32 => if Self::in_range(-2147483648_i32, 2147483647_i32, Integer::from_number_str(&integer_token.value)) { return Ok(Some(desired_type.clone())) },
                        Integer::U32 => if Self::in_range(0_u32, 4294967295_u32, Integer::from_number_str(&integer_token.value)) { return Ok(Some(desired_type.clone())) },
                        Integer::I64 => if Self::in_range(-9223372036854775808_i64, 9223372036854775807_i64, Integer::from_number_str(&integer_token.value)) { return Ok(Some(desired_type.clone())) },
                        Integer::U64 => {
                            return Ok(Some(desired_type.clone()))
                        },
                        _ => return Err(InferTypeError::IntegerTooSmall { ty: desired_type.clone() , literal: integer_token.value.to_string(), code_line: code_line.clone() })
                    }
                }
            }
            (TypeToken::Float(_), TypeToken::Float(desired)) => {
                if let AssignableToken::FloatToken(float_token) = assignable_token {
                    match desired {
                        Float::Float32 if Self::in_range(-3.40282347e+38, 3.40282347e+38 , Ok(float_token.value)) => { return Ok(Some(desired_type.clone())) },
                        Float::Float64 if Self::in_range(-1.797_693_134_862_315_7e308, 1.797_693_134_862_315_7e308, Ok(float_token.value)) => { return Ok(Some(desired_type.clone())) },
                        _ => return Err(InferTypeError::FloatTooSmall { ty: desired_type.clone(), float: float_token.value, code_line: code_line.clone() })
                    }
                }

            },
            _ => { }
        }

        Ok(None)
    }

    fn in_range<T: PartialEq<P>, P: PartialOrd<T>>(min: T, max: T, value: Result<P, InferTypeError>) -> bool {
        if let Ok(value) = value {
            value >= min && value <= max
        } else {
            false
        }
    }

    pub fn byte_size(&self) -> usize {
        match self {
            TypeToken::Integer(int) => int.byte_size(),
            TypeToken::Float(float) => float.byte_size(),
            TypeToken::Bool => 4,
            TypeToken::Void => 0,
            TypeToken::Custom(_) => 8 // todo: calculate custom data types recursively
        }
    }
}
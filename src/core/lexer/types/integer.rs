use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::OperatorToASM;
use crate::core::code_generator::ASMGenerateError;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;
use crate::core::lexer::tokens::name_token::NameTokenErr;
use crate::core::lexer::types::type_token::{InferTypeError, OperatorMatrixRow, TypeToken};

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

impl OperatorToASM for Integer {
    fn operation_to_asm(&self, operator: &Operator) -> Result<String, ASMGenerateError> {
        match operator {
            Operator::Noop => Err(ASMGenerateError::InternalError("Noop instruction is not supported".to_string())),
            Operator::Add => Ok("add".to_string()),
            Operator::Sub => Ok("sub".to_string()),
            Operator::Div => Ok("div".to_string()),
            Operator::Mul => Ok("imul".to_string()),
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

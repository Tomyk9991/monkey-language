use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::registers::{Bit32, Bit64, FloatRegister, GeneralPurposeRegister};
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;
use crate::core::lexer::tokens::name_token::NameTokenErr;
use crate::core::lexer::types::cast_to::CastTo;
use crate::core::lexer::types::type_token::{InferTypeError, OperatorMatrixRow, TypeToken};

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub enum Float {
    #[default]
    Float32,
    Float64
}

impl Float {
    pub fn cast_from_to(f1: &Float, f2: &Float, source: &str, stack: &mut Stack, meta: &mut MetaInfo) -> Result<String, ASMGenerateError> {
        let cast_to = CastTo {
            from: TypeToken::Float(f1.clone()),
            to: TypeToken::Float(f2.clone()),
        };

        let instruction = cast_to.to_asm(stack, meta)?;

        let cast_from_register: GeneralPurposeRegister = match cast_to.from.byte_size() {
            8 => Bit64::Rax.into(),
            4 => Bit32::Eax.into(),
            _ => unreachable!("Target type {f2} doesnt have a compile time known size")
        };

        let cast_to_register: GeneralPurposeRegister = match cast_to.to.byte_size() {
            8 => Bit64::Rax.into(),
            4 => Bit32::Eax.into(),
            _ => unreachable!("Target type {f2} doesnt have a compile time known size")
        };



        let mut target = String::new();

        target += &ASMBuilder::mov_ident_line(&cast_from_register, source);
        target += &ASMBuilder::mov_x_ident_line(FloatRegister::Xmm0, &cast_from_register, Some(cast_to.from.byte_size()));

        // actual cast
        target += &ASMBuilder::ident_line(&format!("{instruction} {}, {}", FloatRegister::Xmm0, FloatRegister::Xmm0));
        target += &ASMBuilder::mov_x_ident_line(cast_to_register, FloatRegister::Xmm0, Some(cast_to.to.byte_size()));

        Ok(target)
    }

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

impl Display for Float {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Float::Float32 => write!(f, "f32"),
            Float::Float64 => write!(f, "f64"),
        }
    }
}

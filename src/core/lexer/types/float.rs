use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::registers::{Bit64, ByteSize, FloatRegister, GeneralPurposeRegister};
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::{AssemblerOperation, Operator, OperatorToASM};
use crate::core::lexer::tokens::name_token::NameTokenErr;
use crate::core::lexer::types::cast_to::{Castable, CastTo};
use crate::core::lexer::types::float::Float::{Float32, Float64};
use crate::core::lexer::types::type_token::{InferTypeError, TypeToken};

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub enum Float {
    #[default]
    Float32,
    Float64
}

impl Castable<Float, Float> for Float {
    fn add_casts(cast_matrix: &mut HashMap<(TypeToken, TypeToken), &'static str>) {
        cast_matrix.insert((TypeToken::Float(Float32), TypeToken::Float(Float64)), "cvtss2sd");
        cast_matrix.insert((TypeToken::Float(Float64), TypeToken::Float(Float32)), "cvtsd2ss");
    }

    fn cast_from_to(t1: &Float, t2: &Float, source: &str, stack: &mut Stack, meta: &mut MetaInfo) -> Result<String, ASMGenerateError> {
        let cast_to = CastTo {
            from: TypeToken::Float(t1.clone()),
            to: TypeToken::Float(t2.clone()),
        };

        let instruction = cast_to.to_asm(stack, meta)?;
        let last_register = stack.register_to_use
            .last()
            .unwrap_or(&GeneralPurposeRegister::Bit64(Bit64::Rax));


        let cast_from_register: GeneralPurposeRegister = last_register.to_size_register(&ByteSize::try_from(cast_to.from.byte_size())?);
        let cast_to_register = last_register.to_size_register(&ByteSize::try_from(cast_to.to.byte_size())?);


        let mut target = String::new();

        if let Ok(general_purpose_register) = GeneralPurposeRegister::from_str(source) {
            target += &ASMBuilder::mov_x_ident_line(&cast_from_register, general_purpose_register, Some(cast_to.from.byte_size()));
        } else if source.trim().starts_with(';') {
            target += &ASMBuilder::push(source);
        } else {
            target += &ASMBuilder::mov_ident_line(&cast_from_register, source);
        }


        target += &ASMBuilder::mov_x_ident_line(FloatRegister::Xmm7, &cast_from_register, Some(cast_to.from.byte_size()));

        // actual cast
        target += &ASMBuilder::ident_line(&format!("{instruction} {}, {}", FloatRegister::Xmm7, FloatRegister::Xmm7));
        target += &ASMBuilder::mov_x_ident_line(cast_to_register, FloatRegister::Xmm7, Some(cast_to.to.byte_size()));

        Ok(target)
    }
}

impl Float {
    pub fn operation_matrix(base_type_matrix: &mut HashMap<(TypeToken, Operator, TypeToken), TypeToken>) {
        let types = [Float::Float32, Float::Float64];

        for ty in &types {
            base_type_matrix.insert((TypeToken::Float(ty.clone()), Operator::Add, TypeToken::Float(ty.clone())), TypeToken::Float(ty.clone()));
            base_type_matrix.insert((TypeToken::Float(ty.clone()), Operator::Sub, TypeToken::Float(ty.clone())), TypeToken::Float(ty.clone()));
            base_type_matrix.insert((TypeToken::Float(ty.clone()), Operator::Mul, TypeToken::Float(ty.clone())), TypeToken::Float(ty.clone()));
            base_type_matrix.insert((TypeToken::Float(ty.clone()), Operator::Div, TypeToken::Float(ty.clone())), TypeToken::Float(ty.clone()));
        }
    }

    pub fn byte_size(&self) -> usize {
        match self {
            Float::Float32 => 4,
            Float::Float64 => 8,
        }
    }
}

impl OperatorToASM for Float {
    fn operation_to_asm<T: Display>(&self, operator: &Operator, registers: &[T]) -> Result<AssemblerOperation, ASMGenerateError> {
        let suffix = match self {
            Float::Float32 => "ss",
            Float::Float64 => "sd"
        };

        match operator {
            Operator::Noop => Err(ASMGenerateError::InternalError("Noop instruction is not supported".to_string())),
            Operator::Add => Ok(AssemblerOperation::two_operands(&format!("add{suffix}"), &registers[0], &registers[1]).into()),
            Operator::Sub => Ok(AssemblerOperation::two_operands(&format!("sub{suffix}"), &registers[0], &registers[1]).into()),
            Operator::Div => Ok(AssemblerOperation::two_operands(&format!("div{suffix}"), &registers[0], &registers[1]).into()),
            Operator::Mul => Ok(AssemblerOperation::two_operands(&format!("mul{suffix}"), &registers[0], &registers[1]).into()),
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

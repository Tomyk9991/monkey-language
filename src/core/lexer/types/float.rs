use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::core::code_generator::{ASMGenerateError, ASMResult, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::register_destination::word_from_byte_size;
use crate::core::code_generator::registers::{Bit64, ByteSize, FloatRegister, GeneralPurposeRegister};
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::{AssemblerOperation, Operator, OperatorToASM};
use crate::core::lexer::tokens::assignable_tokens::integer_token::IntegerToken;
use crate::core::lexer::tokens::name_token::NameTokenErr;
use crate::core::lexer::types::cast_to::{Castable, CastTo};
use crate::core::lexer::types::float::Float::{Float32, Float64};
use crate::core::lexer::types::integer::Integer;
use crate::core::lexer::types::type_token::{InferTypeError, TypeToken};

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub enum Float {
    #[default]
    Float32,
    Float64,
}


impl Castable<Float, Integer> for Float {
    fn add_casts(cast_matrix: &mut HashMap<(TypeToken, TypeToken), &'static str>) {
        for ty in &[Integer::I8, Integer::I16, Integer::I32, Integer::I64, Integer::U8, Integer::U16, Integer::U32, Integer::U64] {
            cast_matrix.insert((TypeToken::Float(Float32), TypeToken::Integer(ty.clone())), "cvtss2si");
            cast_matrix.insert((TypeToken::Float(Float64), TypeToken::Integer(ty.clone())), "cvtsd2si");
        }
    }

    fn cast_from_to(t1: &Float, t2: &Integer, source: &str, stack: &mut Stack, meta: &mut MetaInfo) -> Result<ASMResult, ASMGenerateError> {
        let cast_to = CastTo {
            from: TypeToken::Float(t1.clone()),
            to: TypeToken::Integer(t2.clone()),
        };

        let instruction = cast_to.to_asm(stack, meta)?;
        let last_register = stack.register_to_use
            .last()
            .unwrap_or(&GeneralPurposeRegister::Bit64(Bit64::Rax))
            .clone();

        let cast_from_register = last_register.to_size_register(&ByteSize::try_from(cast_to.from.byte_size())?);
        let _cast_to_register = last_register.to_size_register(&ByteSize::try_from(cast_to.to.byte_size())?);

        let mut target = String::new();

        target += &ASMBuilder::ident_comment_line(&format!("Cast: ({}) -> ({})", cast_to.from, cast_to.to));

        let mut is_stack_variable = false;
        for (_, word) in [8, 4, 2, 1].map(|a| (a, word_from_byte_size(a))) {
            if source.starts_with(&word) {
                is_stack_variable = true;
                break;
            }
        }



        if IntegerToken::from_str(source).is_ok() || is_stack_variable {
            target += &ASMBuilder::mov_ident_line(&cast_from_register, source);
        }

        let xmm7 = GeneralPurposeRegister::Float(FloatRegister::Xmm7);
        target += &ASMBuilder::mov_x_ident_line(&xmm7, &cast_from_register, Some(cast_to.from.byte_size()));
        target += &ASMBuilder::ident_line(&format!("{instruction} {}, {}", &cast_from_register, &xmm7));

        match t1.byte_size() {
            8 if *t2 != Integer::I64 => {
                match <Integer as Castable<Integer, Integer>>::cast_from_to(&Integer::I64, t2, &cast_from_register.to_string(), stack, meta)? {
                    ASMResult::Inline(r) => {
                        target += &r;
                        return Ok(ASMResult::Inline(target))
                    },
                    ASMResult::MultilineResulted(r, g) => {
                        target += &r;
                        return Ok(ASMResult::MultilineResulted(target, g))
                    },
                    ASMResult::Multiline(r) => {
                        target += &r;
                        return Ok(ASMResult::Multiline(target))
                    }
                };
            }
            4 if *t2 != Integer::I32 => {
                match <Integer as Castable<Integer, Integer>>::cast_from_to(&Integer::I32, t2, &cast_from_register.to_string(), stack, meta)? {
                    ASMResult::Inline(r) => {
                        target += &r;
                        return Ok(ASMResult::Inline(target))
                    },
                    ASMResult::MultilineResulted(r, g) => {
                        target += &r;
                        return Ok(ASMResult::MultilineResulted(target, g))
                    },
                    ASMResult::Multiline(r) => {
                        target += &r;
                        return Ok(ASMResult::Multiline(target))
                    }
                };
            }
            8 | 4 => {}
            l => unreachable!("Float cannot be of size: {}", l)
        }

        Ok(ASMResult::MultilineResulted(target, cast_from_register))
    }
}

impl Castable<Float, Float> for Float {
    fn add_casts(cast_matrix: &mut HashMap<(TypeToken, TypeToken), &'static str>) {
        cast_matrix.insert((TypeToken::Float(Float32), TypeToken::Float(Float64)), "cvtss2sd");
        cast_matrix.insert((TypeToken::Float(Float64), TypeToken::Float(Float32)), "cvtsd2ss");
    }

    fn cast_from_to(t1: &Float, t2: &Float, source: &str, stack: &mut Stack, meta: &mut MetaInfo) -> Result<ASMResult, ASMGenerateError> {
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
            target += source;
        } else {
            target += &ASMBuilder::mov_ident_line(&cast_from_register, source);
        }


        target += &ASMBuilder::mov_x_ident_line(FloatRegister::Xmm7, &cast_from_register, Some(cast_to.from.byte_size()));

        // actual cast
        target += &ASMBuilder::ident_line(&format!("{instruction} {}, {}", FloatRegister::Xmm7, FloatRegister::Xmm7));
        target += &ASMBuilder::mov_x_ident_line(&cast_to_register, FloatRegister::Xmm7, Some(cast_to.to.byte_size()));

        Ok(ASMResult::MultilineResulted(target, cast_to_register))
    }
}

impl Float {
    pub fn operation_matrix(base_type_matrix: &mut HashMap<(TypeToken, Operator, TypeToken), TypeToken>) {
        let types = [Float32, Float64];

        for ty in &types {
            base_type_matrix.insert((TypeToken::Float(ty.clone()), Operator::Add, TypeToken::Float(ty.clone())), TypeToken::Float(ty.clone()));
            base_type_matrix.insert((TypeToken::Float(ty.clone()), Operator::Sub, TypeToken::Float(ty.clone())), TypeToken::Float(ty.clone()));
            base_type_matrix.insert((TypeToken::Float(ty.clone()), Operator::Mul, TypeToken::Float(ty.clone())), TypeToken::Float(ty.clone()));
            base_type_matrix.insert((TypeToken::Float(ty.clone()), Operator::Div, TypeToken::Float(ty.clone())), TypeToken::Float(ty.clone()));

            base_type_matrix.insert((TypeToken::Float(ty.clone()), Operator::LessThan, TypeToken::Float(ty.clone())), TypeToken::Bool);
            base_type_matrix.insert((TypeToken::Float(ty.clone()), Operator::GreaterThan, TypeToken::Float(ty.clone())), TypeToken::Bool);
            base_type_matrix.insert((TypeToken::Float(ty.clone()), Operator::LessThanEqual, TypeToken::Float(ty.clone())), TypeToken::Bool);
            base_type_matrix.insert((TypeToken::Float(ty.clone()), Operator::GreaterThanEqual, TypeToken::Float(ty.clone())), TypeToken::Bool);

            base_type_matrix.insert((TypeToken::Float(ty.clone()), Operator::Equal, TypeToken::Float(ty.clone())), TypeToken::Bool);
            base_type_matrix.insert((TypeToken::Float(ty.clone()), Operator::NotEqual, TypeToken::Float(ty.clone())), TypeToken::Bool);
        }
    }

    pub fn byte_size(&self) -> usize {
        match self {
            Float32 => 4,
            Float64 => 8,
        }
    }
}

impl OperatorToASM for Float {
    fn operation_to_asm<T: Display>(&self, operator: &Operator, registers: &[T], stack: &mut Stack, meta: &mut MetaInfo) -> Result<AssemblerOperation, ASMGenerateError> {
        let suffix = match self {
            Float32 => "ss",
            Float64 => "sd"
        };

        match operator {
            Operator::Noop => Err(ASMGenerateError::InternalError("Noop instruction is not supported on".to_string())),
            Operator::Add | Operator::Sub | Operator::Div | Operator::Mul => AssemblerOperation::two_operands(
                &format!("{}{suffix}", operator.to_asm(stack, meta)?),
                &registers[0],
                &registers[1]
            ),
            Operator::Mod => Err(ASMGenerateError::InternalError("Modulo instruction is not supported on floats".to_string())),
            Operator::LeftShift => Err(ASMGenerateError::InternalError("Left Shift instruction is not supported on floats".to_string())),
            Operator::RightShift => Err(ASMGenerateError::InternalError("Left Shift instruction is not supported on floats".to_string())),
            Operator::LessThan | Operator::GreaterThan | Operator::LessThanEqual | Operator::GreaterThanEqual | Operator::Equal | Operator::NotEqual => {
                let float_operator = match operator {
                    Operator::LessThan => "setb".to_string(),
                    Operator::GreaterThan => "seta".to_string(),
                    Operator::LessThanEqual => "setbe".to_string(),
                    Operator::GreaterThanEqual => "setae".to_string(),
                    Operator::Equal => "sete".to_string(),
                    Operator::NotEqual => "setne".to_string(),
                    _ => operator.to_asm(stack, meta)?
                };

                let first_register = GeneralPurposeRegister::from_str(&registers[0].to_string()).map_err(|_| ASMGenerateError::InternalError(format!("Cannot build {} from register", &registers[0])))?;

                Ok(AssemblerOperation {
                    prefix: None,
                    operation: AssemblerOperation::compare_float(suffix, &float_operator, &registers[0], &registers[1])?,
                    postfix: None,
                    result_expected: first_register.to_64_bit_register().to_size_register(&ByteSize::_1),
                })
            },
            a => Err(ASMGenerateError::InternalError(format!("`{a}` is not a supported operation on {self}")))
        }
    }
}

impl FromStr for Float {
    type Err = InferTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "f32" => Float32,
            "f64" => Float64,
            _ => return Err(InferTypeError::TypeNotAllowed(NameTokenErr::UnmatchedRegex { target_value: String::from(s) }))
        })
    }
}

impl Display for Float {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Float32 => write!(f, "f32"),
            Float64 => write!(f, "f64"),
        }
    }
}

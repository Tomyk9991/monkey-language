use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::asm_result::{ASMResult};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::register_destination::word_from_byte_size;
use crate::core::code_generator::registers::{Bit64, ByteSize, FloatRegister, GeneralPurposeRegister};
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::equation_parser::operator::{AssemblerOperation, Operator, OperatorToASM};
use crate::core::scanner::abstract_syntax_tree_nodes::identifier::IdentifierErr;
use crate::core::scanner::types::cast_to::{Castable, CastTo};
use crate::core::scanner::types::float::Float::{Float32, Float64};
use crate::core::scanner::types::r#type::{InferTypeError, Mutability, Type};

type Integer = crate::core::scanner::abstract_syntax_tree_nodes::assignables::integer::IntegerAST;
type IntegerType = crate::core::scanner::types::integer::Integer;
#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub enum Float {
    #[default]
    Float32,
    Float64,
}


impl Castable<Float, IntegerType> for Float {
    fn add_casts(cast_matrix: &mut HashMap<(Type, Type), &'static str>) {
        for ty in &[IntegerType::I8, IntegerType::I16, IntegerType::I32, IntegerType::I64, IntegerType::U8, IntegerType::U16, IntegerType::U32, IntegerType::U64] {
            cast_matrix.insert((Type::Float(Float32, Mutability::Immutable), Type::Integer(ty.clone(), Mutability::Immutable)), "cvtss2si");
            cast_matrix.insert((Type::Float(Float64, Mutability::Immutable), Type::Integer(ty.clone(), Mutability::Immutable)), "cvtsd2si");
        }
    }

    fn cast_from_to(t1: &Float, t2: &IntegerType, source: &str, stack: &mut Stack, meta: &mut MetaInfo) -> Result<ASMResult, ASMGenerateError> {
        let cast_to = CastTo {
            from: Type::Float(t1.clone(), Mutability::Immutable),
            to: Type::Integer(t2.clone(), Mutability::Immutable),
        };

        let instruction = cast_to.to_asm::<InterimResultOption>(stack, meta, None)?.to_string();
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



        if Integer::from_str(source).is_ok() || is_stack_variable {
            target += &ASMBuilder::mov_ident_line(&cast_from_register, source);
        }

        let xmm7 = GeneralPurposeRegister::Float(FloatRegister::Xmm7);
        target += &ASMBuilder::mov_x_ident_line(&xmm7, &cast_from_register, Some(cast_to.from.byte_size()));
        target += &ASMBuilder::ident_line(&format!("{instruction} {}, {}", &cast_from_register, &xmm7));

        match t1.byte_size() {
            8 if *t2 != IntegerType::I64 => {
                match <IntegerType as Castable<IntegerType, IntegerType>>::cast_from_to(&IntegerType::I64, t2, &cast_from_register.to_string(), stack, meta)? {
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
            4 if *t2 != IntegerType::I32 => {
                match <IntegerType as Castable<IntegerType, IntegerType>>::cast_from_to(&IntegerType::I32, t2, &cast_from_register.to_string(), stack, meta)? {
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
    fn add_casts(cast_matrix: &mut HashMap<(Type, Type), &'static str>) {
        cast_matrix.insert((Type::Float(Float32, Mutability::Immutable), Type::Float(Float64, Mutability::Immutable)), "cvtss2sd");
        cast_matrix.insert((Type::Float(Float64, Mutability::Immutable), Type::Float(Float32, Mutability::Immutable)), "cvtsd2ss");
    }

    fn cast_from_to(t1: &Float, t2: &Float, source: &str, stack: &mut Stack, meta: &mut MetaInfo) -> Result<ASMResult, ASMGenerateError> {
        let cast_to = CastTo {
            from: Type::Float(t1.clone(), Mutability::Immutable),
            to: Type::Float(t2.clone(), Mutability::Immutable),
        };

        let instruction = cast_to.to_asm::<InterimResultOption>(stack, meta, None)?.to_string();
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
    pub fn operation_matrix(base_type_matrix: &mut HashMap<(Type, Operator, Type), Type>) {
        let types = [Float32, Float64];

        for ty in &types {
            base_type_matrix.insert((Type::Float(ty.clone(), Mutability::Immutable), Operator::Add, Type::Float(ty.clone(), Mutability::Immutable)), Type::Float(ty.clone(), Mutability::Immutable));
            base_type_matrix.insert((Type::Float(ty.clone(), Mutability::Immutable), Operator::Sub, Type::Float(ty.clone(), Mutability::Immutable)), Type::Float(ty.clone(), Mutability::Immutable));
            base_type_matrix.insert((Type::Float(ty.clone(), Mutability::Immutable), Operator::Mul, Type::Float(ty.clone(), Mutability::Immutable)), Type::Float(ty.clone(), Mutability::Immutable));
            base_type_matrix.insert((Type::Float(ty.clone(), Mutability::Immutable), Operator::Div, Type::Float(ty.clone(), Mutability::Immutable)), Type::Float(ty.clone(), Mutability::Immutable));

            base_type_matrix.insert((Type::Float(ty.clone(), Mutability::Immutable), Operator::LessThan, Type::Float(ty.clone(), Mutability::Immutable)), Type::Bool(Mutability::Immutable));
            base_type_matrix.insert((Type::Float(ty.clone(), Mutability::Immutable), Operator::GreaterThan, Type::Float(ty.clone(), Mutability::Immutable)), Type::Bool(Mutability::Immutable));
            base_type_matrix.insert((Type::Float(ty.clone(), Mutability::Immutable), Operator::LessThanEqual, Type::Float(ty.clone(), Mutability::Immutable)), Type::Bool(Mutability::Immutable));
            base_type_matrix.insert((Type::Float(ty.clone(), Mutability::Immutable), Operator::GreaterThanEqual, Type::Float(ty.clone(), Mutability::Immutable)), Type::Bool(Mutability::Immutable));

            base_type_matrix.insert((Type::Float(ty.clone(), Mutability::Immutable), Operator::Equal, Type::Float(ty.clone(), Mutability::Immutable)), Type::Bool(Mutability::Immutable));
            base_type_matrix.insert((Type::Float(ty.clone(), Mutability::Immutable), Operator::NotEqual, Type::Float(ty.clone(), Mutability::Immutable)), Type::Bool(Mutability::Immutable));
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
                &format!("{}{suffix}", operator.to_asm::<InterimResultOption>(stack, meta, None)?),
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
                    _ => operator.to_asm::<InterimResultOption>(stack, meta, None)?.to_string()
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
            _ => return Err(InferTypeError::TypeNotAllowed(IdentifierErr::UnmatchedRegex { target_value: String::from(s) }))
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

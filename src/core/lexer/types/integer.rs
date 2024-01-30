use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::register_destination::word_from_byte_size;
use crate::core::code_generator::registers::{Bit64, ByteSize, FloatRegister, GeneralPurposeRegister};
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::{AssemblerOperation, OperatorToASM};
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;
use crate::core::lexer::tokens::assignable_tokens::integer_token::IntegerToken;
use crate::core::lexer::tokens::name_token::NameTokenErr;
use crate::core::lexer::types::cast_to::{Castable, CastTo};
use crate::core::lexer::types::float::Float;
use crate::core::lexer::types::type_token::{InferTypeError, TypeToken};

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
    U64,
}

impl Castable<Integer, Float> for Integer {
    fn add_casts(cast_matrix: &mut HashMap<(TypeToken, TypeToken), &'static str>) {
        let types = [Integer::U8, Integer::I8, Integer::U16, Integer::I16, Integer::U32, Integer::I32, Integer::U64, Integer::I64];

        for t1 in &types {
            cast_matrix.insert((TypeToken::Integer(t1.clone()), TypeToken::Float(Float::Float32)), "cvtsi2ss");
            cast_matrix.insert((TypeToken::Integer(t1.clone()), TypeToken::Float(Float::Float64)), "cvtsi2sd");
        }
    }

    fn cast_from_to(t1: &Integer, t2: &Float, source: &str, stack: &mut Stack, meta: &mut MetaInfo) -> Result<String, ASMGenerateError> {
        let cast_to = CastTo {
            from: TypeToken::Integer(t1.clone()),
            to: TypeToken::Float(t2.clone()),
        };

        let instruction = cast_to.to_asm(stack, meta)?;
        let last_register = stack.register_to_use
            .last()
            .unwrap_or(&GeneralPurposeRegister::Bit64(Bit64::Rax))
            .clone();

        let mut cast_from_register = last_register.to_size_register(&ByteSize::try_from(cast_to.from.byte_size())?);
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

        if *t1 != Integer::U32 { // Convert to unsigned U32
            target += &<Integer as Castable<Integer, Integer>>::cast_from_to(t1, &Integer::U32, source, stack, meta)?;
            cast_from_register = last_register.to_size_register(&ByteSize::_4);
        } else if IntegerToken::from_str(source).is_ok() || is_stack_variable {
            target += &ASMBuilder::mov_ident_line(&cast_from_register, source);
        }


        target += &ASMBuilder::ident_line(&format!("{instruction} {}, {}", GeneralPurposeRegister::Float(FloatRegister::Xmm7), &cast_from_register));
        target += &ASMBuilder::mov_x_ident_line(_cast_to_register, &GeneralPurposeRegister::Float(FloatRegister::Xmm7), Some(cast_to.to.byte_size()));

        Ok(target)
    }
}


impl Castable<Integer, Integer> for Integer {
    fn add_casts(cast_matrix: &mut HashMap<(TypeToken, TypeToken), &'static str>) {
        let types = [Integer::U8, Integer::I8, Integer::U16, Integer::I16, Integer::U32, Integer::I32, Integer::U64, Integer::I64];

        for t1 in &types {
            for t2 in &types {
                if t1 == t2 { continue; }
                let instruction = match (t1.signed(), t2.signed()) {
                    (true, true) if t2.byte_size() > t1.byte_size() => "movsx",
                    (false, true) if t2.byte_size() > t1.byte_size() => "movzx",
                    (true, false) if t2.byte_size() > t1.byte_size() => "movsx",
                    (false, false) if t2.byte_size() > t1.byte_size() => "movzx",
                    _ => "mov"
                };

                cast_matrix.insert((TypeToken::Integer(t1.clone()), TypeToken::Integer(t2.clone())), instruction);
            }
        }

        // // U8
        // cast_matrix.insert((TypeToken::Integer(Integer::U8), TypeToken::Integer(Integer::I8)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::U8), TypeToken::Integer(Integer::I16)), "movzx");
        // cast_matrix.insert((TypeToken::Integer(Integer::U8), TypeToken::Integer(Integer::I32)), "movzx");
        // cast_matrix.insert((TypeToken::Integer(Integer::U8), TypeToken::Integer(Integer::I64)), "movzx");
        //
        // cast_matrix.insert((TypeToken::Integer(Integer::U8), TypeToken::Integer(Integer::U16)), "movzx");
        // cast_matrix.insert((TypeToken::Integer(Integer::U8), TypeToken::Integer(Integer::U32)), "movzx");
        // cast_matrix.insert((TypeToken::Integer(Integer::U8), TypeToken::Integer(Integer::U64)), "movzx");
        //
        // // I8
        // cast_matrix.insert((TypeToken::Integer(Integer::I8), TypeToken::Integer(Integer::U8)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::I8), TypeToken::Integer(Integer::U16)), "movsx");
        // cast_matrix.insert((TypeToken::Integer(Integer::I8), TypeToken::Integer(Integer::U32)), "movsx");
        // cast_matrix.insert((TypeToken::Integer(Integer::I8), TypeToken::Integer(Integer::U64)), "movsx");
        //
        // cast_matrix.insert((TypeToken::Integer(Integer::I8), TypeToken::Integer(Integer::I16)), "movsx");
        // cast_matrix.insert((TypeToken::Integer(Integer::I8), TypeToken::Integer(Integer::I32)), "movsx");
        // cast_matrix.insert((TypeToken::Integer(Integer::I8), TypeToken::Integer(Integer::I64)), "movsx");
        //
        // // U16
        // cast_matrix.insert((TypeToken::Integer(Integer::U16), TypeToken::Integer(Integer::I8)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::U16), TypeToken::Integer(Integer::I16)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::U16), TypeToken::Integer(Integer::I32)), "movzx");
        // cast_matrix.insert((TypeToken::Integer(Integer::U16), TypeToken::Integer(Integer::I64)), "movzx");
        //
        // cast_matrix.insert((TypeToken::Integer(Integer::U16), TypeToken::Integer(Integer::U8)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::U16), TypeToken::Integer(Integer::U16)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::U16), TypeToken::Integer(Integer::U32)), "movsx");
        // cast_matrix.insert((TypeToken::Integer(Integer::U16), TypeToken::Integer(Integer::U64)), "movsx");
        //
        // // I16
        // cast_matrix.insert((TypeToken::Integer(Integer::I16), TypeToken::Integer(Integer::I8)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::I16), TypeToken::Integer(Integer::I16)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::I16), TypeToken::Integer(Integer::I32)), "movsx");
        // cast_matrix.insert((TypeToken::Integer(Integer::I16), TypeToken::Integer(Integer::I64)), "movsx");
        //
        // cast_matrix.insert((TypeToken::Integer(Integer::I16), TypeToken::Integer(Integer::U8)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::I16), TypeToken::Integer(Integer::U16)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::I16), TypeToken::Integer(Integer::U32)), "movzx");
        // cast_matrix.insert((TypeToken::Integer(Integer::I16), TypeToken::Integer(Integer::U64)), "movzx");
        //
        // // U32
        // cast_matrix.insert((TypeToken::Integer(Integer::U32), TypeToken::Integer(Integer::I8)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::U32), TypeToken::Integer(Integer::I16)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::U32), TypeToken::Integer(Integer::I32)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::U32), TypeToken::Integer(Integer::I64)), "");
        //
        // cast_matrix.insert((TypeToken::Integer(Integer::U32), TypeToken::Integer(Integer::U8)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::U32), TypeToken::Integer(Integer::U16)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::U32), TypeToken::Integer(Integer::U32)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::U32), TypeToken::Integer(Integer::U64)), "movsx");
        //
        // // I32
        // cast_matrix.insert((TypeToken::Integer(Integer::I32), TypeToken::Integer(Integer::I8)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::I32), TypeToken::Integer(Integer::I16)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::I32), TypeToken::Integer(Integer::I32)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::I32), TypeToken::Integer(Integer::I64)), "movsx");
        //
        // cast_matrix.insert((TypeToken::Integer(Integer::I32), TypeToken::Integer(Integer::U8)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::I32), TypeToken::Integer(Integer::U16)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::I32), TypeToken::Integer(Integer::U32)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::I32), TypeToken::Integer(Integer::U64)), "movzx");
        //
        // // U64
        // cast_matrix.insert((TypeToken::Integer(Integer::U64), TypeToken::Integer(Integer::I8)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::U64), TypeToken::Integer(Integer::I16)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::U64), TypeToken::Integer(Integer::I32)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::U64), TypeToken::Integer(Integer::I64)), "mov");
        //
        // cast_matrix.insert((TypeToken::Integer(Integer::U64), TypeToken::Integer(Integer::U8)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::U64), TypeToken::Integer(Integer::U16)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::U64), TypeToken::Integer(Integer::U32)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::U64), TypeToken::Integer(Integer::U64)), "mov");
        //
        // // I64
        // cast_matrix.insert((TypeToken::Integer(Integer::I64), TypeToken::Integer(Integer::I8)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::I64), TypeToken::Integer(Integer::I16)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::I64), TypeToken::Integer(Integer::I32)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::I64), TypeToken::Integer(Integer::I64)), "mov");
        //
        // cast_matrix.insert((TypeToken::Integer(Integer::I64), TypeToken::Integer(Integer::U8)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::I64), TypeToken::Integer(Integer::U16)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::I64), TypeToken::Integer(Integer::U32)), "mov");
        // cast_matrix.insert((TypeToken::Integer(Integer::I64), TypeToken::Integer(Integer::U64)), "mov");
    }


    fn cast_from_to(i1: &Integer, i2: &Integer, source: &str, stack: &mut Stack, meta: &mut MetaInfo) -> Result<String, ASMGenerateError> {
        let mut source = source.to_string();

        let cast_to = CastTo {
            from: TypeToken::Integer(i1.clone()),
            to: TypeToken::Integer(i2.clone()),
        };

        let instruction = cast_to.to_asm(stack, meta)?;
        let last_register = stack.register_to_use
            .last()
            .unwrap_or(&GeneralPurposeRegister::Bit64(Bit64::Rax));

        let cast_from_register = last_register.to_size_register(&ByteSize::try_from(cast_to.from.byte_size())?);
        let cast_to_register = last_register.to_size_register(&ByteSize::try_from(cast_to.to.byte_size())?);

        let mut target = String::new();

        if cast_to.casting_down() {
            for (_, word) in [8, 4, 2, 1].map(|a| (a, word_from_byte_size(a))) {
                if source.starts_with(&word) {
                    source = source.replace(&format!("{word} "), "");
                    break;
                }
            }
        }

        target += &ASMBuilder::ident_comment_line(&format!("Cast: ({}) -> ({})", cast_to.from, cast_to.to));

        // Special case: u32 -> i64
        // Special case: i32 -> u64
        // movzx cant handle DWORD on rhs
        // *i1 == Integer::I32 && *i2 == Integer::I64
        if (*i2 == Integer::U64 || *i2 == Integer::I64) && *i1 == Integer::U32 || *i1 == Integer::I32 && *i2 == Integer::U64 {
            let r14 = GeneralPurposeRegister::Bit64(Bit64::R14).to_size_register(&cast_from_register.size());
            target += &ASMBuilder::ident_line(&format!("mov {}, {}", &r14, &source));
            target += &ASMBuilder::ident_line(&format!("xor {}, {}", cast_to_register, cast_to_register));
            // since we are using xor, we can use mov because mov eax, eax will get optimized out, but in order to make the
            // conversion complete, we need this
            target += &ASMBuilder::ident_line(&format!("mov {}, {}", &cast_from_register, r14));

            return Ok(target);
        }

        if IntegerToken::from_str(&source).is_ok() {
            target += &ASMBuilder::mov_ident_line(&cast_from_register, &source);
        } else {
            let destination_register = if cast_to.casting_down() { cast_from_register } else { cast_to_register };
            if instruction == "mov" {
                if let Ok(source_register) = GeneralPurposeRegister::from_str(&source) {
                    if destination_register.to_64_bit_register() == source_register.to_64_bit_register() {}
                } else {
                    target += &ASMBuilder::mov_ident_line(&destination_register, &source);
                }
            } else {
                target += &ASMBuilder::ident_line(&format!("{instruction} {}, {}", destination_register, source));
            }
        }


        Ok(target)
    }
}

impl Integer {
    pub fn from_number_str<T: FromStr>(value: &str) -> Result<T, InferTypeError> {
        value.parse().map_err(|_| InferTypeError::TypeNotAllowed(NameTokenErr::UnmatchedRegex { target_value: String::from(value) }))
    }

    pub fn signed(&self) -> bool {
        matches!(self, Integer::I8 | Integer::I16 | Integer::I32 | Integer::I64)
    }

    pub fn operation_matrix(base_type_matrix: &mut HashMap<(TypeToken, Operator, TypeToken), TypeToken>) {
        let types = [Integer::I8, Integer::U8, Integer::I16, Integer::U16, Integer::I32, Integer::U32, Integer::I64, Integer::U64];

        for ty in &types {
            base_type_matrix.insert((TypeToken::Integer(ty.clone()), Operator::Add, TypeToken::Integer(ty.clone())), TypeToken::Integer(ty.clone()));
            base_type_matrix.insert((TypeToken::Integer(ty.clone()), Operator::Sub, TypeToken::Integer(ty.clone())), TypeToken::Integer(ty.clone()));
            base_type_matrix.insert((TypeToken::Integer(ty.clone()), Operator::Mul, TypeToken::Integer(ty.clone())), TypeToken::Integer(ty.clone()));
            base_type_matrix.insert((TypeToken::Integer(ty.clone()), Operator::Div, TypeToken::Integer(ty.clone())), TypeToken::Integer(ty.clone()));
            base_type_matrix.insert((TypeToken::Integer(ty.clone()), Operator::LeftShift, TypeToken::Integer(ty.clone())), TypeToken::Integer(ty.clone()));
            base_type_matrix.insert((TypeToken::Integer(ty.clone()), Operator::RightShift, TypeToken::Integer(ty.clone())), TypeToken::Integer(ty.clone()));

            base_type_matrix.insert((TypeToken::Integer(ty.clone()), Operator::LessThan, TypeToken::Integer(ty.clone())), TypeToken::Bool);
            base_type_matrix.insert((TypeToken::Integer(ty.clone()), Operator::GreaterThan, TypeToken::Integer(ty.clone())), TypeToken::Bool);
            base_type_matrix.insert((TypeToken::Integer(ty.clone()), Operator::LessThanEqual, TypeToken::Integer(ty.clone())), TypeToken::Bool);
            base_type_matrix.insert((TypeToken::Integer(ty.clone()), Operator::GreaterThanEqual, TypeToken::Integer(ty.clone())), TypeToken::Bool);

            base_type_matrix.insert((TypeToken::Integer(ty.clone()), Operator::Equal, TypeToken::Integer(ty.clone())), TypeToken::Bool);
            base_type_matrix.insert((TypeToken::Integer(ty.clone()), Operator::NotEqual, TypeToken::Integer(ty.clone())), TypeToken::Bool);
        }
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
    fn operation_to_asm<T: Display>(&self, operator: &Operator, registers: &[T]) -> Result<AssemblerOperation, ASMGenerateError> {
        let prefix = if self.signed() { "i" } else { "" };

        let integer_size = self.byte_size();

        match operator {
            Operator::Noop => Err(ASMGenerateError::InternalError("Noop instruction is not supported".to_string())),
            Operator::Add => Ok(AssemblerOperation::two_operands("add", &registers[0], &registers[1]).into()),
            Operator::Sub => Ok(AssemblerOperation::two_operands("sub", &registers[0], &registers[1]).into()),
            Operator::Div => Ok(AssemblerOperation {
                prefix: Some(AssemblerOperation::save_rax_rcx_rdx(self.byte_size(), registers)?),
                operation: format!("{prefix}div {}", GeneralPurposeRegister::Bit64(Bit64::Rcx).to_size_register(&ByteSize::try_from(integer_size)?)),
                postfix: Some(AssemblerOperation::load_rax_rcx_rdx(self.byte_size(), registers)?),
            }),
            Operator::Mul => if self.signed() {
                Ok(AssemblerOperation::two_operands("imul", &registers[0], &registers[1]).into())
            } else {
                Ok(AssemblerOperation {
                    prefix: Some(AssemblerOperation::save_rax_rcx_rdx(self.byte_size(), registers)?),
                    operation: format!("{prefix}mul, {}", &GeneralPurposeRegister::Bit64(Bit64::Rdx).to_size_register(&ByteSize::try_from(integer_size)?)),
                    postfix: Some(AssemblerOperation::load_rax_rcx_rdx(self.byte_size(), registers)?),
                })
            },
            Operator::LeftShift => {
                Ok(AssemblerOperation {
                    prefix: Some(AssemblerOperation::save_rax_rcx_rdx(self.byte_size(), registers)?),
                    operation: format!("shl {}, cl", &registers[0]),
                    postfix: Some(AssemblerOperation::load_rax_rcx_rdx(self.byte_size(), registers)?),
                })
            }
            Operator::RightShift => {
                Ok(AssemblerOperation {
                    prefix: Some(AssemblerOperation::save_rax_rcx_rdx(self.byte_size(), registers)?),
                    operation: format!("shr {}, cl", &registers[0]),
                    postfix: Some(AssemblerOperation::load_rax_rcx_rdx(self.byte_size(), registers)?),
                })
            }
            Operator::LessThan | Operator::GreaterThan | Operator::LessThanEqual | Operator::GreaterThanEqual | Operator::Equal | Operator::NotEqual => Ok(AssemblerOperation {
                prefix: None,
                operation: AssemblerOperation::compare(&operator.to_asm(&mut Default::default(), &mut Default::default())?, &registers[0], &registers[1])?,
                postfix: None,
            }),
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

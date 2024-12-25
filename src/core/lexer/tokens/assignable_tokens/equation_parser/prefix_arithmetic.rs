use std::any::Any;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::core::code_generator::{ASMGenerateError, MetaInfo, register_destination, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_result::{ASMOptions, ASMResult, ASMResultError, ASMResultVariance};
use crate::core::code_generator::generator::{Stack, StackLocation};
use crate::core::code_generator::registers::{ByteSize, GeneralPurposeRegister};
use crate::core::lexer::tokens::assignable_token::AssignableToken;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;
use crate::core::lexer::types::boolean::Boolean;
use crate::core::lexer::types::cast_to::{Castable, CastToError};
use crate::core::lexer::types::float::Float;
use crate::core::lexer::types::integer::Integer;
use crate::core::lexer::types::type_token::TypeToken;

#[derive(Clone, PartialEq, Debug)]
pub enum PointerArithmetic {
    /// *
    Asterics,
    /// &
    Ampersand,
}

#[derive(Clone)]
pub struct PrefixArithmeticOptions {
    pub value: AssignableToken,
    pub register_or_stack_address: String,
    pub register_64: GeneralPurposeRegister,
    pub target_register: GeneralPurposeRegister,
    pub child_has_pointer_arithmetic: bool,
    pub target: String,
}

impl ASMOptions for PrefixArithmeticOptions {}

impl ToASM for PrefixArithmetic {
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        if let Some(mut p) = options {
            let any_t = &mut p as &mut dyn Any;
            if let Some(options) = any_t.downcast_mut::<PrefixArithmeticOptions>() {
                match self {
                    PrefixArithmetic::PointerArithmetic(pointer_arithmetic) => {
                        return match pointer_arithmetic {
                            PointerArithmetic::Ampersand => {
                                // trying to lea rax, rax. this is not good
                                // you must write to an anonymous stack position and dereference that one
                                if GeneralPurposeRegister::from_str(&options.register_or_stack_address).is_ok() {
                                    let byte_size = options.value.infer_type_with_context(&meta.static_type_information, &meta.code_line)?.byte_size();
                                    stack.variables.push(StackLocation::new_anonymous_stack_location(stack.stack_position, byte_size));
                                    stack.stack_position += byte_size;

                                    let offset = stack.stack_position;
                                    let anonymous_stack_position = format!("{} [rbp - {}]", register_destination::word_from_byte_size(byte_size), offset);
                                    options.target.push_str(&ASMBuilder::mov_x_ident_line(&anonymous_stack_position, &options.register_64, Some(byte_size)));
                                    options.register_or_stack_address = anonymous_stack_position;
                                }

                                options.target.push_str(&ASMBuilder::ident_line(&format!("lea {}, {}", options.register_64, options.register_or_stack_address
                                    .replace("QWORD ", "")
                                    .replace("DWORD ", "")
                                    .replace("BYTE ", "")
                                    .replace("WORD ", "")
                                )));

                                Ok(ASMResult::MultilineResulted(options.target.clone(), options.register_64.clone()))
                            }
                            PointerArithmetic::Asterics => {
                                options.target.push_str(&ASMBuilder::mov_ident_line(&options.register_64, &options.register_or_stack_address));

                                if !options.child_has_pointer_arithmetic {
                                    options.target.push_str(&ASMBuilder::mov_ident_line(&options.register_64, format!("QWORD [{}]", options.register_64)));
                                    let value_type = options.value.infer_type_with_context(&meta.static_type_information, &meta.code_line).ok();


                                    if let (GeneralPurposeRegister::Float(destination_float_register), Some(f)) = (&options.target_register, &value_type) {
                                        options.target.push_str(&ASMBuilder::mov_x_ident_line(destination_float_register, &options.register_64, Some(f.byte_size())));
                                    }
                                }

                                Ok(ASMResult::MultilineResulted(options.target.clone(), options.target_register.clone()))
                            }
                        }
                    }
                    PrefixArithmetic::Cast(ty) => {
                        let assignable_type = options.value.infer_type_with_context(&meta.static_type_information, &meta.code_line)?;
                        let cast_to = assignable_type.cast_to(ty);

                        if options.child_has_pointer_arithmetic {
                            options.register_or_stack_address = options.register_64.to_string();
                        }


                        let result = match (&cast_to.from, &cast_to.to) {
                            (TypeToken::Float(f1), TypeToken::Float(f2)) => Float::cast_from_to(f1, f2, &options.register_or_stack_address, stack, meta)?,
                            (TypeToken::Integer(i1), TypeToken::Float(f2)) => Integer::cast_from_to(i1, f2, &options.register_or_stack_address, stack, meta)?,
                            (TypeToken::Bool, TypeToken::Integer(i2)) => Boolean::cast_from_to(&Boolean::True, i2, &options.register_or_stack_address, stack, meta)?,
                            (TypeToken::Float(f1), TypeToken::Integer(i2)) => Float::cast_from_to(f1, i2, &options.register_or_stack_address, stack, meta)?,
                            (TypeToken::Integer(i1), TypeToken::Integer(i2)) => Integer::cast_from_to(i1, i2, &options.register_or_stack_address, stack, meta)?,
                            _ => return Err(ASMGenerateError::CastUnsupported(CastToError::CastUnsupported(cast_to.clone()), meta.code_line.clone()))
                        };

                        result.apply_with(&mut options.target)
                            .allow(ASMResultVariance::Inline)
                            .allow(ASMResultVariance::MultilineResulted)
                            .allow(ASMResultVariance::Multiline)
                            .token("Expression")
                            .finish()?;



                        return if let TypeToken::Float(_) = &cast_to.to {
                            let d = options.register_64.to_float_register();
                            let r = options.register_64.to_size_register_ignore_float(&ByteSize::try_from(cast_to.to.byte_size())?);
                            options.target.push_str(&ASMBuilder::mov_x_ident_line(&d, r, Some(cast_to.to.byte_size())));
                            Ok(ASMResult::MultilineResulted(options.target.clone(), d))
                        } else {
                            Ok(ASMResult::MultilineResulted(options.target.clone(), options.register_64.to_size_register_ignore_float(&ByteSize::try_from(cast_to.to.byte_size())?)))
                        }
                    }
                    PrefixArithmetic::Operation(_) => unimplemented!("Not finished yet"),
                }
            }
        }

        Err(ASMGenerateError::ASMResult(ASMResultError::NoOptionProvided("Prefix arithmetic".to_string())))
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        8
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum PrefixArithmetic {
    #[allow(unused)]
    Operation(Operator),
    // For example the "-" like let a = -5;
    PointerArithmetic(PointerArithmetic),
    Cast(TypeToken),
}

impl Display for PrefixArithmetic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PrefixArithmetic::Operation(operation) => operation.to_string(),
            PrefixArithmetic::PointerArithmetic(p) => p.to_string(),
            PrefixArithmetic::Cast(c) => format!("({c})")
        })
    }
}

impl Display for PointerArithmetic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PointerArithmetic::Asterics => "*".to_string(),
            PointerArithmetic::Ampersand => "&".to_string()
        })
    }
}
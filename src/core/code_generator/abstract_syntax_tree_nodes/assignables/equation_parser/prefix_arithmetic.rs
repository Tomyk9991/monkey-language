use std::str::FromStr;

use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::{ASMResult, ASMResultError, ASMResultVariance};
use crate::core::code_generator::generator::{Stack, StackLocation};
use crate::core::code_generator::registers::{ByteSize, GeneralPurposeRegister};
use crate::core::code_generator::{register_destination, ASMGenerateError, MetaInfo, ToASM};
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::operator::Operator;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::prefix_arithmetic::{PointerArithmetic, PrefixArithmetic};
use crate::core::model::types::float::FloatType;
use crate::core::model::types::integer::IntegerType;
use crate::core::model::types::ty::Type;
use crate::core::parser::types::boolean::Boolean;
use crate::core::parser::types::cast_to::{CastToError, Castable};
use crate::core::semantics::type_infer::infer_type::InferType;

impl ToASM for PrefixArithmetic {
    fn to_asm(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<ASMOptions>) -> Result<ASMResult, ASMGenerateError> {
        if let Some(ASMOptions::PrefixArithmeticOptions(mut options)) = options {
            match self {
                PrefixArithmetic::PointerArithmetic(pointer_arithmetic) => {
                    return match pointer_arithmetic {
                        PointerArithmetic::Ampersand => {
                            // trying to lea rax, rax. this is not good
                            // you must write to an anonymous stack position and dereference that one
                            if GeneralPurposeRegister::from_str(&options.register_or_stack_address).is_ok() {
                                let byte_size = options.value.infer_type(&mut meta.static_type_information)?.byte_size();
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

                            if !options.child_has_pointer_arithmetic && !options.is_lvalue {
                                options.target.push_str(&ASMBuilder::mov_ident_line(&options.register_64, format!("QWORD [{}]", options.register_64)));
                                let value_type = options.value.infer_type(&mut meta.static_type_information).ok();


                                if let (GeneralPurposeRegister::Float(destination_float_register), Some(f)) = (&options.target_register, &value_type) {
                                    options.target.push_str(&ASMBuilder::mov_x_ident_line(destination_float_register, &options.register_64, Some(f.byte_size())));
                                }
                            }

                            Ok(ASMResult::MultilineResulted(options.target.clone(), options.target_register.clone()))
                        }
                    }
                }
                PrefixArithmetic::Cast(ty) => {
                    let assignable_type = options.value.infer_type(&mut meta.static_type_information)?;
                    let cast_to = assignable_type.cast_to(ty);

                    if options.child_has_pointer_arithmetic {
                        options.register_or_stack_address = options.register_64.to_string();
                    }


                    let result = match (&cast_to.from, &cast_to.to) {
                        (Type::Float(f1, _), Type::Float(f2, _)) => FloatType::cast_from_to(f1, f2, &options.register_or_stack_address, stack, meta)?,
                        (Type::Integer(i1, _), Type::Float(f2, _)) => IntegerType::cast_from_to(i1, f2, &options.register_or_stack_address, stack, meta)?,
                        (Type::Bool(_), Type::Integer(i2, _)) => Boolean::cast_from_to(&Boolean::True, i2, &options.register_or_stack_address, stack, meta)?,
                        (Type::Float(f1, _), Type::Integer(i2, _)) => FloatType::cast_from_to(f1, i2, &options.register_or_stack_address, stack, meta)?,
                        (Type::Integer(i1, _), Type::Integer(i2, _)) => IntegerType::cast_from_to(i1, i2, &options.register_or_stack_address, stack, meta)?,
                        _ => return Err(ASMGenerateError::CastUnsupported(CastToError::CastUnsupported(cast_to.clone()), meta.file_position.clone()))
                    };

                    result.apply_with(&mut options.target)
                        .allow(ASMResultVariance::Inline)
                        .allow(ASMResultVariance::MultilineResulted)
                        .allow(ASMResultVariance::Multiline)
                        .ast_node("Expression")
                        .finish()?;


                    return if let Type::Float(_, _) = &cast_to.to {
                        let d = options.register_64.to_float_register();
                        let r = options.register_64.to_size_register_ignore_float(&ByteSize::try_from(cast_to.to.byte_size())?);
                        options.target.push_str(&ASMBuilder::mov_x_ident_line(&d, r, Some(cast_to.to.byte_size())));
                        Ok(ASMResult::MultilineResulted(options.target.clone(), d))
                    } else {
                        Ok(ASMResult::MultilineResulted(options.target.clone(), options.register_64.to_size_register_ignore_float(&ByteSize::try_from(cast_to.to.byte_size())?)))
                    }
                },
                PrefixArithmetic::Operation(Operator::Noop) => {
                    return Ok(ASMResult::MultilineResulted(options.target.clone(), options.register_64.clone()));
                }
                PrefixArithmetic::Operation(_) => unimplemented!("Not finished yet"),
            }
        }

        Err(ASMGenerateError::ASMResult(ASMResultError::NoOptionProvided("Prefix arithmetic".to_string())))
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, _meta: &MetaInfo) -> usize {
        8
    }
}

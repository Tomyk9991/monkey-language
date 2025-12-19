use std::any::Any;
use std::error::Error;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_options::prepare_register::PrepareRegisterOption;
use crate::core::code_generator::asm_result::{ASMResult};
use crate::core::code_generator::generator::{Stack, StackLocation};
use crate::core::code_generator::register_destination::{byte_size_from_word, word_from_byte_size};
use crate::core::code_generator::registers::{ByteSize, GeneralPurposeRegister};
use crate::core::constants::KEYWORDS;
use crate::core::model::abstract_syntax_tree_nodes::identifier::{Identifier, IdentifierError};
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::types::ty::Type;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::{InferTypeError};



impl ToASM for Identifier {
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        if let Some(options) = options {
            let any_t = &options as &dyn Any;
            if let Some(s) = any_t.downcast_ref::<PrepareRegisterOption>() {
                if let Type::Float(_, _) = self.get_type(&meta.static_type_information).ok_or(InferTypeError::NoTypePresent(
                    LValue::Identifier(Identifier { name: self.name.clone() }), meta.file_position.clone()
                ))? {
                    return s.transform(stack, meta);
                }
            }
        }


        if let Some(stack_location) = stack.variables.iter().rfind(|&variable| variable.name.identifier() == self.name.as_str()) {
            if let Some(found_variable) = meta.static_type_information.context.iter().rfind(|v| {
                if let LValue::Identifier(n) = &v.l_value {
                    n.name == *self.name
                } else {
                    false
                }
            }) {
                if let Some(ty) = &found_variable.ty {
                    let operand_hint = word_from_byte_size(ty.byte_size());
                    let amount_elements = stack_location.elements;
                    let element_size = stack_location.size / stack_location.elements;

                    return match &stack.indexing {
                        Some(ASMResult::Inline(offset)) => {
                            return match offset.parse::<i32>() {
                                Ok(offset) => {
                                    let base_address = stack_location.position + element_size;
                                    let index = (amount_elements as i32) - offset - 1;

                                    Ok(ASMResult::Inline(format!("{operand_hint} [rbp - ({base_address} + {index} * {element_size})]")))
                                }
                                Err(_) => {
                                    let inline_stack_word_size = byte_size_from_word(offset.split(" ").next().ok_or(ASMGenerateError::InternalError(format!("Could not parse {offset} as a byte size"), meta.file_position.clone()))?);
                                    let register_iterator = GeneralPurposeRegister::iter_from_byte_size(inline_stack_word_size)?.current();
                                    let resulting_register = stack.register_to_use.last().unwrap_or(&register_iterator).to_size_register(&ByteSize::try_from(inline_stack_word_size)?);
                                    let index_operation = &ASMBuilder::mov_x_ident_line(&resulting_register, offset, Some(inline_stack_word_size));
                                    to_multi_line_index_calculation(&operand_hint, index_operation, &resulting_register, stack_location, element_size)
                                }
                            }
                        }
                        Some(ASMResult::MultilineResulted(index_operation, resulting_register)) => {
                            to_multi_line_index_calculation(&operand_hint, index_operation, resulting_register, stack_location, element_size)
                        }
                        Some(ASMResult::Multiline(_)) => unreachable!("Could not think of a scenario where this would happen"),
                        None => Ok(ASMResult::Inline(format!("{operand_hint} [rbp - {}]", stack_location.position + element_size)))
                    }
                }
            }

            Ok(ASMResult::Inline(format!("DWORD [rbp - {}]", stack_location.position + stack_location.size / stack_location.elements)))
        } else {
            Err(ASMGenerateError::UnresolvedReference { name: self.name.to_string(), file_position: meta.file_position.clone() })
        }
    }


    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        true
    }

    fn byte_size(&self, meta: &mut MetaInfo) -> usize {
        if let Some(v) = meta.static_type_information.iter().rfind(|v| {
            if let LValue::Identifier(n) = &v.l_value {
                n.name == *self.name
            } else {
                false
            }
        }) {
            if let Some(ty) = &v.ty {
                return ty.byte_size();
            }
        }

        0
    }
}

fn to_multi_line_index_calculation(operand_hint: &str, index_operation: &str, resulting_register: &GeneralPurposeRegister, stack_location: &StackLocation, element_size: usize) -> Result<ASMResult, ASMGenerateError> {
    let mut target = String::new();
    target += index_operation;

    if resulting_register.size() as usize != 8 {
        target += &ASMBuilder::ident_line("cdqe");
    }

    let resulting_register = resulting_register.to_64_bit_register();

    let base_address = stack_location.position + element_size;
    let first_element_address = base_address + (element_size * (stack_location.elements - 1));


    target += &ASMBuilder::ident_line(&format!("imul {resulting_register}, {element_size}"));
    let assignment = format!("{operand_hint} [rbp - {} + {resulting_register}]", first_element_address);
    Ok(ASMResult::MultilineResulted(target, GeneralPurposeRegister::Memory(assignment)))
}
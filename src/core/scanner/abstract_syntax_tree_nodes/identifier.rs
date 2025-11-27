use std::any::Any;
use std::fmt::{Display};
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
use crate::core::io::code_line::CodeLine;
use crate::core::model::abstract_syntax_tree_nodes::identifier::{Identifier, IdentifierError};
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::types::ty::Type;
use crate::core::scanner::static_type_context::StaticTypeContext;
use crate::core::scanner::types::r#type::{InferTypeError};



impl Identifier {
    pub fn uuid() -> Identifier {
        Identifier {
            name: Uuid::new_v4().to_string(),
        }
    }

    pub fn from_str(s: &str, allow_reserved: bool) -> Result<Identifier, IdentifierError> {
        if !allow_reserved && KEYWORDS.iter().any(|keyword| keyword.to_lowercase() == s.to_lowercase()) {
            return Err(IdentifierError::KeywordReserved(s.to_string()));
        }

        if !lazy_regex::regex_is_match!(r"^[a-zA-Z_$][a-zA-Z_$0-9]*$", s) {
            return Err(IdentifierError::UnmatchedRegex {
                target_value: s.to_string(),
            });
        }

        Ok(Identifier {
            name: s.to_string()
        })
    }

    pub fn infer_type_with_context(&self, context: &StaticTypeContext, code_line: &CodeLine) -> Result<Type, InferTypeError> {
        if let Some(v) = context.iter().rfind(|v| {
            if let LValue::Identifier(n) = &v.l_value {
                n.name == *self.name
            } else {
                false
            }
        }) {
            return if let Some(ty) = &v.ty {
                Ok(ty.clone())
            } else {
                Err(InferTypeError::NoTypePresent(v.l_value.clone(), CodeLine::default()/*v.code_line.clone()*/))
            };
        }

        Err(InferTypeError::UnresolvedReference(self.to_string(), code_line.clone()))
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
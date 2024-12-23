use std::any::Any;
use std::error::Error;
use std::fmt::{Display, Formatter};

use uuid::Uuid;

use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_result::{ASMOptions, ASMResult, PrepareRegisterOption};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::register_destination::word_from_byte_size;
use crate::core::constants::KEYWORDS;
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::types::type_token::{InferTypeError, TypeToken};

/// Token for a name. Basically a string that can be used as a variable name.
/// Everything is allowed except for reserved keywords and special characters in the beginning
#[derive(Debug, Eq, PartialEq, Default, Hash, Clone)]
pub struct NameToken {
    pub name: String,
}

#[derive(Debug)]
pub enum NameTokenErr {
    UnmatchedRegex { target_value: String },
    KeywordReserved(String),
}

impl Display for NameToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Error for NameTokenErr {}

impl Display for NameTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            NameTokenErr::UnmatchedRegex { target_value } => format!("\"{target_value}\" must match: ^[a-zA-Z_$][a-zA-Z_$0-9$]*$"),
            NameTokenErr::KeywordReserved(value) => {
                format!("The variable name \"{}\" variable name can't have the same name as a reserved keyword", value)
            }
        };
        write!(f, "{}", message)
    }
}

impl NameToken {
    pub fn uuid() -> NameToken {
        NameToken {
            name: Uuid::new_v4().to_string(),
        }
    }

    pub fn from_str(s: &str, allow_reserved: bool) -> Result<NameToken, NameTokenErr> {
        if !allow_reserved && KEYWORDS.iter().any(|keyword| keyword.to_lowercase() == s.to_lowercase()) {
            return Err(NameTokenErr::KeywordReserved(s.to_string()));
        }

        if !lazy_regex::regex_is_match!(r"^[a-zA-Z_$][a-zA-Z_$0-9]*$", s) {
            return Err(NameTokenErr::UnmatchedRegex {
                target_value: s.to_string(),
            });
        }

        Ok(NameToken {
            name: s.to_string()
        })
    }

    pub fn infer_type_with_context(&self, context: &StaticTypeContext, code_line: &CodeLine) -> Result<TypeToken, InferTypeError> {
        if let Some(v) = context.iter().rfind(|v| v.name_token == *self) {
            return if let Some(ty) = &v.ty {
                Ok(ty.clone())
            } else {
                Err(InferTypeError::NoTypePresent(v.name_token.clone(), v.code_line.clone()))
            };
        }

        Err(InferTypeError::UnresolvedReference(self.to_string(), code_line.clone()))
    }
}

impl ToASM for NameToken {
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        if let Some(options) = options {
            let any_t = &options as &dyn Any;
            if let Some(s) = any_t.downcast_ref::<PrepareRegisterOption>() {
                if let TypeToken::Float(_) = self.infer_type_with_context(&meta.static_type_information, &meta.code_line)? {
                    return s.transform(stack, meta);
                }
            }
        }

        if let Some(stack_location) = stack.variables.iter().rfind(|&variable| variable.name.name == self.name.as_str()) {
            if let Some(found_variable) = meta.static_type_information.context.iter().rfind(|v| v.name_token == *self) {
                if let Some(ty) = &found_variable.ty {
                    let operand_hint = word_from_byte_size(ty.byte_size());
                    return Ok(ASMResult::Inline(format!("{operand_hint} [rbp - {}]", stack_location.position + stack_location.size)));
                }
            }

            Ok(ASMResult::Inline(format!("DWORD [rbp - {}]", stack_location.position + stack_location.size)))
        } else {
            Err(ASMGenerateError::UnresolvedReference { name: self.name.to_string(), code_line: meta.code_line.clone() })
        }
    }


    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        true
    }

    fn byte_size(&self, meta: &mut MetaInfo) -> usize {
        if let Some(v) = meta.static_type_information.iter().rfind(|v| v.name_token == *self) {
            if let Some(ty) = &v.ty {
                return ty.byte_size();
            }
        }

        0
    }
}
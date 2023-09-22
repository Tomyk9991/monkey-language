use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{MetaInfo, ToASM};
use crate::core::constants::KEYWORDS;

/// Token for a name. Basically a string that can be used as a variable name.
/// Everything is allowed except for reserved keywords and special characters in the beginning
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
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
    pub fn from_str(s: &str, allow_reserved: bool) -> Result<NameToken, NameTokenErr> {
        if !allow_reserved && KEYWORDS.iter().any(|keyword| keyword.to_lowercase() == s.to_lowercase()) {
            return Err(NameTokenErr::KeywordReserved(s.to_string()));
        }

        if !lazy_regex::regex_is_match!("^[a-zA-Z_$][a-zA-Z_$0-9$]*$", s) {
            return Err(NameTokenErr::UnmatchedRegex {
                target_value: s.to_string(),
            });
        }

        Ok(NameToken {
            name: s.to_string()
        })
    }
}

impl ToASM for NameToken {
    fn to_asm(&self, stack: &mut Stack, meta: &MetaInfo) -> Result<String, crate::core::code_generator::ASMGenerateError> {
        let mut target = String::new();

        if let Some(stack_location) = stack.variables.iter().rfind(|&variable| variable.name.name == self.name.as_str()) {
            target.push_str(&format!("    ; {}\n", self));
            target.push_str(&stack.push_stack(&format!("QWORD [rsp + {}]", (stack.stack_position - stack_location.position - 1) * 8)));

            Ok(target)
        } else {
            Err(crate::core::code_generator::ASMGenerateError::UnresolvedReference { name: self.name.to_string(), code_line: meta.code_line.clone() })
        }
    }
}
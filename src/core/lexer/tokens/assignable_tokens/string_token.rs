use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::core::code_generator::{ASMGenerateError,
                                  MetaInfo, ToASM};
use crate::core::code_generator::asm_result::{ASMOptions, ASMResult};
use crate::core::code_generator::generator::Stack;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StringToken {
    pub value: String,
}

impl Display for StringToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
pub enum StringTokenErr {
    UnmatchedRegex,
}

impl Error for StringTokenErr {}

impl Display for StringTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            StringTokenErr::UnmatchedRegex => "Name must match: ^\".*\"$ ",
        })
    }
}

impl ToASM for StringToken {
    fn to_asm<T: ASMOptions>(&self, stack: &mut Stack, _meta: &mut MetaInfo, _options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        if let Some(key) = stack.data_section.str_key(&self.value) {
            return Ok(ASMResult::Inline(key.to_string()));
        }

        Ok(ASMResult::Inline(stack.create_label()))
    }


    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        8
    }

    fn data_section(&self, stack: &mut Stack, _meta: &mut MetaInfo) -> bool {
        let new_line_included = replace_add_quote(&self.value, "\\n", 10);
        let tab_included = replace_add_quote(&new_line_included, "\\t", 9);


        let key = if let Some(k) = stack.data_section.str_key(&self.value) {
            k.to_string()
        } else {
            stack.get_latest_label()
        };

        stack.data_section.push_str(&key, &tab_included);
        true
    }
}

/// replaces the occurrence with the provided number and sets quotes
/// ## Example
/// replace_add_quote("\"Hallo \n Welt\"") returns
/// \"Hallo\", 10, \"Welt\"
fn replace_add_quote(value: &str, occurrence: &str, replace_value: usize) -> String {
    format!("\"{}\"", value[1..value.len() - 1].replace(occurrence, &format!("\", {}, \"", replace_value)))
}


impl FromStr for StringToken {
    type Err = StringTokenErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !lazy_regex::regex_is_match!("^\".*\"$", s) {
            return Err(StringTokenErr::UnmatchedRegex);
        }

        Ok(StringToken {
            value: s.to_string()
        })
    }
}
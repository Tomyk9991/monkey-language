use std::collections::HashMap;
use crate::core::code_generator::asm_result::{ASMResult};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;

#[derive(Default, Debug, Clone)]
pub struct DataSection {
    data: HashMap<String, String>
}

impl DataSection {
    pub fn push_str(&mut self, key: &str, value: &str) -> bool {
        self.data.insert(key.to_string(), value.to_string()).is_some()
    }

    pub fn str_key(&self, value: &str) -> Option<&str> {

        for (k, v) in &self.data {
            if value == v {
                return Some(k);
            }
        }

        None
    }
}

impl ToASM for DataSection {
    fn to_asm<T: ASMOptions + 'static>(&self, _stack: &mut Stack, _meta: &mut MetaInfo, _options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        if self.data.is_empty() {
            return Ok(ASMResult::Inline("".to_string()))
        }
        let mut target = String::new();
        target += &ASMBuilder::line("section .data");

        for (key, value) in &self.data {
            target += &ASMBuilder::ident_line(&format!("{key}: db {value}, 0"))
        }

        target += &ASMBuilder::line("");
        target += &ASMBuilder::line("");

        Ok(ASMResult::Multiline(target))
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        0
    }
}
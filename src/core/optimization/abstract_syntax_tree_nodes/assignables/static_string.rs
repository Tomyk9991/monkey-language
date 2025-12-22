use crate::core::model::types::static_string::StaticString;
use crate::core::parser::static_type_context::StaticTypeContext;

impl StaticString {
    pub fn add(&self, right: &StaticString, _static_type_context: &StaticTypeContext) -> Option<StaticString> {
        Some(StaticString {
            value: format!("{}{}", self.value, right.value),
        })
    }
}
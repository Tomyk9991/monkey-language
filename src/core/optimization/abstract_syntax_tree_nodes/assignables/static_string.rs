use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::types::static_string::StaticString;
use crate::core::optimization::optimization_trait::{AssignmentConstFoldable, OptimizationContext};
use crate::core::parser::static_type_context::StaticTypeContext;

impl AssignmentConstFoldable for StaticString {
    fn const_fold(&self, _static_type_context: &StaticTypeContext, _optimization_context: &OptimizationContext) -> Option<Assignable> {
        Some(Assignable::String(self.clone()))
    }
}

impl StaticString {
    pub fn add(&self, right: &StaticString, _static_type_context: &StaticTypeContext) -> Option<StaticString> {
        Some(StaticString {
            value: format!("{}{}", self.value, right.value),
        })
    }
}
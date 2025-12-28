use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::types::boolean::Boolean;
use crate::core::optimization::optimization_trait::{AssignmentConstFoldable, OptimizationContext};
use crate::core::parser::static_type_context::StaticTypeContext;

impl AssignmentConstFoldable for Boolean {
    fn const_fold(&self, _static_type_context: &StaticTypeContext, _optimization_context: &OptimizationContext) -> Option<Assignable> {
        Some(Assignable::Boolean(self.clone()))
    }
}
use std::collections::HashSet;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use crate::core::model::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use crate::core::model::types::ty::Type;
use crate::core::optimization::optimization::{Optimization, OptimizationContext};
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::semantics::type_infer::infer_type::InferType;

impl Optimization for MethodCall {
    fn o1(&mut self, _: &mut StaticTypeContext, optimization: OptimizationContext) -> OptimizationContext {
        optimization
    }
}

impl MethodCall {
    pub fn method_label_name(&self, static_type_context: &mut StaticTypeContext) -> String {
        if self.identifier.identifier() == "main" {
            return "main".to_string();
        }

        let parameters = if self.arguments.is_empty() {
            "void".to_string()
        } else {
            self.arguments.iter().map(|a| a.get_type(static_type_context).unwrap_or(Type::Void).to_string()).collect::<Vec<String>>().join("_")
        }.replace('*', "ptr");


        let mut cloned_self = self.clone();
        let return_type = cloned_self.infer_type(static_type_context).unwrap_or(Type::Void).to_string().replace('*', "ptr");

        format!(".{}_{}~{}", self.identifier.identifier(), parameters, return_type)
    }
}
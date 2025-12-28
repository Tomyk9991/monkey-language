use crate::core::model::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use crate::core::model::types::ty::Type;
use crate::core::optimization::optimization_trait::{AssignmentConstFoldable, ConstFoldable, Optimization, OptimizationContext};
use crate::core::parser::static_type_context::StaticTypeContext;

impl Optimization for MethodCall {

    fn o1(&mut self, _: &mut StaticTypeContext, optimization: OptimizationContext) -> OptimizationContext {
        optimization
    }
}


// impl ConstFoldable for MethodCall {
//     fn is_const(&self) -> bool {
//         for ast_node in &self.arguments {
//             if !ast_node.is_const() {
//                 return false;
//             }
//         }
//
//         true
//     }
//
//     fn const_fold(&self, static_type_context: &StaticTypeContext, optimization_context: &OptimizationContext) -> Option<Self> where Self: Sized {
//         todo!()
//     }
// }

impl AssignmentConstFoldable for MethodCall {
    fn is_const(&self) -> bool {
        for ast_node in &self.arguments {
            if !ast_node.is_const() {
                return false;
            }
        }

        true
    }

    fn const_fold(&self, static_type_context: &StaticTypeContext, optimization_context: &OptimizationContext) -> Option<crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable> {
        if let Some(constant_assignment) = optimization_context.const_method_definitions.get(&self.method_label_name(static_type_context)) {
            return Some(constant_assignment.clone());
        }

        None
    }
}

impl MethodCall {
    pub fn method_label_name(&self, static_type_context: &StaticTypeContext) -> String {
        if self.identifier.identifier() == "main" {
            return "main".to_string();
        }

        let parameters = if self.arguments.is_empty() {
            "void".to_string()
        } else {
            self.arguments.iter().map(|a| a.get_type(static_type_context).unwrap_or(Type::Void).to_string()).collect::<Vec<String>>().join("_")
        }.replace('*', "ptr");

        let return_type = self.get_type(static_type_context).unwrap_or(Type::Void).to_string().replace('*', "ptr");
        format!(".{}_{}~{}", self.identifier.identifier(), parameters, return_type)
    }
}
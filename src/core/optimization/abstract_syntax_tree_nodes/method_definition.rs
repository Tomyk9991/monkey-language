use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::method_definition::MethodDefinition;
use crate::core::optimization::optimization_trait::{ConstFoldable, Optimization, OptimizationContext};
use crate::core::parser::static_type_context::StaticTypeContext;

impl Optimization for MethodDefinition {
    fn o1(&mut self, static_type_context: &mut StaticTypeContext, optimization: OptimizationContext) -> OptimizationContext {
        let mut current_optimization_context: OptimizationContext = optimization;

        for ast_node in &mut self.stack {
            current_optimization_context = ast_node.o1(static_type_context, current_optimization_context);
        }

        let context = current_optimization_context.clone();

        if self.is_const() {
            if let Some(folded) = self.const_fold(static_type_context, &context) {
                if let [AbstractSyntaxTreeNode::Return(return_node)] = &folded.stack[..] {
                    if let Some(assignable) = &return_node.assignable {
                        if let Some(const_folded_assignment) = assignable.const_fold(static_type_context, &context) {
                            current_optimization_context.const_method_definitions.insert(self.method_label_name(), const_folded_assignment);
                        }
                    }
                }

                *self = folded;
            }
        }

        OptimizationContext {
            constant_variables: current_optimization_context.constant_variables,
            const_method_definitions: current_optimization_context.const_method_definitions,
        }
    }
}

impl ConstFoldable for MethodDefinition {
    fn is_const(&self) -> bool {
        for ast_node in &self.stack {
            if !ast_node.is_const() {
                return false;
            }
        }

        true
    }

    fn const_fold(&self, static_type_context: &StaticTypeContext, optimization_context: &OptimizationContext) -> Option<Self> {
        // const fold itself, as long as changes are made
        let mut folded = vec![];

        for ast_node in &self.stack {
            if let Some(folded_node) = ast_node.const_fold(static_type_context, optimization_context) {
                folded.push(folded_node);
            } else {
                folded.push(ast_node.clone());
            }
        }

        Some(MethodDefinition {
            identifier: self.identifier.clone(),
            arguments: self.arguments.clone(),
            return_type: self.return_type.clone(),
            is_extern: self.is_extern,
            stack: folded,
            file_position: self.file_position.clone(),
        })
    }
}
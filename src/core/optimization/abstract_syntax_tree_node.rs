use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::optimization::optimization_trait::{Optimization, OptimizationContext};
use crate::core::parser::static_type_context::StaticTypeContext;

impl Optimization for AbstractSyntaxTreeNode {
    fn o1(&mut self, static_type_context: &mut StaticTypeContext, optimization: OptimizationContext) -> OptimizationContext {
        match self {
            AbstractSyntaxTreeNode::MethodDefinition(node) => {
                node.o1(static_type_context, optimization)
            },
            AbstractSyntaxTreeNode::MethodCall(method_call) => {
                method_call.o1(static_type_context, optimization)
            },
            AbstractSyntaxTreeNode::Variable(variable) => {
                variable.o1(static_type_context, optimization)
            }
            // Add other AST node types and their respective o1 implementations here
            _ => optimization,
        }
    }
}
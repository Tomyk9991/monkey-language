use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::optimization::optimization_trait::{AssignmentConstFoldable, ConstFoldable, Optimization, OptimizationContext};
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

impl ConstFoldable for AbstractSyntaxTreeNode {
    fn is_const(&self) -> bool {
        match self {
            AbstractSyntaxTreeNode::MethodDefinition(node) => node.is_const(),
            AbstractSyntaxTreeNode::MethodCall(node) => node.is_const(),
            AbstractSyntaxTreeNode::Variable(node) => node.is_const(),
            AbstractSyntaxTreeNode::Import(_) => true,
            AbstractSyntaxTreeNode::Return(node) => node.is_const(),
            AbstractSyntaxTreeNode::If(node) => node.is_const(),
            AbstractSyntaxTreeNode::For(node) => node.is_const(),
            AbstractSyntaxTreeNode::While(node) => node.is_const(),
            AbstractSyntaxTreeNode::StructDefinition(_) => false
        }
    }

    fn const_fold(&self, static_type_context: &StaticTypeContext, optimization_context: &OptimizationContext) -> Option<Self> {
        match self {
            AbstractSyntaxTreeNode::MethodDefinition(node) => node.const_fold(static_type_context, optimization_context).map(AbstractSyntaxTreeNode::MethodDefinition),
            AbstractSyntaxTreeNode::MethodCall(_) => todo!(),
            AbstractSyntaxTreeNode::Variable(node) => node.const_fold(static_type_context, optimization_context).map(AbstractSyntaxTreeNode::Variable),
            AbstractSyntaxTreeNode::Import(_) => None,
            AbstractSyntaxTreeNode::Return(node) => node.const_fold(static_type_context, optimization_context).map(AbstractSyntaxTreeNode::Return),
            AbstractSyntaxTreeNode::If(node) => node.const_fold(static_type_context, optimization_context).map(AbstractSyntaxTreeNode::If),
            AbstractSyntaxTreeNode::For(node) => node.const_fold(static_type_context, optimization_context).map(AbstractSyntaxTreeNode::For),
            AbstractSyntaxTreeNode::While(node) => node.const_fold(static_type_context, optimization_context).map(AbstractSyntaxTreeNode::While),
            AbstractSyntaxTreeNode::StructDefinition(_) => None,
        }
    }
}
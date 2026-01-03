use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::types::ty::Type;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::InferTypeError;
use crate::core::semantics::type_infer::infer_type::InferType;

impl InferType for AbstractSyntaxTreeNode {
    fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<Type, Box<InferTypeError>> {
        let ty = match self {
            AbstractSyntaxTreeNode::Variable(variable) => variable.infer_type(type_context)?,
            AbstractSyntaxTreeNode::If(if_definition) => if_definition.infer_type(type_context)?,
            AbstractSyntaxTreeNode::For(for_loop) => for_loop.infer_type(type_context)?,
            AbstractSyntaxTreeNode::While(while_loop) => while_loop.infer_type(type_context)?,
            AbstractSyntaxTreeNode::MethodCall(method_call) => method_call.infer_type(type_context)?, 
            AbstractSyntaxTreeNode::MethodDefinition(method_definition) => method_definition.infer_type(type_context)?,
            AbstractSyntaxTreeNode::StructDefinition(_) | AbstractSyntaxTreeNode::Import(_) | AbstractSyntaxTreeNode::Return(_) => Type::Statement,
        };

        Ok(ty)
    }
}
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::InferTypeError;
use crate::core::semantics::type_infer::infer_type::InferType;

/// Recursively infer types for all nodes in the scope
pub fn infer_type(scope: &mut Vec<AbstractSyntaxTreeNode>) -> Result<(), InferTypeError> {
    let mut type_context: StaticTypeContext = StaticTypeContext::new(scope);
    infer_type_rec(scope, &mut type_context)?;
    
    Ok(())
}

fn infer_type_rec(scope: &mut Vec<AbstractSyntaxTreeNode>, type_context: &mut StaticTypeContext) -> Result<(), InferTypeError> {
    for node in &mut scope.iter_mut() {
        let file_position = node.file_position();
        type_context.current_file_position = file_position.clone();
        node.infer_type(type_context)?;
    }

    Ok(())
}
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::types::ty::Type;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::InferTypeError;
use crate::core::semantics::type_infer::infer_type::InferType;

impl InferType for Vec<AbstractSyntaxTreeNode> {
    fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<Type, Box<InferTypeError>> {
        let variables_len = type_context.len();

        let scoped_checker = StaticTypeContext::new(self);
        type_context.merge(scoped_checker);

        for node in self {
            type_context.current_file_position = node.file_position();
            node.infer_type(type_context)?;
        }

        let amount_pop = type_context.len() - variables_len;

        for _ in 0..amount_pop {
            let _ = type_context.pop();
        }

        Ok(Type::Statement)
    }
}
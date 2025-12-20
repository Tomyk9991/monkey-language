use crate::core::model::abstract_syntax_tree_nodes::while_::While;
use crate::core::model::types::ty::Type;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::InferTypeError;
use crate::core::semantics::type_infer::infer_type::InferType;

impl InferType for While {
    fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<Type, Box<InferTypeError>> {
        self.condition.infer_type(type_context)?;
        self.stack.infer_type(type_context)?;

        Ok(Type::Statement)
    }
}
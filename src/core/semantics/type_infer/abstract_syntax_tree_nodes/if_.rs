use crate::core::model::abstract_syntax_tree_nodes::if_::If;
use crate::core::model::types::ty::Type;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::InferTypeError;
use crate::core::semantics::type_infer::infer_type::InferType;

impl InferType for If {
    fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<Type, Box<InferTypeError>> {
        self.condition.infer_type(type_context)?;

        self.if_stack.infer_type(type_context)?;

        if let Some(else_stack) = &mut self.else_stack {
            else_stack.infer_type(type_context)?;
        }

        Ok(Type::Statement)
    }
}
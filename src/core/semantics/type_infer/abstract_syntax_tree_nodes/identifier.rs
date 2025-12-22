use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::types::mutability::Mutability;
use crate::core::model::types::ty::Type;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::InferTypeError;
use crate::core::semantics::type_infer::infer_type::InferType;

impl InferType for Identifier {
    fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<Type, Box<InferTypeError>> {
        if let Some(v) = type_context.iter().rfind(|v| {
            if let LValue::Identifier(n) = &v.l_value {
                n.name == *self.name
            } else {
                false
            }
        }) {
            return if let Some(ty) = &v.ty {
                let mut ty = ty.clone();
                ty.set_mutability(Mutability::from(v.mutability));

                Ok(ty)
            } else {
                Err(Box::new(InferTypeError::NoTypePresent(v.l_value.clone(), type_context.current_file_position.clone())))
            };
        }

        Err(Box::new(InferTypeError::UnresolvedReference(self.to_string(), type_context.current_file_position.clone())))
    }
}
use crate::core::model::types::ty::Type;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::InferTypeError;

pub trait InferType {
    /// Infers the type of the implementing node and updates the provided type context accordingly.
    /// Also returns the inferred type.
    fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<Type, Box<InferTypeError>>;
}
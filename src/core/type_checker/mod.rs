use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::types::r#type::InferTypeError;
use crate::core::type_checker::static_type_checker::StaticTypeCheckError;

pub mod static_type_checker;

pub trait InferType {
    fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<(), InferTypeError>;
}

pub trait StaticTypeCheck {
    /// This function is used to check the static type of AST node and its assignment.
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError>;
}
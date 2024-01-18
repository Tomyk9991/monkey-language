use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::types::type_token::InferTypeError;

pub mod static_type_checker;

pub trait InferType {
    fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<(), InferTypeError>;
}
use crate::core::code_generator::conventions;
use crate::core::model::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use crate::core::model::types::ty::Type;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::InferTypeError;
use crate::core::semantics::type_infer::infer_type::InferType;

impl InferType for MethodCall {
    fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<Type, Box<InferTypeError>> {
        if let Some(method_def) = conventions::method_definitions(type_context, &self.arguments, &self.identifier.identifier())?.first() {
            return Ok(method_def.return_type.clone());
        }

        Err(Box::new(InferTypeError::UnresolvedReference(self.to_string(), type_context.current_file_position.clone())))
    }
}

impl MethodCall {
    pub fn get_type(&self, type_context: &StaticTypeContext) -> Option<Type> {
        if let Some(method_def) = conventions::method_definitions(type_context, &self.arguments, &self.identifier.identifier()).ok()?.first() {
            return Some(method_def.return_type.clone());
        }

        None
    }
}
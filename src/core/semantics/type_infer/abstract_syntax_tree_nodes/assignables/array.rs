use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::types::array::Array;
use crate::core::model::types::mutability::Mutability;
use crate::core::model::types::ty::Type;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::InferTypeError;
use crate::core::semantics::type_infer::infer_type::InferType;

impl InferType for Array {
    fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<Type, Box<InferTypeError>> {
        let file_position = type_context.current_file_position.clone();

        if self.values.is_empty() {
            return Err(Box::new(InferTypeError::NoTypePresent(LValue::Identifier(Identifier { name: "Array".to_string() }), file_position.clone())))
        }

        if let Ok(ty) = self.values[0].infer_type(type_context) {
            return Ok(Type::Array(Box::new(ty), self.values.len(), Mutability::Immutable));
        }

        Err(Box::new(InferTypeError::NoTypePresent(LValue::Identifier(Identifier { name: "Array".to_string() }), file_position)))
    }
}
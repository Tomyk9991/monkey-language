use crate::core::model::abstract_syntax_tree_nodes::assignables::object::Object;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::InferTypeError;
use crate::core::semantics::static_type_check::static_type_checker::StaticTypeCheckError;
use crate::core::semantics::static_type_check::StaticTypeCheck;

impl StaticTypeCheck for Object {
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        for field in &self.fields {
            field.assignable.static_type_check(type_context)?;
        }

        let custom_defined_type = type_context.custom_defined_types.get(&self.ty).ok_or(StaticTypeCheckError::TypeDefinitionMissing {
            expected_type: self.ty.clone(),
            file_position: type_context.current_file_position.clone(),
        })?;

        // every field must match the defined type
        for field in &self.fields {
            let actual_name = field.l_value.identifier();
            let actual_type = field.ty.clone().ok_or(StaticTypeCheckError::NoTypePresent {
                name: LValue::Identifier(Identifier { name: String::new() }),
                file_position: type_context.current_file_position.clone(),
            })?;

            let expected_type_from_definition = custom_defined_type.fields.iter().find(|f| f.name.name == actual_name).ok_or(StaticTypeCheckError::TypeDefinitionMissing {
                expected_type: self.ty.clone(),
                file_position: type_context.current_file_position.clone(),
            })?.ty.clone();

            if actual_type < expected_type_from_definition {
                return Err(Box::new(InferTypeError::MismatchedTypes {
                    expected: expected_type_from_definition.clone(),
                    actual: actual_type.clone(),
                    file_position: type_context.current_file_position.clone()
                }).into());
            }
        }

        Ok(())
    }
}
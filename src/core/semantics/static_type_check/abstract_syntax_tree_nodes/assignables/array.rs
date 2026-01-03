use crate::core::lexer::token_with_span::FilePosition;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::types::array::Array;
use crate::core::model::types::ty::Type;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::InferTypeError;
use crate::core::semantics::static_type_check::static_type_checker::StaticTypeCheckError;
use crate::core::semantics::static_type_check::StaticTypeCheck;

impl StaticTypeCheck for Array {
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        // check if all types are equal, where the first type is the expected type
        let all_types = self.values
            .iter()
            .map(|a| a.get_type(type_context).ok_or(InferTypeError::NoTypePresent( // we can use a dummy identifier here since the overlaying variable will provide the correct one
                LValue::Identifier(Identifier { name: String::new() }),
                FilePosition::default(),
            )))
            .collect::<Vec<Result<Type, InferTypeError>>>();

        if !all_types.is_empty() {
            let first_type = &all_types[0];
            if let Ok(first_type) = first_type {
                for (index, current_type) in all_types.iter().enumerate() {
                    if let Ok(current_type) = current_type {
                        if current_type != first_type {
                            return Err(StaticTypeCheckError::InferredError(Box::new(InferTypeError::MultipleTypesInArray {
                                expected: first_type.clone(),
                                unexpected_type: current_type.clone(),
                                unexpected_type_index: index,
                                file_position: FilePosition::default(),
                            })))
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
}
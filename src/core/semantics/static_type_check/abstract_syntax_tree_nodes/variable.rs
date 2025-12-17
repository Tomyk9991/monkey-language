use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::types::ty::Type;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::InferTypeError;
use crate::core::semantics::static_type_check::static_type_check::StaticTypeCheck;
use crate::core::semantics::static_type_check::static_type_checker::StaticTypeCheckError;

impl StaticTypeCheck for Variable<'=', ';'> {
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        if self.define {
            if let Assignable::Array(array) = &self.assignable {
                // check if all types are equal, where the first type is the expected type
                let all_types = array.values
                    .iter()
                    .map(|a| a.get_type(type_context).ok_or(InferTypeError::NoTypePresent(
                        LValue::Identifier(Identifier { name: a.identifier().unwrap_or(self.l_value.identifier()) }),
                        self.file_position.clone(),
                    )))
                    .collect::<Vec<Result<Type, InferTypeError>>>();

                if !all_types.is_empty() {
                    let first_type = &all_types[0];
                    if let Ok(first_type) = first_type {
                        for (index, current_type) in all_types.iter().enumerate() {
                            if let Ok(current_type) = current_type {
                                if current_type != first_type {
                                    return Err(StaticTypeCheckError::InferredError(InferTypeError::MultipleTypesInArray {
                                        expected: first_type.clone(),
                                        unexpected_type: current_type.clone(),
                                        unexpected_type_index: index,
                                        file_position: self.file_position.clone(),
                                    }))
                                }
                            }
                        }
                    }
                }
            }

            let ty = self.assignable.get_type(type_context);
            if matches!(ty, Some(Type::Void) | Some(Type::Statement)) {
                return Err(StaticTypeCheckError::VoidType { assignable: self.assignable.clone(), file_position: self.file_position.clone() });
            }


            if self.ty.is_some() {
                type_context.context.push(self.clone());
                return Ok(());
            }
        }

        if !self.define {
            if let Some(found_variable) = type_context.iter().rfind(|v| v.l_value.identifier() == self.l_value.identifier()) {
                let inferred_type = self.assignable.get_type(type_context).ok_or(StaticTypeCheckError::NoTypePresent {
                    name: self.l_value.clone(),
                    file_position: self.file_position.clone(),
                })?;
                if let Some(ty) = &found_variable.ty {
                    if ty > &inferred_type {
                        return Err(InferTypeError::MismatchedTypes { expected: ty.clone(), actual: inferred_type.clone(), file_position: self.file_position.clone() }.into());
                    }

                    if !found_variable.mutability {
                        return Err(StaticTypeCheckError::ImmutabilityViolated {
                            name: self.l_value.clone(),
                            file_position: self.file_position.clone(),
                        });
                    }
                } else {
                    return Err(StaticTypeCheckError::NoTypePresent { name: self.l_value.clone(), file_position: self.file_position.clone() });
                }
            } else {
                return Err(StaticTypeCheckError::UnresolvedReference { name: self.l_value.clone(), file_position: self.file_position.clone() });
            }
        }

        Ok(())
    }
}
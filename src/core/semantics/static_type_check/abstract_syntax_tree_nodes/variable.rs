use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::types::ty::Type;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::InferTypeError;
use crate::core::semantics::static_type_check::static_type_checker::StaticTypeCheckError;
use crate::core::semantics::static_type_check::StaticTypeCheck;

impl StaticTypeCheck for Variable<'=', ';'> {
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        if self.define {
            self.assignable.static_type_check(type_context).map_err(self.map_inner_static_type_check_error())?;
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
                        return Err(Box::new(InferTypeError::MismatchedTypes { expected: ty.clone(), actual: inferred_type.clone(), file_position: self.file_position.clone() }).into());
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

impl Variable<'=', ';'> {
    fn map_inner_static_type_check_error(&self) -> Box<dyn FnOnce(StaticTypeCheckError) -> StaticTypeCheckError + '_> {
        let identifier = self.l_value.identifier();
        let self_file_position = self.file_position.clone();

        Box::new(move |e| {
            match e {
                StaticTypeCheckError::InferredError(infer_error) => {
                    StaticTypeCheckError::InferredError(Box::new(match *infer_error {
                        InferTypeError::NoTypePresent(..) => InferTypeError::NoTypePresent(
                            LValue::Identifier(Identifier { name: identifier }),
                            self_file_position,
                        ),
                        InferTypeError::MismatchedTypes { expected, actual, file_position: _} => InferTypeError::MismatchedTypes {
                            expected,
                            actual,
                            file_position: self_file_position,
                        },
                        InferTypeError::MultipleTypesInArray { expected, unexpected_type, unexpected_type_index, file_position: _ } => InferTypeError::MultipleTypesInArray {
                            expected,
                            unexpected_type,
                            unexpected_type_index,
                            file_position: self_file_position,
                        },
                        other => other,
                    }))
                },
                other => other
            }
        })
    }
}
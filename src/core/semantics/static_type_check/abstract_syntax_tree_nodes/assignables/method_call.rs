use crate::core::model::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::{InferTypeError, MethodCallArgumentTypeMismatch};
use crate::core::semantics::static_type_check::static_type_checker::StaticTypeCheckError;
use crate::core::semantics::static_type_check::StaticTypeCheck;

impl StaticTypeCheck for MethodCall {
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        let method_defs = type_context.methods.iter().filter(|m| m.identifier == self.identifier).collect::<Vec<_>>();

        'outer: for method_def in &method_defs {
            if method_def.arguments.len() != self.arguments.len() {
                if method_defs.len() == 1 {
                    return Err(StaticTypeCheckError::InferredError(Box::new(InferTypeError::MethodCallArgumentAmountMismatch {
                        expected: method_def.arguments.len(),
                        actual: self.arguments.len(),
                        file_position: self.file_position.clone(),
                    })));
                }

                continue;
            }

            let zipped = method_def.arguments
                .iter()
                .zip(&self.arguments);

            for (index, (argument_def, argument_call)) in zipped.enumerate() {
                let def_type = argument_def.ty.clone();
                let call_type = argument_call.get_type(type_context).ok_or(Box::new(InferTypeError::NoTypePresent(
                    LValue::Identifier(Identifier { name: argument_call.identifier().unwrap_or(self.identifier.identifier()) }),
                    self.file_position.clone(),
                )))?;

                if def_type < call_type {
                    if method_defs.len() == 1 {
                        return Err(StaticTypeCheckError::InferredError(Box::new(InferTypeError::MethodCallArgumentTypeMismatch {
                            info: Box::new(MethodCallArgumentTypeMismatch {
                                expected: def_type,
                                actual: call_type,
                                nth_parameter: index + 1,
                                file_position: self.file_position.clone(),
                            })
                        })));
                    }

                    continue 'outer;
                }
            }

            return Ok(());
        }

        if method_defs.is_empty() {
            return Err(StaticTypeCheckError::InferredError(Box::new(InferTypeError::UnresolvedReference(self.identifier.identifier(), self.file_position.clone()))));
        }

        let signatures = method_defs
            .iter()
            .map(|m| m.arguments.iter().map(|a| a.ty.clone()).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Err(StaticTypeCheckError::InferredError(Box::new(InferTypeError::MethodCallSignatureMismatch {
            signatures,
            method_name: self.identifier.clone(),
            file_position: self.file_position.clone(),
            provided: self.arguments.iter().filter_map(|a| a.get_type(type_context)).collect::<Vec<_>>(),
        })))
    }
}
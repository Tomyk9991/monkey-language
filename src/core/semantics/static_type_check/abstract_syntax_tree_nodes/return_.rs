use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::abstract_syntax_tree_nodes::ret::Return;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::InferTypeError;
use crate::core::semantics::static_type_check::static_type_check::StaticTypeCheck;
use crate::core::semantics::static_type_check::static_type_checker::StaticTypeCheckError;

impl StaticTypeCheck for Return {
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        if let Some(expected_return_type) = &type_context.expected_return_type {
            if let Some(assignable) = &self.assignable {
                let actual_type = assignable.get_type(type_context).ok_or(StaticTypeCheckError::NoTypePresent {
                    name: LValue::Identifier(Identifier { name: "return".to_string() }),
                    file_position: self.file_position.clone(),
                })?;

                if expected_return_type.return_type < actual_type {
                    return Err(StaticTypeCheckError::InferredError(InferTypeError::MethodReturnArgumentTypeMismatch {
                        expected: expected_return_type.return_type.clone(),
                        actual: actual_type,
                        file_position: self.file_position.clone()
                    }));
                }
            }
        }

        Ok(())
    }
}
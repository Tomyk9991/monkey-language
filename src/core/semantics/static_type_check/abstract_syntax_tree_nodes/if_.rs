use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::abstract_syntax_tree_nodes::if_::If;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::types::mutability::Mutability;
use crate::core::model::types::ty::Type;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::InferTypeError;
use crate::core::semantics::static_type_check::static_type_checker::{static_type_check_rec, StaticTypeCheckError};
use crate::core::semantics::static_type_check::StaticTypeCheck;

impl StaticTypeCheck for If {
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        let variables_len = type_context.context.len();
        let condition_type = self.condition.get_type(type_context).ok_or(
            StaticTypeCheckError::InferredError(Box::new(InferTypeError::NoTypePresent(
                LValue::Identifier(Identifier { name: "if condition".to_string() }),
                self.file_position.clone(),
            )))
        )?;

        if !matches!(condition_type, Type::Bool(_)) {
            return Err(StaticTypeCheckError::InferredError(Box::new(InferTypeError::MismatchedTypes {
                expected: Type::Bool(Mutability::Immutable),
                actual: condition_type,
                file_position: self.file_position.clone(),
            })));
        }

        static_type_check_rec(&self.if_stack, type_context)?;

        let amount_pop = type_context.context.len() - variables_len;

        for _ in 0..amount_pop {
            let _ = type_context.context.pop();
        }

        if let Some(else_stack) = &self.else_stack {
            let variables_len = type_context.context.len();

            static_type_check_rec(else_stack, type_context)?;

            let amount_pop = type_context.context.len() - variables_len;

            for _ in 0..amount_pop {
                let _ = type_context.context.pop();
            }
        }

        Ok(())
    }
}
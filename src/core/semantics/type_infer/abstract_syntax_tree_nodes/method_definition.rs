use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::abstract_syntax_tree_nodes::method_definition::MethodDefinition;
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::types::ty::Type;
use crate::core::parser::static_type_context::{CurrentMethodInfo, StaticTypeContext};
use crate::core::parser::types::r#type::{InferTypeError, MethodCallSignatureMismatchCause};
use crate::core::semantics::type_infer::infer_type::InferType;

impl InferType for MethodDefinition {
    fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<Type, Box<InferTypeError>> {
        // add the parameters to the type information
        for argument in &self.arguments {
            type_context.context.push(Variable {
                l_value: argument.identifier.clone(),
                mutability: argument.ty.mutable(),
                ty: Some(argument.ty.clone()),
                define: true,
                assignable: Assignable::default(),
                file_position: Default::default(),
            });
        }

        let variables_len = type_context.context.len();
        type_context.expected_return_type = Some(CurrentMethodInfo {
            return_type: self.return_type.clone(),
            method_header_line: self.file_position.clone(),
            method_name: self.identifier.identifier(),
        });


        self.stack.infer_type(type_context)?;

        if self.return_type != Type::Void {
            if let [.., last] = &self.stack[..] {
                let mut method_return_signature_mismatch = false;
                let mut cause = MethodCallSignatureMismatchCause::ReturnMismatch;

                if let AbstractSyntaxTreeNode::If(if_definition) = &last {
                    method_return_signature_mismatch = !if_definition.ends_with_return_in_each_branch();
                    if method_return_signature_mismatch {
                        cause = MethodCallSignatureMismatchCause::IfCondition;
                    }
                } else if !matches!(last, AbstractSyntaxTreeNode::Return(_)) {
                    method_return_signature_mismatch = true;
                }

                if method_return_signature_mismatch {
                    if let Some(expected_return_type) = &type_context.expected_return_type {
                        return Err(Box::new(InferTypeError::MethodReturnSignatureMismatch {
                            expected: expected_return_type.return_type.clone(),
                            method_name: expected_return_type.method_name.to_string(),
                            cause,
                            file_position: last.file_position().clone(),
                        }));
                    }
                }
            }
        }


        let amount_pop = (type_context.context.len() - variables_len) + self.arguments.len();

        for _ in 0..amount_pop {
            let _ = type_context.context.pop();
        }

        type_context.expected_return_type = None;
        Ok(Type::Statement)
    }
}
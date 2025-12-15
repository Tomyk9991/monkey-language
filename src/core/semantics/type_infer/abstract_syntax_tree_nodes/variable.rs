use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::types::ty::Type;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::InferTypeError;
use crate::core::semantics::type_infer::infer_type::InferType;

impl<const ASSIGNMENT: char, const SEPARATOR: char> InferType for Variable<ASSIGNMENT, SEPARATOR> {
    fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<Type, InferTypeError> {
        if type_context.methods.iter().filter(|a| a.identifier == self.l_value).count() > 0 {
            return Err(InferTypeError::NameCollision(self.l_value.identifier(), self.file_position.clone()));
        }

        if !self.define {
            return Ok(self.assignable.infer_type(type_context)?);
        }

        match &self.ty {
            // validity check. is the assignment really the type the programmer used
            // example: let a: i32 = "Hallo"; is not valid since you're assigning a string to an integer

            // if type is present. check, if the type matches the assignment
            // else infer the type with a context
            Some(ty) => {
                let inferred_type = self.assignable.infer_type(type_context)?;

                if ty < &inferred_type {
                    // let a: i64 = 5; instead of let a: i32 = 5;
                    if let Some(implicit_cast) = inferred_type.implicit_cast_to(&mut self.assignable, ty, &type_context.current_file_position)? {
                        self.ty = Some(implicit_cast.clone());
                        return Ok(implicit_cast);
                    } else {
                        return Err(InferTypeError::MismatchedTypes { expected: ty.clone(), actual: inferred_type.clone(), file_position: type_context.current_file_position.clone() });
                    }
                }

                Ok(ty.clone())
            }
            None => {
                let ty = self.assignable.infer_type(type_context)?;
                self.ty = Some(ty.clone());
                type_context.push(Variable {
                    l_value: self.l_value.clone(),
                    ty: Some(ty.clone()),
                    define: self.define,
                    assignable: self.assignable.clone(),
                    mutability: self.mutability,
                    file_position: self.file_position.clone(),
                });

                Ok(ty)
            }
        }
    }
}
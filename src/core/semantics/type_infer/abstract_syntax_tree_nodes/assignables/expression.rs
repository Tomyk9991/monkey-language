use std::collections::HashMap;
use crate::core::code_generator::MetaInfo;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::operator::Operator;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::prefix_arithmetic::{PointerArithmetic, PrefixArithmetic};
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::types::float::FloatType;
use crate::core::model::types::integer::IntegerType;
use crate::core::model::types::mutability::Mutability;
use crate::core::model::types::ty::Type;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::boolean::Boolean;
use crate::core::parser::types::r#type::InferTypeError;
use crate::core::semantics::type_infer::infer_type::InferType;

impl InferType for Expression {
    fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<Type, Box<InferTypeError>> {
        if let Some(value) = &mut self.value {
            let mut value_type = value.infer_type(type_context);
            let has_prefix_arithmetics = self.prefix_arithmetic.is_some();
            let has_index_operation = self.index_operator.is_some();

            if has_index_operation {
                if let Some(index_operator) = &mut self.index_operator {
                    let index_type = index_operator.infer_type(type_context)?;
                    if !matches!(index_type, Type::Integer(_, _)) {
                        return Err(Box::new(InferTypeError::IllegalIndexOperation(index_type, type_context.current_file_position.clone())));
                    }
                }
                let value_type_cloned = value_type?.clone();
                if let Some(element_type) = value_type_cloned.pop_array() {
                    value_type = Ok(element_type);
                } else {
                    return Err(Box::new(InferTypeError::IllegalArrayTypeLookup(value_type_cloned, type_context.current_file_position.clone())));
                }
            }

            return if let (true, Ok(value_type)) = (has_prefix_arithmetics, &value_type) {
                let current_pointer_arithmetic: String = match value_type {
                    Type::Custom(name, _) if name.name.starts_with(['*', '&']) => {
                        if let Some(index) = name.name.chars().position(|m| m.is_ascii_alphanumeric()) {
                            name.name[..index].to_string()
                        } else {
                            "".to_string()
                        }
                    }
                    _ => "".to_string()
                };

                let mut value_type = value_type.clone();

                if let Some(prefix_arithmetic) = &self.prefix_arithmetic {
                    match prefix_arithmetic {
                        PrefixArithmetic::PointerArithmetic(PointerArithmetic::Asterics) if current_pointer_arithmetic.ends_with('*') => {
                            if let Some(new_ty) = value_type.pop_pointer() {
                                value_type = new_ty;
                            } else {
                                return Err(Box::new(InferTypeError::IllegalDereference(*value.clone(), value_type, type_context.current_file_position.clone())));
                            }
                        }
                        PrefixArithmetic::PointerArithmetic(PointerArithmetic::Ampersand) => {
                            value_type = value_type.push_pointer();
                        }
                        PrefixArithmetic::PointerArithmetic(PointerArithmetic::Asterics) => {
                            // just using & in front of non pointer types is illegal. Dereferencing non pointers doesn't make any sense
                            return Err(Box::new(InferTypeError::IllegalDereference(*value.clone(), value_type, type_context.current_file_position.clone())));
                        }
                        PrefixArithmetic::Cast(casting_to) => {
                            value_type = Type::from_str(&casting_to.to_string(), Mutability::Immutable)?;
                        }
                        PrefixArithmetic::Operation(_) => {}
                    }
                }

                if value_type.is_pointer() {
                    Ok(Type::Custom(Identifier { name: format!("{}", value_type) }, Mutability::from(value_type.mutable())))
                } else {
                    Ok(value_type)
                }
            } else {
                value_type
            };
        }

        Self::infer_type_after_operation(self.to_string(), &mut self.lhs, self.operator, &mut self.rhs, type_context)
    }

}

impl Expression {
    pub fn get_type(&self, type_context: &StaticTypeContext) -> Option<Type> {
        let mut type_context_cloned = type_context.clone();
        let mut self_cloned = self.clone();

        self_cloned.infer_type(&mut type_context_cloned).ok()
    }

    pub fn traverse_type(&mut self, meta: &mut MetaInfo) -> Option<Type> {
        self.infer_type(&mut meta.static_type_information).ok()
    }

    fn infer_type_after_operation(error_message: String, lhs: &mut Option<Box<Expression>>, operator: Operator, rhs: &mut Option<Box<Expression>>, context: &mut StaticTypeContext) -> Result<Type, Box<InferTypeError>> {
            if let Some(lhs) = lhs.as_mut() {
                if let Some(rhs) = rhs.as_mut() {
                    let lhs_type = lhs.infer_type(context)?;
                    let rhs_type = rhs.infer_type(context)?;

                    let mut base_type_matrix: HashMap<(Type, Operator, Type), Type> = HashMap::new();
                    base_type_matrix.insert((Type::Custom(Identifier { name: "string".to_string() }, Mutability::Immutable), Operator::Add, Type::Custom(Identifier { name: "string".to_string() }, Mutability::Immutable)), Type::Custom(Identifier { name: "*string".to_string() }, Mutability::Immutable));

                    IntegerType::operation_matrix(&mut base_type_matrix);
                    FloatType::operation_matrix(&mut base_type_matrix);
                    Boolean::operation_matrix(&mut base_type_matrix);

                    // I do not care, if the expression is mutable or not. The type is the relevant factor
                    let mut lhs_clone = lhs_type.clone();
                    lhs_clone.set_mutability(Mutability::Immutable);
                    let mut rhs_clone = rhs_type.clone();
                    rhs_clone.set_mutability(Mutability::Immutable);

                    if let Some(result_type) = base_type_matrix.get(&(lhs_clone, operator, rhs_clone)) {
                        return Ok(result_type.clone());
                    }

                    return Err(Box::new(InferTypeError::TypesNotCalculable(lhs_type, operator, rhs_type, context.current_file_position.clone())));
                }
            }

            Err(Box::new(InferTypeError::UnresolvedReference(error_message, context.current_file_position.clone())))
    }

    // fn check_operator_compatibility(error_message: String, lhs: &mut Option<Box<Expression>>, operator: Operator, rhs: &mut Option<Box<Expression>>, context: &mut StaticTypeContext) -> Result<Type, InferTypeError> {
    //     if let Some(lhs) = &mut lhs {
    //         if let Some(rhs) = &mut rhs {
    //             let lhs_type = lhs.infer_type(context)?;
    //             let rhs_type = rhs.infer_type(context)?;
    //
    //             let mut base_type_matrix: HashMap<(Type, Operator, Type), Type> = HashMap::new();
    //             base_type_matrix.insert((Type::Custom(Identifier { name: "string".to_string() }, Mutability::Immutable), Operator::Add, Type::Custom(Identifier { name: "string".to_string() }, Mutability::Immutable)), Type::Custom(Identifier { name: "*string".to_string() }, Mutability::Immutable));
    //
    //             IntegerType::operation_matrix(&mut base_type_matrix);
    //             FloatType::operation_matrix(&mut base_type_matrix);
    //             Boolean::operation_matrix(&mut base_type_matrix);
    //
    //             // I do not care, if the expression is mutable or not. The type is the relevant factor
    //             let mut lhs_clone = lhs_type.clone();
    //             lhs_clone.set_mutability(Mutability::Immutable);
    //             let mut rhs_clone = rhs_type.clone();
    //             rhs_clone.set_mutability(Mutability::Immutable);
    //
    //             if let Some(result_type) = base_type_matrix.get(&(lhs_clone, operator.clone(), rhs_clone)) {
    //                 return Ok(result_type.clone());
    //             }
    //
    //             return Err(InferTypeError::TypesNotCalculable(lhs_type, operator, rhs_type, context.current_file_position.clone()));
    //         }
    //     }
    //
    //     Err(InferTypeError::UnresolvedReference(error_message, context.current_file_position.clone()))
    // }
}
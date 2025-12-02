use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::str::FromStr;

use crate::core::code_generator::{MetaInfo};
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::operator::Operator;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::prefix_arithmetic::{PointerArithmetic, PrefixArithmetic};
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::types::float::FloatType;
use crate::core::model::types::integer::IntegerType;
use crate::core::model::types::mutability::Mutability;
use crate::core::model::types::ty::Type;
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::equation_parser::Equation;
use crate::core::scanner::static_type_context::StaticTypeContext;
use crate::core::scanner::types::boolean::Boolean;
use crate::core::scanner::types::r#type::{InferTypeError};

impl Expression {
    /// identifier expects a variable name. Expressions per se dont have variables names, but the identifier function is called on a l_value
    pub fn identifier(&self) -> Option<String> {
        if let Some(value) = &self.value {
            return value.identifier();
        }

        None
    }
}

impl Parse for Expression {
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, Error> where Self: Sized {
        let mut s: Equation<'_> = Equation::new(tokens);
        let f = s.parse()?;

        Ok(ParseResult {
            result: *f.result,
            consumed: f.consumed,
        })
    }
}

impl Expression {
    pub fn traverse_type(&self, meta: &MetaInfo) -> Option<Type> {
        self.traverse_type_resulted(&meta.static_type_information, &meta.code_line).ok()
    }
    
    pub fn traverse_type_resulted(&self, context: &StaticTypeContext, code_line: &CodeLine) -> Result<Type, InferTypeError> {
        if let Some(value) = &self.value {
            let mut value_type = value.infer_type_with_context(context, code_line);
            let has_prefix_arithmetics = self.prefix_arithmetic.is_some();
            let has_index_operation = self.index_operator.is_some();

            if has_index_operation {
                if let Some(index_operator) = &self.index_operator {
                    let index_type = index_operator.infer_type_with_context(context, code_line)?;
                    if !matches!(index_type, Type::Integer(_, _)) {
                        return Err(InferTypeError::IllegalIndexOperation(index_type, code_line.clone()));
                    }
                }
                let value_type_cloned = value_type?.clone();
                if let Some(element_type) = value_type_cloned.pop_array() {
                    value_type = Ok(element_type);
                } else {
                    return Err(InferTypeError::IllegalArrayTypeLookup(value_type_cloned, code_line.clone()));
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
                                return Err(InferTypeError::IllegalDereference(*value.clone(), value_type, code_line.clone()));
                            }
                        }
                        PrefixArithmetic::PointerArithmetic(PointerArithmetic::Ampersand) => {
                            value_type = value_type.push_pointer();
                        }
                        PrefixArithmetic::PointerArithmetic(PointerArithmetic::Asterics) => {
                            // just using & in front of non pointer types is illegal. Dereferencing non pointers doesnt make any sense
                            return Err(InferTypeError::IllegalDereference(*value.clone(), value_type, code_line.clone()));
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

        Self::check_operator_compatibility(self.to_string(), &self.lhs, self.operator.clone(), &self.rhs, context, code_line)
    }

    fn check_operator_compatibility(error_message: String, lhs: &Option<Box<Expression>>, operator: Operator, rhs: &Option<Box<Expression>>, context: &StaticTypeContext, code_line: &CodeLine) -> Result<Type, InferTypeError> {
        if let Some(lhs) = &lhs {
            if let Some(rhs) = &rhs {
                let lhs_type = lhs.traverse_type_resulted(context, code_line)?;
                let rhs_type = rhs.traverse_type_resulted(context, code_line)?;

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

                if let Some(result_type) = base_type_matrix.get(&(lhs_clone, operator.clone(), rhs_clone)) {
                    return Ok(result_type.clone());
                }

                return Err(InferTypeError::TypesNotCalculable(lhs_type, operator, rhs_type, code_line.clone()));
            }
        }

        Err(InferTypeError::UnresolvedReference(error_message, code_line.clone()))
    }

    pub fn set(&mut self, lhs: Option<Box<Expression>>, operation: Operator, rhs: Option<Box<Expression>>, value: Option<Box<Assignable>>) {
        self.lhs = lhs;
        self.rhs = rhs;
        self.operator = operation;
        self.positive = true;
        self.value = value;
        self.prefix_arithmetic = None;
    }

    pub fn flip_value(&mut self) {
        if let Some(Assignable::Integer(i)) = &mut self.value.as_deref_mut() {
            i.value = "-".to_string() + &i.value;
        }

        if let Some(Assignable::Float(f)) = &mut self.value.as_deref_mut() {
            f.value *= -1.0;
        }

        if self.value.is_some() {
            self.positive = !self.positive;
        }
    }
}
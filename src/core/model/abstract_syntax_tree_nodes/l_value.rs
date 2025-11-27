use std::fmt::{Display, Formatter};
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;

#[derive(Debug, PartialEq, Clone)]
pub enum LValue {
    Identifier(Identifier),
    Expression(Expression),
}

#[derive(Debug)]
pub enum LValueError {
    KeywordReserved(String),
}

impl Display for LValueError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            LValueError::KeywordReserved(value) => {
                format!("The variable name \"{}\" variable name can't have the same name as a reserved keyword", value)
            }
        };
        write!(f, "{}", message)
    }
}

impl std::error::Error for LValueError { }

impl Display for LValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            LValue::Identifier(name) => name.to_string(),
            LValue::Expression(e) => e.to_string()
        })
    }
}

impl Default for LValue {
    fn default() -> Self {
        LValue::Identifier(Identifier::default())
    }
}
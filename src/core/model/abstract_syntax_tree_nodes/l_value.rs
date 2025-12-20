use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum LValue {
    /// For example let a = 5;
    Identifier(Identifier),
    /// For example a[0] = 5; or obj.field = 10;
    Expression(Expression)
}

impl LValue {
    pub fn uuid() -> Self {
        LValue::Identifier(Identifier::uuid())
    }
}

impl Display for LValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            LValue::Identifier(name) => name.to_string(),
            LValue::Expression(node) => node.to_string(),
        })
    }
}

impl Default for LValue {
    fn default() -> Self {
        LValue::Identifier(Identifier::default())
    }
}
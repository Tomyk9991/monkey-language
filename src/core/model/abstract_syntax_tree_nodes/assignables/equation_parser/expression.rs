use std::fmt::{Debug, Display, Formatter};
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::operator::Operator;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::prefix_arithmetic::PrefixArithmetic;

#[derive(Clone, PartialEq)]
#[allow(unused)]
pub struct Expression {
    pub lhs: Option<Box<Expression>>,
    pub rhs: Option<Box<Expression>>,
    pub operator: Operator,
    pub prefix_arithmetic: Option<PrefixArithmetic>,
    pub value: Option<Box<Assignable>>,
    pub index_operator: Option<Box<Assignable>>,
    pub positive: bool,
}

impl From<Option<Box<Assignable>>> for Expression {
    fn from(value: Option<Box<Assignable>>) -> Self {
        Expression {
            value,
            ..Default::default()
        }
    }
}


impl Debug for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct_formatter = f.debug_struct("Expression");

        if let Some(lhs) = &self.lhs {
            debug_struct_formatter.field("lhs", lhs);
        }

        debug_struct_formatter.field("operator", &self.operator);

        if let Some(rhs) = &self.rhs {
            debug_struct_formatter.field("rhs", rhs);
        }

        if let Some(value) = &self.value {
            debug_struct_formatter.field("value", value);
        }

        debug_struct_formatter.field("positive", &self.positive);
        let prefix_arithmetic = self.prefix_arithmetic.iter().map(|a| a.to_string()).collect::<String>();

        debug_struct_formatter.field("prefix_arithmetic", &prefix_arithmetic);
        if let Some(index_operator) = &self.index_operator {
            debug_struct_formatter.field("index_operator", index_operator);
        }
        debug_struct_formatter.finish()
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let prefix_arithmetic = self.prefix_arithmetic.iter().rev().map(|a| a.to_string()).collect::<String>();

        let index_operator = if let Some(index_operator) = &self.index_operator {
            format!("[{}]", index_operator)
        } else {
            "".to_string()
        };
        match (&self.lhs, &self.rhs) {
            (Some(lhs), Some(rhs)) => {
                write!(f, "{prefix_arithmetic}({lhs} {operator} {rhs}){index_operator}", operator = &self.operator)
            }
            _ => {
                if let Some(ass) = &self.value {
                    write!(f, "{}{}{}", prefix_arithmetic, ass, index_operator)
                } else {
                    write!(f, "Some error. No lhs and rhs and no value found")
                }
            }
        }
    }
}

impl Default for Expression {
    fn default() -> Self {
        Self {
            lhs: None,
            rhs: None,
            operator: Operator::Noop,
            value: None,
            index_operator: None,
            positive: true,
            prefix_arithmetic: None,
        }
    }
}
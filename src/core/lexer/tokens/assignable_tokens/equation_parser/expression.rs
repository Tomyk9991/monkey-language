use std::fmt::{Debug, Display, Formatter};
use crate::core::lexer::tokens::assignable_token::AssignableToken;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;

#[derive(Clone, PartialEq)]
#[allow(unused)]
pub struct Expression {
    pub lhs: Option<Box<Expression>>,
    pub rhs: Option<Box<Expression>>,
    pub operator: Operator,
    pub value: Option<AssignableToken>,
}

#[allow(unused)]
#[derive(Debug)]
pub enum Error {
    DivisionByZero
}

impl Default for Expression {
    fn default() -> Self {
        Self {
            lhs: None,
            rhs: None,
            operator: Operator::Noop,
            value: None,
        }
    }
}

impl Debug for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct_formatter = f.debug_struct("");

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

        debug_struct_formatter.finish()
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match (&self.lhs, &self.rhs) {
            (Some(lhs), Some(rhs)) => {
                write!(f, "({} {} {})", lhs, &self.operator, rhs)
            }
            _ => {
                if let Some(ass) = &self.value {
                    write!(f, "{}", ass)
                } else {
                    write!(f, "Some error. No lhs and rhs and no value found")
                }
            }
        }
    }
}


impl From<Option<AssignableToken>> for Expression {
    fn from(value: Option<AssignableToken>) -> Self {
        Expression {
            value,
            ..Default::default()
        }
    }
}

#[allow(unused)]
impl Expression {
    pub fn new(lhs: Option<Box<Expression>>, operator: Operator, rhs: Option<Box<Expression>>, value: Option<AssignableToken>) -> Self {
        Self {
            lhs,
            rhs,
            operator,
            value,
        }
    }

    pub fn flip_value(&mut self) {
        if let Some(v) = &mut self.value {
            match v {
                AssignableToken::String(_) => {}
                AssignableToken::IntegerToken(a) => a.value *= -1,
                AssignableToken::DoubleToken(a) => a.value *= -1.0,
                AssignableToken::BooleanToken(a) => match a.value {
                    true => a.value = false,
                    false => a.value = true
                }
                AssignableToken::MethodCallToken(_) => {}
                AssignableToken::Variable(_) => {}
                AssignableToken::Object(_) => {}
                AssignableToken::ArithmeticEquation(_) => {}
                AssignableToken::BooleanEquation(_) => {}
            }
        }
    }
}
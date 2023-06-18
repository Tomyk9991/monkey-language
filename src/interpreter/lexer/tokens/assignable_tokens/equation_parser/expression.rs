use crate::interpreter::lexer::tokens::assignable_token::AssignableToken;
use crate::interpreter::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;

#[derive(Debug, Clone, PartialEq)]
#[allow(unused)]
pub struct Expression {
    lhs: Option<Box<Expression>>,
    rhs: Option<Box<Expression>>,
    operator: Operator,
    pub value: AssignableToken
}

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
            value: AssignableToken::default(),
        }
    }
}

impl Expression {
    pub fn new(lhs: Option<Box<Expression>>, operator: Operator, rhs: Option<Box<Expression>>, value: AssignableToken) -> Self {
        Self {
            lhs,
            rhs,
            operator,
            value,
        }
    }

    pub fn flip_value(&mut self) {
        match &mut self.value {
            AssignableToken::String(_) => {}
            AssignableToken::IntegerToken(a) => a.value *= -1,
            AssignableToken::DoubleToken(a) => a.value *= -1.0,
            AssignableToken::MethodCallToken(_) => {}
            AssignableToken::Variable(_) => {}
            AssignableToken::Object(_) => {}
            AssignableToken::Equation(_) => {}
        }
    }

    pub fn new_f64(value: AssignableToken) -> Self {
        Expression {
            value,
            ..Default::default()
        }
    }

    pub fn evaluate(&self) -> f64 {
        match &self.value {
            AssignableToken::String(_) => 0.0,
            AssignableToken::IntegerToken(a) => a.value as f64,
            AssignableToken::DoubleToken(a) => a.value,
            AssignableToken::MethodCallToken(_) => 0.0,
            AssignableToken::Variable(_) => 0.0,
            AssignableToken::Object(_) => 0.0,
            AssignableToken::Equation(_) => 0.0,
        }
    }
}
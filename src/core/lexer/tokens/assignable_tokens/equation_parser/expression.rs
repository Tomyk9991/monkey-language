use crate::core::lexer::tokens::assignable_token::AssignableToken;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;

#[derive(Debug, Clone, PartialEq)]
#[allow(unused)]
pub struct Expression {
    lhs: Option<Box<Expression>>,
    rhs: Option<Box<Expression>>,
    operator: Operator,
    pub value: AssignableToken,
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
            value: AssignableToken::default(),
        }
    }
}

impl From<AssignableToken> for Expression {
    fn from(value: AssignableToken) -> Self {
        Expression {
            value,
            ..Default::default()
        }
    }
}

#[allow(unused)]
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
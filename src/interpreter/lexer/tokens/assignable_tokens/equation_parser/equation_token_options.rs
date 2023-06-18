use crate::interpreter::lexer::tokens::assignable_token::AssignableToken;
use crate::interpreter::lexer::tokens::assignable_tokens::double_token::DoubleToken;
use crate::interpreter::lexer::tokens::assignable_tokens::equation_parser::{Error, expression};
use crate::interpreter::lexer::tokens::assignable_tokens::equation_parser::expression::Expression;
use crate::interpreter::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;

pub trait EquationTokenOptions {
    fn additive() -> char;
    fn inverse_additive() -> char;

    fn multiplicative() -> char;
    fn inverse_multiplicative() -> char;

    fn negate() -> char;

    fn add_operation(value: Box<Expression>, other: Box<Expression>) -> Result<Box<Expression>, Error>;
    fn inverse_add_operation(value: Box<Expression>, other: Box<Expression>) -> Result<Box<Expression>, Error>;

    fn mul_operation(value: Box<Expression>, other: Box<Expression>) -> Result<Box<Expression>, Error>;
    fn inverse_mul_operation(value: Box<Expression>, other: Box<Expression>) -> Result<Box<Expression>, Error>;
}

pub struct ArithmeticEquationOptions;

impl EquationTokenOptions for ArithmeticEquationOptions {
    fn additive() -> char { '+' }
    fn inverse_additive() -> char { '-' }
    fn multiplicative() -> char { '*' }
    fn inverse_multiplicative() -> char { '/' }
    fn negate() -> char { '-' }

    fn add_operation(first: Box<Expression>, other: Box<Expression>) -> Result<Box<Expression>, Error> {
        let other_result = other.evaluate();
        let first_result = first.evaluate();

        let ex = Expression::new(
            Some(first),
            Operator::Add,
            Some(other),
            AssignableToken::DoubleToken(DoubleToken { value: first_result + other_result }),
        );

        Ok(Box::new(ex))
    }

    fn inverse_add_operation(first: Box<Expression>, other: Box<Expression>) -> Result<Box<Expression>, Error> {
        let other_result = other.evaluate();
        let first_result = first.evaluate();

        let ex = Expression::new(
            Some(first),
            Operator::Sub,
            Some(other),
            AssignableToken::DoubleToken(DoubleToken { value: first_result - other_result }),
        );

        Ok(Box::new(ex))
    }

    fn mul_operation(first: Box<Expression>, other: Box<Expression>) -> Result<Box<Expression>, Error> {
        let other_result = other.evaluate();
        let first_result = first.evaluate();

        let ex = Expression::new(
            Some(first),
            Operator::Mul,
            Some(other),
            AssignableToken::DoubleToken(DoubleToken { value: first_result * other_result }),
        );

        Ok(Box::new(ex))
    }

    fn inverse_mul_operation(first: Box<Expression>, other: Box<Expression>) -> Result<Box<Expression>, Error> {
        let other_result = other.evaluate();
        let first_result = first.evaluate();

        if other_result == 0.0 {
            return Err(Error::ExpressionErr(expression::Error::DivisionByZero));
        }

        let ex = Expression::new(
            Some(first),
            Operator::Div,
            Some(other),
            AssignableToken::DoubleToken(DoubleToken { value: first_result / other_result }),
        );

        Ok(Box::new(ex))
    }
}
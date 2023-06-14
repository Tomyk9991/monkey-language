use crate::interpreter::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;

#[derive(Debug, Clone, PartialEq)]
#[allow(unused)]
pub struct Expression {
    lhs: Option<Box<Expression>>,
    rhs: Option<Box<Expression>>,
    operator: Operator,
    pub value: f64,
    func: String
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
            value: 0.0,
            func: String::new(),
        }
    }
}

impl Expression {
    pub fn new(lhs: Option<Box<Expression>>, operator: Operator, rhs: Option<Box<Expression>>, value: f64) -> Self {
        Self {
            lhs,
            rhs,
            operator,
            value,
            func: "".to_string(),
        }
    }

    pub fn flip_value(&mut self) {
        self.value *= -1.0;
    }

    pub fn new_f64(value: f64) -> Self {
        Expression {
            value,
            ..Default::default()
        }
    }

    pub fn new_func(func: impl Into<String>, lhs: Option<Box<Expression>>, value: f64) -> Self {
        Self {
            lhs,
            value,
            func: func.into(),
            ..Default::default()
        }
    }

    pub fn evaluate(&self) -> f64 {
        self.value
    }

    pub fn add(&mut self, other: Box<Expression>) -> Result<Box<Expression>, Error> {
        let other_result = other.evaluate();

        let ex = Expression::new(
            Some(Box::new(self.clone())),
            Operator::Add,
            Some(other),
            self.value + other_result
        );

        Ok(Box::new(ex))
    }

    pub fn sub(&mut self, other: Box<Expression>) -> Result<Box<Expression>, Error> {
        let other_result = other.evaluate();

        let ex = Expression::new(
            Some(Box::new(self.clone())),
            Operator::Sub,
            Some(other),
            self.value - other_result
        );

        Ok(Box::new(ex))
    }

    pub fn div(&mut self, other: Box<Expression>) -> Result<Box<Expression>, Error> {
        let other_result = other.evaluate();

        if other_result == 0.0 {
            return Err(Error::DivisionByZero)
        }

        let ex = Expression::new(
            Some(Box::new(self.clone())),
            Operator::Div,
            Some(other),
            self.value / other_result
        );

        Ok(Box::new(ex))
    }

    pub fn mul(&mut self, other: Box<Expression>) -> Result<Box<Expression>, Error> {
        let other_result = other.evaluate();

        let ex = Expression::new(
            Some(Box::new(self.clone())),
            Operator::Mul,
            Some(other),
            self.value * other_result
        );

        Ok(Box::new(ex))
    }

    pub fn pow(&mut self, other: Box<Expression>) -> Result<Box<Expression>, Error> {
        let other_result = other.evaluate();

        let ex = Expression::new(
            Some(Box::new(self.clone())),
            Operator::Pow,
            Some(other),
            self.value.powf(other_result)
        );

        Ok(Box::new(ex))
    }
}
pub mod expression;
pub mod operator;

use std::fmt::{Debug, Display, Formatter};
use std::num::ParseFloatError;
use crate::interpreter::lexer::tokens::assignable_tokens::equation_parser::expression::Expression;

#[derive(Debug, PartialEq)]
pub struct EquationToken {
    source_code: String,
    pub syntax_tree: Box<Expression>,
    pos: i32,
    ch: i32
}

#[derive(Debug)]
#[allow(unused)]
pub enum Error {
    PositionNotInRange(i32),
    UndefinedSequence,
    FunctionNotFound,
    SourceEmpty,
    NotAFloat(ParseFloatError),
    ExpressionErr(expression::Error),
    ParenExpected
}

impl From<ParseFloatError> for Error {
    fn from(value: ParseFloatError) -> Self {
        Error::NotAFloat(value)
    }
}

impl From<expression::Error> for Error {
    fn from(value: expression::Error) -> Self {
        Error::ExpressionErr(value)
    }
}


impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Error::PositionNotInRange(index) => format!("Index {index} out of range"),
            Error::ExpressionErr(err) => format!("{:?}", err),
            Error::ParenExpected => "Expected \")\"".to_string(),
            Error::NotAFloat(v) => format!("{}", v),
            Error::UndefinedSequence => "Not an a sequence".to_string(),
            Error::FunctionNotFound => "Not a function".to_string(),
            Error::SourceEmpty => "Source code is empty".to_string(),
        })
    }
}

impl std::error::Error for Error { }

#[allow(clippy::should_implement_trait)]
impl EquationToken {
    pub fn from_str(string: &str) -> Result<Box<Expression>, Error> {
        let mut s = EquationToken::new(string);
        Ok(Box::new(s.parse()?.clone()))
    }
}

impl EquationToken {
    pub fn new(source_code: impl Into<String>) -> Self {
        Self {
            source_code: source_code.into(),
            syntax_tree: Box::default(),
            pos: -1,
            ch: -1,
        }
    }

    pub fn _evaluate(&mut self) -> Result<f64, Error> {
        if self.source_code.is_empty() {
            return Err(Error::SourceEmpty)
        }

        return Ok(self.parse()?.evaluate());
    }

    fn next_char(&mut self) {
        self.pos += 1;

        if let Some(char) = self.source_code.chars().nth(self.pos as usize) {
            self.ch = char as i32;
        } else {
            self.ch = -1;
        }
    }

    fn eat(&mut self, char_to_eat: char) -> bool {
        while self.ch == ' ' as i32 {
            self.next_char();
        }

        if self.ch == char_to_eat as i32 {
            self.next_char();
            return true;
        }

        false
    }

    fn parse(&mut self) -> Result<&Expression, Error> {
        self.next_char();
        self.syntax_tree = self.parse_expression()?;

        if self.pos < self.source_code.len() as i32 {
            return Err(Error::PositionNotInRange(self.pos))
        }

        Ok(&self.syntax_tree)
    }

    fn parse_expression(&mut self) -> Result<Box<Expression>, Error> {
        let mut x = self.parse_term()?;

        loop {
            if self.eat('+') {
                let p = self.parse_term()?;
                x = x.add(p)?;
            } else if self.eat('-') {
                let p = self.parse_term()?;
                x = x.sub(p)?;
            } else {
                return Ok(x);
            }
        }
    }

    fn parse_term(&mut self) -> Result<Box<Expression>, Error> {
        let mut x = self.parse_factor()?;
        loop {
            if self.eat('*') {
                let p = self.parse_term()?;
                x = x.mul(p)?;
            } else if self.eat('/') {
                let p = self.parse_term()?;
                x = x.div(p)?;
            } else {
                return Ok(x);
            }
        }
    }


    fn parse_factor(&mut self) -> Result<Box<Expression>, Error> {
        let mut x: Box<Expression>;

        if self.eat('+') {
            x = self.parse_factor()?;
            return Ok(x);
        } else if self.eat('-') {
            x = self.parse_factor()?;
            x.as_mut().flip_value();
            return Ok(x);
        }

        let start_pos: i32 = self.pos;
        if self.eat('(') {
            x = self.parse_expression()?;

            if !self.eat(')') {
                return Err(Error::ParenExpected)
            }

        } else if self.ch >= '0' as i32 && self.ch <= '9' as i32 || self.ch == '.' as i32 || self.ch == ',' as i32 {
            while self.ch >= '0' as i32 && self.ch <= '9' as i32 || self.ch == ',' as i32 || self.ch == '.' as i32 {
                self.next_char()
            }

            let sub_string:&str = &self.source_code[start_pos as usize..self.pos as usize];
            let value: f64 = sub_string.parse::<f64>()?;

            x = Box::new(Expression::new_f64(value));
        } else if self.ch >= 'a' as i32 && self.ch <= 'z' as i32 {
            while self.ch >= 'a' as i32 && self.ch <= 'z' as i32 {
                self.next_char();
            }

            let func: String = self.source_code[start_pos as usize..self.pos as usize].to_string();
            x = self.parse_factor()?;

            x = match func.as_str() {
                "sqrt" => {
                    let result = x.as_ref().evaluate().sqrt();
                    Box::new(Expression::new_func("Sqrt".to_string(), Some(x), result))
                },
                "sin" => {
                    let result = x.as_ref().evaluate().sin();
                    Box::new(Expression::new_func("Sin".to_string(), Some(x), result))
                },
                "cos" => {
                    let result = x.as_ref().evaluate().cos();
                    Box::new(Expression::new_func("Cos".to_string(), Some(x), result))
                },
                "tan" => {
                    let result = x.as_ref().evaluate().tan();
                    Box::new(Expression::new_func("Tan".to_string(), Some(x), result))
                },
                _ => return Err(Error::FunctionNotFound)
            }
        } else {
            return Err(Error::UndefinedSequence);
        }

        if self.eat('^') {
            x = x.pow(self.parse_factor()?)?;
        }

        Ok(x)
    }
}
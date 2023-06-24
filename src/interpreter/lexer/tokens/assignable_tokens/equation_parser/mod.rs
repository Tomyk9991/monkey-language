use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::str::FromStr;
use crate::interpreter::io::code_line::{CodeLine, Normalizable};
use crate::interpreter::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::interpreter::lexer::tokens::assignable_tokens::equation_parser::equation_token_options::EquationTokenOptions;
use crate::interpreter::lexer::tokens::assignable_tokens::equation_parser::expression::Expression;
use crate::interpreter::lexer::tokens::name_token::NameTokenErr;

pub mod expression;
pub mod operator;
pub mod equation_token_options;

const OPENING: char = '(';
const CLOSING: char = ')';

#[derive(Debug, PartialEq)]
pub struct EquationToken<T: EquationTokenOptions> {
    source_code: String,
    pub syntax_tree: Box<Expression>,
    pos: i32,
    ch: Option<char>,
    _marker: PhantomData<T>,
}

#[derive(Debug)]
#[allow(unused)]
pub enum Error {
    PositionNotInRange(i32),
    UndefinedSequence(Option<String>),
    FunctionNotFound,
    SourceEmpty,
    NotAFloat(String),
    ExpressionErr(expression::Error),
    ParenExpected,
    CannotParse
}


impl From<expression::Error> for Error {
    fn from(value: expression::Error) -> Self {
        Error::ExpressionErr(value)
    }
}

impl From<AssignableTokenErr> for Error {
    fn from(value: AssignableTokenErr) -> Self {
        Error::NotAFloat(match value {
            AssignableTokenErr::PatternNotMatched { target_value } => target_value
        })
    }
}

impl From<NameTokenErr> for Error {
    fn from(value: NameTokenErr) -> Self { Error::UndefinedSequence(Some(value.to_string())) }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Error::PositionNotInRange(index) => format!("Index {index} out of range"),
            Error::ExpressionErr(err) => format!("{:?}", err),
            Error::ParenExpected => "Expected \")\"".to_string(),
            Error::NotAFloat(v) => v.to_string(),
            Error::UndefinedSequence(value) => {
                match value {
                    Some(value) => value.to_string(),
                    None => "Undefined sequence".to_string()
                }
            },
            Error::FunctionNotFound => "Not a function".to_string(),
            Error::SourceEmpty => "Source code is empty".to_string(),
            Error::CannotParse => "Cannot parse".to_string()
        })
    }
}

impl std::error::Error for Error {}

#[allow(clippy::should_implement_trait)]
impl<T: EquationTokenOptions> EquationToken<T> {

    pub fn from_str(string: &str) -> Result<Box<Expression>, Error> {
        let mut s: EquationToken<T> = EquationToken::new(string);
        let f = s.parse()?.clone();
        Ok(Box::new(f))
    }

    pub fn new(source_code: impl Into<String>) -> Self {
        Self {
            source_code: source_code.into(),
            syntax_tree: Box::default(),
            pos: -1,
            ch: None,
            _marker: PhantomData::default(),
        }
    }

    pub fn _evaluate(&mut self) -> Result<f64, Error> {
        if self.source_code.is_empty() {
            return Err(Error::SourceEmpty);
        }

        todo!();
        // return Ok(self.parse()?.evaluate());
    }

    fn next_char(&mut self) {
        self.pos += 1;
        self.ch = self.source_code.chars().nth(self.pos as usize);
    }

    fn eat(&mut self, char_to_eat: Option<char>) -> bool {
        while self.ch == Some(' ') {
            self.next_char();
        }

        if self.ch == char_to_eat {
            self.next_char();
            return true;
        }

        false
    }

    fn parse(&mut self) -> Result<&Expression, Error> {
        self.next_char();

        if self.pos < 0 || self.pos >= self.source_code.len() as i32 {
            return Err(Error::PositionNotInRange(self.pos));
        }

        self.syntax_tree = self.parse_expression()?;
        Ok(&self.syntax_tree)
    }

    fn parse_expression(&mut self) -> Result<Box<Expression>, Error> {
        let x = self.parse_term()?;

        loop {
            #[allow(clippy::if_same_then_else)]
            if self.eat(T::additive()) {
                let _p = self.parse_term()?;
                // x = T::add_operation(x, p)?;
            } else if self.eat(T::inverse_additive()) {
                let _p = self.parse_term()?;
                // x = T::inverse_add_operation(x, p)?;
            } else {
                return Ok(x);
            }
        }
    }

    fn parse_term(&mut self) -> Result<Box<Expression>, Error> {
        let x = self.parse_factor()?;
        loop {
            #[allow(clippy::if_same_then_else)]
            if self.eat(T::multiplicative()) {
                let _p = self.parse_term()?;
                // x = T::mul_operation(x, p)?;
            } else if self.eat(T::inverse_multiplicative()) {
                let _p = self.parse_term()?;
                // x = T::inverse_mul_operation(x, p)?;
            } else {
                return Ok(x);
            }
        }
    }


    fn parse_factor(&mut self) -> Result<Box<Expression>, Error> {
        let mut x: Box<Expression>;

        if self.eat(T::additive()) {
            x = self.parse_factor()?;
            return Ok(x);
        } else if self.eat(T::inverse_additive()) {
            x = self.parse_factor()?;
            x.as_mut().flip_value();
            return Ok(x);
        }

        let start_pos: i32 = self.pos;
        if self.eat(Some(OPENING)) {
            x = self.parse_expression()?;

            if !self.eat(Some(CLOSING)) {
                return Err(Error::ParenExpected);
            }
        } else if self.ch.is_some() {
            // digits only
            if self.ch >= Some('0') && self.ch <= Some('9') || self.ch == Some('.') {
                while self.ch >= Some('0') && self.ch <= Some('9') || self.ch == Some('.') {
                    self.next_char()
                }

                let sub_string: &str = &self.source_code[start_pos as usize..self.pos as usize];
                let s = AssignableToken::from_str(sub_string)?;
                x = Box::new(Expression::from(s));
            } else if (self.ch >= Some('A') && self.ch <= Some('Z')) || (self.ch >= Some('a') && self.ch <= Some('z')) {
                // works for variables but not functions
                let mut ident = 0;

                let add_token = T::additive();
                let inv_add_token = T::inverse_additive();
                let mul_token = T::multiplicative();
                let inv_mul_token = T::inverse_multiplicative();


                while ident != 0 || (
                        self.ch != add_token &&
                        self.ch != inv_add_token &&
                        self.ch != mul_token &&
                        self.ch != inv_mul_token
                    ) {
                    if let Some(char) = self.ch {
                        match char {
                            '(' => ident += 1,
                            ')' => ident -= 1,
                            _ => { }
                        }
                    } else {
                        break;
                    }

                    if ident == -1 {
                        break;
                    }

                    self.next_char();
                }


                let sub_string: &str = &self.source_code[start_pos as usize..self.pos as usize];

                let mut temp = vec![CodeLine::imaginary(sub_string)];
                temp.normalize();

                let sub_string = temp[0].line.to_string();

                let assignable_token = AssignableToken::from_str(sub_string.as_str())?;

                x = Box::new(Expression::from(assignable_token));
            } else {
                return Err(Error::UndefinedSequence(self.ch.map(|a| a.to_string())));
            }
        } else {
            return Err(Error::UndefinedSequence(self.ch.map(|a| a.to_string())));
        }

        Ok(x)
    }
}
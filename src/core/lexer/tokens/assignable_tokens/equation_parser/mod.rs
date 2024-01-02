use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::str::FromStr;
use crate::core::io::code_line::{CodeLine, Normalizable};
use crate::core::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::core::lexer::tokens::assignable_tokens::equation_parser::equation_token_options::EquationTokenOptions;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::expression::{Expression, PointerArithmetic};
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;
use crate::core::lexer::tokens::name_token::NameTokenErr;

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
    UndefinedSequence(String),
    FunctionNotFound,
    SourceEmpty,
    TermNotParsable(String),
    ParenExpected,
    CannotParse
}


impl From<AssignableTokenErr> for Error {
    fn from(value: AssignableTokenErr) -> Self {
        Error::TermNotParsable(match value {
            AssignableTokenErr::PatternNotMatched { target_value } => target_value
        })
    }
}

impl From<NameTokenErr> for Error {
    fn from(value: NameTokenErr) -> Self { Error::UndefinedSequence(value.to_string()) }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Error::PositionNotInRange(index) => format!("Index {index} out of range"),
            Error::ParenExpected => "Expected \")\"".to_string(),
            Error::TermNotParsable(v) => v.to_string(),
            Error::UndefinedSequence(value) => value.to_string(),
            Error::FunctionNotFound => "Not a function".to_string(),
            Error::SourceEmpty => "Source code is empty".to_string(),
            Error::CannotParse => "Cannot parse".to_string()
        })
    }
}

impl std::error::Error for Error {}

#[allow(clippy::should_implement_trait)]
impl<T: EquationTokenOptions> EquationToken<T> {

    pub fn from_str(string: &str) -> Result<Expression, Error> {
        let mut s: EquationToken<T> = EquationToken::new(string);
        let f = s.parse()?.clone();
        Ok(f)
    }

    pub fn new(source_code: impl Into<String>) -> Self {
        let s = source_code.into();
        Self {
            source_code: s,
            syntax_tree: Box::default(),
            pos: -1,
            ch: None,
            _marker: PhantomData,
        }
    }

    fn next_char(&mut self) {
        self.pos += 1;
        self.ch = self.source_code.chars().nth(self.pos as usize);
    }

    fn eat(&mut self, char_to_eat: Option<char>) -> bool {
        if char_to_eat.is_none() {
            return false;
        }

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
        let mut x = self.parse_term()?;

        loop {
            #[allow(clippy::if_same_then_else)]
            if self.eat(T::additive()) {
                let term = self.parse_term()?;
                x.set(Some(x.clone()), Operator::Add, Some(term), None);
            } else if self.eat(T::inverse_additive()) {
                let term = self.parse_term()?;
                x.set(Some(x.clone()), Operator::Sub, Some(term), None);
            } else {
                return Ok(x);
            }
        }
    }

    fn parse_term(&mut self) -> Result<Box<Expression>, Error> {
        let mut x = self.parse_factor()?;
        loop {
            #[allow(clippy::if_same_then_else)]
            if self.eat(T::multiplicative()) {
                let term = self.parse_term()?;
                x.set(Some(x.clone()), Operator::Mul, Some(term), None);
            } else if self.eat(T::inverse_multiplicative()) {
                let term = self.parse_term()?;
                x.set(Some(x.clone()), Operator::Div, Some(term), None);
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
            x.flip_value();
            return Ok(x);
        }

        if self.eat(Some('*')) {
            x = self.parse_factor()?;
            x.pointer_arithmetic.push(PointerArithmetic::Asterics);

            return Ok(x);
        }

        if self.eat(Some('&')) {
            x = self.parse_factor()?;
            x.pointer_arithmetic.push(PointerArithmetic::Ampersand);

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
                x = Box::new(Expression::from(Some(Box::new(s))));
            } else if (self.ch >= Some('A') && self.ch <= Some('Z')) || (self.ch >= Some('a') && self.ch <= Some('z')) {
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
                let assignable_token = AssignableToken::from_str_ignore(sub_string.as_str(), sub_string.len() == self.source_code.len())?;

                let s = Expression::from(Some(Box::new(assignable_token)));
                x = Box::new(s);
            } else {
                return self.undefined_or_empty();
            }
        } else {
            return self.undefined_or_empty();
        }

        Ok(x)
    }

    fn undefined_or_empty(&self) -> Result<Box<Expression>, Error> {
        let s = self.ch.map(|a| a.to_string());

        return if let Some(character) = s {
            Err(Error::UndefinedSequence(character))
        } else if let Some(last_character) = self.source_code.chars().last() {
            Err(Error::UndefinedSequence(last_character.to_string()))
        } else {
            Err(Error::SourceEmpty)
        }
    }
}
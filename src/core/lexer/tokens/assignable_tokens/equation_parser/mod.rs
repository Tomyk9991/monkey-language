use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::str::FromStr;
use crate::core::io::code_line::{CodeLine, Normalizable};
use crate::core::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::core::lexer::tokens::assignable_tokens::equation_parser::equation_token_options::EquationTokenOptions;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::expression::{Expression, PointerArithmetic, PrefixArithmetic};
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;
use crate::core::lexer::tokens::name_token::NameTokenErr;
use crate::core::lexer::type_token::{InferTypeError, TypeToken};

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
    NotAType(String), // Message
    TermNotParsable(String),
    ParenExpected,
    CannotParse
}

impl From<InferTypeError> for Error {
    fn from(value: InferTypeError) -> Self {
        match value {
            InferTypeError::TypeNotAllowed(t) => {
                Error::NotAType(t.to_string())
            },
            _ => unreachable!("Cannot reach this"),
        }
    }
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
            Error::CannotParse => "Cannot parse".to_string(),
            Error::NotAType(f) => format!("Unexpeted type: {f}")
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

    fn prev_char(&mut self) {
        self.pos -= 1;
        self.ch = self.source_code.chars().nth(self.pos as usize);
    }

    // skips the provided amount of characters
    fn next_char_amount(&mut self, amount_skip: usize) {
        for _ in 0..amount_skip {
            self.next_char();
        }
    }

    fn previous_char_amount(&mut self, amount_skip: usize) {
        for _ in 0..amount_skip {
            self.prev_char();
        }
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
                // x.set_keep_arithmetic(Some(x.clone()), Operator::Add, Some(term), None, x.positive, vec![]);
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

    fn peek(&self, expected_char: char) -> bool {
        if let Some(a) = self.source_code.chars().nth(self.pos as usize) {
            return a == expected_char;
        }

        false
    }

    /// collects until the specified character is found and checks each substring, if a certain predicate is met
    /// returns a trimmed string between the the beginning and the expected last character and the amount of characters skipped, including the spaces
    fn collect_until(&self, offset: usize, expected_last_char: char, predicate: fn(&str) -> bool) -> Option<(String, usize)> {
        let starting_position = (self.pos as usize) + offset;
        let mut end_position = starting_position;

        for char in self.source_code.chars().skip(starting_position) {
            end_position += 1;

            let current_string = self.source_code.chars()
                .skip(starting_position)
                .take(end_position - starting_position)
                .collect::<String>().trim().to_string();

            // check if it contains something else than *
            // if it does, check if it is a type
            let contains_only_stars = !current_string.chars().any(|a| {
                a != '*'
            });

            if current_string.is_empty() || contains_only_stars { continue; }

            if char == expected_last_char {
                let current_string = self.source_code.chars()
                    .skip(starting_position)
                    .take(end_position - 1 - starting_position)
                    .collect::<String>().trim().to_string();

                return if !predicate(&current_string) {
                    None
                } else {
                    Some((current_string, end_position - starting_position + 1))
                }
            }

            if !predicate(&current_string) {
                return None;
            }
        }

        None
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

        if self.peek('(') {
            if let Some((cast_type, amount_skip)) = self.collect_until(1, ')', |a| TypeToken::from_str(a).is_ok()) {
                self.next_char_amount(amount_skip);

                if !self.peek(')') && self.pos < self.source_code.chars().count() as i32 {
                    let value = self.parse_expression()?;
                    x = Box::<Expression>::default();

                    x.value = Some(Box::new(AssignableToken::ArithmeticEquation(*value)));
                    x.prefix_arithmetic.push(PrefixArithmetic::Cast(TypeToken::from_str(&cast_type)?));
                    return Ok(x);
                } else {
                    self.previous_char_amount(amount_skip);
                }
                // not a type cast, resume
            }
        }

        if self.eat(Some('*')) {
            let value = self.parse_factor()?;
            x = Box::<Expression>::default();

            x.value = Some(Box::new(AssignableToken::ArithmeticEquation(*value)));
            x.prefix_arithmetic.push(PrefixArithmetic::PointerArithmetic(PointerArithmetic::Asterics));

            return Ok(x);

            // x = self.parse_factor()?;
            // x.prefix_arithmetic.push(PrefixArithmetic::PointerArithmetic(PointerArithmetic::Asterics));
            //
            // return Ok(x);
        }

        if self.eat(Some('&')) {
            let value = self.parse_factor()?;
            x = Box::<Expression>::default();

            x.value = Some(Box::new(AssignableToken::ArithmeticEquation(*value)));
            x.prefix_arithmetic.push(PrefixArithmetic::PointerArithmetic(PointerArithmetic::Ampersand));

            return Ok(x);
            // x = self.parse_factor()?;
            // x.prefix_arithmetic.push(PrefixArithmetic::PointerArithmetic(PointerArithmetic::Ampersand));
            //
            // return Ok(x);
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
                let assignment = AssignableToken::from_str(sub_string)?;
                x = Box::new(Expression::from(Some(Box::new(assignment))));
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
                        self.ch != inv_mul_token // todo check for other values not allowed: let a = r);
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
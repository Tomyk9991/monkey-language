use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use crate::core::io::code_line::{CodeLine, Normalizable};
use crate::core::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::core::lexer::tokens::assignable_tokens::equation_parser::expression::{Expression};
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::prefix_arithmetic::{PointerArithmetic, PrefixArithmetic};
use crate::core::lexer::tokens::assignable_tokens::method_call_token::dyck_language;
use crate::core::lexer::tokens::name_token::NameTokenErr;
use crate::core::lexer::types::type_token::{InferTypeError, TypeToken};

pub mod expression;
pub mod operator;
pub mod prefix_arithmetic;

const OPENING: char = '(';
const CLOSING: char = ')';

#[derive(Debug, PartialEq)]
pub struct EquationToken<> {
    source_code: String,
    pub syntax_tree: Box<Expression>,
    pos: i32,
    ch: Option<char>,
}

#[derive(Debug)]
#[allow(unused)]
pub enum Error {
    PositionNotInRange(i32),
    UndefinedSequence(String),
    FunctionNotFound,
    SourceEmpty,
    NotAType(String),
    // Message
    TermNotParsable(String),
    ParenExpected,
    CannotParse,
}

impl From<InferTypeError> for Error {
    fn from(value: InferTypeError) -> Self {
        match value {
            InferTypeError::TypeNotAllowed(t) => {
                Error::NotAType(t.to_string())
            }
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
            Error::NotAType(f) => format!("Unexpected type: {f}")
        })
    }
}

impl std::error::Error for Error {}

#[allow(clippy::should_implement_trait)]
impl EquationToken {
    pub fn from_str(string: &str) -> Result<Expression, Error> {
        let mut s: EquationToken = EquationToken::new(string);
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

    fn eat_multiple(&mut self, chars_to_eat: Option<&str>) -> bool {
        if let Some(chars_to_eat) = chars_to_eat {
            while self.ch == Some(' ') {
                self.next_char();
            }

            let latest_pos = self.pos;
            let latest_ch = self.ch;


            for char in chars_to_eat.chars() {
                if self.ch == Some(' ') {
                    self.next_char()
                }

                if self.ch != Some(char) {
                    self.ch = latest_ch;
                    self.pos = latest_pos;
                    return false;
                }

                self.next_char();
            }

            true
        } else {
            false
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

        if dyck_language(&self.source_code, ['(', ',', ')']).is_err() {
            return Err(Error::CannotParse);
        }

        if self.pos < 0 || self.pos >= self.source_code.len() as i32 {
            return Err(Error::PositionNotInRange(self.pos));
        }

        self.syntax_tree = self.parse_logical_or()?;

        if self.pos as usize != self.source_code.chars().count() {
            return Err(Error::UndefinedSequence(self.source_code.chars().collect::<Vec<_>>()[self.pos as usize..].iter().collect::<String>()));
        }

        Ok(&self.syntax_tree)
    }

    fn parse_logical_or(&mut self) -> Result<Box<Expression>, Error> {
        let mut x = self.parse_logical_and()?;

        loop {
            if self.eat_multiple(Some("||")) {
                let expression = self.parse_logical_and()?;
                x.set(Some(x.clone()), Operator::LogicalOr, Some(expression), None);
            } else {
                return Ok(x);
            }
        }
    }

    fn parse_logical_and(&mut self) -> Result<Box<Expression>, Error> {
        let mut x = self.parse_bitwise_or()?;

        loop {
            if self.eat_multiple(Some("&&")) {
                let expression = self.parse_bitwise_or()?;
                x.set(Some(x.clone()), Operator::LogicalAnd, Some(expression), None);
            } else {
                return Ok(x);
            }
        }
    }

    fn parse_bitwise_or(&mut self) -> Result<Box<Expression>, Error> {
        let mut x = self.parse_bitwise_xor()?;

        loop {
            let latest_ch = self.ch;
            let latest_pos = self.pos;

            if !self.eat_multiple(Some("||")) && self.eat_multiple(Some("|")) {
                let expression = self.parse_bitwise_xor()?;
                x.set(Some(x.clone()), Operator::BitwiseOr, Some(expression), None);
            } else {
                self.ch = latest_ch;
                self.pos = latest_pos;
                return Ok(x);
            }
        }
    }

    fn parse_bitwise_xor(&mut self) -> Result<Box<Expression>, Error> {
        let mut x = self.parse_bitwise_and()?;

        loop {
            if self.eat_multiple(Some("^")) {
                let expression = self.parse_bitwise_and()?;
                x.set(Some(x.clone()), Operator::BitwiseXor, Some(expression), None);
            } else {
                return Ok(x);
            }
        }
    }

    fn parse_bitwise_and(&mut self) -> Result<Box<Expression>, Error> {
        let mut x = self.parse_equality_expression()?;

        loop {
            let latest_ch = self.ch;
            let latest_pos = self.pos;

            if !self.eat_multiple(Some("&&")) && self.eat_multiple(Some("&")) {
                let expression = self.parse_equality_expression()?;
                x.set(Some(x.clone()), Operator::BitwiseAnd, Some(expression), None);
            } else {
                self.ch = latest_ch;
                self.pos = latest_pos;
                return Ok(x);
            }
        }
    }

    fn parse_equality_expression(&mut self) -> Result<Box<Expression>, Error> {
        let mut x = self.parse_relational_expression()?;

        loop {
            if self.eat_multiple(Some("==")) {
                let expression = self.parse_relational_expression()?;
                x.set(Some(x.clone()), Operator::Equal, Some(expression), None);
            } else if self.eat_multiple(Some("!=")) {
                let expression = self.parse_relational_expression()?;
                x.set(Some(x.clone()), Operator::NotEqual, Some(expression), None);
            } else {
                return Ok(x);
            }
        }
    }

    fn parse_relational_expression(&mut self) -> Result<Box<Expression>, Error> {
        let mut x = self.parse_bitwise_shift_expression()?;

        loop {
            if self.eat_multiple(Some("<=")) {
                let expression = self.parse_bitwise_shift_expression()?;
                x.set(Some(x.clone()), Operator::LessThanEqual, Some(expression), None);
            } else if self.eat_multiple(Some(">=")) {
                let expression = self.parse_bitwise_shift_expression()?;
                x.set(Some(x.clone()), Operator::GreaterThanEqual, Some(expression), None);
            } else if self.eat(Some('<')) {
                let expression = self.parse_bitwise_shift_expression()?;
                x.set(Some(x.clone()), Operator::LessThan, Some(expression), None);
            } else if self.eat(Some('>')) {
                let expression = self.parse_bitwise_shift_expression()?;
                x.set(Some(x.clone()), Operator::GreaterThan, Some(expression), None);
            } else {
                return Ok(x);
            }
        }
    }

    fn parse_bitwise_shift_expression(&mut self) -> Result<Box<Expression>, Error> {
        let mut x = self.parse_expression()?;

        loop {
            if self.eat_multiple(Some("<<")) {
                let expression = self.parse_expression()?;
                x.set(Some(x.clone()), Operator::LeftShift, Some(expression), None);
            } else if self.eat_multiple(Some(">>")) {
                let expression = self.parse_expression()?;
                x.set(Some(x.clone()), Operator::RightShift, Some(expression), None);
            } else {
                return Ok(x);
            }
        }
    }

    fn parse_expression(&mut self) -> Result<Box<Expression>, Error> {
        let mut x = self.parse_term()?;

        loop {
            #[allow(clippy::if_same_then_else)]
            if self.eat(Some('+')) {
                let term = self.parse_term()?;
                x.set(Some(x.clone()), Operator::Add, Some(term), None);
            } else if self.eat(Some('-')) {
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
            if self.eat(Some('*')) {
                let term = self.parse_factor()?;
                x.set(Some(x.clone()), Operator::Mul, Some(term), None);
            } else if self.eat(Some('/')) {
                let term = self.parse_factor()?;
                x.set(Some(x.clone()), Operator::Div, Some(term), None);
            } else if self.eat(Some('%')) {
                let term = self.parse_factor()?;
                x.set(Some(x.clone()), Operator::Mod, Some(term), None);
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
                };
            }

            if !predicate(&current_string) {
                return None;
            }
        }

        None
    }


    fn parse_factor(&mut self) -> Result<Box<Expression>, Error> {
        let mut x: Box<Expression>;

        //Start Prefix
        if self.eat(Some('+')) {
            x = self.parse_factor()?;
            return Ok(x);
        } else if self.eat(Some('-')) {
            x = self.parse_factor()?;
            x.flip_value();
            return Ok(x);
        }

        if self.peek('(') {
            if let Some((cast_type, amount_skip)) = self.collect_until(1, ')', |a| TypeToken::from_str(a).is_ok()) {
                self.next_char_amount(amount_skip);

                if !self.peek(')') && self.pos < self.source_code.chars().count() as i32 {
                    let value = self.parse_factor()?;
                    x = Box::<Expression>::default();

                    x.value = Some(Box::new(AssignableToken::ArithmeticEquation(*value)));
                    x.prefix_arithmetic = Some(PrefixArithmetic::Cast(TypeToken::from_str(&cast_type)?));

                    return Ok(x);
                }

                self.previous_char_amount(amount_skip);
                // not a type cast, resume
            }
        }

        if self.eat(Some('*')) {
            let value = self.parse_factor()?;
            x = Box::<Expression>::default();

            x.value = Some(Box::new(AssignableToken::ArithmeticEquation(*value)));
            x.prefix_arithmetic = Some(PrefixArithmetic::PointerArithmetic(PointerArithmetic::Asterics));

            return Ok(x);
        }

        if self.eat(Some('&')) {
            let value = self.parse_factor()?;
            x = Box::<Expression>::default();

            x.value = Some(Box::new(AssignableToken::ArithmeticEquation(*value)));
            x.prefix_arithmetic = Some(PrefixArithmetic::PointerArithmetic(PointerArithmetic::Ampersand));

            return Ok(x);
        }

        //End Prefix

        let start_pos: i32 = self.pos;
        if self.eat(Some(OPENING)) {
            // todo: change this to latest parse_expression
            x = self.parse_logical_or()?;

            if !self.eat(Some(CLOSING)) {
                return Err(Error::ParenExpected);
            }
        } else if self.ch.is_some() {
            // digits only
            if self.ch >= Some('0') && self.ch <= Some('9') || self.ch == Some('.') {
                while self.ch >= Some('0') && self.ch <= Some('9') || self.ch == Some('.') || self.ch == Some('_') || (self.ch >= Some('A') && self.ch <= Some('Z')) || (self.ch >= Some('a') && self.ch <= Some('z')) {
                    self.next_char()
                }

                let sub_string: &str = &self.source_code[start_pos as usize..self.pos as usize];
                let assignment = AssignableToken::from_str(sub_string)?;
                x = Box::new(Expression::from(Some(Box::new(assignment))));
            } else if (self.ch >= Some('A') && self.ch <= Some('Z')) || (self.ch >= Some('a') && self.ch <= Some('z')) {
                let mut ident = 0;

                while ident != 0 || !self.operator_sequence() {
                    match self.ch {
                        Some('(') => ident += 1,
                        Some(')') => ident -= 1,
                        None => break,
                        _ => {}
                    }

                    if ident == -1 {
                        break;
                    }

                    self.next_char();
                }

                let sub_string: &str = &self.source_code.chars().skip(start_pos as usize).take((self.pos - start_pos) as usize).collect::<String>();
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
        };
    }
    fn operator_sequence(&mut self) -> bool {
        static OPERATORS: [&str; 18] = ["+", "-", "*", "%", "/", "<<", ">>", "<", ">", "<=", ">=", "==", "!=", "&&", "||", "&", "^", "|"];

        for operator in OPERATORS {
            let latest_ch = self.ch;
            let latest_pos = self.pos;

            if self.eat_multiple(Some(operator)) {
                self.ch = latest_ch;
                self.pos = latest_pos;

                return true;
            }

            self.ch = latest_ch;
            self.pos = latest_pos;
        }

        false
    }
}
use std::fmt::{Debug, Display, Formatter};
use crate::core::lexer::token::Token;

#[derive(Debug)]
pub enum Error {
    InvalidCharacter(char),
    ExpectedPattern(Vec<Token>),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Error::InvalidCharacter(character) => format!("Invalid character: `{}`", character),
            Error::ExpectedPattern(pattern) => format!("Expected pattern: {:?}", pattern),
        })
    }
}

impl std::error::Error for Error {}

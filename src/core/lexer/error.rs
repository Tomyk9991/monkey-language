use std::fmt::{Debug, Display, Formatter};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_with_span::TokenWithSpan;

#[derive(Debug, Clone)]
pub enum Error {
    InvalidCharacter(char),
    UnexpectedToken(TokenWithSpan),
    Callstack(Box<Error>),
    ExpectedToken(Token),
    WithContext {
        error: Box<Error>,
        context: TokenWithSpan,
    },
    UnexpectedEOF,
}

impl Error {
    pub fn with_context(&self, start: &TokenWithSpan) -> Error {
        Error::WithContext {
            error: Box::new(self.clone()),
            context: start.clone(),
        }
    }
}

pub enum ErrorMatch {
    Token(Token),
    Collect(usize),
}



impl Error {
    pub fn first_unexpected_token(tokens: &[TokenWithSpan], pattern: &[ErrorMatch]) -> Self {
        let mut index = 0;
        while index < tokens.len() {
            let current_token = &tokens[index];
            let current_pattern = &pattern[index];

            match current_pattern {
                ErrorMatch::Token(t) => {
                    if current_token.token != *t {
                        return Error::UnexpectedToken(current_token.clone());
                    }
                }
                ErrorMatch::Collect(amount_collect) => {
                    index += amount_collect;
                }
            }
            index += 1;
        }

        Error::UnexpectedEOF
    }
}

impl From<Token> for ErrorMatch {
    fn from(value: Token) -> Self {
        ErrorMatch::Token(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Error::InvalidCharacter(character) => format!("Invalid character: `{}`", character),
            Error::UnexpectedToken(token) => format!("Unexpected token: {}", token),
            Error::ExpectedToken(f) => format!("Expected token: `{}`", f), 
            Error::UnexpectedEOF => "Unexpected EOF".to_string(),
            Error::Callstack(trace) => trace.to_string(),
            Error::WithContext { error, context } => {
                format!("{}\n\tInside: {}", error, context)
            }
        })
    }
}


impl std::error::Error for Error {}

use std::str::FromStr;

use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::types::static_string::{StaticString, StaticStringError};

impl Parse for StaticString {
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default {
        if let [string_literal, ..] = tokens {
            if let Token::Literal(s) = &string_literal.token {
                if lazy_regex::regex_is_match!("^\".*\"$", s) {
                    return Ok(ParseResult {
                        result: StaticString {
                            value: s.to_string()
                        },
                        consumed: 1,
                    })
                }
            }
        }

        Err(Error::UnexpectedToken(tokens[0].clone()))
    }
}

impl FromStr for StaticString {
    type Err = StaticStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !lazy_regex::regex_is_match!("^\".*\"$", s) {
            return Err(StaticStringError::UnmatchedRegex);
        }

        Ok(StaticString {
            value: s.to_string()
        })
    }
}
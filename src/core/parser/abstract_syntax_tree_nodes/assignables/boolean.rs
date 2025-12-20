use std::fmt::{Display, Formatter};
use std::str::{FromStr, ParseBoolError};

use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_result::{ASMResult};
use crate::core::code_generator::generator::Stack;
use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::types::boolean::{Boolean, BooleanErr};


impl Parse for Boolean {
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default {
        if let [Token::True, ..] = tokens.iter().map(|x| x.token.clone()).collect::<Vec<Token>>().as_slice() {
            return Ok(ParseResult {
                result: Boolean { value: true },
                consumed: 1,
            })
        }
        
        if let [Token::False, ..] = tokens.iter().map(|x| x.token.clone()).collect::<Vec<Token>>().as_slice() {
            return Ok(ParseResult {
                result: Boolean { value: false },
                consumed: 1,
            })
        }

        Err(Error::UnexpectedToken(tokens[0].clone()))
    }
}


impl From<ParseBoolError> for BooleanErr {
    fn from(value: ParseBoolError) -> Self { BooleanErr::ParseBoolError(value) }
}

impl std::error::Error for BooleanErr {}

impl FromStr for Boolean {
    type Err = BooleanErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !lazy_regex::regex_is_match!("^(?i:true|false)$", s) {
            return Err(BooleanErr::UnmatchedRegex);
        }

        Ok(Boolean {
            value: s.parse::<bool>()?
        })
    }
}
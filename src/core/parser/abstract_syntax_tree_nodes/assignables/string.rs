use std::any::Any;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::core::code_generator::{ASMGenerateError,
                                  MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::asm_result::{ASMResult};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::registers::GeneralPurposeRegister;
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



/// replaces the occurrence with the provided number and sets quotes
/// ## Example
/// replace_add_quote("\"Hallo \n Welt\"") returns
/// \"Hallo\", 10, \"Welt\"
fn replace_add_quote(value: &str, occurrence: &str, replace_value: usize) -> String {
    format!("\"{}\"", value[1..value.len() - 1].replace(occurrence, &format!("\", {}, \"", replace_value)))
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
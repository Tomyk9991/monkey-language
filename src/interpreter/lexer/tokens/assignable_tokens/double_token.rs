use std::str::FromStr;
use regex::Regex;
use crate::interpreter::lexer::tokens::assignable_tokens::integer_token::NumberTokenErr;

#[derive(Debug)]
pub struct DoubleToken {
    value: f64
}

impl FromStr for DoubleToken {
    type Err = NumberTokenErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(regex) = Regex::new("^[+-]?(\\d+\\.\\d*|\\d*\\.\\d+)$") {
            if !regex.is_match(s) {
                return Err(NumberTokenErr::UnmatchedRegex);
            }
        }
        
        Ok(DoubleToken {
            value: s.parse::<f64>()?,
        })
    }
}
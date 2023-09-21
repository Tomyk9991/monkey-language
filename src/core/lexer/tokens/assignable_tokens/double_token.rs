use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::core::lexer::tokens::assignable_tokens::integer_token::NumberTokenErr;

#[derive(Debug, PartialEq, Clone)]
pub struct FloatToken {
    pub value: f64
}

impl Display for FloatToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}


impl FromStr for FloatToken {
    type Err = NumberTokenErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !lazy_regex::regex_is_match!("^[+-]?(\\d+\\.\\d*|\\d*\\.\\d+)$", s) {
            return Err(NumberTokenErr::UnmatchedRegex);
        }
        
        Ok(FloatToken {
            value: s.parse::<f64>()?,
        })
    }
}
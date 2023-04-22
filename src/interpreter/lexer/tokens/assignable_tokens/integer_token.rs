use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
use regex::Regex;

#[derive(Debug)]
pub struct IntegerToken {
    value: i32
}

#[derive(Debug)]
pub enum IntegerTokenErr {
    UnmatchedRegex,
    ParsingError(ParseIntError)
}

impl From<ParseIntError> for IntegerTokenErr {
    fn from(value: ParseIntError) -> Self {
        IntegerTokenErr::ParsingError(value)
    }
}

impl Error for IntegerTokenErr { }

impl Display for IntegerTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            IntegerTokenErr::UnmatchedRegex => "Integer must match ^[+-]?\\d+$".to_string(),
            IntegerTokenErr::ParsingError(err) => err.to_string()
        })
    }
}

impl FromStr for IntegerToken {
    type Err = IntegerTokenErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(regex) = Regex::new("^[+-]?\\d+$") {
            if !regex.is_match(s) {
                return Err(IntegerTokenErr::UnmatchedRegex);
            }
        }

        Ok(IntegerToken {
            value: s.parse::<i32>()?,
        })
    }
}
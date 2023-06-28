use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

#[derive(Default, Debug, Eq, PartialEq, Clone)]
pub struct IntegerToken {
    pub value: i32
}


impl Display for IntegerToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}


#[derive(Debug)]
pub enum NumberTokenErr {
    UnmatchedRegex,
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError)
}

impl From<ParseIntError> for NumberTokenErr {
    fn from(value: ParseIntError) -> Self {
        NumberTokenErr::ParseIntError(value)
    }
}

impl From<ParseFloatError> for NumberTokenErr {
    fn from(value: ParseFloatError) -> Self { NumberTokenErr::ParseFloatError(value) }
}

impl Error for NumberTokenErr { }

impl Display for NumberTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            NumberTokenErr::UnmatchedRegex => "Integer must match ^[+-]?\\d+$".to_string(),
            NumberTokenErr::ParseIntError(err) => err.to_string(),
            NumberTokenErr::ParseFloatError(err) => err.to_string()
        })
    }
}

impl FromStr for IntegerToken {
    type Err = NumberTokenErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !lazy_regex::regex_is_match!("^[+-]?\\d+$", s) {
            return Err(NumberTokenErr::UnmatchedRegex);
        }

        Ok(IntegerToken {
            value: s.parse::<i32>()?,
        })
    }
}
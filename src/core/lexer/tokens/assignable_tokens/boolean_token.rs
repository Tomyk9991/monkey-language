use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::{FromStr, ParseBoolError};

#[derive(Debug, PartialEq, Clone)]
pub struct BooleanToken {
    pub value: bool
}

#[derive(Debug)]
pub enum BooleanTokenErr {
    UnmatchedRegex,
    ParseBoolError(ParseBoolError)
}

impl From<ParseBoolError> for BooleanTokenErr {
    fn from(value: ParseBoolError) -> Self { BooleanTokenErr::ParseBoolError(value) }
}

impl Display for BooleanTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            BooleanTokenErr::UnmatchedRegex => "Boolean must match ^(?i:true|false)$".to_string(),
            BooleanTokenErr::ParseBoolError(err) => err.to_string()
        })
    }
}

impl Error for BooleanTokenErr { }

impl Display for BooleanToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.value) }
}

impl FromStr for BooleanToken {
    type Err = BooleanTokenErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !lazy_regex::regex_is_match!("^(?i:true|false)$", s) {
            return Err(BooleanTokenErr::UnmatchedRegex);
        }

        Ok(BooleanToken {
            value: s.parse::<bool>()?
        })
    }
}
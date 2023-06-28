use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StringToken {
    pub value: String
}

impl Display for StringToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
pub enum StringTokenErr {
    UnmatchedRegex,
}

impl Error for StringTokenErr { }

impl Display for StringTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            StringTokenErr::UnmatchedRegex => "Name must match: ^\".*\"$ ",
        })
    }
}


impl FromStr for StringToken {
    type Err = StringTokenErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !lazy_regex::regex_is_match!("^\".*\"$", s) {
            return Err(StringTokenErr::UnmatchedRegex);
        }

        Ok(StringToken {
            value: s.to_string()
        })
    }
}
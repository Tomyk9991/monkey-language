use std::fmt::{Display, Formatter};
use std::str::ParseBoolError;

#[derive(Debug, PartialEq, Clone)]
pub struct Boolean {
    pub value: bool,
}

#[derive(Debug)]
pub enum BooleanErr {
    UnmatchedRegex,
    ParseBoolError(ParseBoolError),
}


#[derive(Debug)]
pub enum BooleanError {
    UnmatchedRegex,
    ParseBoolError(ParseBoolError),
}

impl Display for Boolean {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.value) }
}

impl Display for BooleanErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            BooleanErr::UnmatchedRegex => "Boolean must match ^(?i:true|false)$".to_string(),
            BooleanErr::ParseBoolError(err) => err.to_string()
        })
    }
}



impl Default for Boolean {
    fn default() -> Self {
        Boolean {
            value: false
        }
    }
}

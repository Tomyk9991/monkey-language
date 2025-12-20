use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialOrd, PartialEq, Clone, Default)]
pub struct StaticString {
    pub value: String,
}

#[derive(Debug)]
pub enum StaticStringError {
    UnmatchedRegex,
}

impl std::error::Error for StaticStringError {}

impl Display for StaticStringError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            StaticStringError::UnmatchedRegex => "Name must match: ^\".*\"$ ",
        })
    }
}

impl Display for StaticString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
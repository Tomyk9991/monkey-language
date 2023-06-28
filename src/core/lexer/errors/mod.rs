use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default)]
pub struct EmptyIteratorErr;

impl Display for EmptyIteratorErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Iterator is empty")
    }
}

impl Error for EmptyIteratorErr { }
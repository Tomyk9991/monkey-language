use std::error::Error;
use std::fmt::{Display, Formatter};

/// AST node for a name. Basically a string that can be used as a variable name.
/// Everything is allowed except for reserved keywords and special characters in the beginning
#[derive(Debug, Eq, PartialEq, Default, Hash, Clone)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug)]
pub enum IdentifierError {
    UnmatchedRegex { target_value: String },
    KeywordReserved(String),
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl std::error::Error for IdentifierError {}

impl Display for IdentifierError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            IdentifierError::UnmatchedRegex { target_value } => format!("\"{target_value}\" must match: ^[a-zA-Z_$][a-zA-Z_$0-9$]*$"),
            IdentifierError::KeywordReserved(value) => {
                format!("The variable name \"{}\" variable name can't have the same name as a reserved keyword", value)
            }
        };
        write!(f, "{}", message)
    }
}
use std::fmt::{Display, Formatter};

/// AST node for a name. Basically a string that can be used as a variable name.
/// Everything is allowed except for reserved keywords and special characters in the beginning
#[derive(Debug, Eq, PartialOrd, PartialEq, Default, Hash, Clone)]
pub struct Identifier {
    pub name: String,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
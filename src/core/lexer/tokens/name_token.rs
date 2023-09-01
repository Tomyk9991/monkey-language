use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::core::constants::KEYWORDS;

/// Token for a name. Basically a string that can be used as a variable name.
/// Everything is allowed except for reserved keywords and special characters in the beginning
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NameToken {
    pub name: String,
}

#[derive(Debug)]
pub enum NameTokenErr {
    UnmatchedRegex { target_value: String },
    KeywordReserved(String),
}

impl Display for NameToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Error for NameTokenErr {}

impl Display for NameTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            NameTokenErr::UnmatchedRegex { target_value } => format!("\"{target_value}\" must match: ^[a-zA-Z_$][a-zA-Z_$0-9$]*$"),
            NameTokenErr::KeywordReserved(value) => {
                format!("The variable name \"{}\" variable name can't have the same name as a reserved keyword", value)
            }
        };
        write!(f, "{}", message)
    }
}

impl NameToken {
    pub fn from_str(s: &str, allow_reserved: bool) -> Result<NameToken, NameTokenErr> {
        if !allow_reserved && KEYWORDS.iter().any(|keyword| keyword.to_lowercase() == s.to_lowercase()) {
            return Err(NameTokenErr::KeywordReserved(s.to_string()));
        }

        if !lazy_regex::regex_is_match!("^[a-zA-Z_$][a-zA-Z_$0-9$]*$", s) {
            return Err(NameTokenErr::UnmatchedRegex {
                target_value: s.to_string(),
            });
        }

        Ok(NameToken {
            name: s.to_string()
        })
    }
}
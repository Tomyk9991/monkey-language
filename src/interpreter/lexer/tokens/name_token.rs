use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use regex::Regex;
use crate::interpreter::constants::KEYWORDS;

#[derive(Debug)]
pub struct NameToken {
    name: String,
}

#[derive(Debug)]
pub enum NameTokenErr {
    UnmatchedRegex { target_value: String },
    KeywordReserved(String),
}

impl Error for NameTokenErr {}

impl Display for NameTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            NameTokenErr::UnmatchedRegex { target_value} => format!("\"{target_value}\" must match: ^[a-zA-Z_$][a-zA-Z_$0-9$]*$"),
            NameTokenErr::KeywordReserved(value) => {
                format!("The variable name \"{}\" variable name can't have the same name as a reserved keyword", value.to_string())
            }
        };
            write!(f, "{}", message)
        }
    }

    impl FromStr for NameToken {
        type Err = NameTokenErr;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if KEYWORDS.iter().any(|keyword| keyword.to_lowercase() == s.to_lowercase()) {
                return Err(NameTokenErr::KeywordReserved(s.to_string()));
            }

            if let Ok(regex) = Regex::new("^[a-zA-Z_$][a-zA-Z_$0-9$]*$") {
                if !regex.is_match(s) {
                    return Err(NameTokenErr::UnmatchedRegex {
                        target_value: s.to_string(),
                    });
                }
            }

            Ok(NameToken {
                name: s.to_string()
            })
        }
    }
use std::arch::x86_64::_mm_extract_si64;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use regex::Regex;
use crate::interpreter::constants::KEYWORDS;
use crate::interpreter::lexer::tokens::assignable_token::AssignableToken;
use crate::interpreter::lexer::tokens::assignable_tokens::string_token::StringToken;

pub struct NameToken {
    name: String,
}

#[derive(Debug)]
pub enum NameTokenErr {
    UnmatchedRegex,
    KeywordReserved(String),
}

impl Error for NameTokenErr {}

impl Display for NameTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            NameTokenErr::UnmatchedRegex => "Name must match: ^[a-zA-Z_$][a-zA-Z_$0-9$]*$".to_string(),
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
                    return Err(NameTokenErr::UnmatchedRegex);
                }
            }

            Ok(NameToken {
                name: s.to_string()
            })
        }
    }
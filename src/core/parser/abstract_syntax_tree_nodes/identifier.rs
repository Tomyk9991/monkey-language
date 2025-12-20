use uuid::Uuid;

use crate::core::constants::KEYWORDS;
use crate::core::lexer::token_with_span::FilePosition;
use crate::core::model::abstract_syntax_tree_nodes::identifier::{Identifier, IdentifierError};


impl Identifier {
    pub fn uuid() -> Identifier {
        Identifier {
            name: Uuid::new_v4().to_string(),
        }
    }

    pub fn from_str(s: &str, allow_reserved: bool) -> Result<Identifier, IdentifierError> {
        if !allow_reserved && KEYWORDS.iter().any(|keyword| keyword.to_lowercase() == s.to_lowercase()) {
            return Err(IdentifierError::KeywordReserved(s.to_string(), FilePosition::default()));
        }

        if !lazy_regex::regex_is_match!(r"^[a-zA-Z_$][a-zA-Z_$0-9]*$", s) {
            return Err(IdentifierError::UnmatchedRegex {
                target_value: s.to_string(),
            });
        }

        Ok(Identifier {
            name: s.to_string()
        })
    }
}
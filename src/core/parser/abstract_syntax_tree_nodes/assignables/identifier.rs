use crate::core::constants::KEYWORDS;
use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token_with_span::{TokenWithSpan};
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;

impl Parse for Identifier {
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default {
        let allow_reserved = false;
        let target_identifier = format!("{}", tokens[0].token);

        if !allow_reserved && KEYWORDS.iter().any(|keyword| keyword.to_lowercase() == target_identifier.to_lowercase()) {
            return Err(Error::UnexpectedToken(tokens[0].clone()));
        }

        if !lazy_regex::regex_is_match!(r"^[a-zA-Z_$][a-zA-Z_$0-9]*$", &target_identifier) {
            return Err(Error::UnexpectedToken(tokens[0].clone()));
        }

        Ok(ParseResult {
            consumed: 1,
            result: Identifier {
                name: target_identifier.to_string()
            }
        })
    }
}
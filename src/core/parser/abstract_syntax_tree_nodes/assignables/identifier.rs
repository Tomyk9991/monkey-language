use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;

impl Parse for Identifier {
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default {
        if let Ok(identifier) = Identifier::from_str(&format!("{}", tokens[0].token), false) {
            Ok(ParseResult {
                consumed: 1,
                result: identifier
            })
        } else {
            Err(Error::UnexpectedToken(tokens[0].clone()))
        }
    }
}
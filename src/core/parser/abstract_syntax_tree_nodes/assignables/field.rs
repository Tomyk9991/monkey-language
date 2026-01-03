use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::abstract_syntax_tree_nodes::struct_::Field;
use crate::core::model::types::ty::Type;
use crate::pattern;

impl Parse for Field {
    fn parse(tokens: &[TokenWithSpan], _options: ParseOptions) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default {
        if let Some(MatchResult::Parse(identifier)) = pattern!(tokens, @ parse Identifier, Colon,) {
            if let Some(MatchResult::Parse(ty)) = pattern!(&tokens[identifier.consumed + 1..], @ parse Type,) {
                let consumed = identifier.consumed + ty.consumed + 1;
                return Ok(ParseResult {
                    result: Field {
                        name: identifier.result,
                        ty: ty.result,
                    },
                    consumed,
                });
            }
        }
        
        Err(Error::UnexpectedToken(tokens[0].clone()))
    }
}
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable};
use crate::core::model::abstract_syntax_tree_nodes::for_::For;
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::scope::Scope;
use crate::pattern;

impl Parse for For {
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, crate::core::lexer::error::Error> where Self: Sized, Self: Default {
        if let Some(MatchResult::Parse(variable)) = pattern!(tokens, For, ParenthesisOpen, @ parse Variable::<'=', ';'>,) {
            if let Some(MatchResult::Parse(assignable)) = pattern!(&tokens[variable.consumed + 2..], @ parse Assignable, SemiColon) {
                if let Some(MatchResult::Parse(update_variable)) = pattern!(&tokens[variable.consumed + assignable.consumed + 3..], @ parse Variable<'=', ';'>, ParenthesisClose) {
                    let scope = Scope::parse(&tokens[variable.consumed + assignable.consumed + update_variable.consumed + 4..], ParseOptions::default())
                        .map_err(|e| crate::core::lexer::error::Error::Callstack(Box::new(e)).with_context(&tokens[0]))?;

                    return Ok(ParseResult {
                        result: For {
                            initialization: variable.result,
                            condition: assignable.result,
                            update: update_variable.result,
                            stack: scope.result.ast_nodes,
                            file_position: FilePosition::from_min_max(&tokens[0], &tokens[variable.consumed + assignable.consumed + update_variable.consumed + scope.consumed + 3]),
                        },
                        consumed: variable.consumed + assignable.consumed + update_variable.consumed + scope.consumed + 4,
                    });
                }
            }
        }

        Err(crate::core::lexer::error::Error::first_unexpected_token(&tokens[0..1], &[Token::For.into()]))
    }
}
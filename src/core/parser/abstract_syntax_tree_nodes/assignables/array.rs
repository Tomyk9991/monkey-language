use crate::core::lexer::collect_tokens_until_scope_close::CollectTokensFromUntil;
use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::types::array::Array;
use crate::core::parser::utils::dyck::dyck_language_generic;
use crate::pattern;

fn contains(a: &[TokenWithSpan], b: &TokenWithSpan) -> bool {
    a.iter().any(|x| x.token == b.token)
}

impl Parse for Array {
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default {
        let slice = tokens.iter().map(|x| x.token.clone()).collect::<Vec<Token>>();

        if let [Token::SquareBracketOpen, Token::SquareBracketClose] = &slice[..] {
            return Ok(ParseResult {
                result: Array {
                    values: vec![]
                },
                consumed: 2,
            })
        }

        if let Some(MatchResult::Collect(array_content)) = pattern!(tokens, SquareBracketOpen, @ parse CollectTokensFromUntil<'[', ']'>, SquareBracketClose) {
            let array_elements = dyck_language_generic(&array_content, [vec!['{', '('], vec![','], vec!['}', ')']], vec![], contains)
                .map_err(|_| Error::UnexpectedToken(tokens[0].clone()))?;

            if array_elements.is_empty() {
                return Err(Error::UnexpectedToken(tokens[0].clone()));
            }

            let mut values = vec![];

            for array_element in &array_elements {
                values.push(Assignable::parse(array_element, ParseOptions::default())?);
            }


            let tokens_consumed_square_brackets = 2;
            let tokens_consumed_assign = array_elements.iter().fold(0, |acc, x| acc + x.len());
            let tokens_consumed_separator = array_elements.len() - 1;

            return Ok(ParseResult {
                result: Array {
                    values: values.iter().map(|x| x.result.clone()).collect::<Vec<Assignable>>(),
                },
                consumed: tokens_consumed_square_brackets + tokens_consumed_assign + tokens_consumed_separator,
            })
        }

        Err(Error::UnexpectedToken(tokens[0].clone()))
    }
}
use crate::core::lexer::collect_tokens_until_scope_close::CollectTokensFromUntil;
use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::abstract_syntax_tree_nodes::assignables::object::Object;
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::types::ty::Type;
use crate::core::parser::utils::dyck::dyck_language;
use crate::pattern;

fn contains(a: &[TokenWithSpan], b: &TokenWithSpan) -> bool {
    a.iter().any(|x| x.token == b.token)
}

impl Parse for Object {
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default {
        if let Some(MatchResult::Parse(ty)) = pattern!(tokens, @ parse Type,) {
            if let Some(MatchResult::Collect(parsed_fields)) = pattern!(&tokens[ty.consumed..], CurlyBraceOpen, @ parse CollectTokensFromUntil<'{', '}'>, CurlyBraceClose,) {
                let parsed_fields = dyck_language(&parsed_fields, [vec!['(', '{'], vec![','], vec![')', '}']], vec!['}'], contains)
                    .map_err(|_| Error::UnexpectedToken(tokens[0].clone()))?
                    .iter()
                    .map(|param| Variable::<':', ','>::parse(param, ParseOptions::default()))
                    .collect::<Result<Vec<ParseResult<_>>, Error>>()?;

                let amount_kommata = (parsed_fields.len() as isize - 1).max(0) as usize;

                let consumed = ty.consumed +
                    parsed_fields.iter().map(|p| p.consumed).sum::<usize>() +
                    amount_kommata +
                    2;

                return Ok(ParseResult {
                    result: Object {
                        ty: ty.result,
                        fields: parsed_fields.iter().map(|p| p.result.clone()).collect(),
                    },
                    consumed,
                });
            }
        }

        Err(Error::UnexpectedToken(tokens[0].clone()))
    }
}
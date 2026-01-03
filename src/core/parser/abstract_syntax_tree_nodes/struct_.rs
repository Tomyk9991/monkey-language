use crate::core::lexer::collect_tokens_until_scope_close::CollectTokensFromUntil;
use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};
use crate::core::model::abstract_syntax_tree_nodes::struct_::{Field, Struct};
use crate::core::model::types::ty::Type;
use crate::core::parser::utils::dyck::dyck_language;
use crate::pattern;

fn contains(a: &[TokenWithSpan], b: &TokenWithSpan) -> bool {
    a.iter().any(|x| x.token == b.token)
}

impl Parse for Struct {
    fn parse(tokens: &[TokenWithSpan], _options: ParseOptions) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default {
        if let Some(MatchResult::Parse(struct_type)) = pattern!(tokens, Struct, @ parse Type,) {
            if let Some(MatchResult::Collect(parsed_fields)) = pattern!(&tokens[struct_type.consumed + 1..], CurlyBraceOpen, @ parse CollectTokensFromUntil<'{', '}'>, CurlyBraceClose) {
                let parsed_fields = dyck_language(
                    &parsed_fields,
                    [vec!['(', '{'], vec![','], vec![')', '}']],
                    vec!['}'],
                    contains,
                )
                .map_err(|_| Error::UnexpectedToken(tokens[0].clone()))?
                .iter()
                .map(|field_tokens| Field::parse(field_tokens, ParseOptions::default()))
                .collect::<Result<Vec<ParseResult<_>>, Error>>()?;

                let amount_kommata = (parsed_fields.len() as isize - 1).max(0) as usize;
                let consumed = struct_type.consumed
                    + parsed_fields.iter().map(|f| f.consumed).sum::<usize>()
                    + amount_kommata
                    + 3;

                return Ok(ParseResult {
                    result: Struct {
                        ty: struct_type.result,
                        fields: parsed_fields.iter().map(|p| p.result.clone()).collect(),
                        file_position: FilePosition::from_min_max(
                            &tokens[0],
                            &tokens[consumed - 1],
                        ),
                    },
                    consumed,
                });
            }
        }

        Err(Error::UnexpectedToken(tokens[0].clone()))
    }
}

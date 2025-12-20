use crate::core::lexer::collect_tokens_until_scope_close::CollectTokensFromUntil;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::abstract_syntax_tree_nodes::method_definition::{MethodArgument, MethodDefinition};
use crate::core::model::scope::Scope;
use crate::core::model::types::ty::Type;
use crate::core::parser::utils::dyck::dyck_language_generic;
use crate::pattern;

fn contains(a: &[TokenWithSpan], b: &TokenWithSpan) -> bool {
    a.iter().any(|x| x.token == b.token)
}

impl Parse for MethodDefinition {
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, crate::core::lexer::error::Error> where Self: Sized, Self: Default {
        // extern fn name(args): return_type;
        if let Some(MatchResult::Parse(parsed_fn_name)) = pattern!(tokens, Extern, Fn, @ parse LValue,) {
            if let Some(MatchResult::Collect(parsed_parameters)) = pattern!(&tokens[parsed_fn_name.consumed + 2..], ParenthesisOpen, @ parse CollectTokensFromUntil<'(', ')'>, ParenthesisClose) {
                if let Some(MatchResult::Parse(parsed_return_type)) = pattern!(&tokens[parsed_fn_name.consumed + parsed_parameters.len() + 4..], Colon, @ parse Type, SemiColon) {
                    let const_tokens = 6;
                    let parsed_parameters = dyck_language_generic(&parsed_parameters, [vec!['(', '{'], vec![','], vec![')', '}']], vec![')'], contains)
                        .map_err(|_| crate::core::lexer::error::Error::UnexpectedToken(tokens[0].clone()))?
                        .iter()
                        .map(|param| MethodArgument::parse(param, ParseOptions::default()))
                        .collect::<Result<Vec<ParseResult<_>>, crate::core::lexer::error::Error>>()?;


                    let amount_kommata = (parsed_parameters.len() as isize - 1).max(0) as usize;
                    let consumed = parsed_fn_name.consumed +
                        parsed_return_type.consumed +
                        parsed_parameters.iter().map(|p| p.consumed).sum::<usize>() +
                        amount_kommata +
                        const_tokens;
                    return Ok(ParseResult {
                        result: MethodDefinition {
                            identifier: parsed_fn_name.result,
                            return_type: parsed_return_type.result,
                            arguments: parsed_parameters.iter().map(|p| p.result.clone()).collect(),
                            stack: vec![],
                            is_extern: true,
                            file_position: FilePosition::from_min_max(&tokens[0], &tokens[consumed - 1]),
                        },
                        consumed,
                    })
                }
            }

        }

        if let Some(MatchResult::Parse(parsed_fn_name)) = pattern!(tokens, Fn, @ parse LValue,) {
            if let Some(MatchResult::Collect(parsed_parameters)) = pattern!(&tokens[parsed_fn_name.consumed + 1..], ParenthesisOpen, @ parse CollectTokensFromUntil<'(', ')'>, ParenthesisClose) {
                if let Some(MatchResult::Parse(parsed_return_type)) = pattern!(&tokens[parsed_fn_name.consumed + parsed_parameters.len() + 3..], Colon, @ parse Type,) {
                    // fn name(args): return_type
                    let const_tokens = 4;
                    let parsed_parameters_tokens_consumed = parsed_parameters.len();
                    let parsed_parameters = dyck_language_generic(&parsed_parameters, [vec!['(', '{'], vec![','], vec![')', '}']], vec![')'], contains)
                        .map_err(|_| crate::core::lexer::error::Error::UnexpectedToken(tokens[0].clone()))?
                        .iter()
                        .map(|param| MethodArgument::parse(param, ParseOptions::default()))
                        .collect::<Result<Vec<ParseResult<_>>, crate::core::lexer::error::Error>>()?;

                    let scope = Scope::parse(&tokens[parsed_fn_name.consumed + parsed_parameters_tokens_consumed + parsed_return_type.consumed + 4..], ParseOptions::default())
                        .map_err(|e| crate::core::lexer::error::Error::Callstack(Box::new(e)).with_context(&tokens[0]))?;

                    let amount_kommata = (parsed_parameters.len() as isize - 1).max(0) as usize;
                    let consumed = parsed_fn_name.consumed +
                        parsed_return_type.consumed +
                        parsed_parameters.iter().map(|p| p.consumed).sum::<usize>() +
                        amount_kommata +
                        const_tokens +
                        scope.consumed;
                    return Ok(ParseResult {
                        result: MethodDefinition {
                            identifier: parsed_fn_name.result,
                            return_type: parsed_return_type.result,
                            arguments: parsed_parameters.iter().map(|p| p.result.clone()).collect(),
                            stack: scope.result.ast_nodes.to_vec(),
                            is_extern: false,
                            file_position: FilePosition::from_min_max(&tokens[0], &tokens[consumed - 1]),
                        },
                        consumed,
                    })
                }
            }
        }

        if let Some(MatchResult::Parse(parsed_fn_name)) = pattern!(tokens, Fn, @ parse LValue,) {
            if let Some(MatchResult::Collect(parsed_parameters)) = pattern!(&tokens[parsed_fn_name.consumed + 1..], ParenthesisOpen, @ parse CollectTokensFromUntil<'(', ')'>, ParenthesisClose) {
                // fn name(args)
                let const_tokens = 3;
                let parsed_parameters_tokens_consumed = parsed_parameters.len();
                let parsed_parameters = dyck_language_generic(&parsed_parameters, [vec!['(', '{'], vec![','], vec![')', '}']], vec![')'], contains)
                    .map_err(|_| crate::core::lexer::error::Error::UnexpectedToken(tokens[0].clone()))?
                    .iter()
                    .map(|param| MethodArgument::parse(param, ParseOptions::default()))
                    .collect::<Result<Vec<ParseResult<_>>, crate::core::lexer::error::Error>>()?;

                let scope = Scope::parse(&tokens[parsed_fn_name.consumed + parsed_parameters_tokens_consumed + 3..], ParseOptions::default())
                    .map_err(|e| crate::core::lexer::error::Error::Callstack(Box::new(e)).with_context(&tokens[0]))?;

                let return_type = ParseResult {
                    result: Type::Void,
                    consumed: 0,
                };

                let amount_kommata = (parsed_parameters.len() as isize - 1).max(0) as usize;
                let consumed = parsed_fn_name.consumed +
                    return_type.consumed +
                    parsed_parameters.iter().map(|p| p.consumed).sum::<usize>() +
                    amount_kommata +
                    const_tokens +
                    scope.consumed;
                return Ok(ParseResult {
                    result: MethodDefinition {
                        identifier: parsed_fn_name.result,
                        return_type: return_type.result,
                        arguments: parsed_parameters.iter().map(|p| p.result.clone()).collect(),
                        stack: scope.result.ast_nodes.to_vec(),
                        is_extern: false,
                        file_position: FilePosition::from_min_max(&tokens[0], &tokens[consumed - 1]),
                    },
                    consumed,
                })
            }
        }

        Err(crate::core::lexer::error::Error::UnexpectedToken(tokens[0].clone()))
    }
}

impl Parse for MethodArgument {
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, crate::core::lexer::error::Error> where Self: Sized, Self: Default {
        if let Some(MatchResult::Parse(name)) = pattern!(tokens, @ parse LValue, Colon,) {
            if let Some(MatchResult::Parse(ty)) = pattern!(&tokens[name.consumed + 1..], @ parse Type,) {
                return Ok(ParseResult {
                    result: MethodArgument {
                        identifier: name.result,
                        ty: ty.result,
                    },
                    consumed: name.consumed + ty.consumed + 1,
                });
            }
        }

        Err(crate::core::lexer::error::Error::first_unexpected_token(&tokens[0..1], &[Token::Colon.into()]))
    }
}
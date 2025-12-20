use crate::core::lexer::collect_tokens_until_scope_close::CollectTokensFromUntil;
use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable};
use crate::core::model::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use crate::core::model::abstract_syntax_tree_nodes::identifier::IdentifierError;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::parser::utils::dyck::{dyck_language_generic, DyckError};
use crate::pattern;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum MethodCallErr {
    IdentifierErr(IdentifierError),
    DyckLanguageErr { target_value: String, ordering: Ordering },
}

impl std::error::Error for MethodCallErr {}

impl From<IdentifierError> for MethodCallErr {
    fn from(value: IdentifierError) -> Self {
        MethodCallErr::IdentifierErr(value)
    }
}

impl From<DyckError> for MethodCallErr {
    fn from(s: DyckError) -> Self {
        MethodCallErr::DyckLanguageErr { target_value: s.target_value, ordering: s.ordering }
    }
}

impl Display for MethodCallErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            MethodCallErr::IdentifierErr(a) => a.to_string(),
            MethodCallErr::DyckLanguageErr { target_value, ordering } =>
                {
                    let error: String = match ordering {
                        Ordering::Less => String::from("Expected `)`"),
                        Ordering::Equal => String::from("Expected expression between `,`"),
                        Ordering::Greater => String::from("Expected `(`")
                    };
                    format!("\"{target_value}\": {error}")
                }
        };

        write!(f, "{}", message)
    }
}


fn contains(a: &[TokenWithSpan], b: &TokenWithSpan) -> bool {
    a.iter().any(|x| x.token == b.token)
}


impl Parse for MethodCall {
    fn parse(tokens: &[TokenWithSpan], parse_options: ParseOptions) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default {
        if parse_options.ends_with_semicolon {
            if let Some(MatchResult::Parse(fn_name)) = pattern!(tokens, @ parse LValue,) {
                if let Some(MatchResult::Collect(parsed_parameters)) = pattern!(&tokens[fn_name.consumed..], ParenthesisOpen, @ parse CollectTokensFromUntil<'(', ')'>, ParenthesisClose, SemiColon) {
                    let parameters = dyck_language_generic(&parsed_parameters, [vec!['(', '{'], vec![','], vec![')', '}']], vec![')'], contains)
                        .map_err(|_| Error::UnexpectedToken(tokens[0].clone()))?
                        .iter()
                        .map(|param| Assignable::parse(param, ParseOptions::default()))
                        .collect::<Result<Vec<ParseResult<_>>, Error>>()?;

                    let amount_kommata = (parameters.len() as isize - 1).max(0) as usize;

                    // Ensure all tokens which were parsed as parameters were consumed
                    if parameters.iter().map(|p| p.consumed).sum::<usize>() + amount_kommata != parsed_parameters.len() {
                        return Err(Error::UnexpectedToken(tokens[0].clone()));
                    }

                    let consumed = fn_name.consumed +
                        parameters.iter().map(|p| p.consumed).sum::<usize>() +
                        amount_kommata +
                        3;

                    return Ok(ParseResult {
                        result: MethodCall {
                            identifier: fn_name.result,
                            arguments: parameters.iter().map(|p| p.result.clone()).collect(),
                            file_position: FilePosition::from_min_max(&tokens[0], &tokens[consumed - 1]),
                        },
                        consumed,
                    })
                }
            }
        }

        if let Some(MatchResult::Parse(fn_name)) = pattern!(tokens, @ parse LValue,) {
            if let Some(MatchResult::Collect(parsed_parameters)) = pattern!(&tokens[fn_name.consumed..], ParenthesisOpen, @ parse CollectTokensFromUntil<'(', ')'>, ParenthesisClose) {
                let parameters = dyck_language_generic(&parsed_parameters, [vec!['(', '{'], vec![','], vec![')', '}']], vec![')'], contains)
                    .map_err(|_| Error::UnexpectedToken(tokens[0].clone()))?
                    .iter()
                    .map(|param| Assignable::parse(param, ParseOptions::default()))
                    .collect::<Result<Vec<ParseResult<_>>, Error>>()?;

                let amount_kommata = (parameters.len() as isize - 1).max(0) as usize;

                // Ensure all tokens which were parsed as parameters were consumed
                if parameters.iter().map(|p| p.consumed).sum::<usize>() + amount_kommata != parsed_parameters.len() {
                    return Err(Error::UnexpectedToken(tokens[0].clone()));
                }

                let consumed = fn_name.consumed +
                    parameters.iter().map(|p| p.consumed).sum::<usize>() +
                    amount_kommata +
                    2;

                return Ok(ParseResult {
                    result: MethodCall {
                        identifier: fn_name.result,
                        arguments: parameters.iter().map(|p| p.result.clone()).collect(),
                        file_position: FilePosition::from_min_max(&tokens[0], &tokens[consumed - 1]),
                    },
                    consumed,
                })
            }
        }

        Err(Error::UnexpectedToken(tokens[0].clone()))
    }
}
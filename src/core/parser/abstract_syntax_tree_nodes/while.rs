use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable};
use crate::core::model::abstract_syntax_tree_nodes::while_::While;
use crate::core::model::scope::Scope;
use crate::core::parser::utils::dyck::DyckError;
use crate::pattern;
use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum WhileErr {
    DyckLanguageErr { target_value: String, ordering: Ordering },
}

impl From<DyckError> for WhileErr {
    fn from(value: DyckError) -> Self {
        WhileErr::DyckLanguageErr { target_value: value.target_value, ordering: value.ordering }
    }
}

impl Error for WhileErr { }

impl Display for WhileErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            WhileErr::DyckLanguageErr { target_value, ordering } => {
                let error: String = match ordering {
                    Ordering::Less => String::from("Expected `)`"),
                    Ordering::Equal => String::from("Expected expression between `,`"),
                    Ordering::Greater => String::from("Expected `(`")
                };
                format!("\"{target_value}\": {error}")
            }
        })
    }
}


impl Parse for While {
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, crate::core::lexer::error::Error> where Self: Sized, Self: Default {
        if let Some(MatchResult::Parse(condition)) = pattern!(tokens, While, ParenthesisOpen, @ parse Assignable, ParenthesisClose) {
            let scope = Scope::parse(&tokens[condition.consumed + 3..], ParseOptions::default())
                .map_err(|e| crate::core::lexer::error::Error::Callstack(Box::new(e)).with_context(&tokens[0]))?;

            return Ok(ParseResult {
                result: While {
                    condition: condition.result,
                    stack: scope.result.ast_nodes,
                    file_position: FilePosition::from_min_max(&tokens[0], &tokens[condition.consumed + scope.consumed + 2]),
                },
                consumed: condition.consumed + scope.consumed + 3,
            })
        }

        Err(crate::core::lexer::error::Error::first_unexpected_token(&tokens[0..1], &[Token::While.into()]))
    }
}
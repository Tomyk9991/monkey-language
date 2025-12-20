use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable};
use crate::core::model::abstract_syntax_tree_nodes::ret::Return;
use crate::core::model::types::integer::{IntegerAST, IntegerType};
use crate::pattern;
use std::error::Error;
use std::fmt::{Display, Formatter};

impl Return {
    /// returns a `Return` with an assignable, the assignable is an integer containing 0
    pub fn num_0() -> Return {
        Return {
            assignable: Some(Assignable::Integer(IntegerAST { value: "0".to_string(), ty: IntegerType::I32 })),
            file_position: FilePosition::default(),
        }
    }
}

#[derive(Debug)]
pub enum ReturnError {
    PatternNotMatched { target_value: String },
}

impl Display for ReturnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ReturnError::PatternNotMatched { target_value } => {
                format!("Pattern not matched for: `{}?\n\t return assignable;", target_value)
            }
        })
    }
}

impl Error for ReturnError { }

impl From<anyhow::Error> for ReturnError {
    fn from(value: anyhow::Error) -> Self {
        ReturnError::PatternNotMatched { target_value: value.to_string() }
    }
}

impl Parse for Return {
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, crate::core::lexer::error::Error> where Self: Sized, Self: Default {
        if let Some(MatchResult::Parse(assignable)) = pattern!(tokens, Return, @ parse Assignable, SemiColon) {
            return Ok(ParseResult {
                result: Return {
                    assignable: Some(assignable.result),
                    file_position: FilePosition::from_min_max(&tokens[0], &tokens[assignable.consumed + 1]),
                },
                consumed: assignable.consumed + 2,
            })
        }

        if let [TokenWithSpan { token: Token::Return, ..}, TokenWithSpan { token: Token::SemiColon, ..}, ..] = &tokens {
            return Ok(ParseResult {
                result: Return {
                    assignable: None,
                    file_position: FilePosition::from_min_max(&tokens[0], &tokens[1]),
                },
                consumed: 2,
            })
        }

        Err(crate::core::lexer::error::Error::first_unexpected_token(&tokens[0..1], &[Token::Return.into()]))
    }
}
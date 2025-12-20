use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::asm_result::{ASMResult, ASMResultError, ASMResultVariance};
use crate::core::code_generator::conventions::return_calling_convention;
use crate::core::code_generator::registers::{ByteSize, GeneralPurposeRegister};
use crate::core::lexer::error::ErrorMatch;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::model::abstract_syntax_tree_nodes::ret::Return;
use crate::core::model::types::integer::{IntegerAST, IntegerType};
use crate::core::parser::errors::EmptyIteratorErr;
use crate::core::parser::scope::PatternNotMatchedError;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::{InferTypeError};
use crate::core::semantics::static_type_check::static_type_checker::StaticTypeCheckError;
use crate::core::semantics::static_type_check::static_type_check::StaticTypeCheck;
use crate::pattern;

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
    AssignableError(AssignableError),
    EmptyIterator(EmptyIteratorErr)
}

impl PatternNotMatchedError for ReturnError {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, ReturnError::PatternNotMatched {..})
    }
}


impl Display for ReturnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ReturnError::PatternNotMatched { target_value } => {
                format!("Pattern not matched for: `{}?\n\t return assignable;", target_value)
            }
            ReturnError::AssignableError(e) => e.to_string(),
            ReturnError::EmptyIterator(e) => e.to_string(),
        })
    }
}

impl Error for ReturnError { }

impl From<AssignableError> for ReturnError {
    fn from(value: AssignableError) -> Self {
        ReturnError::AssignableError(value)
    }
}

impl From<anyhow::Error> for ReturnError {
    fn from(value: anyhow::Error) -> Self {
        ReturnError::PatternNotMatched { target_value: value.to_string() }
    }
}

impl Parse for Return {
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, crate::core::lexer::error::Error> where Self: Sized, Self: Default {
        if let Some((MatchResult::Parse(assignable))) = pattern!(tokens, Return, @ parse Assignable, SemiColon) {
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

        Err(crate::core::lexer::error::Error::first_unexpected_token(&tokens[0..1], &vec![Token::Return.into()]))
    }
}
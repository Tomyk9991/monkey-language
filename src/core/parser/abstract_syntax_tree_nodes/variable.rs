use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::model::abstract_syntax_tree_nodes::identifier::{IdentifierError};
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::types::ty::Type;
use crate::core::parser::errors::EmptyIteratorErr;
use crate::core::parser::scope::PatternNotMatchedError;
use crate::core::parser::abstract_syntax_tree_nodes::l_value::LValueErr;
use crate::core::parser::types::r#type::InferTypeError;
use crate::pattern;

impl Parse for Variable<'=', ';'> {
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, crate::core::lexer::error::Error> where Self: Sized, Self: Default {
        if let Some(MatchResult::Parse(l_value)) = pattern!(tokens, Let, @parse LValue, Equals) {
            if let Some(MatchResult::Parse(assign)) = pattern!(&tokens[l_value.consumed + 2..], @parse Assignable, SemiColon) {
                return Ok(ParseResult {
                    result: Variable {
                        l_value: l_value.result,
                        mutability: false,
                        ty: None,
                        define: true,
                        assignable: assign.result,
                        file_position: FilePosition::from_min_max(&tokens[0], &tokens[l_value.consumed + assign.consumed + 2]),
                    },
                    consumed: l_value.consumed + assign.consumed + 3,
                });
            }
        }

        if let Some(MatchResult::Parse(l_value)) = pattern!(tokens, @parse LValue, Equals) {
            if let Some(MatchResult::Parse(assign)) = pattern!(&tokens[l_value.consumed + 1..], @parse Assignable, SemiColon) {
                return Ok(ParseResult {
                    result: Variable {
                        l_value: l_value.result,
                        mutability: false,
                        ty: None,
                        define: false,
                        assignable: assign.result,
                        file_position: FilePosition::from_min_max(&tokens[0], &tokens[l_value.consumed + assign.consumed + 1]),
                    },
                    consumed: l_value.consumed + assign.consumed + 2,
                });
            }
        }

        if let Some(MatchResult::Parse(l_value)) = pattern!(tokens, Let, Mut, @parse LValue, Equals) {
            if let Some(MatchResult::Parse(assign)) = pattern!(&tokens[l_value.consumed + 3..], @parse Assignable, SemiColon) {
                return Ok(ParseResult {
                    result: Variable {
                        l_value: l_value.result,
                        mutability: true,
                        ty: None,
                        define: true,
                        assignable: assign.result,
                        file_position: FilePosition::from_min_max(&tokens[0], &tokens[l_value.consumed + assign.consumed + 3]),
                    },
                    consumed: l_value.consumed + assign.consumed + 4,
                });
            }
        }

        if let Some(MatchResult::Parse(l_value)) = pattern!(tokens, Let, @parse LValue, Colon) {
            if let Some(MatchResult::Parse(ty)) = pattern!(&tokens[l_value.consumed + 2..], @parse Type, Equals) {
                if let Some(MatchResult::Parse(assign)) = pattern!(&tokens[l_value.consumed + ty.consumed + 3..], @parse Assignable, SemiColon) {
                    return Ok(ParseResult {
                        result: Variable {
                            l_value: l_value.result,
                            mutability: false,
                            ty: Some(ty.result),
                            define: true,
                            assignable: assign.result,
                            file_position: FilePosition::from_min_max(&tokens[0], &tokens[l_value.consumed + ty.consumed + assign.consumed + 3]),
                        },
                        consumed: l_value.consumed + ty.consumed + assign.consumed + 4,
                    });
                }
            }
        }

        if let Some(MatchResult::Parse(l_value)) = pattern!(tokens, Let, Mut, @parse LValue, Colon) {
            if let Some(MatchResult::Parse(ty)) = pattern!(&tokens[l_value.consumed + 3..], @parse Type, Equals) {
                if let Some(MatchResult::Parse(assign)) = pattern!(&tokens[l_value.consumed + ty.consumed + 4..], @parse Assignable, SemiColon) {
                    return Ok(ParseResult {
                        result: Variable {
                            l_value: l_value.result,
                            mutability: true,
                            ty: Some(ty.result),
                            define: true,
                            assignable: assign.result,
                            file_position: FilePosition::from_min_max(&tokens[0], &tokens[l_value.consumed + ty.consumed + assign.consumed + 4]),
                        },
                        consumed: l_value.consumed + ty.consumed + assign.consumed + 5,
                    });
                }
            }
        }


        Err(crate::core::lexer::error::Error::UnexpectedToken(tokens[0].clone()))
    }
}

impl<const ASSIGNMENT: char, const SEPARATOR: char> TryFrom<Result<ParseResult<Self>, crate::core::lexer::error::Error>> for Variable<ASSIGNMENT, SEPARATOR> {
    type Error = crate::core::lexer::error::Error;

    fn try_from(value: Result<ParseResult<Self>, crate::core::lexer::error::Error>) -> Result<Self, Self::Error> {
        match value {
            Ok(value) => Ok(value.result),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug)]
pub enum ParseVariableErr {
    PatternNotMatched { target_value: String },
    IdentifierErr(IdentifierError),
    AssignableErr(AssignableError),
    LValue(LValueErr),
    InferType(InferTypeError),
    EmptyIterator(EmptyIteratorErr),
}

impl PatternNotMatchedError for ParseVariableErr {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, ParseVariableErr::PatternNotMatched {..})
    }
}

impl Error for ParseVariableErr {}

impl From<InferTypeError> for ParseVariableErr {
    fn from(value: InferTypeError) -> Self {
        ParseVariableErr::InferType(value)
    }
}

impl From<LValueErr> for ParseVariableErr {
    fn from(value: LValueErr) -> Self {
        ParseVariableErr::LValue(value)
    }
}

impl From<IdentifierError> for ParseVariableErr {
    fn from(a: IdentifierError) -> Self { ParseVariableErr::IdentifierErr(a) }
}

impl From<anyhow::Error> for ParseVariableErr {
    fn from(value: anyhow::Error) -> Self {
        let mut buffer = String::new();
        buffer += &value.to_string();
        buffer += "\n";

        if let Some(e) = value.downcast_ref::<AssignableError>() {
            buffer += &e.to_string();
        }
        ParseVariableErr::PatternNotMatched { target_value: buffer }
    }
}

impl From<AssignableError> for ParseVariableErr {
    fn from(a: AssignableError) -> Self { ParseVariableErr::AssignableErr(a) }
}

impl Display for ParseVariableErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ParseVariableErr::PatternNotMatched { target_value } => format!("`{target_value}`\n\tThe pattern for a variable is defined as: lvalue = assignment;"),
            ParseVariableErr::IdentifierErr(a) => a.to_string(),
            ParseVariableErr::AssignableErr(a) => a.to_string(),
            ParseVariableErr::EmptyIterator(e) => e.to_string(),
            ParseVariableErr::InferType(err) => err.to_string(),
            ParseVariableErr::LValue(err) => err.to_string(),
        })
    }
}
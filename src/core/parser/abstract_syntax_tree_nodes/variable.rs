use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable};
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::types::ty::Type;
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
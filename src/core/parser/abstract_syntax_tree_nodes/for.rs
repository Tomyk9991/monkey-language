use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use crate::core::lexer::token::Token;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::model::abstract_syntax_tree_nodes::for_::For;
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::scope::Scope;
use crate::core::parser::errors::EmptyIteratorErr;
use crate::core::parser::scope::{PatternNotMatchedError, ScopeError};
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::abstract_syntax_tree_nodes::variable::ParseVariableErr;
use crate::core::parser::types::r#type::InferTypeError;
use crate::core::parser::utils::dyck::DyckError;
use crate::core::semantics::static_type_check::static_type_check::StaticTypeCheck;
use crate::core::semantics::static_type_check::static_type_checker::StaticTypeCheckError;
use crate::pattern;

#[derive(Debug)]
pub enum ForErr {
    PatternNotMatched { target_value: String },
    AssignableErr(AssignableError),
    ParseVariableErr(ParseVariableErr),
    ScopeErrorErr(ScopeError),
    DyckLanguageErr { target_value: String, ordering: Ordering },
    EmptyIterator(EmptyIteratorErr),
}

impl PatternNotMatchedError for ForErr {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, ForErr::PatternNotMatched { .. })
    }
}

impl From<DyckError> for ForErr {
    fn from(s: DyckError) -> Self {
        ForErr::DyckLanguageErr { target_value: s.target_value, ordering: s.ordering }
    }
}

impl From<ParseVariableErr> for ForErr {
    fn from(value: ParseVariableErr) -> Self {
        ForErr::ParseVariableErr(value)
    }
}

impl From<AssignableError> for ForErr {
    fn from(value: AssignableError) -> Self {
        ForErr::AssignableErr(value)
    }
}

impl Error for ForErr {}

impl Display for ForErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ForErr::PatternNotMatched { target_value } =>
                format!("Pattern mot matched for: `{target_value}`\n\t for (initializiation; condition; update) {{}}"),
            ForErr::AssignableErr(a) => a.to_string(),
            ForErr::ParseVariableErr(a) => a.to_string(),
            ForErr::ScopeErrorErr(a) => a.to_string(),
            ForErr::EmptyIterator(e) => e.to_string(),
            ForErr::DyckLanguageErr { target_value, ordering } =>
                {
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

impl Parse for For {
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, crate::core::lexer::error::Error> where Self: Sized, Self: Default {
        if let Some((MatchResult::Parse(variable))) = pattern!(tokens, For, ParenthesisOpen, @ parse Variable::<'=', ';'>,) {
            if let Some((MatchResult::Parse(assignable))) = pattern!(&tokens[variable.consumed + 2..], @ parse Assignable, SemiColon) {
                if let Some((MatchResult::Parse(update_variable))) = pattern!(&tokens[variable.consumed + assignable.consumed + 3..], @ parse Variable<'=', ';'>, ParenthesisClose) {
                    let scope = Scope::parse(&tokens[variable.consumed + assignable.consumed + update_variable.consumed + 4..], ParseOptions::default())
                        .map_err(|e| crate::core::lexer::error::Error::Callstack(Box::new(e)).with_context(&tokens[0]))?;

                    return Ok(ParseResult {
                        result: For {
                            initialization: variable.result,
                            condition: assignable.result,
                            update: update_variable.result,
                            stack: scope.result.ast_nodes,
                            file_position: FilePosition::from_min_max(&tokens[0], &tokens[variable.consumed + assignable.consumed + update_variable.consumed + scope.consumed + 3]),
                        },
                        consumed: variable.consumed + assignable.consumed + update_variable.consumed + scope.consumed + 4,
                    });
                }
            }
        }

        Err(crate::core::lexer::error::Error::first_unexpected_token(&tokens[0..1], &vec![Token::For.into()]))
    }
}
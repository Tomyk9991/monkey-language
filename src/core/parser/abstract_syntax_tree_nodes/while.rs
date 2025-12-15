use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::model::abstract_syntax_tree_nodes::while_::While;
use crate::core::model::scope::Scope;
use crate::core::parser::errors::EmptyIteratorErr;
use crate::core::parser::scope::{PatternNotMatchedError, ScopeError};
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::InferTypeError;
use crate::core::parser::utils::dyck::DyckError;
use crate::core::semantics::static_type_check::static_type_check::StaticTypeCheck;
use crate::core::semantics::static_type_check::static_type_checker::StaticTypeCheckError;
use crate::pattern;

#[derive(Debug)]
pub enum WhileErr {
    PatternNotMatched { target_value: String },
    AssignableErr(AssignableError),
    ScopeErrorErr(ScopeError),
    DyckLanguageErr { target_value: String, ordering: Ordering },
    EmptyIterator(EmptyIteratorErr)
}

impl PatternNotMatchedError for WhileErr {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, WhileErr::PatternNotMatched { .. })
    }
}

impl From<DyckError> for WhileErr {
    fn from(value: DyckError) -> Self {
        WhileErr::DyckLanguageErr { target_value: value.target_value, ordering: value.ordering }
    }
}

impl From<AssignableError> for WhileErr {
    fn from(value: AssignableError) -> Self {
        WhileErr::AssignableErr(value)
    }
}

impl Error for WhileErr { }

impl Display for WhileErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            WhileErr::PatternNotMatched { target_value } =>
                format!("Pattern not matched for: `{target_value}`\n\t while (condition) {{}}"),
            WhileErr::AssignableErr(a) => a.to_string(),
            WhileErr::EmptyIterator(e) => e.to_string(),
            WhileErr::ScopeErrorErr(a) => a.to_string(),
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

impl StaticTypeCheck for While {
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        // let variables_len = type_context.context.len();
        // let condition_type = self.condition.infer_type_with_context(type_context, &self.code_line)?;
        //
        // if !matches!(condition_type, Type::Bool(_)) {
        //     return Err(StaticTypeCheckError::InferredError(InferTypeError::MismatchedTypes {
        //         expected: Type::Bool(Mutability::Immutable),
        //         actual: condition_type,
        //         code_line: self.code_line.clone(),
        //     }));
        // }
        //
        // static_type_check_rec(&self.stack, type_context)?;
        //
        // let amount_pop = type_context.context.len() - variables_len;
        //
        // for _ in 0..amount_pop {
        //     let _ = type_context.context.pop();
        // }
        
        Ok(())
    }
}

impl Parse for While {
    fn parse(tokens: &[TokenWithSpan], options: ParseOptions) -> Result<ParseResult<Self>, crate::core::lexer::error::Error> where Self: Sized, Self: Default {
        if let Some((MatchResult::Parse(condition))) = pattern!(tokens, While, ParenthesisOpen, @ parse Assignable, ParenthesisClose) {
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

        Err(crate::core::lexer::error::Error::first_unexpected_token(&tokens[0..1], &vec![Token::While.into()]))
    }
}
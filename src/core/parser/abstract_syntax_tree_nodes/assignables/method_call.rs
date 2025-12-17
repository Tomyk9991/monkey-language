use std::any::Any;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use crate::core::code_generator::{ASMGenerateError, conventions, MetaInfo};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_options::in_expression_method_call::InExpressionMethodCall;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::asm_result::{ASMResult, ASMResultError, ASMResultVariance};
use crate::core::code_generator::conventions::CallingRegister;
use crate::core::code_generator::generator::{Stack};
use crate::core::code_generator::register_destination::byte_size_from_word;
use crate::core::code_generator::registers::{Bit64, ByteSize, GeneralPurposeRegister};
use crate::core::code_generator::ToASM;
use crate::core::lexer::collect_tokens_until_scope_close::CollectTokensFromUntil;
use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::model::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use crate::core::model::abstract_syntax_tree_nodes::identifier::{Identifier, IdentifierError};
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::abstract_syntax_tree_nodes::method_definition::{MethodArgument, MethodDefinition};
use crate::core::model::types::ty::Type;
use crate::core::parser::errors::EmptyIteratorErr;
use crate::core::parser::scope::PatternNotMatchedError;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::{InferTypeError, MethodCallArgumentTypeMismatch};
use crate::core::parser::utils::dyck::{dyck_language_generic, DyckError};
use crate::core::semantics::static_type_check::static_type_checker::StaticTypeCheckError;
use crate::core::semantics::static_type_check::static_type_check::StaticTypeCheck;
use crate::pattern;

#[derive(Debug)]
pub enum MethodCallErr {
    PatternNotMatched { target_value: String },
    IdentifierErr(IdentifierError),
    DyckLanguageErr { target_value: String, ordering: Ordering },
    AssignableErr(AssignableError),
    EmptyIterator(EmptyIteratorErr),
}

impl PatternNotMatchedError for MethodCallErr {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, MethodCallErr::PatternNotMatched {..}) || matches!(self, MethodCallErr::IdentifierErr(_))
    }
}

impl std::error::Error for MethodCallErr {}

impl From<IdentifierError> for MethodCallErr {
    fn from(value: IdentifierError) -> Self {
        MethodCallErr::IdentifierErr(value)
    }
}

impl From<AssignableError> for MethodCallErr {
    fn from(value: AssignableError) -> Self { MethodCallErr::AssignableErr(value) }
}

impl From<DyckError> for MethodCallErr {
    fn from(s: DyckError) -> Self {
        MethodCallErr::DyckLanguageErr { target_value: s.target_value, ordering: s.ordering }
    }
}

impl Display for MethodCallErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            MethodCallErr::PatternNotMatched { target_value } => format!("\"{target_value}\" must match: methodName(assignable1, ..., assignableN)"),
            MethodCallErr::AssignableErr(a) => a.to_string(),
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
            MethodCallErr::EmptyIterator(e) => e.to_string()
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
            if let Some((MatchResult::Parse(fn_name))) = pattern!(tokens, @ parse LValue,) {
                if let Some((MatchResult::Collect(parsed_parameters))) = pattern!(&tokens[fn_name.consumed..], ParenthesisOpen, @ parse CollectTokensFromUntil<'(', ')'>, ParenthesisClose, SemiColon) {
                    let parameters = dyck_language_generic(&parsed_parameters, [vec!['(', '{'], vec![','], vec![')', '}']], vec![')'], contains)
                        .map_err(|e| Error::UnexpectedToken(tokens[0].clone()))?
                        .iter()
                        .map(|param| Ok(Assignable::parse(&param, ParseOptions::default())?))
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

        if let Some((MatchResult::Parse(fn_name))) = pattern!(tokens, @ parse LValue,) {
            if let Some((MatchResult::Collect(parsed_parameters))) = pattern!(&tokens[fn_name.consumed..], ParenthesisOpen, @ parse CollectTokensFromUntil<'(', ')'>, ParenthesisClose) {
                let parameters = dyck_language_generic(&parsed_parameters, [vec!['(', '{'], vec![','], vec![')', '}']], vec![')'], contains)
                    .map_err(|e| Error::UnexpectedToken(tokens[0].clone()))?
                    .iter()
                    .map(|param| Ok(Assignable::parse(&param, ParseOptions::default())?))
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
use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::core::code_generator::conventions::calling_convention_from;

use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::asm_result::{ASMResult, ASMResultVariance};
use crate::core::code_generator::conventions::CallingRegister;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::registers::ByteSize;
use crate::core::lexer::collect_tokens_until_scope_close::CollectTokensFromUntil;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::model::abstract_syntax_tree_nodes::identifier::{Identifier, IdentifierError};
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::abstract_syntax_tree_nodes::method_definition::{MethodArgument, MethodDefinition};
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::scope::Scope;
use crate::core::model::types::mutability::Mutability;
use crate::core::model::types::ty::Type;
use crate::core::parser::errors::EmptyIteratorErr;
use crate::core::parser::scope::{PatternNotMatchedError, ScopeError};
use crate::core::parser::static_type_context::{CurrentMethodInfo, StaticTypeContext};
use crate::core::parser::types::r#type::{InferTypeError, MethodCallSignatureMismatchCause};
use crate::core::parser::utils::dyck::dyck_language_generic;
use crate::core::semantics::static_type_check::static_type_checker::{static_type_check_rec, StaticTypeCheckError};
use crate::core::semantics::static_type_check::static_type_check::StaticTypeCheck;
use crate::pattern;
use crate::utils::math;


#[derive(Debug)]
pub enum MethodDefinitionErr {
    PatternNotMatched { target_value: String },
    IdentifierErr(IdentifierError),
    ReturnErr(InferTypeError),
    AssignableErr(AssignableError),
    ScopeErrorErr(ScopeError),
    EmptyIterator(EmptyIteratorErr),
}

impl PatternNotMatchedError for MethodDefinitionErr {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, MethodDefinitionErr::PatternNotMatched {..})
    }
}

impl From<AssignableError> for MethodDefinitionErr {
    fn from(value: AssignableError) -> Self {
        MethodDefinitionErr::AssignableErr(value)
    }
}

impl From<IdentifierError> for MethodDefinitionErr {
    fn from(value: IdentifierError) -> Self {
        MethodDefinitionErr::IdentifierErr(value)
    }
}

impl From<InferTypeError> for MethodDefinitionErr {
    fn from(value: InferTypeError) -> Self {
        MethodDefinitionErr::ReturnErr(value)
    }
}


impl Error for MethodDefinitionErr {}

impl Display for MethodDefinitionErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            MethodDefinitionErr::PatternNotMatched { target_value }
            => format!("Pattern not matched for: `{target_value}`\n\t fn function_name(argument1, ..., argumentN): returnType {{ }}"),
            MethodDefinitionErr::AssignableErr(a) => a.to_string(),
            MethodDefinitionErr::IdentifierErr(a) => a.to_string(),
            MethodDefinitionErr::ReturnErr(a) => a.to_string(),
            MethodDefinitionErr::EmptyIterator(e) => e.to_string(),
            MethodDefinitionErr::ScopeErrorErr(a) => a.to_string(),
        })
    }
}



fn contains(a: &[TokenWithSpan], b: &TokenWithSpan) -> bool {
    a.iter().any(|x| x.token == b.token)
}

impl Parse for MethodDefinition {
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, crate::core::lexer::error::Error> where Self: Sized, Self: Default {
        let mut parameters: Vec<ParseResult<MethodArgument>> = vec![];
        let mut is_extern = false;
        let mut const_tokens = 0;
        let mut return_type = ParseResult {
            result: Type::Void,
            consumed: 0,
        };

        // extern fn name(args): return_type;
        if let Some((MatchResult::Parse(parsed_fn_name))) = pattern!(tokens, Extern, Fn, @ parse LValue,) {
            if let Some((MatchResult::Collect(parsed_parameters))) = pattern!(&tokens[parsed_fn_name.consumed + 2..], ParenthesisOpen, @ parse CollectTokensFromUntil<'(', ')'>, ParenthesisClose) {
                if let Some((MatchResult::Parse(parsed_return_type))) = pattern!(&tokens[parsed_fn_name.consumed + parsed_parameters.len() + 4..], Colon, @ parse Type, SemiColon) {
                    const_tokens = 6;
                    let parsed_parameters = dyck_language_generic(&parsed_parameters, [vec!['(', '{'], vec![','], vec![')', '}']], vec![')'], contains)
                        .map_err(|e| crate::core::lexer::error::Error::UnexpectedToken(tokens[0].clone()))?
                        .iter()
                        .map(|param| Ok(MethodArgument::parse(&param, ParseOptions::default())?))
                        .collect::<Result<Vec<ParseResult<_>>, crate::core::lexer::error::Error>>()?;


                    parameters = parsed_parameters;
                    is_extern = true;
                    return_type = parsed_return_type;

                    let amount_kommata = (parameters.len() as isize - 1).max(0) as usize;
                    let consumed = parsed_fn_name.consumed +
                        return_type.consumed +
                        parameters.iter().map(|p| p.consumed).sum::<usize>() +
                        amount_kommata +
                        const_tokens;
                    return Ok(ParseResult {
                        result: MethodDefinition {
                            identifier: parsed_fn_name.result,
                            return_type: return_type.result,
                            arguments: parameters.iter().map(|p| p.result.clone()).collect(),
                            stack: vec![],
                            is_extern,
                            file_position: FilePosition::from_min_max(&tokens[0], &tokens[consumed - 1]),
                        },
                        consumed,
                    })
                }
            }

        }

        if let Some((MatchResult::Parse(parsed_fn_name))) = pattern!(tokens, Fn, @ parse LValue,) {
            if let Some((MatchResult::Collect(parsed_parameters))) = pattern!(&tokens[parsed_fn_name.consumed + 1..], ParenthesisOpen, @ parse CollectTokensFromUntil<'(', ')'>, ParenthesisClose) {
                if let Some((MatchResult::Parse(parsed_return_type))) = pattern!(&tokens[parsed_fn_name.consumed + parsed_parameters.len() + 3..], Colon, @ parse Type,) {
                    // fn name(args): return_type
                    const_tokens = 4;
                    let parsed_parameters_tokens_consumed = parsed_parameters.len();
                    let parsed_parameters = dyck_language_generic(&parsed_parameters, [vec!['(', '{'], vec![','], vec![')', '}']], vec![')'], contains)
                        .map_err(|e| crate::core::lexer::error::Error::UnexpectedToken(tokens[0].clone()))?
                        .iter()
                        .map(|param| Ok(MethodArgument::parse(&param, ParseOptions::default())?))
                        .collect::<Result<Vec<ParseResult<_>>, crate::core::lexer::error::Error>>()?;

                    let scope = Scope::parse(&tokens[parsed_fn_name.consumed + parsed_parameters_tokens_consumed + parsed_return_type.consumed + 4..], ParseOptions::default())
                        .map_err(|e| return crate::core::lexer::error::Error::Callstack(Box::new(e)).with_context(&tokens[0]))?;

                    parameters = parsed_parameters;
                    is_extern = false;
                    return_type = parsed_return_type;

                    let amount_kommata = (parameters.len() as isize - 1).max(0) as usize;
                    let consumed = parsed_fn_name.consumed +
                        return_type.consumed +
                        parameters.iter().map(|p| p.consumed).sum::<usize>() +
                        amount_kommata +
                        const_tokens +
                        scope.consumed;
                    return Ok(ParseResult {
                        result: MethodDefinition {
                            identifier: parsed_fn_name.result,
                            return_type: return_type.result,
                            arguments: parameters.iter().map(|p| p.result.clone()).collect(),
                            stack: scope.result.ast_nodes.iter().cloned().collect(),
                            is_extern,
                            file_position: FilePosition::from_min_max(&tokens[0], &tokens[consumed - 1]),
                        },
                        consumed,
                    })
                }
            }
        }

        if let Some((MatchResult::Parse(parsed_fn_name))) = pattern!(tokens, Fn, @ parse LValue,) {
            if let Some((MatchResult::Collect(parsed_parameters))) = pattern!(&tokens[parsed_fn_name.consumed + 1..], ParenthesisOpen, @ parse CollectTokensFromUntil<'(', ')'>, ParenthesisClose) {
                // fn name(args)
                const_tokens = 3;
                let parsed_parameters_tokens_consumed = parsed_parameters.len();
                let parsed_parameters = dyck_language_generic(&parsed_parameters, [vec!['(', '{'], vec![','], vec![')', '}']], vec![')'], contains)
                    .map_err(|e| crate::core::lexer::error::Error::UnexpectedToken(tokens[0].clone()))?
                    .iter()
                    .map(|param| Ok(MethodArgument::parse(&param, ParseOptions::default())?))
                    .collect::<Result<Vec<ParseResult<_>>, crate::core::lexer::error::Error>>()?;

                let scope = Scope::parse(&tokens[parsed_fn_name.consumed + parsed_parameters_tokens_consumed + 3..], ParseOptions::default())
                    .map_err(|e| return crate::core::lexer::error::Error::Callstack(Box::new(e)).with_context(&tokens[0]))?;

                parameters = parsed_parameters;
                is_extern = false;
                return_type = ParseResult {
                    result: Type::Void,
                    consumed: 0,
                };

                let amount_kommata = (parameters.len() as isize - 1).max(0) as usize;
                let consumed = parsed_fn_name.consumed +
                    return_type.consumed +
                    parameters.iter().map(|p| p.consumed).sum::<usize>() +
                    amount_kommata +
                    const_tokens +
                    scope.consumed;
                return Ok(ParseResult {
                    result: MethodDefinition {
                        identifier: parsed_fn_name.result,
                        return_type: return_type.result,
                        arguments: parameters.iter().map(|p| p.result.clone()).collect(),
                        stack: scope.result.ast_nodes.iter().cloned().collect(),
                        is_extern,
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
        if let Some((MatchResult::Parse(name))) = pattern!(tokens, @ parse LValue, Colon,) {
            if let Some((MatchResult::Parse(ty))) = pattern!(&tokens[name.consumed + 1..], @ parse Type,) {
                return Ok(ParseResult {
                    result: MethodArgument {
                        identifier: name.result,
                        ty: ty.result,
                    },
                    consumed: name.consumed + ty.consumed + 1,
                });
            }
        }

        Err(crate::core::lexer::error::Error::first_unexpected_token(&tokens[0..1], &vec![Token::Colon.into()]))
    }
}
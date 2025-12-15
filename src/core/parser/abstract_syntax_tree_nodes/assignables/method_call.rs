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

impl StaticTypeCheck for MethodCall {
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        // let method_defs = type_context.methods.iter().filter(|m| m.identifier == self.identifier).collect::<Vec<_>>();
        //
        // 'outer: for method_def in &method_defs {
        //     if method_def.arguments.len() != self.arguments.len() {
        //         if method_defs.len() == 1 {
        //             return Err(StaticTypeCheckError::InferredError(InferTypeError::MethodCallArgumentAmountMismatch {
        //                 expected: method_def.arguments.len(),
        //                 actual: self.arguments.len(),
        //                 code_line: self.file_position.clone(),
        //             }));
        //         }
        //
        //         continue;
        //     }
        //
        //     let zipped = method_def.arguments
        //         .iter()
        //         .zip(&self.arguments);
        //
        //     for (index, (argument_def, argument_call)) in zipped.enumerate() {
        //         let def_type = argument_def.ty.clone();
        //         let call_type = argument_call.infer_type_with_context(type_context, &self.file_position)?;
        //
        //         if def_type < call_type {
        //             if method_defs.len() == 1 {
        //                 return Err(StaticTypeCheckError::InferredError(InferTypeError::MethodCallArgumentTypeMismatch {
        //                     info: Box::new(MethodCallArgumentTypeMismatch {
        //                         expected: def_type,
        //                         actual: call_type,
        //                         nth_parameter: index + 1,
        //                         code_line: self.file_position.clone(),
        //                     })
        //                 }));
        //             }
        //
        //             continue 'outer;
        //         }
        //     }
        //
        //     return Ok(());
        // }
        //
        // if method_defs.is_empty() {
        //     return Err(StaticTypeCheckError::InferredError(InferTypeError::UnresolvedReference(self.identifier.identifier(), self.file_position.clone())));
        // }
        //
        // let signatures = method_defs
        //     .iter()
        //     .map(|m| m.arguments.iter().map(|a| a.ty.clone()).collect::<Vec<_>>())
        //     .collect::<Vec<_>>();
        //
        // Err(StaticTypeCheckError::InferredError(InferTypeError::MethodCallSignatureMismatch {
        //     signatures,
        //     method_name: self.identifier.clone(),
        //     code_line: self.file_position.clone(),
        //     provided: self.arguments.iter().filter_map(|a| a.infer_type_with_context(type_context, &self.file_position).ok()).collect::<Vec<_>>(),
        // }))
        Ok(())
    }
}

fn contains(a: &[TokenWithSpan], b: &TokenWithSpan) -> bool {
    a.iter().any(|x| x.token == b.token)
}


impl Parse for MethodCall {
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default {
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
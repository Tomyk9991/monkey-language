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
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::token::Token;
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::model::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use crate::core::model::abstract_syntax_tree_nodes::identifier::{Identifier, IdentifierError};
use crate::core::model::types::ty::Type;
use crate::core::scanner::errors::EmptyIteratorErr;
use crate::core::scanner::scope::PatternNotMatchedError;
use crate::core::scanner::static_type_context::StaticTypeContext;
use crate::core::scanner::{Lines, TryParse};
use crate::core::scanner::types::r#type::{InferTypeError, MethodCallArgumentTypeMismatch};
use crate::core::semantics::type_checker::static_type_checker::StaticTypeCheckError;
use crate::core::semantics::type_checker::StaticTypeCheck;

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

impl FromStr for MethodCall {
    type Err = MethodCallErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut code_line = CodeLine::imaginary(s);

        if !s.ends_with(';') {
            code_line.line += " ;";
        }

        MethodCall::try_parse(&code_line)
    }
}

impl TryParse for MethodCall {
    type Output = MethodCall;
    type Err = MethodCallErr;

    fn try_parse(code_lines_iterator: &mut Lines<'_>) -> anyhow::Result<Self::Output, Self::Err> {
        let code_line = *code_lines_iterator.peek().ok_or(MethodCallErr::EmptyIterator(EmptyIteratorErr))?;
        MethodCall::try_parse(code_line)
    }
}

impl StaticTypeCheck for MethodCall {
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        let method_defs = type_context.methods.iter().filter(|m| m.identifier == self.identifier).collect::<Vec<_>>();

        'outer: for method_def in &method_defs {
            if method_def.arguments.len() != self.arguments.len() {
                if method_defs.len() == 1 {
                    return Err(StaticTypeCheckError::InferredError(InferTypeError::MethodCallArgumentAmountMismatch {
                        expected: method_def.arguments.len(),
                        actual: self.arguments.len(),
                        code_line: self.code_line.clone(),
                    }));
                }

                continue;
            }

            let zipped = method_def.arguments
                .iter()
                .zip(&self.arguments);

            for (index, (argument_def, argument_call)) in zipped.enumerate() {
                let def_type = argument_def.ty.clone();
                let call_type = argument_call.infer_type_with_context(type_context, &self.code_line)?;

                if def_type < call_type {
                    if method_defs.len() == 1 {
                        return Err(StaticTypeCheckError::InferredError(InferTypeError::MethodCallArgumentTypeMismatch {
                            info: Box::new(MethodCallArgumentTypeMismatch {
                                expected: def_type,
                                actual: call_type,
                                nth_parameter: index + 1,
                                code_line: self.code_line.clone(),
                            })
                        }));
                    }

                    continue 'outer;
                }
            }

            return Ok(());
        }

        if method_defs.is_empty() {
            return Err(StaticTypeCheckError::InferredError(InferTypeError::UnresolvedReference(self.identifier.name.clone(), self.code_line.clone())));
        }

        let signatures = method_defs
            .iter()
            .map(|m| m.arguments.iter().map(|a| a.ty.clone()).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Err(StaticTypeCheckError::InferredError(InferTypeError::MethodCallSignatureMismatch {
            signatures,
            method_name: self.identifier.clone(),
            code_line: self.code_line.clone(),
            provided: self.arguments.iter().filter_map(|a| a.infer_type_with_context(type_context, &self.code_line).ok()).collect::<Vec<_>>(),
        }))
    }
}

impl MethodCall {
    pub fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self, MethodCallErr> {
        let split_alloc = code_line.split(vec![' ', ';']);
        let split = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();

        if let [name, "(", ")", ";"] = &split[..] {
            Ok(MethodCall {
                identifier: Identifier::from_str(name, false)?,
                arguments: vec![],
                code_line: code_line.clone(),
            })
        } else if let [name, "(", argument_segments @ .., ")", ";"] = &split[..] {
            let name = Identifier::from_str(name, false)?;
            let joined = &argument_segments.join(" ");
            let argument_strings = dyck_language(joined, [vec!['{', '('], vec![','], vec!['}', ')']])?;

            let arguments = argument_strings
                .iter()
                .map(|s| Assignable::from_str(s))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(MethodCall {
                identifier: name,
                arguments,
                code_line: code_line.clone(),
            })
        } else {
            Err(MethodCallErr::PatternNotMatched { target_value: code_line.line.to_string() })
        }
    }

    pub fn infer_type_with_context(&self, context: &StaticTypeContext, code_line: &CodeLine) -> Result<Type, InferTypeError> {
        if let Some(method_def) = conventions::method_definitions(context, code_line, &self.arguments, &self.identifier.name)?.first() {
            let mut context = context.clone();
            if let Err(StaticTypeCheckError::InferredError(err)) = self.static_type_check(&mut context) {
                return Err(err);
            }
            return Ok(method_def.return_type.clone());
        }

        Err(InferTypeError::UnresolvedReference(self.to_string(), code_line.clone()))
    }
}

#[derive(Debug)]
pub struct DyckError {
    pub target_value: String,
    pub ordering: Ordering,
}

pub trait ArrayOrObject<T> {
    fn list(&self) -> Vec<T>;
}

impl ArrayOrObject<char> for char {
    fn list(&self) -> Vec<char> {
        vec![*self]
    }
}

impl ArrayOrObject<char> for Vec<char> {
    fn list(&self) -> Vec<char> {
        self.clone()
    }
}

impl ArrayOrObject<TokenWithSpan> for Vec<char> {
    fn list(&self) -> Vec<TokenWithSpan> {
        self.iter().map(|c| TokenWithSpan {
            token: Token::from(*c),
            span: FilePosition::default(),
        }).collect::<Vec<_>>()
    }
}

/// # Formal definition
/// Let Σ = {( ) [a-z A-Z]}
///
/// {u ∈ Σ* | all prefixes of u contain no more )'s than ('s and the number of ('s in equals the number of )'s }
pub fn dyck_language_generic<T: ArrayOrObject<K>, K, F>(sequence: &[K], values: [T; 3], breaker: T, contains: F) -> Result<Vec<Vec<K>>, DyckError>
where
    K: Clone + Debug,
    F: Fn(&[K], &K) -> bool {
    let mut individual_parameters: Vec<Vec<K>> = Vec::new();
    let mut counter = 0;
    let mut current_start_index = 0;

    let openings = values[0].list();
    let separators = values[1].list();
    let closings = values[2].list();
    let breaker = breaker.list();

    for (index, c) in sequence.iter().enumerate() {
        if contains(&breaker, &c) && counter == 0 {
            break;
        }

        if contains(&openings, &c) { // opening
            counter += 1;
        } else if contains(&closings, &c) { // closing
            counter -= 1;
        } else if contains(&separators, &c) && counter == 0 { // seperator
            let value = &sequence[current_start_index..index];

            if value.is_empty() {
                return Err(DyckError {
                    target_value: format!("{:?}", sequence),
                    ordering: Ordering::Equal,
                });
            }

            individual_parameters.push(value.to_vec());
            current_start_index = index + 1;
        }

        if counter < 0 {
            return Err(DyckError {
                target_value: format!("{:?}", sequence),
                ordering: Ordering::Less,
            });
        }
    }

    match counter {
        number if number > 0 => Err(DyckError {
            target_value: format!("{:?}", sequence),
            ordering: Ordering::Less,
        }),
        number if number < 0 => Err(DyckError {
            target_value: format!("{:?}", sequence),
            ordering: Ordering::Greater,
        }),
        _ => {
            let s = &sequence[current_start_index..sequence.len()];
            if !s.is_empty() {
                individual_parameters.push(sequence[current_start_index..sequence.len()].to_vec());
            }

            Ok(individual_parameters)
        }
    }
}


/// # Formal definition
/// Let Σ = {( ) [a-z A-Z]}
///
/// {u ∈ Σ* | all prefixes of u contain no more )'s than ('s and the number of ('s in equals the number of )'s }
pub fn dyck_language<T: ArrayOrObject<char>>(parameter_string: &str, values: [T; 3]) -> Result<Vec<String>, DyckError> {
    let mut individual_parameters: Vec<String> = Vec::new();
    let mut counter = 0;
    let mut current_start_index = 0;

    for (index, c) in parameter_string.chars().enumerate() {
        if values[0].list().contains(&c) { // opening
            counter += 1;
        } else if values[2].list().contains(&c) { // closing
            counter -= 1;
        } else if values[1].list().contains(&c) && counter == 0 { // seperator
            let value = &parameter_string[current_start_index..index].trim();

            if value.is_empty() {
                return Err(DyckError {
                    target_value: parameter_string.to_string(),
                    ordering: Ordering::Equal,
                });
            }

            individual_parameters.push(value.to_string());
            current_start_index = index + 1;
        }

        if counter < 0 {
            return Err(DyckError {
                target_value: parameter_string.to_string(),
                ordering: Ordering::Less,
            });
        }
    }

    match counter {
        number if number > 0 => Err(DyckError {
            target_value: parameter_string.to_string(),
            ordering: Ordering::Less,
        }),
        number if number < 0 => Err(DyckError {
            target_value: parameter_string.to_string(),
            ordering: Ordering::Greater,
        }),
        _ => {
            let s = parameter_string[current_start_index..parameter_string.len()].trim().to_string();
            if !s.is_empty() {
                individual_parameters.push(parameter_string[current_start_index..parameter_string.len()].trim().to_string());
            }

            Ok(individual_parameters)
        }
    }
}

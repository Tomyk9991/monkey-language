use std::fmt::{Debug, Display, Formatter};

use crate::core::io::code_line::CodeLine;
use crate::core::lexer::scope::Scope;
use crate::core::lexer::static_type_context::{CurrentMethodInfo, StaticTypeContext};
use crate::core::lexer::token::Token;
use crate::core::lexer::tokens::assignable_token::AssignableToken;
use crate::core::lexer::tokens::name_token::NameToken;
use crate::core::lexer::tokens::variable_token::VariableToken;
use crate::core::lexer::types::type_token::{InferTypeError, MethodCallSignatureMismatchCause, TypeToken};

#[derive(Debug)]
pub enum StaticTypeCheckError {
    UnresolvedReference { name: NameToken, code_line: CodeLine },
    NoTypePresent { name: NameToken, code_line: CodeLine },
    ImmutabilityViolated { name: NameToken, code_line: CodeLine },
    InferredError(InferTypeError),
}

impl std::error::Error for StaticTypeCheckError {}

impl Display for StaticTypeCheckError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            StaticTypeCheckError::UnresolvedReference { name, code_line } => format!("Line: {:?}\tUnresolved reference: `{name}`", code_line.actual_line_number),
            StaticTypeCheckError::InferredError(err) => err.to_string(),
            StaticTypeCheckError::NoTypePresent { name, code_line } => format!("Line: {:?}\tType not inferred: `{name}`", code_line.actual_line_number),
            StaticTypeCheckError::ImmutabilityViolated { name, code_line } => format!("Line: {:?}\tThis symbol isn't declared mutable: `{name}`", code_line.actual_line_number),
        })
    }
}

impl From<InferTypeError> for StaticTypeCheckError {
    fn from(value: InferTypeError) -> Self {
        StaticTypeCheckError::InferredError(value)
    }
}

pub fn static_type_check(scope: &Scope) -> Result<(), StaticTypeCheckError> {
    // check if a variable, which is not a defined variable has an invalid re-assignment
    // let a = 1.0;
    // a = 5;
    let mut type_context: StaticTypeContext = StaticTypeContext::new(&scope.tokens);
    type_context.colliding_symbols()?;
    static_type_check_rec(&scope.tokens, &mut type_context)
}

fn static_type_check_rec(scope: &Vec<Token>, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
    if let Some(ty) = &type_context.expected_return_type {
        if scope.is_empty() && ty.return_type != TypeToken::Void {
            return Err(StaticTypeCheckError::InferredError(InferTypeError::MethodReturnSignatureMismatch {
                expected: ty.return_type.clone(),
                method_name: ty.method_name.to_string(),
                method_head_line: ty.method_header_line.clone(),
                cause: MethodCallSignatureMismatchCause::ReturnMissing,
            }));
        }
    }

    for token in scope {
        match token {
            Token::Variable(variable) if variable.define => {
                if variable.ty.is_some() {
                    type_context.context.push(variable.clone());
                    continue;
                }

                return Err(StaticTypeCheckError::NoTypePresent {
                    name: variable.name_token.clone(),
                    code_line: variable.code_line.clone(),
                });
            }
            Token::Variable(variable) if !variable.define => {
                if let Some(found_variable) = type_context.iter().rfind(|v| v.name_token == variable.name_token) {
                    let inferred_type = variable.assignable.infer_type_with_context(type_context, &variable.code_line)?;

                    if let Some(ty) = &found_variable.ty {
                        if ty != &inferred_type {
                            return Err(InferTypeError::MismatchedTypes { expected: ty.clone(), actual: inferred_type.clone(), code_line: variable.code_line.clone() }.into());
                        }

                        if !found_variable.mutability {
                            return Err(StaticTypeCheckError::ImmutabilityViolated {
                                name: variable.name_token.clone(),
                                code_line: variable.code_line.clone(),
                            });
                        }
                    } else {
                        return Err(StaticTypeCheckError::NoTypePresent { name: variable.name_token.clone(), code_line: variable.code_line.clone() });
                    }
                } else {
                    return Err(StaticTypeCheckError::UnresolvedReference { name: variable.name_token.clone(), code_line: variable.code_line.clone() });
                }
            }
            Token::If(if_definition) => {
                let variables_len = type_context.context.len();
                let condition_type = if_definition.condition.infer_type_with_context(type_context, &if_definition.code_line)?;

                if condition_type != TypeToken::Bool {
                    return Err(StaticTypeCheckError::InferredError(InferTypeError::MismatchedTypes {
                        expected: TypeToken::Bool,
                        actual: condition_type,
                        code_line: if_definition.code_line.clone(),
                    }))
                }

                static_type_check_rec(&if_definition.if_stack, type_context)?;

                let amount_pop = type_context.context.len() - variables_len;

                for _ in 0..amount_pop {
                    let _ = type_context.context.pop();
                }

                if let Some(else_stack) = &if_definition.else_stack {
                    let variables_len = type_context.context.len();

                    static_type_check_rec(else_stack, type_context)?;

                    let amount_pop = type_context.context.len() - variables_len;

                    for _ in 0..amount_pop {
                        let _ = type_context.context.pop();
                    }
                }
            }
            Token::MethodDefinition(method_definition) => {
                // add the parameters to the type information
                for (argument_name, argument_type) in &method_definition.arguments {
                    type_context.context.push(VariableToken {
                        name_token: argument_name.clone(),
                        mutability: false,
                        ty: Some(argument_type.clone()),
                        define: true,
                        assignable: AssignableToken::default(),
                        code_line: Default::default(),
                    });
                }

                let variables_len = type_context.context.len();
                type_context.expected_return_type = Some(CurrentMethodInfo {
                    return_type: method_definition.return_type.clone(),
                    method_header_line: method_definition.code_line.actual_line_number.clone(),
                    method_name: method_definition.name.name.to_string(),
                });

                static_type_check_rec(&method_definition.stack, type_context)?;

                if method_definition.return_type != TypeToken::Void {
                    if let [.., last] = &method_definition.stack[..] {
                        let mut method_return_signature_mismatch = false;
                        let mut cause = MethodCallSignatureMismatchCause::ReturnMissing;

                        if let Token::If(if_definition) = &last {
                            method_return_signature_mismatch = !if_definition.ends_with_return_in_each_branch();
                            if method_return_signature_mismatch {
                                cause = MethodCallSignatureMismatchCause::IfCondition;
                            }
                        } else if !matches!(last, Token::Return(_)) {
                            method_return_signature_mismatch = true;
                        }

                        if method_return_signature_mismatch {
                            if let Some(expected_return_type) = &type_context.expected_return_type {
                                return Err(StaticTypeCheckError::InferredError(InferTypeError::MethodReturnSignatureMismatch {
                                    expected: expected_return_type.return_type.clone(),
                                    method_name: expected_return_type.method_name.to_string(),
                                    method_head_line: expected_return_type.method_header_line.to_owned(),
                                    cause,
                                }))
                            }
                        }
                    }
                }



                let amount_pop = (type_context.context.len() - variables_len) + method_definition.arguments.len();

                for _ in 0..amount_pop {
                    let _ = type_context.context.pop();
                }

                type_context.expected_return_type = None;
            }
            Token::ForToken(for_loop) => {
                // add for header variables
                type_context.context.push(for_loop.initialization.clone());

                let variables_len = type_context.context.len();
                let condition_type = for_loop.condition.infer_type_with_context(type_context, &for_loop.code_line)?;

                if condition_type != TypeToken::Bool {
                    return Err(StaticTypeCheckError::InferredError(InferTypeError::MismatchedTypes {
                        expected: TypeToken::Bool,
                        actual: condition_type,
                        code_line: for_loop.code_line.clone(),
                    }))
                }

                if for_loop.update.define {
                    return Err(StaticTypeCheckError::InferredError(InferTypeError::DefineNotAllowed(for_loop.update.clone(), for_loop.code_line.clone())))
                }

                static_type_check_rec(&for_loop.stack, type_context)?;

                let amount_pop = type_context.context.len() - variables_len;

                for _ in 0..amount_pop {
                    let _ = type_context.context.pop();
                }
            },
            Token::WhileToken(while_loop) => {
                let variables_len = type_context.context.len();
                let condition_type = while_loop.condition.infer_type_with_context(type_context, &while_loop.code_line)?;

                if condition_type != TypeToken::Bool {
                    return Err(StaticTypeCheckError::InferredError(InferTypeError::MismatchedTypes {
                        expected: TypeToken::Bool,
                        actual: condition_type,
                        code_line: while_loop.code_line.clone(),
                    }))
                }

                static_type_check_rec(&while_loop.stack, type_context)?;

                let amount_pop = type_context.context.len() - variables_len;

                for _ in 0..amount_pop {
                    let _ = type_context.context.pop();
                }
            }
            Token::MethodCall(method_call) => {
                method_call.type_check(type_context, &method_call.code_line)?
            }
            Token::Return(return_statement) => {
                if let Some(expected_return_type) = &type_context.expected_return_type {
                    if let Some(assignable) = &return_statement.assignable {
                        let actual_type = assignable.infer_type_with_context(type_context, &token.code_line())?;

                        if expected_return_type.return_type != actual_type {
                            return Err(StaticTypeCheckError::InferredError(InferTypeError::MethodReturnArgumentTypeMismatch {
                                expected: expected_return_type.return_type.clone(),
                                actual: actual_type,
                                code_line: token.code_line(),
                            }));
                        }
                    }
                }
            }
            Token::Variable(_) | Token::ScopeClosing(_) | Token::Import(_) => {}
        }
    }


    Ok(())
}
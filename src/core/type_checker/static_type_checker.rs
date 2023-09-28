use std::fmt::{Debug, Display, Formatter};
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::scope::Scope;
use crate::core::lexer::token::Token;
use crate::core::lexer::tokenizer::StaticTypeContext;
use crate::core::lexer::tokens::name_token::NameToken;
use crate::core::lexer::tokens::variable_token::VariableToken;
use crate::core::lexer::type_token::InferTypeError;

#[derive(Debug)]
pub enum StaticTypeCheckError {
    UnresolvedReference { name: NameToken, code_line: CodeLine },
    InferredError(InferTypeError),
}

impl std::error::Error for StaticTypeCheckError { }

impl Display for StaticTypeCheckError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            StaticTypeCheckError::UnresolvedReference { name, code_line } => format!("Line: {:?}\tUnresolved reference: `{name}`", code_line.actual_line_number),
            StaticTypeCheckError::InferredError(err) => err.to_string()
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
    let mut type_context: StaticTypeContext = StaticTypeContext::type_context(&scope.tokens);
    static_type_check_rec(&scope.tokens, &mut vec![], &mut type_context)
}

fn static_type_check_rec(scope: &Vec<Token>, visible_variables: &mut Vec<VariableToken<'=', ';'>>, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
    for token in scope {
        match token {
            Token::Variable(variable) if variable.define => {
                visible_variables.push(variable.clone());
                continue;
            },
            Token::Variable(variable) if !variable.define => {
                if let Some(found_variable) = visible_variables.iter().rfind(|v| v.name_token == variable.name_token) {
                    match &found_variable.ty {
                        Some(ty) => {
                            let inferred_type = variable.assignable.infer_type_with_context(type_context, &variable.code_line)?;

                            if ty != &inferred_type {
                                return Err(InferTypeError::MismatchedTypes { expected: ty.clone(), actual: inferred_type.clone(), code_line: variable.code_line.clone() }.into())
                            }
                        }
                        None => return Err(InferTypeError::UnresolvedReference(found_variable.name_token.name.clone(), variable.code_line.clone()).into())
                    }
                } else {
                    return Err(StaticTypeCheckError::UnresolvedReference { name: variable.name_token.clone(), code_line: variable.code_line.clone() })
                }
            }
            Token::IfDefinition(if_definition) => {
                let variables_len = visible_variables.len();

                static_type_check_rec(&if_definition.if_stack, visible_variables, type_context)?;

                let amount_pop = visible_variables.len() - variables_len;

                for _ in 0..amount_pop {
                    let _ = visible_variables.pop();
                }


                if let Some(else_stack) = &if_definition.else_stack {
                    let variables_len = visible_variables.len();

                    static_type_check_rec(else_stack, visible_variables, type_context)?;

                    let amount_pop = visible_variables.len() - variables_len;

                    for _ in 0..amount_pop {
                        let _ = visible_variables.pop();
                    }
                }
            }
            Token::MethodDefinition(method_definition) => {
                let variables_len = visible_variables.len();

                static_type_check_rec(&method_definition.stack, visible_variables, type_context)?;

                let amount_pop = visible_variables.len() - variables_len;

                for _ in 0..amount_pop {
                    let _ = visible_variables.pop();
                }
            }
            Token::MethodCall(method_call) => {
                method_call.type_check(type_context, &method_call.code_line)?
            },
            Token::Variable(_) | Token::ScopeClosing(_) | Token::Import(_) => {}
        }
    }


    Ok(())
}
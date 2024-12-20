use std::collections::HashMap;
use std::ops::{Deref, DerefMut, Range};
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::token::Token;
use crate::core::lexer::tokens::method_definition::{MethodDefinition};
use crate::core::lexer::tokens::variable_token::VariableToken;
use crate::core::lexer::types::type_token::{InferTypeError, TypeToken};

#[derive(Debug, Clone)]
pub struct CurrentMethodInfo {
    pub return_type: TypeToken,
    pub method_header_line: Range<usize>,
    pub method_name: String,
}

/// Contains all static type information about the provided scope
/// At the moment variables and method definitions are included
#[derive(Debug, Default, Clone)]
pub struct StaticTypeContext {
    pub context: Vec<VariableToken<'=', ';'>>,
    pub expected_return_type: Option<CurrentMethodInfo>,
    pub methods: Vec<MethodDefinition>
}

impl StaticTypeContext {
    // adds all information from the other context to this context
    pub fn merge(&mut self, other: StaticTypeContext) {
        for token in other.context {
            self.context.push(token);
        }
    }

    /// checks, if the provided methods have any name collisions
    pub fn colliding_symbols(&self) -> Result<(), InferTypeError> {
        for method in &self.methods {
            let context = StaticTypeContext::new(&method.stack);
            let mut hash_map: HashMap<&str, (usize, &CodeLine)> = HashMap::new();

            for (argument_name, _) in &method.arguments {
                if let Some((counter, _)) = hash_map.get_mut(argument_name.name.as_str()) {
                    *counter += 1;
                } else {
                    hash_map.insert(&argument_name.name, (1, &method.code_line));
                }
            }

            for variable in &context.context {
                if !variable.define { continue; }
                if let Some((counter, _)) = hash_map.get_mut(variable.name_token.name.as_str()) {
                    *counter += 1;
                } else {
                    hash_map.insert(&variable.name_token.name, (1, &variable.code_line));
                }
            }

            for (key, (value, code_line)) in &hash_map {
                if *value > 1 {
                    return Err(InferTypeError::NameCollision(key.to_string(), (*code_line).clone()));
                }
            }
        }

        Ok(())
    }
}

impl Deref for StaticTypeContext {
    type Target = Vec<VariableToken<'=', ';'>>;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl DerefMut for StaticTypeContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.context
    }
}

impl StaticTypeContext {
    /// Constructs a context containing all type information the ast can infer. This is especially useful to infer further types, that could not be inferred before like function calls and variable assignments
    pub fn new(scope: &Vec<Token>) -> StaticTypeContext {
        let mut context: Vec<VariableToken<'=', ';'>> = Vec::new();
        let mut methods = Vec::new();

        for token in scope {
            match token {
                Token::Variable(variable) => {
                    if variable.ty.is_some() {
                        context.push(variable.clone());
                    }
                }
                Token::MethodDefinition(method_definition) => {
                    methods.push(method_definition.clone());
                },
                Token::For(for_loop) => {
                    if for_loop.initialization.ty.is_some() {
                        context.push(for_loop.initialization.clone());
                    }
                },
                Token::While(_) | Token::ScopeClosing(_) | Token::MethodCall(_) | Token::If(_) | Token::Import(_) | Token::Return(_) => {}
            }
        }


        Self {
            context,
            expected_return_type: None,
            methods,
        }
    }
}
use std::ops::{Deref, DerefMut};
use crate::core::lexer::token::Token;
use crate::core::lexer::tokens::method_definition::MethodDefinition;
use crate::core::lexer::tokens::variable_token::VariableToken;



/// Contains all static type information about the provided scope
/// At the moment variables and method definitions are included
#[derive(Debug, Default)]
pub struct StaticTypeContext {
    pub context: Vec<VariableToken<'=', ';'>>,
    pub methods: Vec<MethodDefinition>
}

impl StaticTypeContext {
    pub fn merge(&mut self, other: StaticTypeContext) {
        for token in other.context {
            self.context.push(token);
        }
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
        let mut context = Vec::new();
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
                }
                Token::ScopeClosing(_) | Token::MethodCall(_) | Token::IfDefinition(_) | Token::Import(_) => {}
            }
        }


        Self {
            context,
            methods,
        }
    }
}
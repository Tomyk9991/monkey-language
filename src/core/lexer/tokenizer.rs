use std::ops::{Deref, DerefMut};
use crate::core::io::monkey_file::MonkeyFile;
use crate::core::lexer::scope::{Scope, ScopeError};
use crate::core::lexer::token::Token;
use crate::core::lexer::tokens::method_definition::MethodDefinition;
use crate::core::lexer::tokens::name_token::NameToken;
use crate::core::lexer::TryParse;
use crate::core::lexer::type_token::{InferTypeError, TypeToken};

#[derive(Debug, Default)]
pub struct StaticTypeContext {
    pub context: Vec<(NameToken, TypeToken)>
}

impl StaticTypeContext {
    pub fn merge(&mut self, other: StaticTypeContext) {
        for token in other.context {
            self.context.push(token);
        }
    }
}

impl Deref for StaticTypeContext {
    type Target = Vec<(NameToken, TypeToken)>;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl DerefMut for StaticTypeContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.context
    }
}

impl StaticTypeContext {
    /// Constructs a hashmap containing all type information the ast can infer. This is especially useful to infer further types, that could not be inferred before like function calls and variable assignments
    pub fn type_context(scope: &Vec<Token>) -> StaticTypeContext {
        let mut context = Vec::new();

        Self::build_type_context_rec(scope, &mut context);

        Self {
            context
        }
    }

    fn build_type_context_rec(scope: &Vec<Token>, context: &mut Vec<(NameToken, TypeToken)>) {
        for token in scope {
            match token {
                Token::Variable(variable) => {
                    if let Some(ty) = &variable.ty {
                        context.push((variable.name_token.clone(), ty.clone()));
                    }
                }
                Token::MethodDefinition(method_definition) => {
                    context.push((method_definition.name.clone(), method_definition.return_type.clone()));
                }
                Token::ScopeClosing(_) | Token::MethodCall(_) | Token::IfDefinition(_) => {}
            }
        }
    }
}

pub struct Lexer {
    current_file: MonkeyFile
}

impl From<MonkeyFile> for Lexer {
    fn from(value: MonkeyFile) -> Self {
        Self {
            current_file: value
        }
    }
}

impl Lexer {
    /// Tokenize the current file
    /// # Returns
    /// A `Scope` containing all the tokens
    /// # Errors
    /// - If the file is empty
    /// - If the file contains an invalid token
    pub fn tokenize(&mut self) -> Result<Scope, ScopeError> {
        let mut scope = Scope {
            tokens: vec![],
        };

        let mut iterator = self.current_file.lines.iter().peekable();

        while iterator.peek().is_some() {
            let token = Scope::try_parse(&mut iterator)?;
            scope.tokens.push(token);
        }

        // top level type context. all methods are visible
        let mut type_context: StaticTypeContext = StaticTypeContext::type_context(&scope.tokens);
        let mut methods: Vec<*mut MethodDefinition> = Vec::new();

        for token in &mut scope.tokens {
            if let Token::MethodDefinition(method_def) = token {
                methods.push(method_def);
            }
        }

        let method_names = methods.iter()
            .map(|m| unsafe { (*(*m)).name.clone() })
            .collect::<Vec<NameToken>>();

        Self::infer_types(&mut scope.tokens, &mut type_context, &method_names)?;


        for method in methods.iter_mut() {
            let variables_len = type_context.len();

            let scoped_checker = StaticTypeContext::type_context(unsafe { &(*(*method)).stack });
            type_context.merge(scoped_checker);

            Self::infer_types(unsafe { &mut (*(*method)).stack }, &mut type_context, &method_names)?;

            let amount_pop = type_context.len() - variables_len;

            for _ in 0..amount_pop {
                let _ = type_context.pop();
            }
        }

        Ok(scope)
    }

    fn infer_types(scope: &mut Vec<Token>, type_context: &mut StaticTypeContext, method_names: &Vec<NameToken>) -> Result<(), ScopeError> {
        for token in scope {
            match token {
                Token::Variable(variable) => {
                    if method_names.iter().filter(|a| a == &&variable.name_token).count() > 0 {
                        return Err(InferTypeError::NameCollision(variable.name_token.name.clone()).into());
                    }

                    if !variable.define {
                        continue;
                    }

                    let _ = variable.infer_type(type_context)?;
                }
                Token::IfDefinition(if_definition) => {
                    let variables_len = type_context.len();

                    let scoped_checker = StaticTypeContext::type_context(&if_definition.if_stack);
                    type_context.merge(scoped_checker);

                    Self::infer_types(&mut if_definition.if_stack, type_context, method_names)?;

                    let amount_pop = type_context.len() - variables_len;

                    for _ in 0..amount_pop {
                        let _ = type_context.pop();
                    }

                    if let Some(else_stack) = &mut if_definition.else_stack {
                        let variables_len = type_context.len();

                        let scoped_checker = StaticTypeContext::type_context(else_stack);
                        type_context.merge(scoped_checker);

                        Self::infer_types(else_stack, type_context, method_names)?;

                        let amount_pop = type_context.len() - variables_len;

                        for _ in 0..amount_pop {
                            let _ = type_context.pop();
                        }
                    }
                }
                Token::MethodDefinition(_) | Token::MethodCall(_) | Token::ScopeClosing(_) => {}
            }

        }

        Ok(())
    }
}
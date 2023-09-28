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
    pub context: Vec<(NameToken, TypeToken)>,
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
    type Target = Vec<(NameToken, TypeToken)>;

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
    /// Constructs a hashmap containing all type information the ast can infer. This is especially useful to infer further types, that could not be inferred before like function calls and variable assignments
    pub fn type_context(scope: &Vec<Token>) -> StaticTypeContext {
        let (context, methods) = Self::build_type_context_rec(scope);

        Self {
            context,
            methods
        }
    }

    fn build_type_context_rec(scope: &Vec<Token>) -> (Vec<(NameToken, TypeToken)>, Vec<MethodDefinition>) {
        let mut context = Vec::new();
        let mut methods = Vec::new();
        for token in scope {
            match token {
                Token::Variable(variable) => {
                    if let Some(ty) = &variable.ty {
                        context.push((variable.name_token.clone(), ty.clone()));
                    }
                }
                Token::MethodDefinition(method_definition) => {
                    context.push((method_definition.name.clone(), method_definition.return_type.clone()));
                    methods.push(method_definition.clone());
                }
                Token::ScopeClosing(_) | Token::MethodCall(_) | Token::IfDefinition(_) | Token::Import(_) => {}
            }
        }


        (context, methods)
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
            // extern_methods: vec![],
        };

        let mut iterator = self.current_file.lines.iter().peekable();

        while iterator.peek().is_some() {
            let token = Scope::try_parse(&mut iterator)?;

            match token {
                // Token::MethodDefinition(method_def) if method_def.is_extern => scope.extern_methods.push(method_def),
                Token::Import(imported_monkey_file) => {
                    let inner_scope = Lexer::from(imported_monkey_file.monkey_file.clone()).tokenize()?;
                    // todo: this could result in collisions.

                    for t in inner_scope.tokens {
                        scope.tokens.push(t);
                    }

                    // for extern_method in inner_scope.extern_methods {
                    //     scope.extern_methods.push(extern_method.clone());
                    //     scope.tokens.push(Token::MethodDefinition(extern_method));
                    // }

                    scope.tokens.push(Token::Import(imported_monkey_file));
                }
                a => scope.tokens.push(a)
            }
        }

        // top level type context. top level variables and all methods are visible
        let mut type_context: StaticTypeContext = StaticTypeContext::type_context(&scope.tokens);
        let mut methods: Vec<*mut MethodDefinition> = Vec::new();

        for token in &mut scope.tokens {
            if let Token::MethodDefinition(method_ref) = token {
                methods.push(method_ref);
            }
        }

        Self::infer_types(&mut scope.tokens, &mut type_context)?;

        for method in methods.iter_mut() {
            Scope::infer_type(unsafe { &mut (*(*method)).stack }, &mut type_context)?;
        }

        Ok(scope)
    }

    pub fn infer_types(scope: &mut Vec<Token>, type_context: &mut StaticTypeContext) -> Result<(), InferTypeError> {
        for token in scope {
            token.infer_type(type_context)?;
        }

        Ok(())
    }
}
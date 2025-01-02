use crate::core::code_generator::conventions::calling_convention_from;
use crate::core::code_generator::target_os::TargetOS;
use crate::core::io::monkey_file::MonkeyFile;
use crate::core::lexer::scope::{Scope, ScopeError};
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::token::Token;
use crate::core::lexer::tokens::assignable_token::AssignableToken;
use crate::core::lexer::tokens::l_value::LValue;
use crate::core::lexer::tokens::method_definition::MethodDefinition;
use crate::core::lexer::tokens::parameter_token::ParameterToken;
use crate::core::lexer::tokens::variable_token::VariableToken;
use crate::core::lexer::TryParse;
use crate::core::lexer::types::type_token::InferTypeError;

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

            if let Token::Import(imported_monkey_file) = token {
                let inner_scope = Lexer::from(imported_monkey_file.monkey_file.clone()).tokenize()?;

                // todo: this could result in collisions.
                for t in inner_scope.tokens {
                    scope.tokens.push(t);
                }

                scope.tokens.push(Token::Import(imported_monkey_file));
            } else {
                scope.tokens.push(token)
            }
        }

        // top level type context. top level variables and all methods are visible
        let mut type_context: StaticTypeContext = StaticTypeContext::new(&scope.tokens);

        let mut methods: Vec<*mut MethodDefinition> = Vec::new();

        for token in &mut scope.tokens {
            if let Token::MethodDefinition(method_ref) = token {
                methods.push(method_ref);
            }
        }

        Self::infer_types(&mut scope.tokens, &mut type_context)?;

        for method in methods.iter_mut() {
            let calling_convention = calling_convention_from(unsafe { &(*(*method)) }, &TargetOS::Windows);

            for (index, argument) in unsafe { &(*(*method)) }.arguments.iter().enumerate() {
                let parameter_token = ParameterToken {
                    name_token: argument.name.clone(),
                    ty: argument.type_token.clone(),
                    register: calling_convention[index][0].clone(),
                    mutability: argument.mutability,
                    code_line: unsafe { &(*(*method)) }.code_line.clone(),
                };

                type_context.context.push(VariableToken {
                    l_value: LValue::Name(argument.name.clone()),
                    mutability: argument.mutability,
                    ty: Some(argument.type_token.clone()),
                    define: true,
                    assignable: AssignableToken::Parameter(parameter_token),
                    code_line: unsafe { &(*(*method)) }.code_line.clone(),
                });
            }

            Scope::infer_type(unsafe { &mut (*(*method)).stack }, &mut type_context)?;

            for _ in 0..unsafe { &(*(*method)) }.arguments.len() {
                type_context.context.pop();
            }
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
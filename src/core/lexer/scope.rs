use std::collections::{HashSet};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::errors::EmptyIteratorErr;
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::token::Token;
use crate::core::lexer::tokenizer::{Lexer};
use crate::core::lexer::tokens::assignable_token::AssignableToken;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::expression::Expression;
use crate::core::lexer::tokens::assignable_tokens::method_call_token::MethodCallToken;
use crate::core::lexer::tokens::for_token::ForToken;
use crate::core::lexer::tokens::if_token::IfToken;
use crate::core::lexer::tokens::method_definition::MethodDefinition;
use crate::core::lexer::tokens::scope_ending::ScopeEnding;
use crate::core::lexer::tokens::variable_token::VariableToken;
use crate::core::lexer::TryParse;
use crate::core::lexer::tokens::import_token::ImportToken;
use crate::core::lexer::tokens::r#while::WhileToken;
use crate::core::lexer::tokens::return_token::ReturnToken;
use crate::core::lexer::types::type_token::InferTypeError;

/// Tokens inside scope
pub struct Scope {
    pub tokens: Vec<Token>
}

impl Scope {
    ///Optimizing for level 1
    pub fn o1(&mut self) {
        self.optimize_methods();
        // self.remove_double_strings();
    }
}

impl Scope {
    pub fn infer_type(stack: &mut Vec<Token>, type_context: &mut StaticTypeContext) -> Result<(), InferTypeError> {
        let variables_len = type_context.len();

        let scoped_checker = StaticTypeContext::new(stack);
        type_context.merge(scoped_checker);

        Lexer::infer_types(stack, type_context)?;

        let amount_pop = type_context.len() - variables_len;

        for _ in 0..amount_pop {
            let _ = type_context.pop();
        }

        Ok(())
    }

    fn method_call_in_assignable(assignable_token: &AssignableToken) -> Option<Vec<String>> {
        match assignable_token {
            AssignableToken::MethodCallToken(method_call) => {
                Some(vec![method_call.name.name.clone()])
            }
            AssignableToken::ArithmeticEquation(a) => {
                Self::method_calls_in_expression(a)
            }
            AssignableToken::String(_) | AssignableToken::IntegerToken(_) |
            AssignableToken::FloatToken(_) | AssignableToken::Parameter(_) |
            AssignableToken::BooleanToken(_) | AssignableToken::NameToken(_) |
            AssignableToken::Object(_) => None,
            AssignableToken::ArrayToken(array_token) => {
                let mut elements = vec![];
                for value in &array_token.values {
                    let mut more = Self::method_call_in_assignable(value);
                    if let Some(mut more) = more {
                        elements.append(&mut more);
                    }
                }

                if elements.is_empty() {
                    None
                } else {
                    Some(elements)
                }
            }
        }
    }

    fn method_calls_in_expression(expression: &Expression) -> Option<Vec<String>> {
        if let Some(a) = &expression.value {
            return Self::method_call_in_assignable(a.as_ref());
        }

        let mut final_result = vec![];
        if let Some(lhs) = &expression.lhs {
            if let Some(lhs_result) = Self::method_calls_in_expression(lhs.as_ref()) {
                lhs_result.iter().for_each(|a| final_result.push(a.clone()));
            }
        }

        if let Some(rhs) = &expression.rhs {
            if let Some(rhs_result) = Self::method_calls_in_expression(rhs.as_ref()) {
                rhs_result.iter().for_each(|a| final_result.push(a.clone()));
            }
        }

        if final_result.is_empty() {
            None
        } else {
            Some(final_result)
        }
    }

    fn method_calls_in_stack(stack: &Vec<Token>) -> Vec<String> {
        let mut called_methods = HashSet::new();

        for token in stack {
            match token {
                Token::Variable(variable_token) => {
                    if let Some(calls) = Self::method_call_in_assignable(&variable_token.assignable) {
                        calls.iter().for_each(|a| { called_methods.insert(a.clone()); });
                    }
                }
                Token::MethodCall(method_call) => {
                    for args in &method_call.arguments {
                        if let Some(calls) = Self::method_call_in_assignable(args) {
                            calls.iter().for_each(|a| { called_methods.insert(a.clone()); });
                        }
                    }

                    called_methods.insert(method_call.name.to_string());
                }
                Token::If(if_definition) => {
                    Self::method_calls_in_stack(&if_definition.if_stack).iter().for_each(|a| { called_methods.insert(a.clone()); });
                    if let Some(else_stack) = &if_definition.else_stack {
                        Self::method_calls_in_stack(else_stack).iter().for_each(|a| { called_methods.insert(a.clone()); })
                    }
                }
                Token::ForToken(for_loop) => {
                    Self::method_calls_in_stack(&for_loop.stack).iter().for_each(|a| { called_methods.insert(a.clone()); });
                }
                Token::WhileToken(while_loop) => {
                    Self::method_calls_in_stack(&while_loop.stack).iter().for_each(|a| { called_methods.insert(a.clone()); });
                }
                Token::MethodDefinition(_) | Token::Import(_) | Token::Return(_) | Token::ScopeClosing(_) => {}
            }
        }

        called_methods.iter().cloned().collect::<Vec<_>>()
    }

    /// Optimize methods out, which are not traversed down from the main method
    pub fn optimize_methods(&mut self) {
        if let Some(Token::MethodDefinition(main_method)) = self.tokens.iter().find(|a| matches!(a, Token::MethodDefinition(main) if main.name.name == "main")) {
            let called_methods = Self::method_calls_in_stack(&main_method.stack);
            let mut uncalled_methods = vec![];

            for token in &self.tokens {
                if let Token::MethodDefinition(method_definition) = token {
                    if method_definition.name.name == "main" { continue; }

                    if !called_methods.contains(&method_definition.name.name) {
                        uncalled_methods.push(method_definition.name.name.clone());
                    }
                }
            }

            let mut indices = uncalled_methods.iter().filter_map(|called_method| {
                self.tokens.iter().position(|token| matches!(token, Token::MethodDefinition(method_def) if method_def.name.name == *called_method))
            }).collect::<Vec<_>>();
            indices.sort();

            indices.iter().rev().for_each(|index| { self.tokens.remove(*index); });
        }
    }
}

pub enum ScopeError {
    ParsingError { message: String },
    InferredError(InferTypeError),
    EmptyIterator(EmptyIteratorErr)
}

impl PatternNotMatchedError for ScopeError {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, ScopeError::ParsingError { .. })
    }
}

impl Debug for ScopeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for ScopeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ScopeError::ParsingError { message } => message.to_string(),
            ScopeError::EmptyIterator(e) => e.to_string(),
            ScopeError::InferredError(e) => e.to_string()
        })
    }
}


impl Error for ScopeError {}

macro_rules! token_expand {
    ($code_lines_iterator: ident, $(($token_implementation:ty, $token_type:ident, $iterates_over_same_scope:ident)),*) => {
        $(
            match <$token_implementation as TryParse>::try_parse($code_lines_iterator) {
                Ok(t) => {
                    if $iterates_over_same_scope {
                        $code_lines_iterator.next();
                    }
                    return Ok(Token::$token_type(t))
                },
                Err(err) => {
                    if !err.is_pattern_not_matched_error() {
                        return Err(ScopeError::ParsingError {
                            message: format!("{}", err)
                        })
                    }
                }
            }
        )*
    }
}


impl Debug for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scope: [\n{}]", self.tokens
            .iter()
            .map(|token| format!("\t{:?}\n", token))
            .collect::<String>(),
        )
    }
}

impl Display for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scope: [\n{}]", self.tokens
            .iter()
            .map(|token| {
                if let Token::MethodDefinition(md) = token {
                    let postfix = if !md.is_extern {
                        let mut target = String::new();
                        for inner_token in &md.stack { target += &format!("\n\t\t{inner_token}"); }
                        target
                    } else {
                        String::new()
                    };
                    format!("\t{}{postfix}\n", md)
                } else {
                    format!("\t{}\n", token)
                }
            })
            .collect::<String>())
    }
}

impl From<InferTypeError> for ScopeError {
    fn from(value: InferTypeError) -> Self {
        ScopeError::InferredError(value)
    }
}

pub trait PatternNotMatchedError {
    fn is_pattern_not_matched_error(&self) -> bool;
}


impl TryParse for Scope {
    type Output = Token;
    type Err = ScopeError;

    /// Tries to parse the code lines into a scope using a peekable iterator and a greedy algorithm
    /// # Returns
    /// * Ok(Token) if the code lines iterator can be parsed into a scope
    /// * Err(ScopeError) if the code lines iterator cannot be parsed into a scope
    fn try_parse(code_lines_iterator: &mut Peekable<Iter<CodeLine>>) -> anyhow::Result<Self::Output, ScopeError> {
        // let mut pattern_distances: Vec<(usize, Box<dyn Error>)> = vec![];
        let code_line = *code_lines_iterator.peek().ok_or(ScopeError::EmptyIterator(EmptyIteratorErr))?;

        token_expand!(code_lines_iterator,
            (ImportToken,               Import,             true),
            (VariableToken<'=', ';'>,   Variable,           true),
            (MethodCallToken,           MethodCall,         true),
            (ScopeEnding,               ScopeClosing,       true),
            (ReturnToken,               Return,             true),
            (IfToken,                   If,                 false),
            (MethodDefinition,          MethodDefinition,   false),
            (ForToken,                  ForToken,           false),
            (WhileToken,                WhileToken,         false)
        );

        let c = *code_lines_iterator.peek().ok_or(ScopeError::EmptyIterator(EmptyIteratorErr))?;
        Err(ScopeError::ParsingError {
            message: format!("Unexpected token: {:?}: {}", c.actual_line_number, code_line.line)
        })
    }
}
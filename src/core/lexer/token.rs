use std::fmt::{Debug, Display, Formatter};
use crate::core::code_generator::generator::{Stack};
use crate::core::code_generator::{Error, ToASM};
use crate::core::code_generator::target_os::TargetOS;
use crate::core::lexer::tokens::scope_ending::ScopeEnding;
use crate::core::lexer::tokens::method_definition::MethodDefinition;
use crate::core::lexer::tokens::assignable_tokens::method_call_token::MethodCallToken;
use crate::core::lexer::tokens::if_definition::IfDefinition;
use crate::core::lexer::tokens::variable_token::VariableToken;

/// A token is a piece of code that is used to represent atomic elements of a program.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Variable(VariableToken<'=', ';'>),
    MethodCall(MethodCallToken),
    MethodDefinition(MethodDefinition),
    ScopeClosing(ScopeEnding),
    IfDefinition(IfDefinition),
}

impl ToASM for Token {
    fn to_asm(&self, stack: &mut Stack, target_os: &TargetOS) -> Result<String, Error> {
        match self {
            Token::Variable(variable) => variable.to_asm(stack, target_os),
            Token::MethodCall(method_call_token) => method_call_token.to_asm(stack, target_os),
            Token::IfDefinition(if_definition) => if_definition.to_asm(stack, target_os),
            rest => Err(Error::NotImplemented { token: format!("{}", rest) }),
            // Token::MethodDefinition(_) => {}
            // Token::ScopeClosing(_) => {}
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Token::Variable(v) => format!("{}", v),
            Token::MethodCall(m) => format!("{}", m),
            Token::MethodDefinition(m) => format!("{}", m),
            Token::ScopeClosing(m) => format!("{}", m),
            Token::IfDefinition(m) => format!("{}", m)
        })
    }
}


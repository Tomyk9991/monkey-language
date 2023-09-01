use std::fmt::{Debug, Display, Formatter};
use crate::core::lexer::tokens::scope_ending::ScopeEnding;
use crate::core::lexer::tokens::method_definition::MethodDefinition;
use crate::core::lexer::tokens::assignable_tokens::method_call_token::MethodCallToken;
use crate::core::lexer::tokens::if_definition::IfDefinition;
use crate::core::lexer::tokens::variable_token::VariableToken;

/// A token is a piece of code that is used to represent atomic elements of a program.
#[derive(Debug, PartialEq)]
pub enum Token {
    Variable(VariableToken<'=', ';'>),
    MethodCall(MethodCallToken),
    MethodDefinition(MethodDefinition),
    ScopeClosing(ScopeEnding),
    IfDefinition(IfDefinition)
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


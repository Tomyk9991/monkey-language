use std::fmt::{Display, Formatter};
use crate::interpreter::lexer::tokens::scope_ending::ScopeEnding;
use crate::interpreter::lexer::tokens::method_definition::MethodDefinition;
use crate::interpreter::lexer::tokens::assignable_tokens::method_call_token::MethodCallToken;
use crate::interpreter::lexer::tokens::if_definition::IfDefinition;
use crate::interpreter::lexer::tokens::variable_token::VariableToken;

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
use std::fmt::{Display, Formatter};
use crate::interpreter::lexer::tokens::assignable_tokens::method_call_token::MethodCallToken;
use crate::interpreter::lexer::tokens::variable_token::VariableToken;

#[derive(Debug)]
pub enum Token {
    Variable(VariableToken<'=', ';'>),
    MethodCall(MethodCallToken),
    None
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Token::Variable(v) => format!("{}", v),
            Token::MethodCall(m) => format!("{}", m),
            Token::None => "None".to_string()
        })
    }
}
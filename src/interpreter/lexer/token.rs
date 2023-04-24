use crate::interpreter::lexer::tokens::variable_token::VariableToken;

#[derive(Debug)]
pub enum Token {
    VariableToken(VariableToken),
    None
}
use crate::interpreter::lexer::tokens::assignable_tokens::method_call_token::MethodCallToken;
use crate::interpreter::lexer::tokens::variable_token::VariableToken;

#[derive(Debug)]
pub enum Token {
    VariableToken(VariableToken<'=', ';'>),
    MethodToken(MethodCallToken),
    None
}
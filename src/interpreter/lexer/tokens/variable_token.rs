use crate::interpreter::lexer::tokens::assignable_token::AssignableToken;
use crate::interpreter::lexer::tokens::name_token::NameToken;

pub struct VariableToken {
    pub name_token: NameToken,
    pub assignable: AssignableToken
}
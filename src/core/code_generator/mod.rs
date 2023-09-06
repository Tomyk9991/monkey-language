use std::fmt::{Display, Formatter};
use crate::core::code_generator::generator::Stack;
use crate::core::lexer::tokens::assignable_token::AssignableToken;

pub mod generator;
pub mod target_creator;



#[derive(Debug)]
pub enum Error {
    VariableAlreadyUsed { name: String },
    UnresolvedReference { name: String },
    TokenNotParsable { assignable_token: AssignableToken}
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {

}

pub trait ToASM {
    fn to_asm(&self, stack: &mut Stack) -> Result<String, Error>;
}
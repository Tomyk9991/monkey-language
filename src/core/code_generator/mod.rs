use std::fmt::{Display, Formatter};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::target_os::TargetOS;
use crate::core::lexer::tokens::assignable_token::AssignableToken;

pub mod generator;
pub mod target_creator;

pub mod target_os;


#[derive(Debug)]
pub enum Error {
    VariableAlreadyUsed { name: String },
    UnresolvedReference { name: String },
    TokenNotParsable { assignable_token: AssignableToken},
    NotImplemented { token: String, }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error { }

pub trait ToASM {
    // todo: ultimately target_os as a parameter should not be relevant. this is a temporary solution until a proper syscalls implementation is done
    fn to_asm(&self, stack: &mut Stack, target_os: &TargetOS) -> Result<String, Error>;
}
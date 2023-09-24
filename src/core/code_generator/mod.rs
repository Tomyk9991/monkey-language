use std::fmt::{Display, Formatter};
use crate::core::io::code_line::CodeLine;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::target_os::TargetOS;
use crate::core::lexer::tokens::assignable_token::AssignableToken;

pub mod generator;
pub mod target_creator;

pub mod target_os;


#[derive(Debug)]
pub enum ASMGenerateError {
    _VariableAlreadyUsed { name: String, code_line: CodeLine },
    UnresolvedReference { name: String, code_line: CodeLine },
    TokenNotBuildable { assignable_token: AssignableToken, },
    NotImplemented { token: String, },
}

impl Display for ASMGenerateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ASMGenerateError::_VariableAlreadyUsed { name, code_line } => write!(f, "Line:{:?}:\tVariable already in use: {}", code_line.actual_line_number, name),
            ASMGenerateError::UnresolvedReference { name, code_line } => write!(f, "Line:{:?}:\tCannot resolve variable: {}", code_line.actual_line_number, name),
            ASMGenerateError::TokenNotBuildable { assignable_token } => write!(f, "Token not buildable: {}", assignable_token),
            ASMGenerateError::NotImplemented { token } => write!(f, "ASM cannot build this token: {}", token),
        }
    }
}

impl std::error::Error for ASMGenerateError { }

#[derive(Debug)]
pub struct MetaInfo {
    pub code_line: CodeLine,
    // todo: ultimately target_os as a parameter should not be relevant. this is a temporary solution until a proper sys-calls implementation is done
    pub target_os: TargetOS
}

pub trait ToASM {
    fn to_asm(&self, stack: &mut Stack, meta: &MetaInfo) -> Result<String, ASMGenerateError>;
    fn is_stack_look_up(&self, stack: &mut Stack, meta: &MetaInfo) -> bool;
}
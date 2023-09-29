use std::fmt::{Display, Formatter};
use crate::core::io::code_line::CodeLine;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::target_os::TargetOS;
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::tokens::assignable_token::AssignableToken;
use crate::core::lexer::type_token::InferTypeError;

pub mod generator;
pub mod target_creator;
pub mod target_os;
pub mod asm_builder;
pub mod conventions;
pub mod register_destination;


#[derive(Debug)]
pub enum ASMGenerateError {
    _VariableAlreadyUsed { name: String, code_line: CodeLine },
    UnresolvedReference { name: String, code_line: CodeLine },
    TypeNotInferrable(InferTypeError),
    AssignmentNotImplemented { assignable_token: AssignableToken, },
    NotImplemented { token: String, },
}

impl From<InferTypeError> for ASMGenerateError {
    fn from(value: InferTypeError) -> Self {
        ASMGenerateError::TypeNotInferrable(value)
    }
}

impl Display for ASMGenerateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ASMGenerateError::_VariableAlreadyUsed { name, code_line } => write!(f, "Line:{:?}:\tVariable already in use: {}", code_line.actual_line_number, name),
            ASMGenerateError::UnresolvedReference { name, code_line } => write!(f, "Line:{:?}:\tCannot resolve variable: {}", code_line.actual_line_number, name),
            ASMGenerateError::AssignmentNotImplemented { assignable_token } => write!(f, "ASM implementation for this Assignment is missing: {}", assignable_token),
            ASMGenerateError::NotImplemented { token } => write!(f, "Cannot build ASM from this token: {}", token),
            ASMGenerateError::TypeNotInferrable(infer) => write!(f, "{}", infer),
        }
    }
}

impl std::error::Error for ASMGenerateError { }

#[derive(Debug)]
pub struct MetaInfo {
    pub code_line: CodeLine,
    // todo: ultimately target_os as a parameter should not be relevant. this is a temporary solution until a proper sys-calls implementation is done
    pub target_os: TargetOS,
    pub static_type_information: StaticTypeContext
}

pub trait ToASM {
    /// Generates a String that represents the token in assembler language
    fn to_asm(&self, stack: &mut Stack, meta: &mut MetaInfo) -> Result<String, ASMGenerateError>;
    /// returns a bool, if the current implementor needs to look up it's state in the stack
    fn is_stack_look_up(&self, stack: &mut Stack, meta: &MetaInfo) -> bool;
    /// returns the size in byte to indicate how much space on the stack must be reserved
    fn byte_size(&self, meta: &mut MetaInfo) -> usize;
    /// returns a possible string containing ASM that belongs before the actual label
    fn before_label(&self, stack: &mut Stack, meta: &mut MetaInfo) -> Option<Result<String, ASMGenerateError>>;
}
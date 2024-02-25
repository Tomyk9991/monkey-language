use std::fmt::{Debug, Display, Formatter};
use crate::core::code_generator::asm_result::{ASMOptions, ASMResult, ASMResultError};
use crate::core::io::code_line::CodeLine;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::target_os::TargetOS;
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::tokens::assignable_token::AssignableToken;
use crate::core::lexer::types::cast_to::CastToError;
use crate::core::lexer::types::type_token::InferTypeError;

pub mod generator;
pub mod target_creator;
pub mod target_os;
pub mod asm_builder;
pub mod conventions;
pub mod register_destination;
pub mod registers;
pub mod asm_result;


#[derive(Debug)]
pub enum ASMGenerateError {
    _VariableAlreadyUsed { name: String, code_line: CodeLine },
    UnresolvedReference { name: String, code_line: CodeLine },
    CastUnsupported(CastToError, CodeLine),
    EntryPointNotFound,
    MultipleEntryPointsFound(Vec<CodeLine>),
    TypeNotInferrable(InferTypeError),
    InternalError(String),
    ASMResult(ASMResultError),
    AssignmentNotImplemented { assignable_token: AssignableToken, },
    NotImplemented { token: String, },
}

impl From<ASMResultError> for ASMGenerateError {
    fn from(value: ASMResultError) -> Self {
        ASMGenerateError::ASMResult(value)
    }
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
            ASMGenerateError::InternalError(message) => write!(f, "Internal Error: {}", message),
            ASMGenerateError::CastUnsupported(cast_to, code_line) => write!(f, "Line: {:?}:\t{}", code_line.actual_line_number, cast_to),
            ASMGenerateError::ASMResult(r) => write!(f, "{}", r),
            ASMGenerateError::EntryPointNotFound => write!(f, "No entry point for the program was found. Consider adding `main` function"),
            ASMGenerateError::MultipleEntryPointsFound(e) => write!(f, "Multiple entry points were found: [\n{}\n]", {
                e.iter().map(|l| format!("\tLine: {:?}", l.actual_line_number))
                    .collect::<Vec<String>>()
                    .join(",\n")
            }),
        }
    }
}

impl std::error::Error for ASMGenerateError { }

#[derive(Debug, Default)]
pub struct MetaInfo {
    pub code_line: CodeLine,
    pub target_os: TargetOS,
    pub static_type_information: StaticTypeContext
}



pub trait ToASM {
    /// Generates a String that represents the token in assembler language
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError>;
    /// returns a bool, if the current implementor needs to look up it's state in the stack
    fn is_stack_look_up(&self, stack: &mut Stack, meta: &MetaInfo) -> bool;
    /// returns the size in byte to indicate how much space on the stack must be reserved
    fn byte_size(&self, meta: &mut MetaInfo) -> usize;
    /// returns a possible string containing ASM that belongs before the actual label
    fn data_section(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> bool {
        false
    }
}
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::io::code_line::CodeLine;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::registers::{ByteSize, GeneralPurposeRegister};
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

pub enum ASMResult {
    /// If the result is Inline this means statements like "mov rax, [String] is possible
    Inline(String),
    /// If the result is multiline this means you need to write to the target first and only after you can assign something
    MultilineResulted(String, GeneralPurposeRegister),
    Multiline(String),
}

impl Display for ASMResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ASMResult::Inline(s) => write!(f, "{s}"),
            ASMResult::Multiline(s) => write!(f, "{s}"),
            ASMResult::MultilineResulted(_, _) => write!(f, ""),
        }
    }
}

#[derive(Debug)]
pub enum ASMResultVariance {
    Inline,
    MultilineResulted,
    Multiline
}

#[derive(Debug)]
pub enum ASMResultError {
    UnexpectedVariance { expected: Vec<ASMResultVariance>, actual: ASMResultVariance, token: String },
    NoOptionProvided(String),
}



impl Display for ASMResultError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ASMResultError::UnexpectedVariance { expected, actual, token } => {
                write!(f, "Expected `{expected:?}` asm result but `{actual:?}` was provided in token: `{token}`")
            }
            ASMResultError::NoOptionProvided(s) => write!(f, "No option for the asm result is provided in: {s}")
        }
    }
}

impl Error for ASMResultError { }

pub trait ASMOptions: Clone {
    fn transform(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Result<ASMResult, ASMGenerateError> {
        Ok(ASMResult::Inline(String::new()))
    }
}


#[derive(Debug, Clone)]
pub struct InterimResultOption {
    pub general_purpose_register: GeneralPurposeRegister
}

impl ASMOptions for InterimResultOption { }

impl From<&GeneralPurposeRegister> for InterimResultOption {
    fn from(value: &GeneralPurposeRegister) -> Self {
        InterimResultOption {
            general_purpose_register: value.clone(),
        }
    }
}

/// Builds the assembly instructions to load a float token into a general purpose register
/// and finally to a register, where a float operation can be operated on
#[derive(Clone)]
pub struct PrepareRegisterOption {
    pub general_purpose_register: GeneralPurposeRegister,
    pub assignable_token: Option<AssignableToken>,
}

impl ASMOptions for PrepareRegisterOption {
    fn transform(&self, stack: &mut Stack, meta: &mut MetaInfo) -> Result<ASMResult, ASMGenerateError> {
        if let Some(AssignableToken::FloatToken(float_token)) = &self.assignable_token {
            let size = float_token.byte_size(meta);
            let general_purpose_register_sized = self.general_purpose_register.to_size_register(&ByteSize::try_from(size)?);
            let float_register = &self.general_purpose_register.to_float_register();

            let mut target = match float_token.to_asm(stack, meta, Some(InterimResultOption::from(&general_purpose_register_sized)))? {
                ASMResult::Inline(t) | ASMResult::MultilineResulted(t, _) | ASMResult::Multiline(t) => t
            };

            target += &ASMBuilder::mov_x_ident_line(float_register, &general_purpose_register_sized, Some(size));
            return Ok(ASMResult::MultilineResulted(target, float_register.clone()));
        }

        if let Some(AssignableToken::NameToken(name_token)) = &self.assignable_token {
            let size = name_token.byte_size(meta);
            let general_purpose_register_sized = self.general_purpose_register.to_size_register(&ByteSize::try_from(size)?);
            let float_register = &self.general_purpose_register.to_float_register();

            let mut target = match name_token.to_asm::<InterimResultOption>(stack, meta, None)? {
                ASMResult::Inline(t) | ASMResult::MultilineResulted(t, _) | ASMResult::Multiline(t) => {
                    ASMBuilder::mov_ident_line(&general_purpose_register_sized, t)
                }
            };

            target += &ASMBuilder::mov_x_ident_line(float_register, &general_purpose_register_sized, Some(size));
            return Ok(ASMResult::MultilineResulted(target, float_register.clone()));
        }

        Err(ASMGenerateError::ASMResult(ASMResultError::NoOptionProvided("Wrong assignable in Float calculation".to_string())))
    }
}

pub trait ToASM {
    /// Generates a String that represents the token in assembler language
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError>;
    /// returns a bool, if the current implementor needs to look up it's state in the stack
    fn is_stack_look_up(&self, stack: &mut Stack, meta: &MetaInfo) -> bool;
    /// returns the size in byte to indicate how much space on the stack must be reserved
    fn byte_size(&self, meta: &mut MetaInfo) -> usize;
    /// returns a possible string containing ASM that belongs before the actual label
    fn before_label(&self, stack: &mut Stack, meta: &mut MetaInfo) -> Option<Result<String, ASMGenerateError>>;
    /// returns true and the register where the result is stored, if the generated assembly code has multiple lines.
    fn multi_line_asm(&self, stack: &mut Stack, meta: &mut MetaInfo) -> Result<(bool, String, Option<GeneralPurposeRegister>), ASMGenerateError>;
}
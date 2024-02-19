use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::registers::{ByteSize, GeneralPurposeRegister};
use crate::core::lexer::tokens::assignable_token::AssignableToken;

pub enum ASMResult {
    /// If the result is Inline this means statements like "mov rax, [String] is possible
    Inline(String),
    /// If the result is multiline this means you need to write to the target first and only after you can assign something
    MultilineResulted(String, GeneralPurposeRegister),
    Multiline(String),
}


pub struct ApplyWith<'a> {
    value: &'a ASMResult,
    target: &'a mut String,
    actual: ASMResultVariance,
    allowed: Vec<ASMResultVariance>,
    token: String
}
pub struct ApplyWithToken<'a> {
    apply_with: &'a mut ApplyWith<'a>
}

impl ApplyWithToken<'_> {
    pub fn finish(&mut self) -> Result<Option<GeneralPurposeRegister>, ASMResultError> {
        if !self.apply_with.allowed.contains(&self.apply_with.actual) {
            return Err(ASMResultError::UnexpectedVariance {
                expected: self.apply_with.allowed.clone(),
                actual: self.apply_with.actual.clone(),
                token: self.apply_with.token.to_string(),
            })
        }

        match self.apply_with.value {
            ASMResult::Inline(t) => {
                self.apply_with.target.push_str(t);
                Ok(None)
            }
            ASMResult::MultilineResulted(t, g) => {
                self.apply_with.target.push_str(t);
                Ok(Some(g.clone()))
            }
            ASMResult::Multiline(t) => {
                self.apply_with.target.push_str(t);
                Ok(None)
            }
        }
    }
}

impl<'a> ApplyWith<'a> {
    pub fn allow(mut self, allowing: ASMResultVariance) -> Self {
        self.allowed.push(allowing);
        self
    }

    pub fn token(&'a mut self, token: &str) -> ApplyWithToken {
        self.token = token.to_owned();

        ApplyWithToken {
            apply_with: self,
        }
    }
}

impl ASMResult {
    pub fn apply_with<'a>(&'a self, target: &'a mut String) -> ApplyWith {
        ApplyWith {
            value: self,
            target,
            actual: ASMResultVariance::from(self),
            allowed: vec![],
            token: "".to_string(),
        }
    }
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

#[derive(Debug, PartialEq, Clone)]
pub enum ASMResultVariance {
    Inline,
    MultilineResulted,
    Multiline
}

impl From<&ASMResult> for ASMResultVariance {
    fn from(value: &ASMResult) -> Self {
        match value {
            ASMResult::Inline(_) => ASMResultVariance::Inline,
            ASMResult::MultilineResulted(_, _) => ASMResultVariance::MultilineResulted,
            ASMResult::Multiline(_) => ASMResultVariance::Multiline,
        }
    }
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
pub struct InExpressionMethodCall;

impl ASMOptions for InExpressionMethodCall { }


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
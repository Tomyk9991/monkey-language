use std::fmt::{Display, Formatter};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::registers::{Bit64, ByteSize, GeneralPurposeRegister};
use crate::core::lexer::types::type_token::TypeToken;

#[allow(unused)]
#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub enum Operator {
    Noop,
    Add,
    Sub,
    Div,
    LeftShift,
    RightShift,
    Mul,
}

pub trait OperatorToASM {
    /// This function is used to convert an operator to an assembly instruction. First tuple element is a possible string of other instructions to the instruction.
    fn operation_to_asm<T: Display>(&self, operator: &Operator, registers: &[T]) -> Result<AssemblerOperation, ASMGenerateError>;
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ToASM for Operator {
    fn to_asm(&self, _: &mut Stack, _: &mut MetaInfo) -> Result<String, ASMGenerateError> {
        Ok(match self {
            Operator::Noop =>"noop".to_string(),
            Operator::Add => "add".to_string(),
            Operator::Sub => "sub".to_string(),
            Operator::Mul => "imul".to_string(),
            Operator::Div => "div".to_string(),
            Operator::LeftShift => "shl".to_string(),
            Operator::RightShift => "shr".to_string(),
        })
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        0
    }

    fn before_label(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Option<Result<String, ASMGenerateError>> {
        None
    }
}

impl Operator {
    pub fn specific_operation<T: Display>(&self, ty: &TypeToken, registers: &[T]) -> Result<AssemblerOperation, ASMGenerateError> {
        ty.operation_to_asm(self, registers)
    }
}

pub struct AssemblerOperation {
    pub prefix: Option<String>,
    pub operation: String,
    pub postfix: Option<String>
}

impl From<String> for AssemblerOperation {
    fn from(value: String) -> Self {
        Self {
            prefix: None,
            operation: value,
            postfix: None
        }
    }
}

impl AssemblerOperation {
    pub fn two_operands<T: Display, P: Display>(instruction: &str, register_a: &T, register_b: &P) -> String {
        format!("{instruction} {register_a}, {register_b}")
    }

    pub fn save_rax_rcx_rdx<T: Display>(size: usize, registers: &[T]) -> Result<String, ASMGenerateError> {
        let byte_size = ByteSize::try_from(size)?;

        let r14 = GeneralPurposeRegister::Bit64(Bit64::R14).to_size_register(&byte_size);
        let rdx = GeneralPurposeRegister::Bit64(Bit64::Rdx).to_size_register(&byte_size);
        let mut prefix = ASMBuilder::mov_line(r14, &rdx);

        let r13 = GeneralPurposeRegister::Bit64(Bit64::R13).to_size_register(&byte_size);
        let rax = GeneralPurposeRegister::Bit64(Bit64::Rax).to_size_register(&byte_size);
        prefix += &ASMBuilder::mov_ident_line(r13, &rax);

        let r12 = GeneralPurposeRegister::Bit64(Bit64::R12).to_size_register(&byte_size);
        let rcx = GeneralPurposeRegister::Bit64(Bit64::Rcx).to_size_register(&byte_size);
        prefix += &ASMBuilder::mov_ident_line(r12, &rcx);

        prefix += &ASMBuilder::mov_ident_line(rcx, &registers[1]);
        prefix += &ASMBuilder::mov_ident_line(rax, &registers[0]);
        prefix += &ASMBuilder::mov_ident_line(rdx, 0);

        Ok(prefix)
    }

    pub fn load_rax_rcx_rdx<T: Display>(size: usize, registers: &[T]) -> Result<String, ASMGenerateError> {
        let byte_size = ByteSize::try_from(size)?;

        let mut postfix = ASMBuilder::mov_ident_line(&registers[0],GeneralPurposeRegister::Bit64(Bit64::Rax).to_size_register(&byte_size));

        let r14 = GeneralPurposeRegister::Bit64(Bit64::R14).to_size_register(&byte_size);
        let rdx = GeneralPurposeRegister::Bit64(Bit64::Rdx).to_size_register(&byte_size);

        if !registers.iter().map(|register| register.to_string()).any(|register| register == rdx.to_string()) {
            postfix += &ASMBuilder::mov_ident_line(rdx, r14);
        }

        let rax = GeneralPurposeRegister::Bit64(Bit64::Rax).to_size_register(&byte_size);
        let r13 = GeneralPurposeRegister::Bit64(Bit64::R13).to_size_register(&byte_size);

        if !registers.iter().map(|register| register.to_string()).any(|register| register == rax.to_string()) {
            postfix += &ASMBuilder::mov_ident_line(rax, r13);
        }

        let r12 = GeneralPurposeRegister::Bit64(Bit64::R12).to_size_register(&byte_size);
        let rcx = GeneralPurposeRegister::Bit64(Bit64::Rcx).to_size_register(&byte_size);

        if !registers.iter().map(|register| register.to_string()).any(|register| register == rcx.to_string()) {
            postfix += &format!("    mov {rcx}, {r12}");
        }

        Ok(postfix)
    }

    pub fn inject_registers(&self) -> String {
        let mut result = String::new();

        if let Some(prefix) = &self.prefix {
            result += &ASMBuilder::push(prefix);
            result += &ASMBuilder::ident_line(&self.operation);
        } else {
            result += &ASMBuilder::push(&self.operation);
        }


        if let Some(postfix) = &self.postfix {
            result += &ASMBuilder::push(postfix);
        }

        result
    }
}
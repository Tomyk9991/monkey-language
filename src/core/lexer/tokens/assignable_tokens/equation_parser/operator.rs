use std::fmt::{Display, Formatter};
use std::str::FromStr;
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
    Mod,
    Mul,
    LeftShift,
    RightShift,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    Equal,
    NotEqual,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    LogicalAnd,
    LogicalOr,
}

pub trait OperatorToASM {
    /// This function is used to convert an operator to an assembly instruction. First tuple element is a possible string of other instructions to the instruction.
    fn operation_to_asm<T: Display>(&self, operator: &Operator, registers: &[T], stack: &mut Stack, meta: &mut MetaInfo) -> Result<AssemblerOperation, ASMGenerateError>;
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Operator::Noop => "Noop",
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
            Operator::LeftShift => "<<",
            Operator::RightShift => ">>",
            Operator::LessThan => "<",
            Operator::GreaterThan => ">",
            Operator::LessThanEqual => "<=",
            Operator::GreaterThanEqual => ">=",
            Operator::Equal => "==",
            Operator::NotEqual => "!=",
            Operator::BitwiseAnd => "&",
            Operator::BitwiseXor => "^",
            Operator::BitwiseOr => "|",
            Operator::LogicalAnd => "&&",
            Operator::LogicalOr => "||",
            Operator::Mod => "%",
        })
    }
}

impl ToASM for Operator {
    fn to_asm(&self, _: &mut Stack, _: &mut MetaInfo) -> Result<String, ASMGenerateError> {
        Ok(match self {
            Operator::Noop =>"noop",
            Operator::Add => "add",
            Operator::Sub => "sub",
            Operator::Mul => "mul",
            Operator::Div => "div",
            Operator::LeftShift => "shl",
            Operator::RightShift => "shr",
            Operator::LessThan => "setl",
            Operator::GreaterThan => "setg",
            Operator::LessThanEqual => "setle",
            Operator::GreaterThanEqual => "setge",
            Operator::Equal => "sete",
            Operator::NotEqual => "setne",
            Operator::BitwiseAnd => "and",
            Operator::BitwiseXor => "xor",
            Operator::BitwiseOr => "or",
            Operator::LogicalAnd => "je",
            Operator::LogicalOr => "jne",
            Operator::Mod => "div",
        }.to_string())
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
    pub fn specific_operation<T: Display>(&self, ty: &TypeToken, registers: &[T], stack: &mut Stack, meta: &mut MetaInfo) -> Result<AssemblerOperation, ASMGenerateError> {
        ty.operation_to_asm(self, registers, stack, meta)
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
    pub fn compare<P: Display, T: Display>(instruction: &str, destination: &T, source: &P) -> Result<String, ASMGenerateError> {
        let register_a = GeneralPurposeRegister::from_str(&destination.to_string())?.to_size_register(&ByteSize::_1);
        Ok(format!("cmp {}, {}\n    {} {}", destination, source, instruction, register_a))
    }

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
            postfix += &ASMBuilder::ident(&format!("mov {}, {}", rdx, r14));
        }

        let rax = GeneralPurposeRegister::Bit64(Bit64::Rax).to_size_register(&byte_size);
        let r13 = GeneralPurposeRegister::Bit64(Bit64::R13).to_size_register(&byte_size);

        if !registers.iter().map(|register| register.to_string()).any(|register| register == rax.to_string()) {
            postfix += &ASMBuilder::push(&format!("\n    mov {rax}, {r13}"));
        }

        let r12 = GeneralPurposeRegister::Bit64(Bit64::R12).to_size_register(&byte_size);
        let rcx = GeneralPurposeRegister::Bit64(Bit64::Rcx).to_size_register(&byte_size);

        if !registers.iter().map(|register| register.to_string()).any(|register| register == rcx.to_string()) {
            postfix += &format!("\n    mov {rcx}, {r12}");
        } else {

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
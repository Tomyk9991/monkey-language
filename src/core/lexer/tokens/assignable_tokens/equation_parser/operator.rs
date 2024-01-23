use std::fmt::{Display, Formatter};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::lexer::types::type_token::TypeToken;

#[allow(unused)]
#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub enum Operator {
    Noop,
    Add,
    Sub,
    Div,
    Mul,
}

pub trait OperatorToASM {
    fn operation_to_asm(&self, operator: &Operator) -> Result<String, ASMGenerateError>;
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
    pub fn specific_operation(&self, ty: &TypeToken) -> Result<String, ASMGenerateError> {
        ty.operation_to_asm(&self)
    }
}
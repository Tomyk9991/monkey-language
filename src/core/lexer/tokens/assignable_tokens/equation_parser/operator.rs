use std::fmt::{Display, Formatter};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{Error, ToASM};
use crate::core::code_generator::target_os::TargetOS;

#[allow(unused)]
#[derive(PartialEq, Clone, Debug)]
pub enum Operator {
    Noop,
    Add,
    Sub,
    Div,
    Mul,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ToASM for Operator {
    fn to_asm(&self, _: &mut Stack, _: &TargetOS) -> Result<String, Error> {
        Ok(match self {
            Operator::Noop =>"    noop".to_string(),
            Operator::Add => "    add rax, rbx".to_string(),
            Operator::Sub => "    sub rax, rbx".to_string(),
            Operator::Mul => "    mul rbx".to_string(),
            Operator::Div => "    div rbx".to_string(),
        })
    }
}
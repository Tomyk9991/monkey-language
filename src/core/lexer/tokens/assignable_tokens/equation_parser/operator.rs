use std::fmt::{Display, Formatter};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{Error, ToASM};

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
    fn to_asm(&self, _: &mut Stack) -> Result<String, Error> {
        Ok(match self {
            Operator::Noop => format!("    noop"),
            Operator::Add => format!("    add rax, rbx"),
            Operator::Sub => format!("    sub rax, rbx"),
            Operator::Mul => format!("    mul rbx"),
            Operator::Div => format!("    div rbx"),
        })
    }
}
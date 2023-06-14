use std::fmt::{Display, Formatter};

#[derive(PartialEq, Clone, Debug)]
pub enum Operator {
    Noop,
    Add,
    Sub,
    Div,
    Mul,
    Pow
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
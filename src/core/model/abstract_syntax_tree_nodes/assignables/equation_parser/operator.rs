use std::fmt::{Display, Formatter};

#[allow(unused)]
#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
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
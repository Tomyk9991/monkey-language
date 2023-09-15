use std::fmt::{Debug, Display, Formatter};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{Error, ToASM};
use crate::core::lexer::tokens::assignable_token::AssignableToken;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;

#[derive(Clone, PartialEq)]
#[allow(unused)]
pub struct Expression {
    pub lhs: Option<Box<Expression>>,
    pub rhs: Option<Box<Expression>>,
    pub operator: Operator,
    pub value: Option<Box<AssignableToken>>,
    pub positive: bool
}

impl Default for Expression {
    fn default() -> Self {
        Self {
            lhs: None,
            rhs: None,
            operator: Operator::Noop,
            value: None,
            positive: true
        }
    }
}

impl Debug for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct_formatter = f.debug_struct("Expression");
        
        if let Some(lhs) = &self.lhs {
            debug_struct_formatter.field("lhs", lhs);
        }
        
        debug_struct_formatter.field("operator", &self.operator);
        
        if let Some(rhs) = &self.rhs {
            debug_struct_formatter.field("rhs", rhs);
        }
        
        if let Some(value) = &self.value {
            debug_struct_formatter.field("value", value);
        }
        
        debug_struct_formatter.field("positive", &self.positive);
        
        debug_struct_formatter.finish()
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match (&self.lhs, &self.rhs) {
            (Some(lhs), Some(rhs)) => {
                write!(f, "{}({} {} {})", if self.positive { "" } else { "-" }, lhs, &self.operator, rhs)
            }
            _ => {
                if let Some(ass) = &self.value {
                    write!(f, "{}{}", if self.positive { "" } else { "-" }, ass)
                } else {
                    write!(f, "Some error. No lhs and rhs and no value found")
                }
            }
        }
    }
}


impl From<Option<Box<AssignableToken>>> for Expression {
    fn from(value: Option<Box<AssignableToken>>) -> Self {
        Expression {
            value,
            ..Default::default()
        }
    }
}

impl ToASM for Expression {
    fn to_asm(&self, stack: &mut Stack) -> Result<String, Error> {
        if let Some(value) = &self.value { // this means, no children are provided. this is the actual value
            return value.to_asm(stack);
        }

        let mut comment = String::new();

        let mut target = String::new();
        if let Some(rhs) = &self.rhs {
            target.push_str(&rhs.to_asm(stack)?);
            comment.push_str(&format!("{} ", rhs));
        }

        comment.push_str(&format!("{} ", self.operator));

        if let Some(lhs) = &self.lhs {
            target.push_str(&lhs.to_asm(stack)?);
            comment.push_str(&format!("{}", lhs));
        }


        target.push_str(&stack.pop_stack("rax"));
        target.push_str(&stack.pop_stack("rbx"));

        target.push_str(&format!("    ; {}\n", comment));
        target.push_str(&format!("{}\n", self.operator.to_asm(stack)?));
        target.push_str(&stack.push_stack("rax"));

        Ok(target)
    }
}

#[allow(unused)]
impl Expression {
    pub fn new(lhs: Option<Box<Expression>>, operator: Operator, rhs: Option<Box<Expression>>, value: Option<Box<AssignableToken>>) -> Self {
        Self {
            lhs,
            rhs,
            operator,
            value,
            positive: true
        }
    }

    pub fn set(&mut self, lhs: Option<Box<Expression>>, operation: Operator, rhs: Option<Box<Expression>>, value: Option<Box<AssignableToken>>) {
        self.lhs = lhs;
        self.rhs = rhs;
        self.operator = operation;
        self.value = value;
    }

    pub fn flip_value(&mut self) {
        if let Some(v) = &mut self.value {
            self.positive = !self.positive;
        }
    }
}
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::generator::Stack;
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::tokens::assignable_token::AssignableToken;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;
use crate::core::lexer::type_token::{InferTypeError, TypeToken};

#[derive(Clone, PartialEq)]
#[allow(unused)]
pub struct Expression {
    pub lhs: Option<Box<Expression>>,
    pub rhs: Option<Box<Expression>>,
    pub operator: Operator,
    pub value: Option<Box<AssignableToken>>,
    pub positive: bool,
}

impl Default for Expression {
    fn default() -> Self {
        Self {
            lhs: None,
            rhs: None,
            operator: Operator::Noop,
            value: None,
            positive: true,
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
    fn to_asm(&self, stack: &mut Stack, meta: &mut MetaInfo) -> Result<String, ASMGenerateError> {

        if let Some(value) = &self.value { // this means, no children are provided. this is the actual value
            return value.to_asm(stack, meta);
        }

        let mut target = String::new();
        target += &ASMBuilder::ident(&ASMBuilder::comment_line(&format!("{}", self)));

        match (&self.rhs, &self.lhs) {
            (Some(lhs), Some(rhs)) => {
                match (&lhs.value, &rhs.value) {
                    (Some(_), Some(_)) => {
                        target += &ASMBuilder::ident_line(&format!("mov eax, {}", rhs.to_asm(stack, meta)?));
                        target += &ASMBuilder::ident_line(&format!("{} eax, {}", self.operator.to_asm(stack, meta)?, lhs.to_asm(stack, meta)?));
                        target += &ASMBuilder::ident_line(&format!("mov {}, eax", stack.register_to_use));
                    }
                    (None, Some(_)) => {
                        target += &ASMBuilder::push(&lhs.to_asm(stack, meta)?.to_string());
                        target += &ASMBuilder::ident_line(&format!("{} eax, {}\n", self.operator.to_asm(stack, meta)?, rhs.to_asm(stack, meta)?));
                    }
                    (Some(_), None) => {
                        target += &ASMBuilder::push(&rhs.to_asm(stack, meta)?.to_string());
                        target += &ASMBuilder::ident_line(&format!("{} edx, {}", self.operator.to_asm(stack, meta)?, lhs.to_asm(stack, meta)?));
                    }
                    (None, None) => {
                        stack.register_to_use = String::from("edx");
                        target += &ASMBuilder::push(&rhs.to_asm(stack, meta)?.to_string());
                        stack.register_to_use = String::from("eax");
                        target += &ASMBuilder::push(&lhs.to_asm(stack, meta)?.to_string());
                        stack.register_to_use = String::from("");

                        target += &ASMBuilder::ident_line(&format!("{} eax, edx", self.operator.to_asm(stack, meta)?));
                    }
                }
            }
            (_, _) => return Err(ASMGenerateError::NotImplemented { token: "Something went wrong. Neither rhs nor lhs are valid".to_string() })
        }

        Ok(target)
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        true
    }

    fn byte_size(&self, meta: &mut MetaInfo) -> usize {
        if let Some(ty) = self.traverse_type(&meta.code_line) {
            return ty.byte_size();
        }

        0
    }

    fn before_label(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Option<Result<String, ASMGenerateError>> {
        None
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
            positive: true,
        }
    }

    pub fn traverse_type(&self, code_line: &CodeLine) -> Option<TypeToken> {
        self.traverse_type_resulted(&StaticTypeContext::default(), code_line).ok()
    }

    pub fn traverse_type_resulted(&self, context: &StaticTypeContext, code_line: &CodeLine) -> Result<TypeToken, InferTypeError> {
        if let Some(value) = &self.value {
            return value.infer_type_with_context(context, code_line);
        }

        if let Some(lhs) = &self.lhs {
            if let Some(rhs) = &self.rhs {
                let lhs_type = lhs.traverse_type_resulted(context, code_line)?;
                let rhs_type = rhs.traverse_type_resulted(context, code_line)?;

                let mut base_type_matrix: HashMap<(TypeToken, Operator, TypeToken), TypeToken> = HashMap::new();
                base_type_matrix.insert((TypeToken::String, Operator::Add, TypeToken::String), TypeToken::String);

                base_type_matrix.insert((TypeToken::I32, Operator::Add, TypeToken::I32), TypeToken::I32);
                base_type_matrix.insert((TypeToken::I32, Operator::Sub, TypeToken::I32), TypeToken::I32);
                base_type_matrix.insert((TypeToken::I32, Operator::Mul, TypeToken::I32), TypeToken::I32);
                base_type_matrix.insert((TypeToken::I32, Operator::Div, TypeToken::I32), TypeToken::F32);

                base_type_matrix.insert((TypeToken::F32, Operator::Add, TypeToken::F32), TypeToken::F32);
                base_type_matrix.insert((TypeToken::F32, Operator::Sub, TypeToken::F32), TypeToken::F32);
                base_type_matrix.insert((TypeToken::F32, Operator::Mul, TypeToken::F32), TypeToken::F32);
                base_type_matrix.insert((TypeToken::F32, Operator::Div, TypeToken::F32), TypeToken::F32);

                base_type_matrix.insert((TypeToken::F32, Operator::Add, TypeToken::I32), TypeToken::F32);
                base_type_matrix.insert((TypeToken::F32, Operator::Sub, TypeToken::I32), TypeToken::F32);
                base_type_matrix.insert((TypeToken::F32, Operator::Mul, TypeToken::I32), TypeToken::F32);
                base_type_matrix.insert((TypeToken::F32, Operator::Div, TypeToken::I32), TypeToken::F32);

                base_type_matrix.insert((TypeToken::I32, Operator::Add, TypeToken::F32), TypeToken::F32);
                base_type_matrix.insert((TypeToken::I32, Operator::Sub, TypeToken::F32), TypeToken::F32);
                base_type_matrix.insert((TypeToken::I32, Operator::Mul, TypeToken::F32), TypeToken::F32);
                base_type_matrix.insert((TypeToken::I32, Operator::Div, TypeToken::F32), TypeToken::F32);

                base_type_matrix.insert((TypeToken::Bool, Operator::Add, TypeToken::Bool), TypeToken::Bool);
                base_type_matrix.insert((TypeToken::Bool, Operator::Sub, TypeToken::Bool), TypeToken::Bool);
                base_type_matrix.insert((TypeToken::Bool, Operator::Mul, TypeToken::Bool), TypeToken::Bool);
                base_type_matrix.insert((TypeToken::Bool, Operator::Div, TypeToken::Bool), TypeToken::Bool);

                if let Some(result_type) = base_type_matrix.get(&(lhs_type.clone(), self.operator.clone(), rhs_type.clone())) {
                    return Ok(result_type.clone());
                }

                return Err(InferTypeError::TypesNotCalculable(lhs_type, self.operator.clone(), rhs_type, code_line.clone()));
            }
        }

        Err(InferTypeError::UnresolvedReference(self.to_string(), code_line.clone()))
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
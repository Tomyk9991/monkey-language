use std::collections::HashMap;
use std::fmt::Display;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::{AssemblerOperation, Operator, OperatorToASM};
use crate::core::lexer::types::cast_to::{Castable, CastTo};
use crate::core::lexer::types::integer::Integer;
use crate::core::lexer::types::type_token::TypeToken;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Boolean {
    True,
    False,
}

impl OperatorToASM for Boolean {
    fn operation_to_asm<T: Display>(&self, operator: &Operator, registers: &[T], stack: &mut Stack, meta: &mut MetaInfo) -> Result<AssemblerOperation, ASMGenerateError> {
        fn no_operation(operation: &str) -> Result<AssemblerOperation, ASMGenerateError> {
            Err(ASMGenerateError::InternalError(format!("No operation `{}` on booleans", operation)))
        }

        match operator {
            Operator::Noop => no_operation("noop"),
            Operator::Add => no_operation("add"),
            Operator::Sub => no_operation("sub"),
            Operator::Div => no_operation("div"),
            Operator::Mul => no_operation("mul"),
            Operator::LeftShift => no_operation("left shift"),
            Operator::RightShift => no_operation("right shift"),
            Operator::LessThan => no_operation("less than"),
            Operator::GreaterThan => no_operation("greater than"),
            Operator::LessThanEqual => no_operation("less than equal"),
            Operator::GreaterThanEqual => no_operation("greater than equal"),
            Operator::BitwiseXor => no_operation("bitwise xor"),
            Operator::Equal | Operator::NotEqual => Ok(AssemblerOperation {
                prefix: None,
                operation: AssemblerOperation::compare(&operator.to_asm(&mut Default::default(), &mut Default::default())?, &registers[0], &registers[1])?,
                postfix: None,
            }),
            Operator::BitwiseAnd | Operator::BitwiseOr => {
                Ok(AssemblerOperation {
                    prefix: None,
                    operation: AssemblerOperation::two_operands(&operator.to_asm(stack, meta)?, &registers[0], &registers[1]),
                    postfix: None,
                })
            }
        }
    }
}

impl Castable<Boolean, Integer> for Boolean {
    fn add_casts(cast_matrix: &mut HashMap<(TypeToken, TypeToken), &'static str>) {
        cast_matrix.insert((TypeToken::Bool, TypeToken::Integer(Integer::I32)), "movzx");
    }

    fn cast_from_to(_: &Boolean, t2: &Integer, source: &str, stack: &mut Stack, meta: &mut MetaInfo) -> Result<String, ASMGenerateError> {
        let cast_to = CastTo {
            from: TypeToken::Bool,
            to: TypeToken::Integer(t2.clone()),
        };

        // no instruction is needed. just pretend the bool is an u8
        let mut target = String::new();
        target += &ASMBuilder::ident_comment_line(&format!("Cast: ({}) -> ({})", cast_to.from, cast_to.to));
        target += &Integer::cast_from_to(&Integer::U8, t2, source, stack, meta)?;


        Ok(target)
    }
}

impl Boolean {
    pub fn operation_matrix(base_type_matrix: &mut HashMap<(TypeToken, Operator, TypeToken), TypeToken>) {
        base_type_matrix.insert((TypeToken::Bool, Operator::BitwiseAnd, TypeToken::Bool), TypeToken::Bool);
        base_type_matrix.insert((TypeToken::Bool, Operator::BitwiseOr, TypeToken::Bool), TypeToken::Bool);
    }
}
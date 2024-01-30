use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::{FromStr, ParseBoolError};

use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::generator::Stack;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::{AssemblerOperation, Operator, OperatorToASM};
use crate::core::lexer::types::cast_to::{Castable, CastTo};
use crate::core::lexer::types::integer::Integer;
use crate::core::lexer::types::type_token::TypeToken;

#[derive(Debug, PartialEq, Clone)]
pub struct BooleanToken {
    pub value: bool,
}

#[derive(Debug)]
pub enum BooleanTokenErr {
    UnmatchedRegex,
    ParseBoolError(ParseBoolError),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Boolean {
    True,
    False,
}

impl OperatorToASM for Boolean {
    fn operation_to_asm<T: Display>(&self, operator: &Operator, registers: &[T]) -> Result<AssemblerOperation, ASMGenerateError> {
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
            Operator::Equal | Operator::NotEqual => Ok(AssemblerOperation {
                prefix: None,
                operation: AssemblerOperation::compare(&operator.to_asm(&mut Default::default(), &mut Default::default())?, &registers[0], &registers[1])?,
                postfix: None,
            }),
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
            to: TypeToken::Integer(t2.clone())
        };

        // no instruction is needed. just pretend the bool is an u8
        let mut target = String::new();
        target += &ASMBuilder::ident_comment_line(&format!("Cast: ({}) -> ({})", cast_to.from, cast_to.to));
        target += &<Integer as Castable<Integer, Integer>>::cast_from_to(&Integer::U8, t2, source, stack, meta)?;


        Ok(target)
    }
}

impl Boolean {
    pub fn operation_matrix(base_type_matrix: &mut HashMap<(TypeToken, Operator, TypeToken), TypeToken>) {
        base_type_matrix.insert((TypeToken::Bool, Operator::Add, TypeToken::Bool), TypeToken::Bool);
        base_type_matrix.insert((TypeToken::Bool, Operator::Sub, TypeToken::Bool), TypeToken::Bool);
        base_type_matrix.insert((TypeToken::Bool, Operator::Mul, TypeToken::Bool), TypeToken::Bool);
        base_type_matrix.insert((TypeToken::Bool, Operator::Div, TypeToken::Bool), TypeToken::Bool);
    }
}

impl From<ParseBoolError> for BooleanTokenErr {
    fn from(value: ParseBoolError) -> Self { BooleanTokenErr::ParseBoolError(value) }
}

impl Display for BooleanTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            BooleanTokenErr::UnmatchedRegex => "Boolean must match ^(?i:true|false)$".to_string(),
            BooleanTokenErr::ParseBoolError(err) => err.to_string()
        })
    }
}

impl Error for BooleanTokenErr {}

impl Display for BooleanToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.value) }
}

impl ToASM for BooleanToken {
    fn to_asm(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Result<String, ASMGenerateError> {
        todo!()
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        todo!()
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        4
    }

    fn before_label(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Option<Result<String, ASMGenerateError>> {
        None
    }
}

impl FromStr for BooleanToken {
    type Err = BooleanTokenErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !lazy_regex::regex_is_match!("^(?i:true|false)$", s) {
            return Err(BooleanTokenErr::UnmatchedRegex);
        }

        Ok(BooleanToken {
            value: s.parse::<bool>()?
        })
    }
}
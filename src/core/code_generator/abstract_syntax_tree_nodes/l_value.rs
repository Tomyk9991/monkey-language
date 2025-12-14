use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::asm_result::ASMResult;
use crate::core::code_generator::register_destination::word_from_byte_size;
use crate::core::code_generator::registers::{GeneralPurposeRegister};
use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseResult};
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::abstract_syntax_tree_nodes::l_value::{LValue, LValueError};
use crate::core::parser::abstract_syntax_tree_nodes::assignables::equation_parser::Equation;


impl ToASM for LValue {
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        match self {
            LValue::Identifier(name) => name.to_asm(stack, meta, options),
        }
    }

    fn is_stack_look_up(&self, stack: &mut Stack, meta: &MetaInfo) -> bool {
        match self {
            LValue::Identifier(a) => a.is_stack_look_up(stack, meta)
        }
    }

    fn byte_size(&self, meta: &mut MetaInfo) -> usize {
        match self {
            LValue::Identifier(a) => a.byte_size(meta)
        }
    }
}
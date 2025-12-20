use std::any::Any;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::core::code_generator::{ASMGenerateError, MetaInfo, register_destination, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_result::{ASMResult, ASMResultError, ASMResultVariance};
use crate::core::code_generator::generator::{Stack, StackLocation};
use crate::core::code_generator::registers::{ByteSize, GeneralPurposeRegister};
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::operator::Operator;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::prefix_arithmetic::{PointerArithmetic, PrefixArithmetic};
use crate::core::model::types::float::FloatType;
use crate::core::model::types::integer::IntegerType;
use crate::core::model::types::ty::Type;
use crate::core::parser::types::boolean::Boolean;
use crate::core::parser::types::cast_to::{Castable, CastToError};

#[derive(Clone)]
pub struct PrefixArithmeticOptions {
    pub value: Assignable,
    pub register_or_stack_address: String,
    pub register_64: GeneralPurposeRegister,
    pub target_register: GeneralPurposeRegister,
    pub child_has_pointer_arithmetic: bool,
    pub target: String,
}

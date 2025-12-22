use crate::core::code_generator::registers::GeneralPurposeRegister;
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;

#[derive(Clone)]
pub struct PrefixArithmeticOptions {
    pub value: Assignable,
    pub register_or_stack_address: String,
    pub register_64: GeneralPurposeRegister,
    pub target_register: GeneralPurposeRegister,
    pub child_has_pointer_arithmetic: bool,
    pub is_lvalue: bool,
    pub target: String,
}

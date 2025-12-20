pub mod in_expression_method_call;
pub mod interim_result;
pub mod prepare_register;
pub mod identifier_present;

use crate::core::code_generator::asm_options::identifier_present::IdentifierPresent;
use crate::core::code_generator::asm_options::in_expression_method_call::InExpressionMethodCall;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::asm_options::prepare_register::PrepareRegisterOption;
use crate::core::parser::abstract_syntax_tree_nodes::assignables::equation_parser::prefix_arithmetic::PrefixArithmeticOptions;

#[derive(Clone)]
pub enum ASMOptions {
    PrefixArithmeticOptions(PrefixArithmeticOptions),
    InExpressionMethodCall(InExpressionMethodCall),
    InterimResultOption(InterimResultOption),
    PrepareRegisterOption(PrepareRegisterOption),
    IdentifierPresent(IdentifierPresent)
}
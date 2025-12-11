use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;

/// ASM option that represents an identifier present in the stack symbol table
#[derive(Debug, Clone)]
pub struct IdentifierPresent {
    pub identifier: LValue,
}

impl ASMOptions for IdentifierPresent {

}
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::scanner::abstract_syntax_tree_nodes::identifier::Identifier;

/// ASM option that represents an identifier present in the stack symbol table
#[derive(Debug, Clone)]
pub struct IdentifierPresent {
    pub identifier: Identifier,
}

impl ASMOptions for IdentifierPresent {

}
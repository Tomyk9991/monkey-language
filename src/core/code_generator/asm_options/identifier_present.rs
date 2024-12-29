use crate::core::code_generator::asm_options::ASMOptions;

/// ASM option that represents an identifier present in the stack symbol table
#[derive(Debug, Clone)]
pub struct IdentifierPresent {
    pub identifier: String,
}

impl ASMOptions for IdentifierPresent {

}
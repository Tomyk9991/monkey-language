use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::lexer::tokens::name_token::NameToken;

/// ASM option that represents an identifier present in the stack symbol table
#[derive(Debug, Clone)]
pub struct IdentifierPresent {
    pub identifier: NameToken,
}

impl ASMOptions for IdentifierPresent {

}
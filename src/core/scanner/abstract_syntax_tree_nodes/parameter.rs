use std::fmt::{Display, Formatter};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::{ASMResult};
use crate::core::code_generator::conventions::CallingRegister;
use crate::core::io::code_line::CodeLine;
use crate::core::scanner::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::scanner::types::r#type::Type;

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    /// name of the variable
    pub identifier: Identifier,
    /// Type of the parameter
    pub ty: Type,
    /// Where is the data stored?
    pub register: CallingRegister,
    pub mutability: bool,
    pub code_line: CodeLine
}

impl Display for Parameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Parameter")
            .field("identifier", &self.identifier)
            .field("type", &self.ty)
            .field("register", &self.register)
            .finish()
    }
}

impl ToASM for Parameter {
    fn to_asm<T: ASMOptions>(&self, _stack: &mut Stack, _meta: &mut MetaInfo, _options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        Ok(ASMResult::Inline(self.register.to_string()))
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        self.ty.byte_size()
    }
}
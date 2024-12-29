use std::fmt::{Display, Formatter};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::{ASMResult};
use crate::core::code_generator::conventions::CallingRegister;
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::tokens::name_token::NameToken;
use crate::core::lexer::types::type_token::TypeToken;

#[derive(Debug, PartialEq, Clone)]
pub struct ParameterToken {
    /// name of the variable
    pub name_token: NameToken,
    /// Type of the parameter
    pub ty: TypeToken,
    /// Where is the data stored?
    pub register: CallingRegister,
    pub mutablility: bool,
    pub code_line: CodeLine
}

impl Display for ParameterToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParameterToken")
            .field("name_token", &self.name_token)
            .field("type", &self.ty)
            .field("register", &self.register)
            .finish()
    }
}

impl ToASM for ParameterToken {
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
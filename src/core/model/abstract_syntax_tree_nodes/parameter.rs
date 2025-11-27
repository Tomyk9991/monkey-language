use crate::core::code_generator::conventions::CallingRegister;
use crate::core::io::code_line::CodeLine;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::types::ty::Type;

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
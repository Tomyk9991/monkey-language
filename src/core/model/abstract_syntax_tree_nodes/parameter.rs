use crate::core::code_generator::conventions::CallingRegister;
use crate::core::lexer::token_with_span::FilePosition;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::types::ty::Type;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    /// name of the variable
    pub identifier: Identifier,
    /// Type of the parameter
    pub ty: Type,
    /// Where is the data stored?
    pub register: CallingRegister,
    pub mutability: bool,
    pub file_position: FilePosition
}

impl PartialOrd for Parameter {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // partial order without code_line. every other field is considered
        let identifier_cmp = self.identifier.partial_cmp(&other.identifier)?;
        if identifier_cmp != Ordering::Equal {
            return Some(identifier_cmp);
        }
        let type_cmp = self.ty.partial_cmp(&other.ty)?;
        if type_cmp != Ordering::Equal {
            return Some(type_cmp);
        }

        let register_cmp = self.register.partial_cmp(&other.register)?;
        if register_cmp != Ordering::Equal {
            return Some(register_cmp);
        }

        let mutability_cmp = self.mutability.partial_cmp(&other.mutability)?;
        if mutability_cmp != Ordering::Equal {
            return Some(mutability_cmp);
        }

        Some(Ordering::Equal)
    }
}
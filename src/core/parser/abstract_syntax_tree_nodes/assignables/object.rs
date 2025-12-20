use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_result::{ASMResult};
use crate::core::model::abstract_syntax_tree_nodes::assignable::AssignableError;
use crate::core::model::abstract_syntax_tree_nodes::assignables::object::{Object, ObjectErr};
use crate::core::model::abstract_syntax_tree_nodes::identifier::{Identifier, IdentifierError};
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::types::mutability::Mutability;
use crate::core::model::types::ty::Type;
use crate::core::parser::abstract_syntax_tree_nodes::variable::{ParseVariableErr};
use crate::core::parser::utils::dyck::DyckError;

impl From<IdentifierError> for ObjectErr {
    fn from(err: IdentifierError) -> Self { ObjectErr::IdentifierErr(err) }
}

impl From<AssignableError> for ObjectErr {
    fn from(value: AssignableError) -> Self { ObjectErr::AssignableErr(value) }
}

impl From<ParseVariableErr> for ObjectErr {
    fn from(s: ParseVariableErr) -> Self {
        ObjectErr::ParseVariableErr(s)
    }
}

impl From<DyckError> for ObjectErr {
    fn from(s: DyckError) -> Self {
        ObjectErr::DyckLanguageErr { target_value: s.target_value, ordering: s.ordering }
    }
}
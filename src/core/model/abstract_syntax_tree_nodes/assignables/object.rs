use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::types::ty::Type;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Object {
    pub variables: Vec<Variable<':', ','>>,
    pub ty: Type
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}}}", self.variables.iter().map(|var| format!("{}", var)).collect::<Vec<String>>().join(", "))
    }
}
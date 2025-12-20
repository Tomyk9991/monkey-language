use std::fmt::{Display, Formatter};
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Array {
    pub values: Vec<Assignable>,
}

impl Display for Array {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let a = self.values.iter().map(|a| format!("{}", a)).collect::<Vec<_>>();
        write!(f, "[{}]", a.join(", "))
    }
}
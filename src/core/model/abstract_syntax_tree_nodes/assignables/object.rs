use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::types::ty::Type;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Object {
    pub fields: Vec<Variable<':', ','>>,
    pub ty: Type
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ident = f.width().unwrap_or(0);

        writeln!(f, "{}{{", " ".repeat(ident))?;
        for (index, field) in self.fields.iter().enumerate() {
            if index == self.fields.len() - 1 {
                writeln!(f, "{}{:width$}", " ".repeat(ident), field, width = ident + 4)?;
            } else {
                writeln!(f, "{}{:width$},", " ".repeat(ident), field, width = ident + 4)?;
            }
        }

        write!(f, "{}}}", " ".repeat(ident))?;

        Ok(())
    }
}
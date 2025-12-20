use crate::core::lexer::token_with_span::FilePosition;
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable};
use crate::core::model::abstract_syntax_tree_nodes::identifier::IdentifierError;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct MethodCall {
    pub identifier: LValue,
    pub arguments: Vec<Assignable>,
    pub file_position: FilePosition,
}

#[derive(Debug)]
pub enum MethodCallErr {
    IdentifierErr(IdentifierError),
    DyckLanguageErr { target_value: String, ordering: Ordering },
}

impl Display for MethodCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}({})",
               " ".repeat(f.width().unwrap_or(0)), 
               self.identifier, 
               self.arguments
                   .iter()
                   .map(|ass| format!("{}", ass))
                   .collect::<Vec<String>>()
                   .join(", "))
    }
}
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use crate::core::io::code_line::CodeLine;
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::model::abstract_syntax_tree_nodes::identifier::{Identifier, IdentifierError};
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::scanner::errors::EmptyIteratorErr;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct MethodCall {
    pub identifier: LValue,
    pub arguments: Vec<Assignable>,
    pub code_line: CodeLine,
}

#[derive(Debug)]
pub enum MethodCallErr {
    PatternNotMatched { target_value: String },
    IdentifierErr(IdentifierError),
    DyckLanguageErr { target_value: String, ordering: Ordering },
    AssignableErr(AssignableError),
    EmptyIterator(EmptyIteratorErr),
}


impl Display for MethodCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.identifier, self.arguments
            .iter()
            .map(|ass| format!("{}", ass))
            .collect::<Vec<String>>()
            .join(", "))
    }
}
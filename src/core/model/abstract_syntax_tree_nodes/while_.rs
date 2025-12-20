use crate::core::lexer::token_with_span::FilePosition;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable};
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct While {
    pub condition: Assignable,
    pub stack: Vec<AbstractSyntaxTreeNode>,
    pub file_position: FilePosition
}

impl Display for While {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}while ({}) {{", " ".repeat(f.width().unwrap_or(0)), self.condition)?;

        for a in &self.stack {
            write!(f, "\n{:width$}{}", "", a, width = f.width().unwrap_or(0) + 4)?;
        }
        
        write!(f, "\n{}}}", " ".repeat(f.width().unwrap_or(0)))?;
        Ok(())
    }
}
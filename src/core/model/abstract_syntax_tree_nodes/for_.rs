use crate::core::lexer::token_with_span::FilePosition;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable};
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct For {
    pub initialization: Variable<'=', ';'>,
    pub condition: Assignable,
    pub update: Variable<'=', ';'>,
    pub stack: Vec<AbstractSyntaxTreeNode>,
    pub file_position: FilePosition,
}

impl Display for For {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}for ({} {}; {}) {{", " ".repeat(f.width().unwrap_or(0)), self.initialization, self.condition, self.update)?;
        for a in &self.stack {
            write!(f, "\n{:width$}{}", "", a, width = f.width().unwrap_or(0) + 4)?;
        }
        write!(f, "\n{}}}", " ".repeat(f.width().unwrap_or(0)))?;
        
        Ok(())
    }
}
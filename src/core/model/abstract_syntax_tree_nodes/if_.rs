use crate::core::lexer::token_with_span::FilePosition;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable};
use std::fmt::{Display, Formatter};

/// AST node for if definition.
/// # Pattern
/// - `if (condition) {Body}`
/// - `if (condition) {Body} else {Body}`
#[derive(Debug, PartialEq, Clone, Default)]
pub struct If {
    pub condition: Assignable,
    pub if_stack: Vec<AbstractSyntaxTreeNode>,
    pub else_stack: Option<Vec<AbstractSyntaxTreeNode>>,
    pub file_position: FilePosition,
}

impl Display for If {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ident: usize = f.width().unwrap_or(0);
        writeln!(f, "{}if ({}) {{", " ".repeat(ident), self.condition)?;

        for a in &self.if_stack {
            writeln!(f, "{:width$}{}", "", a, width = ident + 4)?;
        }
        write!(f, "{}}}", " ".repeat(ident))?;
        if let Some(else_stack) = &self.else_stack {
            writeln!(f, " else {{")?;
            for a in else_stack {
                writeln!(f, "{:width$}{}", "", a, width = ident + 4)?;
            }
            writeln!(f, "{}}}", " ".repeat(ident))?;
        }


        Ok(())
    }
}


use std::fmt::{Display, Formatter};
use crate::core::lexer::token_with_span::FilePosition;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::scanner::errors::EmptyIteratorErr;
use crate::core::scanner::scope::ScopeError;

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

#[derive(Debug)]
pub enum IfError {
    PatternNotMatched { target_value: String },
    AssignableErr(AssignableError),
    ScopeErrorErr(ScopeError),
    EmptyIterator(EmptyIteratorErr),
}

impl std::error::Error for IfError {}

impl Display for IfError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            IfError::PatternNotMatched { target_value }
            => format!("Pattern not matched for: `{target_value}`\n\t if(condition) {{ }}"),
            IfError::AssignableErr(a) => a.to_string(),
            IfError::ScopeErrorErr(a) => a.to_string(),
            IfError::EmptyIterator(e) => e.to_string(),
        })
    }
}

impl Display for If {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        buffer.push_str("if (");
        buffer.push_str(&self.condition.to_string());
        buffer.push_str(") {\n");

        for a in &self.if_stack {
            buffer.push_str(&format!("    {};\n", a));
        }
        buffer.push_str("}");
        if let Some(else_stack) = &self.else_stack {
            buffer.push_str(" else {\n");
            for a in else_stack {
                buffer.push_str(&format!("    {};\n", a));
            }
            buffer.push_str("}\n");
        }
        write!(f, "{buffer}")
    }
}


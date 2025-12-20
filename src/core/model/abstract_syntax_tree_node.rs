use std::fmt::{Display, Formatter};
use crate::core::lexer::token_with_span::FilePosition;
use crate::core::model::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use crate::core::model::abstract_syntax_tree_nodes::for_::For;
use crate::core::model::abstract_syntax_tree_nodes::if_::If;
use crate::core::model::abstract_syntax_tree_nodes::import::Import;
use crate::core::model::abstract_syntax_tree_nodes::method_definition::MethodDefinition;
use crate::core::model::abstract_syntax_tree_nodes::ret::Return;
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::abstract_syntax_tree_nodes::while_::While;

/// An abstract syntax tree node is a piece of code that is used to represent atomic elements of a program.
#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum AbstractSyntaxTreeNode {
    Variable(Variable<'=', ';'>),
    MethodCall(MethodCall),
    MethodDefinition(MethodDefinition),
    Import(Import),
    Return(Return),
    If(If),
    For(For),
    While(While),
}

impl AbstractSyntaxTreeNode {
    pub fn file_position(&self) -> FilePosition {
        match self {
            AbstractSyntaxTreeNode::Variable(v) => v.file_position.clone(),
            AbstractSyntaxTreeNode::MethodCall(m) => m.file_position.clone(),
            AbstractSyntaxTreeNode::MethodDefinition(m) => m.file_position.clone(),
            AbstractSyntaxTreeNode::If(m) => m.file_position.clone(),
            AbstractSyntaxTreeNode::Import(m) => m.file_position.clone(),
            AbstractSyntaxTreeNode::Return(m) => m.file_position.clone(),
            AbstractSyntaxTreeNode::While(a) => a.file_position.clone(),
            AbstractSyntaxTreeNode::For(m) => m.file_position.clone(),
        }
    }
}


impl Default for AbstractSyntaxTreeNode {
    fn default() -> Self {
        AbstractSyntaxTreeNode::If(If::default())
    }
}

impl Display for AbstractSyntaxTreeNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let width = f.width().unwrap_or(0);
        write!(f, "{}", match self {
            AbstractSyntaxTreeNode::Variable(node) => format!("{:width$}", node),
            AbstractSyntaxTreeNode::MethodCall(node) => format!("{:width$}", node),
            AbstractSyntaxTreeNode::MethodDefinition(node) => format!("{:width$}", node),
            AbstractSyntaxTreeNode::If(node) => format!("{:width$}", node),
            AbstractSyntaxTreeNode::Import(node) => format!("{:width$}", node),
            AbstractSyntaxTreeNode::Return(node) => format!("{:width$}", node),
            AbstractSyntaxTreeNode::While(node) => format!("{:width$}", node),
            AbstractSyntaxTreeNode::For(node) => format!("{:width$}", node),
        })
    }
}
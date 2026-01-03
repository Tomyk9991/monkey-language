use std::fmt::{Display, Formatter};
use crate::core::lexer::token_with_span::FilePosition;
use crate::core::model::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use crate::core::model::abstract_syntax_tree_nodes::for_::For;
use crate::core::model::abstract_syntax_tree_nodes::if_::If;
use crate::core::model::abstract_syntax_tree_nodes::import::Import;
use crate::core::model::abstract_syntax_tree_nodes::method_definition::MethodDefinition;
use crate::core::model::abstract_syntax_tree_nodes::ret::Return;
use crate::core::model::abstract_syntax_tree_nodes::struct_::Struct;
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::abstract_syntax_tree_nodes::while_::While;

/// An abstract syntax tree node is a piece of code that is used to represent atomic elements of a program.
#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum AbstractSyntaxTreeNode {
    Variable(Variable<'=', ';'>),
    StructDefinition(Struct),
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
            AbstractSyntaxTreeNode::Variable(node) => node.file_position.clone(),
            AbstractSyntaxTreeNode::MethodCall(node) => node.file_position.clone(),
            AbstractSyntaxTreeNode::StructDefinition(node) => node.file_position.clone(),
            AbstractSyntaxTreeNode::MethodDefinition(node) => node.file_position.clone(),
            AbstractSyntaxTreeNode::If(node) => node.file_position.clone(),
            AbstractSyntaxTreeNode::Import(node) => node.file_position.clone(),
            AbstractSyntaxTreeNode::Return(node) => node.file_position.clone(),
            AbstractSyntaxTreeNode::While(node) => node.file_position.clone(),
            AbstractSyntaxTreeNode::For(node) => node.file_position.clone(),
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
        let ident = f.width().unwrap_or(0);
        match self {
            AbstractSyntaxTreeNode::Variable(node) => write!(f, "{:width$}", node, width = ident),
            AbstractSyntaxTreeNode::MethodCall(node) => write!(f, "{:width$}", node, width = ident),
            AbstractSyntaxTreeNode::MethodDefinition(node) => write!(f, "{:width$}", node, width = ident),
            AbstractSyntaxTreeNode::If(node) => write!(f, "{:width$}", node, width = ident),
            AbstractSyntaxTreeNode::Import(node) => write!(f, "{:width$}", node, width = ident),
            AbstractSyntaxTreeNode::Return(node) => write!(f, "{:width$}", node, width = ident),
            AbstractSyntaxTreeNode::While(node) => write!(f, "{:width$}", node, width = ident),
            AbstractSyntaxTreeNode::For(node) => write!(f, "{:width$}", node, width = ident),
            AbstractSyntaxTreeNode::StructDefinition(node) => write!(f, "{:width$}", node, width = ident),
        }
    }
}
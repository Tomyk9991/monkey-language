use std::fmt::{Display, Formatter};
use crate::core::model::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use crate::core::model::abstract_syntax_tree_nodes::for_::For;
use crate::core::model::abstract_syntax_tree_nodes::if_::If;
use crate::core::model::abstract_syntax_tree_nodes::import::Import;
use crate::core::model::abstract_syntax_tree_nodes::method_definition::MethodDefinition;
use crate::core::model::abstract_syntax_tree_nodes::ret::Return;
use crate::core::model::abstract_syntax_tree_nodes::scope_ending::ScopeEnding;
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::abstract_syntax_tree_nodes::while_::While;

/// An abstract syntax tree node is a piece of code that is used to represent atomic elements of a program.
#[derive(Debug, PartialEq, Clone)]
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
    pub fn file_position(&self) -> &crate::core::lexer::token_with_span::FilePosition {
        match self {
            AbstractSyntaxTreeNode::Variable(v) => &v.file_position,
            AbstractSyntaxTreeNode::MethodCall(m) => &m.file_position,
            AbstractSyntaxTreeNode::MethodDefinition(m) => &m.file_position,
            AbstractSyntaxTreeNode::If(m) => &m.file_position,
            AbstractSyntaxTreeNode::Import(m) => &m.file_position,
            AbstractSyntaxTreeNode::Return(m) => &m.file_position,
            AbstractSyntaxTreeNode::While(a) => &a.file_position,
            AbstractSyntaxTreeNode::For(m) => &m.file_position,
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
        write!(f, "{}", match self {
            AbstractSyntaxTreeNode::Variable(v) => format!("{}", v),
            AbstractSyntaxTreeNode::MethodCall(m) => format!("{}", m),
            AbstractSyntaxTreeNode::MethodDefinition(m) => format!("{}", m),
            AbstractSyntaxTreeNode::If(m) => format!("{}", m),
            AbstractSyntaxTreeNode::Import(m) => format!("{}", m),
            AbstractSyntaxTreeNode::Return(m) => format!("{}", m),
            AbstractSyntaxTreeNode::While(a) => format!("{}", a),
            AbstractSyntaxTreeNode::For(m) => format!("{}", m),
        })
    }
}
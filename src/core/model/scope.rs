use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;

#[derive(Clone, Default)]
pub struct Scope {
    pub ast_nodes: Vec<AbstractSyntaxTreeNode>
}
use crate::core::parser::scope::PatternNotMatchedError;

pub mod parser;
pub mod scope;
pub mod abstract_syntax_tree_node;
pub mod abstract_syntax_tree_nodes;
pub mod errors;
pub mod static_type_context;
pub mod types;
pub mod scope_iterator;
pub mod utils;
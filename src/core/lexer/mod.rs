use std::iter::Peekable;
use std::slice::Iter;
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::scope::PatternNotMatchedError;

pub mod parser;
pub mod scope;
pub mod abstract_syntax_tree_node;
pub mod abstract_syntax_tree_nodes;
pub mod errors;
pub mod static_type_context;
pub mod types;


pub type Lines<'a> = Peekable<Iter<'a, CodeLine>>;
pub trait TryParse {
    type Output;
    type Err: PatternNotMatchedError;

    fn try_parse(code_lines_iterator: &mut Lines<'_>) -> anyhow::Result<Self::Output, Self::Err>;
}
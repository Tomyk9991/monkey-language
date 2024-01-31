use std::iter::Peekable;
use std::slice::Iter;
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::scope::PatternNotMatchedError;

pub mod tokenizer;
pub mod scope;
pub mod token;
pub mod tokens;
pub mod levenshtein_distance;
pub mod errors;
pub mod static_type_context;
pub mod types;

pub trait TryParse {
    type Output;
    type Err: PatternNotMatchedError;

    fn try_parse(code_lines_iterator: &mut Peekable<Iter<CodeLine>>) -> anyhow::Result<Self::Output, Self::Err>;
}

#[allow(unused)]
#[derive(Debug)]
pub enum Visibility {
    Public,
    Private
}
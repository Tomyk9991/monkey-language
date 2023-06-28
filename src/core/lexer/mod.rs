use std::iter::Peekable;
use std::slice::Iter;
use crate::core::io::code_line::CodeLine;

pub mod tokenizer;
pub mod scope;
pub mod token;
pub mod tokens;
pub mod levenshtein_distance;
pub mod errors;

pub trait TryParse {
    type Output;
    type Err;

    fn try_parse(code_lines_iterator: &mut Peekable<Iter<CodeLine>>) -> anyhow::Result<Self::Output, Self::Err>;
}

#[allow(unused)]
#[derive(Debug)]
pub enum Visibility {
    Public,
    Private
}
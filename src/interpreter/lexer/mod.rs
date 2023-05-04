use crate::interpreter::io::code_line::CodeLine;

pub mod tokenizer;
pub mod scope;
pub mod token;
pub mod tokens;
pub mod levenshtein_distance;

pub trait TryParse {
    type Output;
    type Err;

    fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self::Output, Self::Err>;
}

#[allow(unused)]
#[derive(Debug)]
pub enum Visibility {
    Public,
    Private
}
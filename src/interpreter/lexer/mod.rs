use crate::interpreter::io::code_line::CodeLine;

pub mod lexer;
pub mod scope;
pub mod token;
pub mod tokens;

pub trait TryParse {
    type Output;

    fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self::Output>;
}
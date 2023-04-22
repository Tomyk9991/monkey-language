use crate::interpreter::io::code_line::CodeLine;
use crate::interpreter::lexer::token::Token;
use crate::interpreter::lexer::TryParse;

pub struct Scope {
    pub tokens: Vec<Token>
}

impl TryParse for Scope {
    type Output = Token;

    fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self::Output> {
        // if let Some(variable_token) =
        Ok(Token::VariableToken)
    }
}
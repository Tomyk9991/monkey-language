use crate::interpreter::io::code_line::CodeLine;
use crate::interpreter::lexer::token::Token;
use crate::interpreter::lexer::tokens::variable_token::VariableToken;
use crate::interpreter::lexer::TryParse;

#[derive(Debug)]
pub struct Scope {
    pub tokens: Vec<Token>
}

impl TryParse for Scope {
    type Output = Token;

    fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self::Output> {
        if let Ok(variable_token) = VariableToken::try_from(code_line) {
            return Ok(Token::VariableToken(variable_token));
        }
        
        return Ok(Token::None)
    }
}
use crate::interpreter::io::code_line::CodeLine;
use crate::interpreter::lexer::levenshtein_distance::{PatternedLevenshteinDistance, PatternedLevenshteinString};
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

        let variable_pattern = PatternedLevenshteinString::default()
            .insert(PatternedLevenshteinString::ignore())
            .insert("=")
            .insert(PatternedLevenshteinString::ignore())
            .insert(";");

        let variable_token_distance = VariableToken::distance(
            PatternedLevenshteinString::match_to(&code_line.line, &variable_pattern),
            variable_pattern
        );

        println!("Variable distance: {}", variable_token_distance);

        if let Ok(variable_token) = VariableToken::try_from(code_line) {
            return Ok(Token::VariableToken(variable_token));
        }


        return Ok(Token::None)
    }
}
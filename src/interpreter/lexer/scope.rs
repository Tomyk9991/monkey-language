use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::interpreter::io::code_line::CodeLine;
use crate::interpreter::lexer::levenshtein_distance::{PatternedLevenshteinDistance, PatternedLevenshteinString};
use crate::interpreter::lexer::token::Token;
use crate::interpreter::lexer::tokens::assignable_tokens::method_call_token::MethodCallToken;
use crate::interpreter::lexer::tokens::variable_token::VariableToken;
use crate::interpreter::lexer::TryParse;

pub struct Scope {
    pub tokens: Vec<Token>,
}

pub enum ScopeError {
    ParsingError { message: String },
}

impl Debug for ScopeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Display for ScopeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ScopeError::ParsingError { message } => message
        })
    }
}


impl Error for ScopeError {}


impl Debug for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scope: [\n{}]", self.tokens.iter().map(|token| format!("\t{:?}\n", token)).collect::<String>())
    }
}

impl TryParse for Scope {
    type Output = Token;
    type Err = ScopeError;

    fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self::Output, ScopeError> {
        let mut pattern_distances: Vec<(usize, Box<dyn Error>)> = vec![];

        match VariableToken::try_parse(code_line) {
            Ok(variable_token) => return Ok(Token::VariableToken(variable_token)),
            Err(err) => pattern_distances.push((VariableToken::distance_from_code_line(code_line), Box::new(err)))
        }

        match MethodCallToken::try_parse(code_line) {
            Ok(method_token) => return Ok(Token::MethodToken(method_token)),
            Err(err) => pattern_distances.push((MethodCallToken::distance_from_code_line(code_line), Box::new(err)))
        }

        pattern_distances.sort_by(|(nearest_a, _), (nearest_b, _)| (*nearest_a).cmp(nearest_b));


        if let Some((nearest_pattern, err)) = pattern_distances.first() {
            return Err(ScopeError::ParsingError {
                message: format!("Codeline: {} with distance: {}\n\t{}", code_line.actual_line_number, nearest_pattern, err)
            });
        }

        return Ok(Token::None);
    }
}
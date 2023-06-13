use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;
use crate::interpreter::io::code_line::CodeLine;
use crate::interpreter::lexer::levenshtein_distance::PatternedLevenshteinDistance;
use crate::interpreter::lexer::token::Token;
use crate::interpreter::lexer::tokens::assignable_tokens::method_call_token::MethodCallToken;
use crate::interpreter::lexer::tokens::if_definition::IfDefinition;
use crate::interpreter::lexer::tokens::method_definition::MethodDefinition;
use crate::interpreter::lexer::tokens::scope_ending::ScopeEnding;
use crate::interpreter::lexer::tokens::variable_token::VariableToken;
use crate::interpreter::lexer::TryParse;

pub struct Scope {
    pub tokens: Vec<Token>,
}

pub enum ScopeError {
    ParsingError { message: String },
    EmptyIterator
}

impl Debug for ScopeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for ScopeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ScopeError::ParsingError { message } => message,
            ScopeError::EmptyIterator => "Iterator is empty"
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

    fn try_parse(code_lines: &mut Peekable<Iter<CodeLine>>) -> anyhow::Result<Self::Output, ScopeError> {
        let mut pattern_distances: Vec<(usize, Box<dyn Error>)> = vec![];

        let code_line = *code_lines.peek().ok_or(ScopeError::EmptyIterator)?;

        match VariableToken::try_parse(code_line) {
            Ok(variable_token) => {
                code_lines.next();
                return Ok(Token::Variable(variable_token))
            },
            Err(err) => pattern_distances.push((
                VariableToken::<'=', ';'>::distance_from_code_line(code_line), Box::new(err))
            )
        }

        match MethodCallToken::try_parse(code_line) {
            Ok(method_token) => {
                code_lines.next();
                return Ok(Token::MethodCall(method_token))
            },
            Err(err) => pattern_distances.push((
                MethodCallToken::distance_from_code_line(code_line), Box::new(err))
            )
        }

        match ScopeEnding::try_parse(code_line) {
            Ok(scope_ending) => {
                code_lines.next();
                return Ok(Token::ScopeClosing(scope_ending))
            }
            Err(err) => pattern_distances.push((
                ScopeEnding::distance_from_code_line(code_line), Box::new(err))
            )
        }

        match IfDefinition::try_parse(code_lines) {
            Ok(if_token) => {
                return Ok(Token::IfDefinition(if_token))
            },
            Err(err) => pattern_distances.push((
                IfDefinition::distance_from_code_line(code_line), Box::new(err))
            )
        }

        match MethodDefinition::try_parse(code_lines) {
            Ok(method_token) => {
                return Ok(Token::MethodDefinition(method_token))
            },
            Err(err) => pattern_distances.push((
                MethodDefinition::distance_from_code_line(code_line), Box::new(err))
            )
        }


        pattern_distances.sort_by(|(nearest_a, _), (nearest_b, _)| (*nearest_a).cmp(nearest_b));


        if let Some((nearest_pattern, err)) = pattern_distances.first() {
            code_lines.next();

            return Err(ScopeError::ParsingError {
                message: format!("Code line: {:?} with distance: {}\n\t{}", code_line.actual_line_number, nearest_pattern, err)
            });
        }

        return Err(ScopeError::ParsingError {
            message: format!("Unexpected token: {:?}: {}", code_line.actual_line_number, code_line.line)
        });
    }
}
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;
use crate::interpreter::io::code_line::CodeLine;
use crate::interpreter::lexer::errors::EmptyIteratorErr;
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
    EmptyIterator(EmptyIteratorErr)
}

impl Debug for ScopeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for ScopeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ScopeError::ParsingError { message } => message.to_string(),
            ScopeError::EmptyIterator(e) => e.to_string()
        })
    }
}


impl Error for ScopeError {}

macro_rules! token_expand {
    ($code_lines_iterator: ident, $pattern_distances: ident, $(($token_implementation:ty, $token_type:ident, $iterate_next:ident)),*) => {
        $(
            match <$token_implementation as TryParse>::try_parse($code_lines_iterator) {
                Ok(t) => {
                    if $iterate_next {
                        $code_lines_iterator.next();
                    }
                    return Ok(Token::$token_type(t))
                },
                Err(err) => {
                    let c = *$code_lines_iterator.peek().ok_or(ScopeError::EmptyIterator(EmptyIteratorErr::default()))?;
                    $pattern_distances.push((<$token_implementation>::distance_from_code_line(c), Box::new(err)))
                }
            }
        )*
    }
}


impl Debug for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scope: [\n{}]", self.tokens.iter().map(|token| format!("\t{:?}\n", token)).collect::<String>())
    }
}

impl TryParse for Scope {
    type Output = Token;
    type Err = ScopeError;

    fn try_parse(code_lines_iterator: &mut Peekable<Iter<CodeLine>>) -> anyhow::Result<Self::Output, ScopeError> {
        let mut pattern_distances: Vec<(usize, Box<dyn Error>)> = vec![];

        let code_line = *code_lines_iterator.peek().ok_or(ScopeError::EmptyIterator(EmptyIteratorErr::default()))?;

        token_expand!(code_lines_iterator, pattern_distances,
            (VariableToken::<'=', ';'>, Variable,           true),
            (MethodCallToken,           MethodCall,         true),
            (ScopeEnding,               ScopeClosing,       true),
            (IfDefinition,              IfDefinition,       true),
            (MethodDefinition,          MethodDefinition,   false)
        );


        pattern_distances.sort_by(|(nearest_a, _), (nearest_b, _)| (*nearest_a).cmp(nearest_b));

        if let Some((nearest_pattern, err)) = pattern_distances.first() {
            code_lines_iterator.next();

            return Err(ScopeError::ParsingError {
                message: format!("Code line: {:?} with distance: {}\n\t{}", code_line.actual_line_number, nearest_pattern, err)
            });
        }

        Err(ScopeError::ParsingError {
            message: format!("Unexpected token: {:?}: {}", code_line.actual_line_number, code_line.line)
        })
    }
}
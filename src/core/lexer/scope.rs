use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::errors::EmptyIteratorErr;
use crate::core::lexer::levenshtein_distance::PatternedLevenshteinDistance;
use crate::core::lexer::token::Token;
use crate::core::lexer::tokenizer::{Lexer, StaticTypeContext};
use crate::core::lexer::tokens::assignable_tokens::method_call_token::MethodCallToken;
use crate::core::lexer::tokens::if_definition::IfDefinition;
use crate::core::lexer::tokens::method_definition::MethodDefinition;
use crate::core::lexer::tokens::name_token::NameToken;
use crate::core::lexer::tokens::scope_ending::ScopeEnding;
use crate::core::lexer::tokens::variable_token::VariableToken;
use crate::core::lexer::TryParse;
use crate::core::lexer::type_token::InferTypeError;
use crate::core::lexer::tokens::import::ImportToken;

/// Tokens inside scope
pub struct Scope {
    pub tokens: Vec<Token>,
    pub extern_methods: Vec<MethodDefinition>
}

impl Scope {
    pub fn infer_type(stack: &mut Vec<Token>, type_context: &mut StaticTypeContext, method_names: &[NameToken]) -> Result<(), InferTypeError> {
        let variables_len = type_context.len();

        let scoped_checker = StaticTypeContext::type_context(stack);
        type_context.merge(scoped_checker);

        Lexer::infer_types(stack, type_context, method_names)?;

        let amount_pop = type_context.len() - variables_len;

        for _ in 0..amount_pop {
            let _ = type_context.pop();
        }

        Ok(())
    }
}

pub enum ScopeError {
    ParsingError { message: String },
    InferredError(InferTypeError),
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
            ScopeError::EmptyIterator(e) => e.to_string(),
            ScopeError::InferredError(e) => e.to_string()
        })
    }
}


impl Error for ScopeError {}

macro_rules! token_expand {
    ($code_lines_iterator: ident, $pattern_distances: ident, $(($token_implementation:ty, $token_type:ident, $iterates_over_same_scope:ident)),*) => {
        $(
            match <$token_implementation as TryParse>::try_parse($code_lines_iterator) {
                Ok(t) => {
                    if $iterates_over_same_scope {
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
        write!(f, "Scope: [\n{}]\n Extern Methods: [\n{}]", self.tokens
            .iter()
            .map(|token| format!("\t{:?}\n", token)).collect::<String>(),
            self.extern_methods
                .iter()
                .map(|token| format!("\t{:?}\n", token)).collect::<String>()
        )
    }
}

impl Display for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scope: [\n{}]", self.tokens
            .iter()
            .map(|token| format!("\t{}\n", token)).collect::<String>())
    }
}

impl From<InferTypeError> for ScopeError {
    fn from(value: InferTypeError) -> Self {
        ScopeError::InferredError(value)
    }
}


impl TryParse for Scope {
    type Output = Token;
    type Err = ScopeError;

    /// Tries to parse the code lines into a scope using a peekable iterator and a greedy algorithm
    /// # Returns
    /// * Ok(Token) if the code lines iterator can be parsed into a scope
    /// * Err(ScopeError) if the code lines iterator cannot be parsed into a scope
    fn try_parse(code_lines_iterator: &mut Peekable<Iter<CodeLine>>) -> anyhow::Result<Self::Output, ScopeError> {
        let mut pattern_distances: Vec<(usize, Box<dyn Error>)> = vec![];
        let code_line = *code_lines_iterator.peek().ok_or(ScopeError::EmptyIterator(EmptyIteratorErr))?;

        token_expand!(code_lines_iterator, pattern_distances,
            (ImportToken,               Import,             true),
            (VariableToken::<'=', ';'>, Variable,           true),
            (MethodCallToken,           MethodCall,         true),
            (ScopeEnding,               ScopeClosing,       true),
            (IfDefinition,              IfDefinition,       false),
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
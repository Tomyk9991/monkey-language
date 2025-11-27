use std::fmt::Debug;
use crate::core::lexer::collect_tokens_until_scope_close::CollectTokensFromUntil;
use crate::core::lexer::parse::{Parse, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::types::ty::Type;

/// Enum representing different types of matches that can occur during token parsing.
#[derive(Debug)]
pub enum Match<T: Parse> {
    /// A single token match.
    Token(Token),
    /// A parsed result match.
    Parse(ParseResult<T>),
    /// A match that collects a specified number of tokens without any constraint.
    Collect(usize), // amount of tokens without any constraint
    /// An error match.
    Error,
}


/// Enum representing the result of a match operation.
#[derive(Debug)]
pub enum MatchResult<T: Default + Clone> {
    /// A parsed result.
    Parse(ParseResult<T>),
    /// A collection of tokens with spans.
    Collect(Vec<TokenWithSpan>),
}

pub trait TokenMatchSingleReturn {
    fn matches<T: Default + Parse + Clone + Debug>(&self, pattern: &[Match<T>]) -> Option<MatchResult<T>>;
}

impl<T: Parse> From<Token> for Match<T> {
    fn from(value: Token) -> Self {
        Match::Token(value)
    }
}


impl TokenMatchSingleReturn for &[TokenWithSpan] {
    fn matches<T: Default + Parse + Clone + Debug>(&self, pattern: &[Match<T>]) -> Option<MatchResult<T>> {
        let mut pattern_iter = pattern.iter();
        let mut tokens_iter = self.iter();

        let mut collected = Vec::new();
        let mut parse_pattern = None;

        // after finding a collect or parse pattern, we should not return immediately, but continue matching the rest of the pattern and then return the collected tokens if all patterns matched

        let is_collection_pattern = pattern.iter().any(|p| matches!(p, Match::Collect(_)));
        let is_parse_pattern = pattern.iter().any(|p| matches!(p, Match::Parse(_)));

        // cannot be both
        debug_assert_ne!(is_collection_pattern && is_parse_pattern, true, "Cannot have both collection and parse patterns in the same match pattern");

        loop {
            let pattern = pattern_iter.next();
            let token = tokens_iter.next();

            if let (Some(pattern), Some(token)) = (pattern, token) {
                match pattern {
                    Match::Token(t) => {
                        if &token.token != t {
                            return None
                        }
                    },
                    Match::Collect(consume) => {
                        collected.push(token.clone());
                        for _ in 0..*consume - 1 {
                            let token = tokens_iter.next();
                            if let Some(token) = token {
                                collected.push(token.clone());
                            }
                        }

                        if collected.is_empty() {
                            return None;
                        }
                    }
                    Match::Error => return None,
                    Match::Parse(value) => {
                        parse_pattern = Some(MatchResult::Parse(value.clone()));
                        let consume = value.consumed;

                        for _ in 0..consume - 1 {
                            let token = tokens_iter.next();
                            if let Some(token) = token {
                                collected.push(token.clone());
                            }
                        }
                    }
                }
            } else {
                if is_collection_pattern && !collected.is_empty() {
                    return Some(MatchResult::Collect(collected))
                }

                if let Some(parse_pattern) = parse_pattern {
                    if is_parse_pattern {
                        return Some(parse_pattern);
                    }
                }

                break;
            }
        }

        None
    }
}


impl From<ParseResult<Type>> for Match<Type> {
    fn from(value: ParseResult<Type>) -> Self {
        Match::Parse(value)
    }
}

impl From<ParseResult<LValue>> for Match<LValue> {
    fn from(value: ParseResult<LValue>) -> Self {
        Match::Parse(value)
    }
}

impl<const OPEN: char, const CLOSE: char> From<ParseResult<CollectTokensFromUntil<OPEN, CLOSE>>> for Match<CollectTokensFromUntil<OPEN, CLOSE>> {
    fn from(value: ParseResult<CollectTokensFromUntil<OPEN, CLOSE>>) -> Self {
        Match::Collect(value.consumed)
    }
}
impl From<ParseResult<Variable<'=', ';'>>> for Match<Variable<'=', ';'>> {
    fn from(value: ParseResult<Variable<'=', ';'>>) -> Self {
        Match::Parse(value)
    }
}

impl From<ParseResult<Assignable>> for Match<Assignable> {
    fn from(value: ParseResult<Assignable>) -> Self {
        Match::Parse(value)
    }
}
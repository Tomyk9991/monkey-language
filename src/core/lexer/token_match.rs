use crate::core::lexer::collect_tokens_until_scope_close::CollectTokensUntilScopeClose;
use crate::core::lexer::parse::{Parse, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::scanner::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::scanner::abstract_syntax_tree_nodes::l_value::LValue;

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
    fn matches<T: Default + Parse + Clone>(&self, pattern: &[Match<T>]) -> Option<MatchResult<T>>;
}

impl<T: Parse> From<Token> for Match<T> {
    fn from(value: Token) -> Self {
        Match::Token(value)
    }
}


impl TokenMatchSingleReturn for &[TokenWithSpan] {
    fn matches<T: Default + Parse + Clone>(&self, pattern: &[Match<T>]) -> Option<MatchResult<T>> {
        let mut pattern_iter = pattern.iter();
        let mut tokens_iter = self.iter();

        let mut collected = Vec::new();

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

                        return Some(MatchResult::Collect(collected))
                    }
                    Match::Error => return None,
                    Match::Parse(value) => {
                        return Some(MatchResult::Parse(value.clone()));
                    }
                }
            } else {
                break;
            }
        }

        None
    }
}


impl From<ParseResult<LValue>> for Match<LValue> {
    fn from(value: ParseResult<LValue>) -> Self {
        Match::Parse(value)
    }
}

impl From<ParseResult<CollectTokensUntilScopeClose>> for Match<CollectTokensUntilScopeClose> {
    fn from(value: ParseResult<CollectTokensUntilScopeClose>) -> Self {
        Match::Collect(value.consumed)
    }
}

impl From<ParseResult<Assignable>> for Match<Assignable> {
    fn from(value: ParseResult<Assignable>) -> Self {
        Match::Parse(value)
    }
}
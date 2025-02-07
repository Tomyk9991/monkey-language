use std::str::FromStr;
use crate::core::constants::KEYWORDS;
use crate::core::lexer::token::Token;
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::float::FloatAST;
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::integer::IntegerAST;
use crate::core::scanner::abstract_syntax_tree_nodes::identifier::{IdentifierErr};

#[derive(Debug, PartialEq)]
pub struct TokenInformation {
    pub token_length: Option<usize>,
    pub token: Token,
}

impl TokenInformation {
    pub fn matches(&self, target: &str) -> bool {
        match self.token.literal() {
            Some(literal) => target == literal,
            None => match self.token {
                Token::Numbers(_) => IntegerAST::from_str(target).is_ok(),
                Token::Float(_) => FloatAST::from_str(target).is_ok(),
                Token::Literal(_) => true, // accept just every string as a literal
                _ => false,
            }
        }
    }
    pub fn from_str(s: &str, allow_reserved: bool) -> Result<TokenInformation, IdentifierErr> {
        if !allow_reserved && KEYWORDS.iter().any(|keyword| keyword.to_lowercase() == s.to_lowercase()) {
            return Err(IdentifierErr::KeywordReserved(s.to_string()));
        }

        if !lazy_regex::regex_is_match!(r"^[a-zA-Z_$][a-zA-Z_$0-9]*$", s) {
            return Err(IdentifierErr::UnmatchedRegex {
                target_value: s.to_string(),
            });
        }

        Ok(TokenInformation {
            token_length: Some(s.len()),
            token: Token::Literal(s.to_string()),
        })
    }
}

impl From<Token> for TokenInformation {
    fn from(value: Token) -> Self {
        TokenInformation {
            token_length: match value.literal() {
                Some(literal) => Some(literal.len()),
                None => None
            },
            token: value,
        }
    }
}

pub struct TokenInformationIterator {
    started: bool,
    index: Token,
}

impl Iterator for TokenInformationIterator {
    type Item = TokenInformation;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            return Some(TokenInformation::from(Token::If));
        }

        let next_token = match self.index {
            Token::If => Token::Else,
            Token::Else => Token::Let,
            Token::Let => Token::Mut,
            Token::Mut => Token::Module,
            Token::Module => Token::While,
            Token::While => Token::For,
            Token::For => Token::ParenthesisOpen,
            Token::ParenthesisOpen => Token::ParenthesisClose,
            Token::ParenthesisClose => Token::CurlyBraceOpen,
            Token::CurlyBraceOpen => Token::CurlyBraceClose,
            Token::CurlyBraceClose => Token::SquareBracketOpen,
            Token::SquareBracketOpen => Token::SquareBracketClose,
            Token::SquareBracketClose => Token::SemiColon,
            Token::SemiColon => Token::Comma,
            Token::Comma => Token::Colon,
            Token::Colon => Token::Dot,
            Token::Dot => Token::Plus,
            Token::Plus => Token::Minus,
            Token::Minus => Token::Multiply,
            Token::Multiply => Token::Divide,
            Token::Divide => Token::Modulo,
            Token::Modulo => Token::LogicalAnd,
            Token::LogicalAnd => Token::LogicalOr,
            Token::LogicalOr => Token::LogicalNot,
            Token::LogicalNot => Token::Pipe,
            Token::Pipe => Token::Xor,
            Token::Xor => Token::GreaterThanEquals,
            Token::GreaterThanEquals => Token::LessThanEquals,
            Token::LessThanEquals => Token::LeftShift,
            Token::LeftShift => Token::RightShift,
            Token::RightShift => Token::LessThan,
            Token::LessThan => Token::GreaterThan,
            Token::GreaterThan => Token::Ampersand,
            Token::Ampersand => Token::Equals,
            Token::Equals => Token::EqualsEquals,
            Token::EqualsEquals => Token::NotEquals,
            Token::NotEquals => Token::Function,
            Token::Function => Token::Numbers("".to_string()),
            Token::Numbers(_) => Token::Float("".to_string()),
            Token::Float(_) => Token::Literal("".to_string()),
            Token::Literal(_) => Token::If,
        };

        self.index = next_token.clone();

        if Token::If == next_token {
            return None;
        }


        Some(TokenInformation::from(next_token))
    }
}

impl Default for TokenInformationIterator {
    fn default() -> Self {
        TokenInformationIterator {
            started: false,
            index: Token::If,
        }
    }
}
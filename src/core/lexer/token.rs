use std::fmt::{Display, Formatter};
use crate::core::lexer::token_information::TokenInformationIterator;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    If,
    Else,
    Let,
    Mut,
    Module,
    Numbers(String),
    Float(String),
    Literal(String),
    While,
    For,
    Equals,                 // =
    Plus,                   // +
    Minus,                  // -
    Multiply,               // *
    Divide,                 // /
    Modulo,                 // %
    Pipe,                   // |
    Xor,                    // ^
    LessThan,               // <
    GreaterThan,            // >
    Ampersand,              // &
    LogicalAnd,             // &&
    LogicalOr,              // ||
    LogicalNot,             // !
    GreaterThanEquals,      // >=
    LessThanEquals,         // <=
    EqualsEquals,           // ==
    NotEquals,              // !=
    LeftShift,              // <<
    RightShift,             // >>
    ParenthesisOpen,        // (
    ParenthesisClose,       // )
    CurlyBraceOpen,         // {
    CurlyBraceClose,        // }
    SquareBracketOpen,      // [
    SquareBracketClose,     // ]
    SemiColon,              // ;
    Comma,                  // ,
    Colon,                  // :
    Function,               // fn
    Dot,                    // .
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(literal) = self.literal() {
            write!(f, "{}", literal)
        } else {
            match self {
                Token::Numbers(value) => write!(f, "{}", value),
                Token::Float(value) => write!(f, "{}", value),
                Token::Literal(value) => write!(f, "{}", value),
                _ => unreachable!("Token not implemented for Display")
            }
        }
    }
}

impl Token {
    pub fn iter() -> TokenInformationIterator {
        TokenInformationIterator::default()
    }

    pub fn literal(&self) -> Option<&'static str> {
        match self {
            Token::If => Some("if"),
            Token::Else => Some("else"),
            Token::Let => Some("let"),
            Token::Mut => Some("mut"),
            Token::While => Some("while"),
            Token::For => Some("for"),
            Token::Module => Some("module"),
            Token::ParenthesisOpen => Some("("),
            Token::ParenthesisClose => Some(")"),
            Token::CurlyBraceOpen => Some("{"),
            Token::CurlyBraceClose => Some("}"),
            Token::SquareBracketOpen => Some("["),
            Token::SquareBracketClose => Some("]"),
            Token::Equals => Some("="),
            Token::Plus => Some("+"),
            Token::Minus => Some("-"),
            Token::Multiply => Some("*"),
            Token::Divide => Some("/"),
            Token::Modulo => Some("%"),
            Token::Pipe => Some("|"),
            Token::Xor => Some("^"),
            Token::LessThan => Some("<"),
            Token::GreaterThan => Some(">"),
            Token::Ampersand => Some("&"),
            Token::LogicalAnd => Some("&&"),
            Token::LogicalOr => Some("||"),
            Token::LogicalNot => Some("!"),
            Token::GreaterThanEquals => Some(">="),
            Token::LessThanEquals => Some("<="),
            Token::EqualsEquals => Some("=="),
            Token::NotEquals => Some("!="),
            Token::LeftShift => Some("<<"),
            Token::RightShift => Some(">>"),
            Token::SemiColon => Some(";"),
            Token::Comma => Some(","),
            Token::Colon => Some(":"),
            Token::Function => Some("fn"),
            Token::Dot => Some("."),
            Token::Numbers(_) | Token::Literal(_) | Token::Float(_) => None,
        }
    }
}
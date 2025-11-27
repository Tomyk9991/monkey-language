use std::fmt::{Display, Formatter};
use crate::core::lexer;
use crate::core::lexer::parse::ParseResult;
use crate::core::lexer::token_information::TokenInformationIterator;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;

#[derive(Debug, PartialEq, Clone, Eq, Hash, Default)]
pub enum Token {
    #[default]
    If,
    Else,
    Let,
    Mut,
    Module,
    Numbers(String),
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

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '(' => Token::ParenthesisOpen,
            ')' => Token::ParenthesisClose,
            '{' => Token::CurlyBraceOpen,
            '}' => Token::CurlyBraceClose,
            '[' => Token::SquareBracketOpen,
            ']' => Token::SquareBracketClose,
            '=' => Token::Equals,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '%' => Token::Modulo,
            '|' => Token::Pipe,
            '^' => Token::Xor,
            '<' => Token::LessThan,
            '>' => Token::GreaterThan,
            '&' => Token::Ampersand,
            '!' => Token::LogicalNot,
            ';' => Token::SemiColon,
            ',' => Token::Comma,
            ':' => Token::Colon,
            '.' => Token::Dot,
            _ => unreachable!("Token not implemented for char")
        }
    }
}

impl From<Result<ParseResult<AbstractSyntaxTreeNode>, lexer::error::Error>> for Token {
    fn from(value: Result<ParseResult<AbstractSyntaxTreeNode>, lexer::error::Error>) -> Self {
        todo!()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(literal) = self.literal() {
            write!(f, "{}", literal)
        } else {
            match self {
                Token::Numbers(value) => write!(f, "{}", value),
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
            Token::Numbers(_) | Token::Literal(_) => None,
        }
    }
}
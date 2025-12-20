use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;


impl Parse for LValue {
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default {
        if let Some(TokenWithSpan { token: Token::SquareBracketOpen, .. }) = tokens.get(1) {
            if let Ok(expr) = Expression::parse(tokens, ParseOptions::default()) {
                return Ok(ParseResult {
                    consumed: expr.consumed,
                    result: LValue::Expression(expr.result)
                })
            }
        }

        if let Ok(identifier) = Identifier::from_str(&format!("{}", tokens[0].token), false) {
            Ok(ParseResult {
                consumed: 1,
                result: LValue::Identifier(identifier)
            })
        } else {
            Err(Error::UnexpectedToken(tokens[0].clone()))
        }
    }
}

#[derive(Debug)]
pub enum LValueErr {
    KeywordReserved(String, FilePosition),
}

impl Display for LValueErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            LValueErr::KeywordReserved(value, file_position) => {
                format!("The variable name \"{}\" {file_position} variable name can't have the same name as a reserved keyword", value)
            }
        };
        write!(f, "{}", message)
    }
}

impl std::error::Error for LValueErr { }


impl FromStr for LValue {
    type Err = LValueErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(name) = Identifier::from_str(s, false) {
            Ok(LValue::Identifier(name))
        } else {
            Err(LValueErr::KeywordReserved(s.to_string(), FilePosition::default()))
        }
    }
}

impl LValue {
    pub fn identifier(&self) -> String {
        match self {
            LValue::Identifier(name) => name.name.clone(),
            LValue::Expression(node) => node.identifier().unwrap_or("Expression".to_string()),
        }
    }
}
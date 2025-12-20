use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_with_span::TokenWithSpan;
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

        if let Ok(identifier) = Identifier::parse(tokens, ParseOptions::default()) {
            Ok(ParseResult {
                consumed: identifier.consumed,
                result: LValue::Identifier(identifier.result)
            })
        } else {
            Err(Error::UnexpectedToken(tokens[0].clone()))
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
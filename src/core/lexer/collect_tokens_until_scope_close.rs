use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_with_span::TokenWithSpan;

#[derive(Debug, Default, Clone)]
pub struct CollectTokensFromUntil<const OPEN: char, const CLOSE: char> {
    pub tokens: Vec<TokenWithSpan>
}

impl<const OPEN: char, const CLOSE: char> TryFrom<Result<ParseResult<Self>, Error>> for CollectTokensFromUntil<OPEN, CLOSE> {
    type Error = Error;

    fn try_from(value: Result<ParseResult<Self>, Error>) -> Result<Self, Self::Error> {
        match value {
            Ok(value) => Ok(value.result),
            Err(err) => Err(err)
        }
    }
}

impl<const OPEN: char, const CLOSE: char> Parse for CollectTokensFromUntil<OPEN, CLOSE> {
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default {
        let mut tokens = tokens.to_vec();
        let mut scope_count = 1;
        let mut index = 0;

        let opening = Token::from(OPEN);
        let closing = Token::from(CLOSE);

        if tokens[index].token == closing {
            return Ok(ParseResult {
                result: CollectTokensFromUntil { tokens: vec![] },
                consumed: 0,
            })
        }

        while scope_count > 0 {
            index += 1;
            if index >= tokens.len() {
                return Err(Error::UnexpectedEOF);
            }

            match &tokens[index].token {
                token if *token == opening => {
                    scope_count += 1;
                    if opening == closing { break; }
                },
                token if *token == closing => {
                    scope_count -= 1;
                    if opening == closing { break; }
                },
                _ => {}
            }
        }

        // let tokens: Vec<TokenWithSpan> = tokens[0..index].iter().cloned().collect();
        let tokens: Vec<_> = tokens.drain(0..index).collect();
        let token_len = tokens.len();

        Ok(ParseResult {
            result: CollectTokensFromUntil { tokens },
            consumed: token_len,
        })
    }
}
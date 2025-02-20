use crate::core::lexer::token_with_span::TokenWithSpan;

#[derive(Debug, Clone)]
pub struct ParseResult<T: Default + Clone> {
    // parsed result
    pub result: T,
    // amount of tokens consumed
    pub consumed: usize,
}


pub trait Parse: Default + Clone {
    fn parse(tokens: &[TokenWithSpan]) -> Result<ParseResult<Self>, crate::core::lexer::error::Error> where Self: Sized, Self: Default;
}
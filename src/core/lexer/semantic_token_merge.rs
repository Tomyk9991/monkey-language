use crate::core::lexer::error::Error;
use crate::core::lexer::token::Token;
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};

type SemanticPattern = (Token, Box<dyn for<'a> FnMut(&'a Token) -> bool>, fn(&[TokenWithSpan]) -> Token);

/// Merge tokens that are semantically related.
///
/// # Arguments
///
/// * `tokens`: &[TokenWithSpan] - The tokens to merge.
///
/// returns: Result<Vec<TokenWithSpan>, Error>
pub fn semantic_token_merge(tokens: &[TokenWithSpan]) -> Result<Vec<TokenWithSpan>, Error> {
    let mut pattern: Vec<SemanticPattern> = vec![
        (Token::Module, until(&Token::SemiColon), collect()),
    ];

    let mut result = vec![];
    let mut token_iter = tokens.iter();

    loop {
        let current_token = token_iter.next();
        if let Some(current_token) = current_token {
            result.push(current_token.clone());

            for (target, predicate, _) in pattern.iter_mut() {
                if *target == current_token.token {
                    let mut collected = vec![];


                    for current_collecting_token in token_iter.by_ref() {
                        if predicate(&current_collecting_token.token) {
                            let min_column = *collected.iter()
                                .map(|t: &TokenWithSpan| &t.span)
                                .map(|span: &FilePosition| span.column.start())
                                .min()
                                .unwrap_or(&0);
                            let max_column = *collected.iter()
                                .map(|t: &TokenWithSpan| &t.span)
                                .map(|span: &FilePosition| span.column.end())
                                .max()
                                .unwrap_or(&0);
                            let line = current_collecting_token.span.line.clone();

                            result.push(TokenWithSpan {
                                token: Token::Literal(collected.iter().map(|token: &TokenWithSpan| token.token.to_string()).collect::<String>()),
                                span: FilePosition {
                                    line,
                                    column: min_column..=max_column,
                                },
                            });
                            result.push(current_collecting_token.clone());
                            break;
                        } else {
                            collected.push(current_collecting_token.clone());
                        }
                    }
                }
            }
        } else {
            break;
        }
    }

    Ok(result)
}

fn collect() -> fn(&[TokenWithSpan]) -> Token {
    |collected: &[TokenWithSpan]| Token::Literal(collected.iter().map(|token: &TokenWithSpan| token.token.to_string()).collect::<String>())
}

fn until(expected: &Token) -> Box<dyn for<'a> FnMut(&'a Token) -> bool + '_> {
    Box::new(|token: &Token| *token == *expected)
}
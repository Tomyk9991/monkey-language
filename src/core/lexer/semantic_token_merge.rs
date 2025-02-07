use crate::core::lexer::error::Error;
use crate::core::lexer::token::Token;


pub fn semantic_token_merge(tokens: &[Token]) -> Result<Vec<Token>, Error> {
    let pattern = vec![
        (Token::Module, collect_until(&Token::SemiColon)),
    ];

    let mut result = vec![];
    let mut token_iter = tokens.iter();

    loop {
        let current_token = token_iter.next();
        if let Some(current_token) = current_token {
            result.push(current_token.clone());

            for (target, predicate) in pattern.iter() {
                if target == current_token {
                    let mut collected = vec![];

                    while let Some(current_collecting_token) = token_iter.next() {
                        if predicate(current_collecting_token) {
                            result.push(Token::Literal(collected.iter().map(|token: &Token| token.to_string()).collect::<String>()));
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

fn collect_until<'a>(expected: &'a Token) -> impl Fn(&'a Token) -> bool {
    |token: &Token| *token == *expected
}
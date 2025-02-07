use crate::core::lexer::error::Error;
use crate::core::lexer::token::Token;


pub fn semantic_token_merge(tokens: &[Token]) -> Result<Vec<Token>, Error> {
    let pattern = vec![
        (Token::Module, collect_until(&Token::SemiColon)),
    ];

    let mut result = vec![];
    let mut token_iter = tokens.iter();

    loop {
        let token = token_iter.next();
        if let Some(token) = token {
            result.push(token.clone());

            for (target, predicate) in pattern.iter() {
                if target == token {
                    let mut collected = vec![];

                    while let Some(token) = token_iter.next() {
                        if predicate(token) {
                            break;
                        }
                        collected.push(token);
                    }

                    result.push(Token::Literal(collected.iter().map(|token| token.to_string()).collect::<String>()));
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
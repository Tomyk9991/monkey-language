use monkey_language::core::lexer::error::Error;
use monkey_language::core::lexer::token::Token;
use monkey_language::core::lexer::tokenizer::{collect_greedy};

#[test]
fn test_greedy_collect() -> Result<(), Error> {
    let tokens = collect_greedy("let a = \"hallo welt\" ;")?;

    assert_eq!(tokens, vec![
        Token::Let,
        Token::Literal("a".to_string()),
        Token::Equals,
        Token::Literal("\"hallo welt\"".to_string()),
        Token::SemiColon,
    ]);
    Ok(())
}
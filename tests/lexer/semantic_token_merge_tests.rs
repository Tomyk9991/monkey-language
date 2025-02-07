use monkey_language::core::lexer::error::Error;
use monkey_language::core::lexer::semantic_token_merge::semantic_token_merge;
use monkey_language::core::lexer::token::Token;

#[test]
fn semantic_token_merge_module() -> Result<(), Error> {
    let tokens = vec![
        Token::Module,
        Token::Literal("monkey".to_string()),
        Token::Minus,
        Token::Literal("language".to_string()),
        Token::Minus,
        Token::Literal("project".to_string()),
        Token::Divide,
        Token::Literal("std".to_string()),
        Token::Dot,
        Token::Literal("monkey".to_string()),
        Token::SemiColon
    ];

    let result = semantic_token_merge(&tokens)?;
    assert_eq!(result, vec![
        Token::Module,
        Token::Literal("monkey-language-project/std.monkey".to_string()),
        Token::SemiColon,
    ]);

    Ok(())
}
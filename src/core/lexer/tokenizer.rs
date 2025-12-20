use crate::core::lexer::error::Error;
use crate::core::lexer::semantic_token_merge::semantic_token_merge;
use crate::core::lexer::token::Token;
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};

pub fn tokenize(string: &str) -> Result<Vec<TokenWithSpan>, Error> {
    let token = semantic_token_merge(&collect_greedy(string)?)?;
    Ok(token)
}

/// Collects tokens from a string greedily.
///
/// # Arguments
///
/// * `string`: &str - The string to collect tokens from.
///
/// returns: Result<Vec<Token>, Error>
///
/// # Examples
///
/// ```
/// use monkey_language::core::lexer::error::Error;
/// use monkey_language::core::lexer::tokenizer::collect_greedy;
/// use monkey_language::core::lexer::token::Token;
/// use monkey_language::core::lexer::token_with_span::FilePosition;
///
/// let tokens = collect_greedy("let a = 10;")?.iter().map(|token| token.token.clone()).collect::<Vec<_>>();
/// assert_eq!(tokens, vec![Token::Let, Token::Literal("a".to_string()), Token::Equals, Token::Numbers("10".to_string()), Token::SemiColon]);
/// let span = collect_greedy("let a = 10;")?.iter().map(|token| token.span.clone()).collect::<Vec<_>>();
/// assert_eq!(span[0], FilePosition { line: 1..=1, column: 1..=3 });
/// # Ok::<(), Error>(())
/// ```
pub fn collect_greedy(string: &str) -> Result<Vec<TokenWithSpan>, Error> {
    let mut tokens = vec![];
    let mut index = 0;
    let mut line = 1;
    let mut column = 1;
    let chars = string.chars().collect::<Vec<_>>();

    while index < chars.len() {
        if chars[index] == '\n' {
            line += 1;
            column = 1;
            index += 1;
            continue;
        }

        if chars[index].is_whitespace() {
            index += 1;
            column += 1;
            continue;
        }

        let start_token = column;
        let token_target = Token::iter();
        let mut found = false;

        for token_information in token_target {
            let mut collected = String::new();
            let before_collect_index = index;
            let before_collect_column = column;

            // check if the next char is a normal letter
            // if in literal mode -> literal mode, if first char is a letter
            let is_literal_mode = collected
                .chars()
                .next()
                .map(|c| c.is_alphabetic() || c == '_' || c == '$')
                .unwrap_or(true);

            // token length is not always known. for expected tokens like if, let, mut, etc. we know the length
            if let Some(token_length) = token_information.token_length {
                while collected.len() < token_length && index < chars.len() {
                    if chars[index].is_whitespace() {
                        if collected.is_empty() {
                            return Err(Error::InvalidCharacter(chars[index]));
                        }

                        index += 1;
                        continue;
                    }

                    collected.push(chars[index]);
                    index += 1;
                    column += 1;
                }
            } else {
                // but for literals we don't know the length; so we need to collect until we find a single character token or whitespace
                'outer: while 
                    index < chars.len() && 
                    !chars[index].is_whitespace()
                    && (is_literal_mode && !done_collecting_literal(chars.get(index)))
                {
                    collected.push(chars[index]);
                    index += 1;
                    column += 1;

                    if collected.starts_with("\"") {
                        while index < chars.len() {
                            collected.push(chars[index]);
                            if collected.ends_with("\"") {
                                index += 1;
                                column += 1;
                                break 'outer;
                            }
                            index += 1;
                            column += 1;
                        }
                    }
                }
            }

            let end_column = column - 1;

            let is_literal_mode = collected
                .chars()
                .next()
                .map(|c| c.is_alphabetic() || c == '_' || c == '$')
                .unwrap_or(true);

            let whole_literal_captured = if is_literal_mode {
                // check if the next char is a something else than an operation or whitespace
                done_collecting_literal(chars.get(index))
            } else {
                true
            };

            if token_information.matches(&collected) && whole_literal_captured {
                let token = match token_information.token {
                    Token::Literal(_) => Token::Literal(collected),
                    Token::Numbers(_) => Token::Numbers(collected),
                    _ => token_information.token,
                };

                tokens.push(TokenWithSpan {
                    token,
                    span: FilePosition {
                        line: line..=line,
                        column: start_token..=end_column,
                    },
                });

                found = true;
                break;
            } else {
                index = before_collect_index;
                column = before_collect_column;
            }
        }

        if !found {
            return Err(Error::InvalidCharacter(chars[index]));
        }
    }

    Ok(tokens)
}

fn done_collecting_literal(target_char: Option<&char>) -> bool {
    // check if the next char is a something else than an operation or whitespace
    if let Some(next_char) = target_char {
        if next_char.is_whitespace() {
            true
        } else {
            !(next_char.is_alphanumeric() || *next_char == '_' || *next_char == '$' || *next_char == '"')
        }
    } else {
        true
    }
}

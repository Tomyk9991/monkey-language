use crate::core::lexer::error::Error;
use crate::core::lexer::semantic_token_merge::semantic_token_merge;
use crate::core::lexer::token::Token;
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::operator::Operator;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::prefix_arithmetic::PrefixArithmetic::Operation;

pub fn tokenize(string: &str) -> Result<Vec<TokenWithSpan>, Error> {
    let token = semantic_token_merge(&collect_greedy(&string)?)?;
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

    let mut single_character_tokens = Token::iter()
        .filter_map(|token_information| token_information.token.literal())
        .filter(|literal| literal.len() == 1)
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

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

        let mut start_token = column;
        let mut token_target = Token::iter();
        let mut found = false;

        while let Some(token_information) = token_target.next() {
            let mut collected = String::new();
            let before_collect_index = index;
            let before_collect_column = column;

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
            } else { // but for literals we don't know the length; so we need to collect until we find a single character token or whitespace
                'outer: while !chars[index].is_whitespace() && !single_character_tokens.contains(&chars[index].to_string()) && index < chars.len() {
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
            // check if the next char is a normal letter
            // if in literal mode -> literal mode, if first char is a letter
            let is_literal_mode = collected.chars().nth(0).map(|c| c.is_alphabetic() || c == '_' || c == '$').unwrap_or(false);

            let whole_literal_captured = if is_literal_mode {
                // check if the next char is a something else than an operation or whitespace
                if let Some(next_char) = chars.get(index) {
                    if next_char.is_whitespace() {
                        true
                    } else {
                        !next_char.is_alphanumeric() || *next_char == '_' || *next_char == '$'
                    }
                } else {
                    true
                }
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
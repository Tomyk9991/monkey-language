use crate::core::lexer::error::Error;
use crate::core::lexer::token::Token;

pub fn tokenize(string: &str) -> Result<Vec<Token>, Error> {
    let string = normalize(string);
    let token = collect_greedy(&string)?;
    // let token = semantic_token_merge(token)?;
    Ok(token)
}

pub fn collect_greedy(string: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = vec![];
    let mut index = 0;
    let chars = string.chars().collect::<Vec<_>>();

    while index < chars.len() {
        if chars[index].is_whitespace() {
            index += 1;
            continue;
        }

        let mut token_target = Token::iter();
        let mut found = false;

        while let Some(token_information) = token_target.next() {
            let mut collected = String::new();
            let before_collect_index = index;

            // token length is not always known. for expected tokens like if, let, mut, etc. we know the length
            if let Some(token_length) = token_information.token_length {
                while collected.len() < token_length && index < chars.len() {
                    if chars[index].is_whitespace() {
                        if collected.len() == 0 {
                            return Err(Error::InvalidCharacter(chars[index - 1]));
                        }
                        index += 1;
                        continue;
                    }

                    collected.push(chars[index]);
                    index += 1;
                }
            } else { // but for literals we don't know the length; so we need to collect until we find a whitespace
                'outer: while !chars[index].is_whitespace() && index < chars.len() {
                    collected.push(chars[index]);
                    index += 1;

                    if collected.starts_with("\"") {
                        while index < chars.len() {
                            collected.push(chars[index]);
                            if collected.ends_with("\"") {
                                index += 1;
                                break 'outer;
                            }
                            index += 1;
                        }
                    }
                }
            }

            if token_information.matches(&collected) {
                match token_information.token {
                    Token::Literal(_) => {
                        tokens.push(Token::Literal(collected));
                    },
                    Token::Numbers(_) => {
                        tokens.push(Token::Numbers(collected));
                    },
                    Token::Float(_) => {
                        tokens.push(Token::Float(collected));
                    },
                    _ => {
                        tokens.push(token_information.token)
                    },
                }
                found = true;
                break;
            } else {
                index = before_collect_index;
            }
        }

        if !found {
            return Err(Error::InvalidCharacter(chars[index]));
        }



    }

    Ok(tokens)
}


fn normalize(target: &str) -> String {
    let mut single_character_tokens = Token::iter()
        .filter_map(|token_information| token_information.token.literal())
        .filter(|literal| literal.len() == 1)
        .map(|literal| (literal.to_string(), format!(" {} ", literal)))
        .collect::<Vec<(String, String)>>();

    single_character_tokens.push(("\n".to_string(), "".to_string()));
    single_character_tokens.push(("\r".to_string(), "".to_string()));
    single_character_tokens.push(("\r\n".to_string(), "".to_string()));


    let mut target = target.to_string();

    for (from, to) in single_character_tokens.iter() {
        target = target.replace(from, to);
    }

    // remove multiple whitespaces
    target = target.split_whitespace().collect::<Vec<_>>().join(" ");
    target
}
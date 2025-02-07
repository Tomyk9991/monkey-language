use crate::core::lexer::error::Error;
use crate::core::lexer::semantic_token_merge::semantic_token_merge;
use crate::core::lexer::token::Token;

pub fn tokenize(string: &str) -> Result<Vec<Token>, Error> {
    let string = normalize(string);
    let token = semantic_token_merge(&collect_greedy(&string)?)?;
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
        .map(|literal| (literal.chars().nth(0), format!(" {} ", literal)))
        .filter_map(|(literal, formatted)| literal.map(|literal| (literal, formatted)))
        .collect::<Vec<(char, String)>>();

    single_character_tokens.push(('\n', "".to_string()));
    single_character_tokens.push(('\r', "".to_string()));


    let mut target = target.to_string();


    for (from, to) in single_character_tokens.iter() {
        target = target.replace_if_outside_quote(*from, to);
    }


    // remove multiple whitespaces
    target.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub trait ReplaceIfOutsideQuote {
    /// Replace a character if it is outside a quote
    ///
    /// # Arguments
    ///
    /// * `from`: The character to be replaced.
    /// * `to`: The string to replace the character with.
    ///
    /// returns: String
    ///
    /// # Examples
    ///
    /// ```
    /// use monkey_language::core::lexer::tokenizer::ReplaceIfOutsideQuote;
    ///
    /// let target = "let%x = \"hello%world\";";
    /// let result = target.replace_if_outside_quote('%', " % ");
    ///
    /// assert_eq!(result, "let % x = \"hello%world\";");
    /// ```
    fn replace_if_outside_quote(&self, from: char, to: &str) -> String;
}

impl ReplaceIfOutsideQuote for str {

    fn replace_if_outside_quote(&self, from: char, to: &str) -> String {
        let mut result = String::with_capacity(self.len());
        let chars: Vec<char> = self.chars().collect();
        let mut inside_quote = false;
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];
            if c == '"' {
                // Bestimme, ob das Anführungszeichen escaped ist:
                // Zähle die unmittelbar davor stehenden Backslashes.
                let mut escape_count = 0;
                let mut j = i;
                while j > 0 {
                    j -= 1;
                    if chars[j] == '\\' {
                        escape_count += 1;
                    } else {
                        break;
                    }
                }
                // Wenn escape_count gerade ist, ist das Anführungszeichen nicht escaped.
                if escape_count % 2 == 0 {
                    inside_quote = !inside_quote;
                }
                result.push(c);
            } else if !inside_quote && c == from {
                result.push_str(to);
            } else {
                result.push(c);
            }
            i += 1;
        }
        result
    }
}
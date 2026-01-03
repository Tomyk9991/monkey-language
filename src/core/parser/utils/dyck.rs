use std::cmp::Ordering;
use std::fmt::Debug;
use crate::core::lexer::token::Token;
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};

#[derive(Debug)]
pub struct DyckError {
    pub target_value: String,
    pub ordering: Ordering,
}

pub trait ArrayOrObject<T> {
    fn list(&self) -> Vec<T>;
}

impl ArrayOrObject<char> for char {
    fn list(&self) -> Vec<char> {
        vec![*self]
    }
}

impl ArrayOrObject<char> for Vec<char> {
    fn list(&self) -> Vec<char> {
        self.clone()
    }
}

impl ArrayOrObject<TokenWithSpan> for Vec<char> {
    fn list(&self) -> Vec<TokenWithSpan> {
        self.iter().map(|c| TokenWithSpan {
            token: Token::from(*c),
            span: FilePosition::default(),
        }).collect::<Vec<_>>()
    }
}

/// # Formal definition
/// Let Σ = {( ) [a-z A-Z]}
///
/// {u ∈ Σ* | all prefixes of u contain no more )'s than ('s and the number of ('s in equals the number of )'s }
pub fn dyck_language<T: ArrayOrObject<K>, K, F>(sequence: &[K], values: [T; 3], breaker: T, contains: F) -> Result<Vec<Vec<K>>, DyckError>
where
    K: Clone + Debug,
    F: Fn(&[K], &K) -> bool {
    let mut individual_parameters: Vec<Vec<K>> = Vec::new();
    let mut counter = 0;
    let mut current_start_index = 0;

    let openings = values[0].list();
    let separators = values[1].list();
    let closings = values[2].list();
    let breaker = breaker.list();

    for (index, c) in sequence.iter().enumerate() {
        if contains(&breaker, c) && counter == 0 {
            break;
        }

        if contains(&openings, c) { // opening
            counter += 1;
        } else if contains(&closings, c) { // closing
            counter -= 1;
        } else if contains(&separators, c) && counter == 0 { // seperator
            let value = &sequence[current_start_index..index];

            if value.is_empty() {
                return Err(DyckError {
                    target_value: format!("{:?}", sequence),
                    ordering: Ordering::Equal,
                });
            }

            individual_parameters.push(value.to_vec());
            current_start_index = index + 1;
        }

        if counter < 0 {
            return Err(DyckError {
                target_value: format!("{:?}", sequence),
                ordering: Ordering::Less,
            });
        }
    }

    match counter {
        number if number > 0 => Err(DyckError {
            target_value: format!("{:?}", sequence),
            ordering: Ordering::Less,
        }),
        number if number < 0 => Err(DyckError {
            target_value: format!("{:?}", sequence),
            ordering: Ordering::Greater,
        }),
        _ => {
            let s = &sequence[current_start_index..sequence.len()];
            if !s.is_empty() {
                individual_parameters.push(sequence[current_start_index..sequence.len()].to_vec());
            }

            Ok(individual_parameters)
        }
    }
}
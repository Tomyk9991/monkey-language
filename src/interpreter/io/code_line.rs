use crate::utils::extension_methods::RemoveWhiteSpacesBetween;
use binary_search_tree::BinarySearchTree;
use regex::Regex;
use regex_split::RegexSplit;
use std::ops::Range;
use crate::interpreter::constants::{IF_KEYWORD, OPENING_SCOPE};
use crate::interpreter::constants::{CLOSING_SCOPE, FUNCTION_KEYWORD};
use crate::interpreter::model::scope_type::{ScopeSplitterIterator, ScopeType};

#[derive(Debug, Default, Clone)]
pub struct CodeLine {
    pub line: String,
    pub actual_line_number: Range<usize>,
    pub virtual_line_number: usize,
}


impl CodeLine {
    pub fn imaginary(l: &str) -> CodeLine {
        Self {
            line: l.to_string(),
            actual_line_number: 0..0,
            virtual_line_number: 0,
        }
    }
}

impl CodeLine {
    pub fn new(line: String, actual_line_number: Range<usize>, virtual_line_number: usize) -> Self {
        Self {
            line,
            actual_line_number,
            virtual_line_number,
        }
    }

    /// Splits the line with the provided chars
    pub fn split(&self, chars: Vec<char>) -> Vec<String> {
        self.line
            .split_inclusive(&chars[..])
            .map(|a| a.trim().to_string())
            .filter(|a| !a.is_empty())
            .collect()
    }
}

pub trait Normalizable {
    /// Splits the given vec of Code lines into multiple code lines if any separator tokens is found and inserts spaces.
    /// Spaces are inserted in front of special characters
    fn normalize(&mut self);
}

impl Normalizable for Vec<CodeLine> {
    fn normalize(&mut self) {
        static INSERT_SPACE: [char; 7] = [';', '(', ')', ':', ',', '{', '}'];

        let mut separators = vec![";"];
        separators.extend(ScopeSplitterIterator::new()
            .flat_map(|s| s.0));


        let split_by_regex = separators.join("|");
        #[allow(clippy::unwrap_used)]
        let regex = Regex::new(&split_by_regex).unwrap();

        let mut result: Vec<CodeLine> = Vec::new();
        let mut line_counter = 1;
        let mut scope_stack: Vec<ScopeType> = vec![];

        for code_line in (*self).iter() {
            let combined_code_line_split =
                regex.split_inclusive(&code_line.line).collect::<Vec<_>>();

            for separated_code_line in combined_code_line_split {
                let mut code_line_string = separated_code_line.remove_whitespaces_between();

                let opening_owned = OPENING_SCOPE.to_string();
                let closing_owned = CLOSING_SCOPE.to_string();

                let opening = opening_owned.as_str();
                let closing = closing_owned.as_str();

                code_line_string = code_line_string
                    .replace(&(String::from(" ") + opening), opening)
                    .replace(&(String::from(" ") + closing), closing);
                code_line_string.remove_whitespaces_between();

                // describes all indices, where a space has to be inserted
                // sort them, so you can iterate backwards over them and all indices are still correct, since
                // no shift is happening for the rest
                let mut indices: BinarySearchTree<usize> = BinarySearchTree::new();

                // search for the pattern in the current code line:
                //
                for searching_char in INSERT_SPACE {
                    for (index, char) in code_line_string
                        .chars()
                        .enumerate()
                        .collect::<Vec<(usize, char)>>()
                    {
                        if char == searching_char {
                            indices.insert_without_dup(index);
                        }
                    }
                }

                let mut sorted_indices = indices.sorted_vec();
                sorted_indices.reverse();

                for index in sorted_indices {
                    if *index < code_line_string.len() {
                        code_line_string.insert(*index + 1, ' ');
                    }

                    code_line_string.insert(*index, ' ');
                }

                code_line_string.remove_whitespaces_between();

                if push_code_line_after_validated(
                    &mut result,
                    &code_line_string,
                    &code_line.actual_line_number,
                    &mut line_counter,
                    &mut scope_stack,
                ) {
                    line_counter += 1;
                }
            }
        }

        *self = result;
    }
}

fn push_code_line_after_validated(vec: &mut Vec<CodeLine>, target: &str, actual_line_number: &Range<usize>, line: &mut usize, in_scope_state: &mut Vec<ScopeType>) -> bool {
    let mut target = target.trim().to_string();
    let mut current_scope = in_scope_state.last().is_some(); // gives a ref to the last element

    while current_scope {
        if target.starts_with(CLOSING_SCOPE) {
            in_scope_state.pop();
            current_scope = in_scope_state.last().is_some();

            vec.push(CodeLine::new(
                CLOSING_SCOPE.to_string(),
                actual_line_number.clone(),
                *line,
            ));
            target = target.replacen(CLOSING_SCOPE, "", 1);
            target = target.trim().to_string();

            *line += 1;
        } else {
            break;
        }
    }


    if target.is_empty() {
        return false;
    }

    if target.starts_with(FUNCTION_KEYWORD) && target.ends_with(OPENING_SCOPE) {
        in_scope_state.push(ScopeType::Fn);
    }

    if target.starts_with(IF_KEYWORD) && target.ends_with(OPENING_SCOPE) {
        in_scope_state.push(ScopeType::If);
    }

    vec.push(CodeLine::new(
        target,
        actual_line_number.clone(),
        *line,
    ));

    true
}

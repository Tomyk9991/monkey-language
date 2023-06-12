use crate::utils::extension_methods::RemoveWhiteSpacesBetween;
use binary_search_tree::BinarySearchTree;
use regex::Regex;
use regex_split::RegexSplit;
use std::ops::Range;

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
        static SEPARATORS: [&str; 3] =
            [";", r"fn .*?\(.*?\):\s.*?\{", r"fn .*?\(.*?\):\s.*?\{.*?\}"];

        let split_by_regex = SEPARATORS.join("|");
        #[allow(clippy::unwrap_used)]
        let regex = Regex::new(&split_by_regex).unwrap();

        let mut result: Vec<CodeLine> = Vec::new();
        let mut line_counter = 1;
        let mut in_scope_state = false;

        for code_line in (*self).iter() {

            let combined_code_line_split =
                regex.split_inclusive(&code_line.line).collect::<Vec<_>>();

            for separated_code_line in combined_code_line_split {
                let mut code_line_string = separated_code_line.remove_whitespaces_between();
                code_line_string = code_line_string.replace(" {", "{").replace(" }", "}");
                code_line_string.remove_whitespaces_between();

                // describes all indices, where a space has to be inserted
                // sort them, so you can iterate backwards over them and all indices are still correct, since
                // no shift is happening for the rest
                let mut indices: BinarySearchTree<usize> = BinarySearchTree::new();

                // search for the pattern in the current code line:
                //
                for searching_char in INSERT_SPACE {
                    for char_window in code_line_string
                        .chars()
                        .enumerate()
                        .collect::<Vec<(usize, char)>>()
                        .windows(2)
                    {
                        let first_char = char_window[0].1;
                        let second_char = char_window[1].1;

                        if first_char != ' ' && second_char == searching_char {
                            indices.insert_without_dup(char_window[1].0);
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

                if push_code_line_after_validated(
                    &mut result,
                    &code_line_string,
                    &code_line.actual_line_number,
                    &mut line_counter,
                    &mut in_scope_state,
                ) {
                    line_counter += 1;
                }
            }
        }

        *self = result;
    }
}

fn push_code_line_after_validated(
    vec: &mut Vec<CodeLine>,
    target: &str,
    actual_line_number: &Range<usize>,
    line: &mut usize,
    in_scope_state: &mut bool,
) -> bool {
    let mut target = target.trim().to_string();
    
    if in_scope_state == &true && target.starts_with('}') {
        *in_scope_state = false;

        vec.push(CodeLine::new(
            "}".to_string(),
            actual_line_number.clone(),
            *line,
        ));
        target = target.replacen('}', "", 1);
        target = target.trim().to_string();
        
        *line += 1;
    }
    
    if target.is_empty() {
        return false;
    }

    if in_scope_state == &false && target.starts_with("fn") && target.ends_with('{') {
        *in_scope_state = true;
    }

    vec.push(CodeLine::new(
        target,
        actual_line_number.clone(),
        *line,
    ));

    true
}

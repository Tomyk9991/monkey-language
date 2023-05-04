use binary_search_tree::BinarySearchTree;
use crate::utils::extension_methods::RemoveWhiteSpacesBetween;

#[derive(Debug)]
pub struct CodeLine {
    pub line: String,
    pub actual_line_number: usize,
    pub virtual_line_number: usize
}

impl CodeLine {
    pub fn imaginary(l: &str) -> CodeLine {
        Self {
            line: l.to_string(),
            actual_line_number: 0,
            virtual_line_number: 0,
        }
    }
}

impl CodeLine {
    pub fn new(line: String, actual_line_number: usize, virtual_line_number: usize) -> Self {
        Self {
            line,
            actual_line_number,
            virtual_line_number,
        }
    }
    
    /// Splits the line with the provided chars
    pub fn split(&self, chars: Vec<char>) -> Vec<String> {
        self.line.split_inclusive(&chars[..])
            .map(|a| a.trim().to_string())
            .filter(|a| !a.is_empty())
            .collect()
    }
}

pub trait Normalizable {
    fn normalize(&mut self);
}

impl Normalizable for Vec<CodeLine> {
    fn normalize(&mut self) {
        static INSERT_SPACE: [char; 5] = [';', '(', ')', ':', ','];
        static SEPARATORS: [char; 1] = [';'];

        let mut result: Vec<CodeLine> = Vec::new();
        let mut line_counter = 1;

        for code_line in (*self).iter() {
            let splits = code_line.line
                .split_inclusive(&SEPARATORS[..])
                .collect::<Vec<_>>();

            for split in splits {
                let mut target = split.remove_whitespaces_between();

                // describes all indices, where a space has to be inserted
                // sort them, so you can iterate backwards over them and all indices are still correct, since
                // no shift is happening for the rest
                let mut indices: BinarySearchTree<usize> = BinarySearchTree::new();

                for searching_char in INSERT_SPACE {
                    for window in target.chars().enumerate().collect::<Vec<(usize, char)>>().windows(2) {
                        if window[0].1 != ' ' && window[1].1 == searching_char {
                            indices.insert_without_dup(window[1].0);
                        }
                    }
                }

                let mut v = indices.sorted_vec();
                v.reverse();

                for index in v {
                    if *index < target.len() {
                        target.insert(*index + 1, ' ');
                    }

                    target.insert(*index, ' ');
                }

                push_code_line_after_validated(&mut result, &target, code_line.actual_line_number, line_counter);
                line_counter += 1;
            }
        }

        *self = result;
    }
}

fn push_code_line_after_validated(vec: &mut Vec<CodeLine>, target: &str, actual_line_number: usize, line: usize) {
    if target.trim().is_empty() { return };
    if target.is_empty() { return; }

    vec.push(CodeLine::new(target.to_string(), actual_line_number, line));
}
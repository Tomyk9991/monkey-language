use crate::utils::extension_methods::RemoveWhiteSpacesBetween;
use binary_search_tree::BinarySearchTree;
use regex::Regex;
use regex_split::RegexSplit;
use std::ops::Range;
use crate::core::constants::OPENING_SCOPE;
use crate::core::constants::CLOSING_SCOPE;
use crate::core::model::scope_type::{ScopeSplitterIterator, ScopeType, ScopeTypeIterator};

/// Represents a line of code
#[derive(Debug, Eq, Hash, Default, Clone, PartialEq)]
pub struct CodeLine {
    pub line: String,
    /// The actual line number in the file
    pub actual_line_number: Range<usize>,
    /// The virtual line number in increments of 1
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
    /// Examples:
    ///
    pub fn split(&self, chars: Vec<char>) -> Vec<String> {
        self.line
            .split_inclusive(&chars[..])
            .map(|a| a.trim().to_string())
            .filter(|a| !a.is_empty())
            .collect()
    }
}

pub trait Normalizable {
    fn normalize(&mut self);
}

impl Normalizable for Vec<CodeLine> {
    /// Splits the given vec of Code lines into multiple code lines if any separator tokens is found and inserts spaces.
    /// Spaces are inserted in front of special characters like `;`, `(`, `)`, `:`, `,`, `{`, `}`.
    fn normalize(&mut self) {
        let opening_owned = OPENING_SCOPE.to_string();
        let closing_owned = CLOSING_SCOPE.to_string();

        let opening = opening_owned.as_str();
        let closing = closing_owned.as_str();

        static INSERT_SPACE: [char; 9] = [';', '(', ')', ':', ',', '{', '}', '<', '>'];

        let mut separators = vec![";"];
        separators.extend(ScopeSplitterIterator::default()
            .flat_map(|s| s.0));


        let split_by_regex = separators.join("|");
        #[allow(clippy::unwrap_used)]
        let regex = Regex::new(&split_by_regex).unwrap();

        let mut result: Vec<CodeLine> = Vec::new();
        let mut line_counter = 1;
        let mut scope_stack: Vec<ScopeType> = vec![];
        // describes the current ident level, if the current character is inside a string or not
        let mut string_ident = 0;

        for code_line in (*self).iter() {
            let combined_code_line_split =
                regex.split_inclusive(&code_line.line).collect::<Vec<_>>();

            for separated_code_line in combined_code_line_split {
                let mut code_line_string = separated_code_line.remove_whitespaces_between();

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
                        if char == '"' { // todo: problem with \" inside the actual string. need some escape system
                            string_ident = (string_ident + 1) % 2;
                        }

                        if char == searching_char && string_ident == 0 {
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

/// Adds a `CodeLine` to the provided vector `vec` after validating and processing scope-related keywords.
///
/// # Arguments
///
/// * `vec` - Vector of CodeLines that will be updated.
/// * `target` - A string that may contain code that starts or ends a scope.
/// * `actual_line_number` - The range that includes line numbers of `target` in the original source file.
/// * `line` - The current line number being processed.
/// * `in_scope_state` - A vector that maintains a stack of the currently open scopes.
///
/// # Returns
///
/// `True` if `CodeLine` was added to `vec`, `false` if `target` was empty after trimming.
fn push_code_line_after_validated(vec: &mut Vec<CodeLine>, target: &str, actual_line_number: &Range<usize>, line: &mut usize, in_scope_state: &mut Vec<ScopeType>) -> bool {
    let mut target = target.trim().to_string();
    let mut current_scope = in_scope_state.last().is_some();

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
    let scope_type_iterator = ScopeTypeIterator::default()
        .map(|(scope_type_str, scope_type)| (scope_type_str.replace(' ', ""), scope_type))
        .collect::<Vec<(String, ScopeType)>>();

    for (scope_keyword, scope_type) in scope_type_iterator {
        if target.starts_with(&scope_keyword) && target.ends_with(OPENING_SCOPE) {
            in_scope_state.push(scope_type);
        }
    }

    vec.push(CodeLine::new(
        target,
        actual_line_number.clone(),
        *line,
    ));

    true
}

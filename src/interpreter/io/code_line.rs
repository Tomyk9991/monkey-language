use std::ops::Range;
use binary_search_tree::BinarySearchTree;
use regex::Regex;
use regex_split::RegexSplit;
use crate::interpreter::lexer::scope::ScopeError::ParsingError;
use crate::utils::extension_methods::RemoveWhiteSpacesBetween;

#[derive(Debug, Default, Clone)]
pub struct CodeLine {
    pub line: String,
    pub actual_line_number: Range<usize>,
    pub virtual_line_number: usize
}

impl CodeLine {
    pub fn contains_token(&self, token: &str) -> bool {
        self.line.split(" ").any(|symbol| symbol == token)
    }
}


impl CodeLine {
    pub fn imaginary(l: &str) -> CodeLine {
        Self {
            line: l.to_string(),
            actual_line_number: 0..0,
            virtual_line_number: 0
        }
    }

    pub fn merge(&mut self, other: &[&Self]) -> anyhow::Result<()> {
        let mut all: Vec<&CodeLine> = Vec::with_capacity(1 + other.len());
        all.push(self);
        all.extend(other);
        let min_line = all
            .iter()
            .min_by(|x, y| x.actual_line_number.start.cmp(&y.actual_line_number.start))
            .ok_or(ParsingError { message: "Iterator is empty".to_string() })?
            .actual_line_number.start;

        let max_line = all
            .iter()
            .max_by(|x, y| x.actual_line_number.end.cmp(&y.actual_line_number.end))
            .ok_or(ParsingError { message: "Iterator is empty".to_string() })?
            .actual_line_number.end;

        let merge_size = all.len();

        drop(all);

        for line in other {
            self.line.push_str(&line.line);
        }

        if merge_size == 2 {
            self.actual_line_number = max_line..max_line;
        } else {
            self.actual_line_number = min_line..max_line;
        }

        Ok(())
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
        self.line.split_inclusive(&chars[..])
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
        static SCOPE_CHAR_PAIRS: (char, char) = ('{', '}');
        static SEPARATORS: [&str; 3] = [";", r"fn .*?\(.*?\):\s.*?\{", r"fn .*?\(.*?\):\s.*?\{.*?\}"];

        let mut result: Vec<CodeLine> = Vec::new();
        let mut line_counter = 1;
        let mut in_scope_state = false;

        for code_line in (*self).iter() {
            let s = format!("{}", SEPARATORS.join("|"));
            let regex = Regex::new(&s).unwrap();
            let splits = regex.split_inclusive(&code_line.line)
                .collect::<Vec<_>>();

            for split in splits {
                let mut target = split.remove_whitespaces_between();
                target = target
                    .replace(" {", "{")
                    .replace(" }", "}");
                target.remove_whitespaces_between();

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

                if push_code_line_after_validated(&mut result, &target, &code_line.actual_line_number, &mut line_counter, &mut in_scope_state) {
                    line_counter += 1;
                }
            }
        }

        *self = result;
    }
}


fn push_code_line_after_validated(vec: &mut Vec<CodeLine>, target: &str, actual_line_number: &Range<usize>, line: &mut usize, in_scope_state: &mut bool) -> bool {
    let mut target = target.trim().to_string();

    if target.is_empty() { return false; }

    if in_scope_state == &true && target.starts_with("}") {
        *in_scope_state = false;

        vec.push(CodeLine::new("}".to_string(), actual_line_number.clone(), *line));
        target = target.replacen("}", "", 1);

        *line += 1;
    }

    if in_scope_state == &false && target.starts_with("fn") && target.ends_with("{") {
        *in_scope_state = true;
    }

    vec.push(CodeLine::new(target.to_string(), actual_line_number.clone(), *line));

    true
}
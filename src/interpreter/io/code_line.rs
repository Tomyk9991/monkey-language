use std::ops::Range;
use binary_search_tree::BinarySearchTree;
use crate::interpreter::lexer::scope::ScopeError::ParsingError;
use crate::utils::extension_methods::RemoveWhiteSpacesBetween;

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

    pub fn merge(&mut self, other: &[Self]) -> anyhow::Result<()> {
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

        drop(all);

        for line in other {
            self.line.push_str(&line.line);
        }

        self.actual_line_number = min_line..max_line;

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
    fn normalize(&mut self);
    fn merge(&mut self)  -> anyhow::Result<()>;
}

impl Normalizable for Vec<CodeLine> {
    fn normalize(&mut self) {
        static INSERT_SPACE: [char; 7] = [';', '(', ')', ':', ',', '{', '}'];
        static SEPARATORS: [char; 1] = [';'];

        let mut result: Vec<CodeLine> = Vec::new();
        let mut line_counter = 1;

        for code_line in (*self).iter() {
            let splits = code_line.line
                .split_inclusive(&SEPARATORS[..])
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

                if push_code_line_after_validated(&mut result, &target, &code_line.actual_line_number, line_counter) {
                    line_counter += 1;
                }
            }
        }

        *self = result;
    }

    fn merge(&mut self) -> anyhow::Result<()> {
        let mut result: Vec<CodeLine> = Vec::new();

        let mut start = 0;
        let mut indent_level = 0;
        let mut counter = 1;

        for (index, code_line) in (*self).iter().enumerate() {
            if code_line.line.contains('{') {
                indent_level += 1;
            }

            if code_line.line.contains('}') {
                indent_level -= 1;

                if indent_level == 0 {
                    let mut virtual_code_line = CodeLine::new(String::new(), index..index, counter);
                    virtual_code_line.merge(&self[start..index + 1])?;
                    result.push(virtual_code_line);

                    start = index + 1;
                    counter += 1;
                    continue;
                }
            }

            if code_line.line.contains(';') {
                let mut c = code_line.clone();
                c.virtual_line_number = counter;

                result.push(c);

                counter += 1;
                continue;
            }
        }

        *self = result;

        Ok(())
    }
}

fn push_code_line_after_validated(vec: &mut Vec<CodeLine>, target: &str, actual_line_number: &Range<usize>, line: usize) -> bool {
    if target.trim().is_empty() { return false; };
    if target.is_empty() { return false; }

    vec.push(CodeLine::new(target.to_string(), actual_line_number.clone(), line));

    true
}
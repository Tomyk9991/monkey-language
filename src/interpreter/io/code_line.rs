use crate::utils::extension_methods::RemoveWhiteSpacesBetween;

#[derive(Debug)]
pub struct CodeLine {
    pub line: String,
    pub actual_line_number: usize,
    pub virtual_line_number: usize
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
        self.line.split_inclusive(&chars[..]).map(|a| a.trim().to_string()).collect()
    }
}

pub trait Normalizable {
    fn normalize(&mut self);
}

impl Normalizable for Vec<CodeLine> {
    fn normalize(&mut self) {
        static SEPERATORS: [char; 1] = [';'];

        let mut result: Vec<CodeLine> = Vec::new();
        let mut line_counter = 1;

        for code_line in (&*self).iter() {
            let splits = code_line.line
                .split_inclusive(&SEPERATORS[..])
                .collect::<Vec<_>>();

            for split in splits {
                push_code_line_if_validated(&mut result, split, code_line.actual_line_number, line_counter);
                line_counter += 1;
            }
        }

        *self = result;
    }
}

fn push_code_line_if_validated(vec: &mut Vec<CodeLine>, target: &str, actual_line_number: usize, line: usize) {
    if target.is_empty() { return; }

    let mut target = target.remove_whitespaces_between();
    if target.ends_with(";") {
        target = target.replace(";" , " ;");
    }

    vec.push(CodeLine::new(target.to_string(), actual_line_number, line));
}
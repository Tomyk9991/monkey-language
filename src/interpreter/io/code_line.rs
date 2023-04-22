#[derive(Debug)]
pub struct CodeLine {
    line: String,
    actual_line_number: usize,
    virtual_line_number: usize
}

impl CodeLine {
    pub fn new(line: String, actual_line_number: usize, virtual_line_number: usize) -> Self {
        Self {
            line,
            actual_line_number,
            virtual_line_number,
        }
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
    let target = target.trim();

    if target.is_empty() { return; }

    vec.push(CodeLine::new(target.to_string(), actual_line_number, line));
}
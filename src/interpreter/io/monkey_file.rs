use std::fs::File;
use std::io::Read;
use std::ops::Range;
use std::path::{Path, PathBuf};
use anyhow::Context;
use crate::interpreter::io::code_line::{CodeLine, Normalizable};

#[derive(Debug)]
pub struct MonkeyFile {
    pub path: PathBuf,
    pub lines: Vec<CodeLine>,
    pub size: usize
}

impl MonkeyFile {
    pub fn read<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let path_buffer = PathBuf::from(path.as_ref());
        let mut file: File = File::open(path)
            .context(format!("Can't find file: {:?}", path_buffer))?;
        let mut buffer = String::new();

        let size = file.read_to_string(&mut buffer)?;
        let actual_lines = get_line_ranges(&buffer);
        buffer = buffer.replace("\r\n", "");

        let mut lines = Self::read_buffer(&buffer);

        lines.normalize();

        lines.iter_mut()
            .zip(actual_lines.iter())
            .for_each(|(mut line, number)| {
                line.actual_line_number = number.clone();
            });

        println!("{:#?}", lines);

        Ok(Self {
            path: path_buffer,
            lines,
            size,
        })
    }

    #[allow(unused)]
    pub fn read_from_str(buffer: &str) -> Self {
        let mut buffer: String = buffer.to_owned();

        let mut lines = Self::read_buffer(&buffer);

        lines.normalize();

        Self {
            path: PathBuf::new(),
            lines,
            size: buffer.chars().count(),
        }
    }

    fn read_buffer(buffer: &String) -> Vec<CodeLine> {
        buffer.lines()
            .enumerate()
            .filter(|(_, line)| !line.trim().starts_with("//"))
            .map(|(index, line)| CodeLine::new(line.to_string(), 1..1, index + 1))
            .collect::<Vec<_>>()
    }
}

fn get_line_ranges(buffer: &str) -> Vec<Range<usize>> {
    let mut line_ranges = Vec::new();
    let mut start = None;

    let mut line_count = 1;
    let mut in_function = false;
    let mut function_ident_level = 0;

    let mut iter = buffer.chars().into_iter();

    while let Some(c) = iter.next() {
        if start.is_none() && !c.is_whitespace() {
            start = Some(line_count);
        }

        // "fn "
        if c == 'f' && &iter.as_str()[..2] == "n " {
            in_function = true;
            start = Some(line_count);
        }

        if c == '{' && in_function {
            function_ident_level += 1;
        }

        if c == '}' && in_function {
            function_ident_level -= 1;

            if function_ident_level == 0 {
                if let Some(s) = start {
                    let range = s..line_count;
                    line_ranges.push(range);
                    start = None;
                    in_function = false;
                }
            }
        }

        if c == ';' && !in_function {
            if let Some(s) = start {
                let range = s..line_count;
                line_ranges.push(range);
                start = None;
            }
        }

        if c == '\n' {
            line_count += 1;
        }
    }

    line_ranges
}


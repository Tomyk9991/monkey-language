use std::fs::File;
use std::io::Read;
use std::ops::Range;
use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::interpreter::constants::CLOSING_SCOPE;
use crate::interpreter::io::code_line::{CodeLine, Normalizable};
use crate::interpreter::model::scope_type::{ScopeType, ScopeTypeIterator};

#[derive(Debug)]
pub struct MonkeyFile {
    pub path: PathBuf,
    pub lines: Vec<CodeLine>,
    pub size: usize,
}

impl MonkeyFile {
    pub fn read<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let path_buffer = PathBuf::from(path.as_ref());

        let mut file: File = File::open(path)
            .context(format!("Can't find file: {:?}", path_buffer))?;
        let mut buffer = String::new();

        let size = file.read_to_string(&mut buffer)?;

        let monkey_file = Self::read_from_str(&buffer);


        Ok(Self {
            path: path_buffer,
            size,
            ..monkey_file
        })
    }

    #[allow(unused)]
    pub fn read_from_str(buffer: &str) -> Self {
        let mut buffer: String = buffer.to_owned();

        let mut lines = Self::read_buffer(&buffer);
        let actual_lines = get_line_ranges(&buffer);

        buffer = buffer.replace('\n', "");
        buffer = buffer.replace('\r', "");

        let mut lines = Self::read_buffer(&buffer);

        lines.normalize();

        lines.iter_mut()
            .zip(actual_lines.iter())
            .for_each(|(mut line, number)| {
                line.actual_line_number = number.clone();
            });

        Self {
            path: PathBuf::new(),
            lines,
            size: buffer.chars().count(),
        }
    }

    fn read_buffer(buffer: &str) -> Vec<CodeLine> {
        buffer.lines()
            .enumerate()
            .filter(|(_, line)| !line.trim().starts_with("//"))
            .map(|(index, line)| CodeLine::new(line.to_string(), index..index, index + 1))
            .collect::<Vec<_>>()
    }
}

fn get_line_ranges(buffer: &str) -> Vec<Range<usize>> {
    let mut line_ranges = Vec::new();
    let mut start = None;

    let mut line_count = 1;

    let mut iter = buffer.chars();
    let mut scope_stack: Vec<ScopeType> = vec![];

    while let Some(char) = iter.next() {
        if start.is_none() && !char.is_whitespace() {
            start = Some(line_count);
        }

        if char == CLOSING_SCOPE && scope_stack.last().is_some() {
            if let Some(s) = start {
                let range = s..line_count;
                line_ranges.push(range);
                start = None;
            }

            scope_stack.pop();
        }

        if char == '\n' {
            line_count += 1;
        }

        let iterator = ScopeTypeIterator::new();

        for (buffer_match, scope_type) in iterator {
            let len = buffer_match.len() - 1;
            let iter_clone = iter.clone();
            let iter_len = iter_clone.count();

            if iter_len < len {
                continue;
            }

            let lookup = String::from(char) + &iter.as_str()[..len];

            if lookup == buffer_match {
                scope_stack.push(scope_type);
                start = Some(line_count);
            }
        }

        if scope_stack.last().is_some() && char == '{' {
            if let Some(s) = start {
                let range = s..line_count;
                line_ranges.push(range);
                start = None;
            }
        }

        if char == ';' {
            if let Some(s) = start {
                let range = s..line_count;
                line_ranges.push(range);
                start = None;
            }
        }
    }

    line_ranges
}


use std::fs::File;
use std::io::Read;
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
        #[deny(clippy::clone_double_ref)]
        let path_buffer = PathBuf::from(path.as_ref());
        let mut file: File = File::open(path)
            .context(format!("Can't find file: {:?}", path_buffer))?;
        let mut buffer = String::new();

        let size = file.read_to_string(&mut buffer)?;

        let mut lines = buffer.lines()
            .enumerate()
            .map(|(index, line)| CodeLine::new(line.to_string(), index + 1, index + 1))
            .collect::<Vec<_>>();

        lines.normalize();

        Ok(Self {
            path: path_buffer,
            lines,
            size,
        })
    }

    #[allow(unused)]
    pub fn read_from_str<>(buffer: &str) -> Self {
        let mut buffer: String = buffer.to_owned();

        let mut lines = buffer.lines()
            .enumerate()
            .map(|(index, line)| CodeLine::new(line.to_string(), index + 1, index + 1))
            .collect::<Vec<_>>();

        lines.normalize();

        Self {
            path: PathBuf::new(),
            lines,
            size: buffer.chars().count(),
        }
    }
}


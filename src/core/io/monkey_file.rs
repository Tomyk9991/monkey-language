use std::fs::File;
use std::io::Read;
use std::ops::Range;
use std::path::{Path, PathBuf};

use anyhow::Context;
use crate::core::lexer::tokenizer::tokenize;
use crate::core::constants::{CLOSING_SCOPE, OPENING_SCOPE};
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::scope_type::{ScopeType, ScopeTypeIterator};

#[derive(Debug, PartialEq, Clone)]
pub struct MonkeyFile {
    pub path: PathBuf,
    pub tokens: Vec<TokenWithSpan>,
    pub size: usize
}

impl MonkeyFile {
    pub fn read<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let path_buffer = PathBuf::from(path.as_ref());

        let mut file: File = File::open(path)
            .context(format!("Can't find file: {:?}", path_buffer))?;

        let mut buffer = String::new();

        let size = file.read_to_string(&mut buffer)?;
        let tokens = tokenize(&buffer)?;

        Ok(Self {
            path: path_buffer,
            tokens,
            size,
        })
    }

    pub fn read_from_str(buffer: &str) -> anyhow::Result<Self> {
        let tokens = tokenize(buffer)?;
        Ok(Self {
            path: PathBuf::new(),
            tokens,
            size: buffer.chars().count(),
        })
    }
}
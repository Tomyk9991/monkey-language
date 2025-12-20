use std::fmt::{Display, Formatter};
use crate::core::io::monkey_file::MonkeyFile;
use crate::core::lexer::token_with_span::FilePosition;

#[derive(Debug, PartialEq, Clone)]
pub struct Import {
    pub monkey_file: MonkeyFile,
    pub file_position: FilePosition
}


impl Display for Import {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}module {};", " ".repeat(f.width().unwrap_or(0)), self.monkey_file.path.display())
    }
}

impl Default for Import {
    fn default() -> Self {
        Import {
            monkey_file: MonkeyFile {
                path: Default::default(),
                tokens: vec![],
                size: 0,
            },
            file_position: Default::default(),
        }
    }
}
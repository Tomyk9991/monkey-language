use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::core::io::code_line::CodeLine;
use crate::core::io::monkey_file::MonkeyFileNew;
use crate::core::scanner::errors::EmptyIteratorErr;

#[derive(Debug, PartialEq, Clone)]
pub struct Import {
    pub monkey_file: MonkeyFileNew,
    pub code_line: CodeLine
}

#[derive(Debug)]
pub enum ImportError {
    PatternNotMatched { target_value: String },
    EmptyIterator(EmptyIteratorErr),
    MonkeyFileRead(anyhow::Error)
}


impl Display for Import {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "module {};", self.monkey_file.path.display())
    }
}

impl Display for ImportError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ImportError::PatternNotMatched { target_value } => {
                format!("Pattern not matched for: `{}?\n\t import name;", target_value)
            },
            ImportError::EmptyIterator(e) => e.to_string(),
            ImportError::MonkeyFileRead(a) => format!("Cannot read the file: {a}")
        })
    }
}

impl Error for ImportError { }
impl Default for Import {
    fn default() -> Self {
        Import {
            monkey_file: MonkeyFileNew {
                path: Default::default(),
                tokens: vec![],
                size: 0,
            },
            code_line: Default::default(),
        }
    }
}
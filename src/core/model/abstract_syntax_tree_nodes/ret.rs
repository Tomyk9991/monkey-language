use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::token_with_span::FilePosition;
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::scanner::errors::EmptyIteratorErr;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Return {
    pub assignable: Option<Assignable>,
    pub file_position: FilePosition,
}

#[derive(Debug)]
pub enum ReturnError {
    PatternNotMatched { target_value: String },
    AssignableError(AssignableError),
    EmptyIterator(EmptyIteratorErr)
}

impl Display for Return {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "return{}", if let Some(assignable) = &self.assignable {
            format!(" {}", assignable)
        } else {
            "".to_string()
        })
    }
}

impl Display for ReturnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ReturnError::PatternNotMatched { target_value } => {
                format!("Pattern not matched for: `{}?\n\t return assignable;", target_value)
            }
            ReturnError::AssignableError(e) => e.to_string(),
            ReturnError::EmptyIterator(e) => e.to_string(),
        })
    }
}

impl Error for ReturnError { }
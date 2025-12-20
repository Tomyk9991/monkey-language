use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::core::lexer::token_with_span::FilePosition;
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Return {
    pub assignable: Option<Assignable>,
    pub file_position: FilePosition,
}

#[derive(Debug)]
pub enum ReturnError {
    PatternNotMatched { target_value: String },
}

impl Display for Return {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ident = f.width().unwrap_or(0);
        
        write!(f, "{}return{}", " ".repeat(ident), if let Some(assignable) = &self.assignable {
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
        })
    }
}

impl Error for ReturnError { }
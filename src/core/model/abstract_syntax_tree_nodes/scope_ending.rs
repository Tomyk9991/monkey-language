use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::core::parser::errors::EmptyIteratorErr;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::semantics::static_type_check::static_type_checker::StaticTypeCheckError;
use crate::core::semantics::static_type_check::static_type_check::StaticTypeCheck;

/// AST node for scope ending. Basically it checks if the codeline is `}`.
#[derive(Debug, PartialEq, Clone)]
pub struct ScopeEnding {
    pub code_line: CodeLine
}

#[derive(Debug)]
pub enum ScopeEndingError {
    PatternNotMatched { target_value: String },
    EmptyIterator(EmptyIteratorErr)
}

impl Default for ScopeEnding {
    fn default() -> Self {
        Self { code_line: CodeLine::default() }
    }
}

impl Error for ScopeEndingError { }

impl Display for ScopeEnding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Display for ScopeEndingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ScopeEndingError::PatternNotMatched { target_value } =>
                format!("Pattern not matched for: `{target_value}`\n\t }}"),
            ScopeEndingError::EmptyIterator(e) => e.to_string()
        })
    }
}

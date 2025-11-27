use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::core::io::code_line::CodeLine;
use crate::core::model::abstract_syntax_tree_nodes::scope_ending::{ScopeEnding, ScopeEndingError};
use crate::core::scanner::errors::EmptyIteratorErr;
use crate::core::scanner::scope::PatternNotMatchedError;
use crate::core::scanner::static_type_context::StaticTypeContext;
use crate::core::scanner::{Lines, TryParse};
use crate::core::semantics::type_checker::static_type_checker::StaticTypeCheckError;
use crate::core::semantics::type_checker::StaticTypeCheck;



impl PatternNotMatchedError for ScopeEndingError {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, ScopeEndingError::PatternNotMatched {..})
    }
}
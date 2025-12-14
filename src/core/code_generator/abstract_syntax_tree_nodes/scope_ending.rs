use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::core::model::abstract_syntax_tree_nodes::scope_ending::{ScopeEnding, ScopeEndingError};
use crate::core::parser::errors::EmptyIteratorErr;
use crate::core::parser::scope::PatternNotMatchedError;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::{Lines, TryParse};
use crate::core::semantics::static_type_check::static_type_checker::StaticTypeCheckError;
use crate::core::semantics::static_type_check::static_type_check::StaticTypeCheck;



impl PatternNotMatchedError for ScopeEndingError {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, ScopeEndingError::PatternNotMatched {..})
    }
}
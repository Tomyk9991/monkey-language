use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::core::model::abstract_syntax_tree_nodes::scope_ending::ScopeEnding;
use crate::core::parser::errors::EmptyIteratorErr;
use crate::core::parser::scope::PatternNotMatchedError;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::{Lines, TryParse};
use crate::core::semantics::static_type_check::static_type_checker::StaticTypeCheckError;
use crate::core::semantics::static_type_check::static_type_check::StaticTypeCheck;



#[derive(Debug)]
pub enum ScopeEndingErr {
    PatternNotMatched { target_value: String },
    EmptyIterator(EmptyIteratorErr)
}

impl PatternNotMatchedError for ScopeEndingErr {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, ScopeEndingErr::PatternNotMatched {..})
    }
}

impl Error for ScopeEndingErr { }

impl StaticTypeCheck for ScopeEnding {
    fn static_type_check(&self, _type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        Ok(())
    }
}

impl Display for ScopeEndingErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ScopeEndingErr::PatternNotMatched { target_value } =>
                format!("Pattern not matched for: `{target_value}`\n\t }}"),
            ScopeEndingErr::EmptyIterator(e) => e.to_string()
        })
    }
}

impl TryParse for ScopeEnding {
    type Output = ScopeEnding;
    type Err = ScopeEndingErr;

    fn try_parse(code_lines_iterator: &mut Lines<'_>) -> anyhow::Result<Self::Output, Self::Err> {
        let code_line = *code_lines_iterator.peek().ok_or(ScopeEndingErr::EmptyIterator(EmptyIteratorErr))?;
        ScopeEnding::try_parse(code_line)
    }
}

impl ScopeEnding {
    pub fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self, ScopeEndingErr> {
        if code_line.line == "}" {
            Ok(Self { code_line: code_line.clone() })
        } else {
            Err(ScopeEndingErr::PatternNotMatched { target_value: code_line.line.to_string() })
        }
    }
}
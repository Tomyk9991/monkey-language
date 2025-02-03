use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::errors::EmptyIteratorErr;
use crate::core::lexer::scope::PatternNotMatchedError;
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::{Lines, TryParse};
use crate::core::type_checker::static_type_checker::StaticTypeCheckError;
use crate::core::type_checker::StaticTypeCheck;

/// AST node for scope ending. Basically it checks if the codeline is `}`.
#[derive(Debug, PartialEq, Clone)]
pub struct ScopeEnding {
    pub code_line: CodeLine
}

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

impl Display for ScopeEnding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

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
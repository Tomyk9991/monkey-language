use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::errors::EmptyIteratorErr;
use crate::core::lexer::levenshtein_distance::{PatternedLevenshteinDistance, PatternedLevenshteinString};
use crate::core::lexer::TryParse;

#[derive(Debug, PartialEq)]
pub struct ScopeEnding;

#[derive(Debug)]
pub enum ScopeEndingErr {
    PatternNotMatched { target_value: String },
    EmptyIterator(EmptyIteratorErr)
}

impl Error for ScopeEndingErr { }

impl Display for ScopeEnding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
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

    fn try_parse(code_lines_iterator: &mut Peekable<Iter<CodeLine>>) -> anyhow::Result<Self::Output, Self::Err> {
        let code_line = *code_lines_iterator.peek().ok_or_else(|| ScopeEndingErr::EmptyIterator(EmptyIteratorErr::default()))?;
        ScopeEnding::try_parse(code_line)
    }
}

impl ScopeEnding {
    pub fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self, ScopeEndingErr> {
        if code_line.line == "}" {
            Ok(Self)
        } else {
            Err(ScopeEndingErr::PatternNotMatched { target_value: code_line.line.to_string() })
        }
    }
}

impl PatternedLevenshteinDistance for ScopeEnding {
    fn distance_from_code_line(code_line: &CodeLine) -> usize {
        let scope_ending_pattern = PatternedLevenshteinString::default()
            .insert("}");

        <ScopeEnding as PatternedLevenshteinDistance>::distance(
            PatternedLevenshteinString::match_to(
                &code_line.line,
                &scope_ending_pattern,
                vec![]
            ),
            scope_ending_pattern
        )
    }
}
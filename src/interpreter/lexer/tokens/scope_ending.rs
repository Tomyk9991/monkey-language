use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::interpreter::io::code_line::CodeLine;
use crate::interpreter::lexer::levenshtein_distance::{PatternedLevenshteinDistance, PatternedLevenshteinString};

#[derive(Debug, PartialEq)]
pub struct ScopeEnding;

#[derive(Debug)]
pub enum ScopeEndingErr {
    PatternNotMatched { target_value: String }
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
        })
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
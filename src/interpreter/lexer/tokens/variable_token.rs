use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use anyhow::Context;
use crate::interpreter::io::code_line::CodeLine;
use crate::interpreter::lexer::levenshtein_distance::{levenshtein_distance, MethodCallSummarizeTransform, PatternedLevenshteinDistance, PatternedLevenshteinString, QuoteSummarizeTransform};
use crate::interpreter::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::interpreter::lexer::tokens::name_token::{NameToken, NameTokenErr};
use crate::interpreter::lexer::TryParse;

#[derive(Debug)]
pub struct VariableToken {
    pub name_token: NameToken,
    pub assignable: AssignableToken
}

#[derive(Debug)]
pub enum ParseVariableTokenErr {
    PatternNotMatched { target_value: String },
    NameTokenErr(NameTokenErr),
    AssignableTokenErr(AssignableTokenErr)
}

impl Error for ParseVariableTokenErr { }

impl From<NameTokenErr> for ParseVariableTokenErr {
    fn from(a: NameTokenErr) -> Self { ParseVariableTokenErr::NameTokenErr(a) }
}

impl From<anyhow::Error> for ParseVariableTokenErr {
    fn from(value: anyhow::Error) -> Self {
        let mut buffer = String::new();
        buffer += &value.to_string();
        buffer += "\n";

        if let Some(e) = value.downcast_ref::<AssignableTokenErr>() {
            buffer += &e.to_string();
        }
        ParseVariableTokenErr::PatternNotMatched { target_value: buffer }
    }
}

impl From<AssignableTokenErr> for ParseVariableTokenErr {
    fn from(a: AssignableTokenErr) -> Self { ParseVariableTokenErr::AssignableTokenErr(a) }
}

impl Display for ParseVariableTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ParseVariableTokenErr::PatternNotMatched { target_value} => format!("`{target_value}`\n\tThe pattern for a variable is defined as: name = assignment;"),
            ParseVariableTokenErr::NameTokenErr(a) => a.to_string(),
            ParseVariableTokenErr::AssignableTokenErr(a) => a.to_string()
        })
    }
}

impl VariableToken {
    pub fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self, ParseVariableTokenErr> {
        let split_alloc = code_line.split(vec![' ', ';']);
        let split = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();

        return if let [name, "=", middle @ .., ";"] = &split[..] {
            Ok(VariableToken {
                name_token: NameToken::from_str(name)?,
                assignable: AssignableToken::try_from(middle.join(" ").as_str()).context(code_line.line.clone())?,
            })
        } else {
            Err(ParseVariableTokenErr::PatternNotMatched { target_value: code_line.line.to_string() })
        }
    }
}


impl PatternedLevenshteinDistance for VariableToken {
    fn distance_from_code_line(code_line: &CodeLine) -> usize {
        let variable_pattern = PatternedLevenshteinString::default()
            .insert(PatternedLevenshteinString::ignore())
            .insert("=")
            .insert(PatternedLevenshteinString::ignore())
            .insert(";");

        <VariableToken as PatternedLevenshteinDistance>::distance(
            PatternedLevenshteinString::match_to(
                &code_line.line,
                &variable_pattern,
                vec![Box::new(QuoteSummarizeTransform), Box::new(MethodCallSummarizeTransform)]
            ),
            variable_pattern,
        )
    }
}
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;
use std::str::FromStr;
use anyhow::Context;
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::errors::EmptyIteratorErr;
use crate::core::lexer::levenshtein_distance::{MethodCallSummarizeTransform, PatternedLevenshteinDistance, PatternedLevenshteinString, QuoteSummarizeTransform};
use crate::core::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::core::lexer::tokens::name_token::{NameToken, NameTokenErr};
use crate::core::lexer::TryParse;

/// Token for a variable. Pattern is defined as: name <Assignment> assignment <Separator>
/// # Examples
/// - `name = assignment;`
/// - `name: assignment,`
#[derive(Debug, PartialEq, Clone)]
pub struct VariableToken<const ASSIGNMENT: char, const SEPARATOR: char> {
    pub name_token: NameToken,
    pub assignable: AssignableToken
}

impl<const ASSIGNMENT: char, const SEPARATOR: char> Display for VariableToken<ASSIGNMENT, SEPARATOR> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<30} {} {}", self.name_token, ASSIGNMENT, self.assignable)
    }
}

#[derive(Debug)]
pub enum ParseVariableTokenErr {
    PatternNotMatched { target_value: String },
    NameTokenErr(NameTokenErr),
    AssignableTokenErr(AssignableTokenErr),
    EmptyIterator(EmptyIteratorErr)
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
            ParseVariableTokenErr::AssignableTokenErr(a) => a.to_string(),
            ParseVariableTokenErr::EmptyIterator(e) => e.to_string()
        })
    }
}

impl<const ASSIGNMENT: char, const SEPARATOR: char> TryParse for VariableToken<ASSIGNMENT, SEPARATOR> {
    type Output = VariableToken<ASSIGNMENT, SEPARATOR>;
    type Err = ParseVariableTokenErr;

    fn try_parse(code_lines_iterator: &mut Peekable<Iter<CodeLine>>) -> anyhow::Result<Self::Output, Self::Err> {
        let code_line = *code_lines_iterator.peek().ok_or(ParseVariableTokenErr::EmptyIterator(EmptyIteratorErr))?;
        VariableToken::try_parse(code_line)
    }
}

impl<const ASSIGNMENT: char, const SEPARATOR: char> VariableToken<ASSIGNMENT, SEPARATOR> {
    pub fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self, ParseVariableTokenErr> {
        let split_alloc = code_line.split(vec![' ', ';']);
        let split = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();


        let assignment = ASSIGNMENT.to_string();
        let separator = SEPARATOR.to_string();

        match &split[..] {
            [name, assignment_token, middle @ .., separator_token] if assignment_token == &assignment && separator_token == &separator => {
                Ok(VariableToken {
                    name_token: NameToken::from_str(name, false)?,
                    assignable: AssignableToken::from_str(middle.join(" ").as_str()).context(code_line.line.clone())?,
                })
            },
            _ => Err(ParseVariableTokenErr::PatternNotMatched { target_value: code_line.line.to_string() })
        }
    }
}


impl<const ASSIGNMENT: char, const SEPARATOR: char> PatternedLevenshteinDistance for VariableToken<ASSIGNMENT, SEPARATOR> {
    fn distance_from_code_line(code_line: &CodeLine) -> usize {
        let variable_pattern = PatternedLevenshteinString::default()
            .insert(PatternedLevenshteinString::ignore())
            .insert(&ASSIGNMENT.to_string())
            .insert(PatternedLevenshteinString::ignore())
            .insert(&SEPARATOR.to_string());

        <VariableToken<ASSIGNMENT, SEPARATOR> as PatternedLevenshteinDistance>::distance(
            PatternedLevenshteinString::match_to(
                &code_line.line,
                &variable_pattern,
                vec![Box::new(QuoteSummarizeTransform), Box::new(MethodCallSummarizeTransform)]
            ),
            variable_pattern,
        )
    }
}
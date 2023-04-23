use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::interpreter::io::code_line::CodeLine;
use crate::interpreter::lexer::levenshtein_distance::{levenshtein_distance, PatternedLevenshteinDistance};
use crate::interpreter::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::interpreter::lexer::tokens::name_token::{NameToken, NameTokenErr};

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

impl From<AssignableTokenErr> for ParseVariableTokenErr {
    fn from(a: AssignableTokenErr) -> Self { ParseVariableTokenErr::AssignableTokenErr(a) }
}

impl Display for ParseVariableTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ParseVariableTokenErr::PatternNotMatched { target_value} => format!("\"{target_value}\"The pattern for a variable is defined as: name = assignment;"),
            ParseVariableTokenErr::NameTokenErr(a) => a.to_string(),
            ParseVariableTokenErr::AssignableTokenErr(a) => a.to_string()
        })
    }
}

impl VariableToken {
    pub fn try_from(code_line: &CodeLine) -> anyhow::Result<Self, ParseVariableTokenErr> {
        let split_alloc = code_line.split(vec![' ', ';']);
        let split = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();

        return if let [name, "=", middle @ .., ";"] = &split[..] {
            Ok(VariableToken {
                name_token: NameToken::from_str(name)?,
                assignable: AssignableToken::try_from(middle.join(" ").as_str())?,
            })
        } else {
            Err(ParseVariableTokenErr::PatternNotMatched { target_value: code_line.line.to_string() })
        }
    }
}

impl PatternedLevenshteinDistance for VariableToken {
    fn distance<P: Into<String>, K: Into<String>>(a: P, b: K) -> usize {
        let string_1 = a.into();
        let string_2 = b.into();

        return levenshtein_distance(&string_1, &string_2);
    }
}

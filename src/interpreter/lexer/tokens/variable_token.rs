use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::interpreter::io::code_line::CodeLine;
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
        let split = code_line.split(vec!['=']);

        if !code_line.ends_with_semicolon() {
            return Err(ParseVariableTokenErr::PatternNotMatched { target_value: code_line.line.to_string()});
        } 
        
        if let [name, assignable] = split.iter().map(|a| a.as_str()).collect::<Vec<&str>>()[..] {
            return Ok(VariableToken {
                name_token: NameToken::from_str(name)?,
                assignable: AssignableToken::try_from(assignable)?,
            })
        }
        
        return Err(ParseVariableTokenErr::PatternNotMatched { target_value: code_line.line.to_string()});
    }
}
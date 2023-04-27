use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::interpreter::lexer::tokens::assignable_token::AssignableToken;
use crate::interpreter::lexer::tokens::name_token::NameToken;
use crate::interpreter::lexer::Visibility;
use crate::interpreter::constants::KEYWORDS;
use crate::interpreter::io::code_line::CodeLine;

#[derive(Debug)]
pub struct MethodCallToken {
    name: NameToken,
    return_type: NameToken,
    arguments: Vec<AssignableToken>,
    visibility: Visibility
}

#[derive(Debug)]
pub enum MethodCallTokenErr {
    UnmatchedRegex { target_value: String }
}

impl Error for MethodCallTokenErr { }

impl Display for MethodCallTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            MethodCallTokenErr::UnmatchedRegex { target_value } => format!("\"{target_value}\"must match: methodName(assignable1, ..., assignableN)"),
        };

        write!(f, "{}", message)
    }
}

impl MethodCallToken {
    pub fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self, MethodCallTokenErr> {
        let split_alloc = code_line.split(vec![' ', ';']);
        let split = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();

        return if let [name, "(", arguments @ .., ")", ";"] = &split[..] {
            
        }
        // return if let [name]
        //
        //
        // Ok(MethodCallToken {
        //
        // })

        Err(MethodCallTokenErr::UnmatchedRegex { target_value: "testing".to_string() })
    }
}
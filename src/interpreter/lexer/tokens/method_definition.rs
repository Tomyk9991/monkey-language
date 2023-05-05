use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::interpreter::io::code_line::CodeLine;
use crate::interpreter::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::interpreter::lexer::tokens::name_token::{NameToken, NameTokenErr};

#[derive(Debug)]
pub struct MethodDefinition {
    name: NameToken,
    return_type: NameToken,
    arguments: Vec<AssignableToken>,
}

#[derive(Debug)]
pub enum MethodDefinitionErr {
    PatternNotMatched { target_value: String },
    NameTokenErr(NameTokenErr),
    AssignableTokenErr(AssignableTokenErr),
}

impl From<AssignableTokenErr> for MethodDefinitionErr {
    fn from(value: AssignableTokenErr) -> Self { MethodDefinitionErr::AssignableTokenErr(value) }
}

impl From<NameTokenErr> for MethodDefinitionErr {
    fn from(value: NameTokenErr) -> Self { MethodDefinitionErr::NameTokenErr(value) }
}

impl Display for MethodDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "fn {}({}): {{Body}}", self.name, self.arguments
            .iter()
            .map(|ass| format!("{}", ass))
            .collect::<Vec<String>>()
            .join(", ")
        )
    }
}

impl Error for MethodDefinitionErr { }

impl Display for MethodDefinitionErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            MethodDefinitionErr::PatternNotMatched { target_value}
            => format!("Pattern not matched for: `{target_value}`\n\t fn function_name(argument1, ..., argumentN): returnType {{ }}"),
            MethodDefinitionErr::AssignableTokenErr(a) => a.to_string(),
            MethodDefinitionErr::NameTokenErr(a) => a.to_string(),
        })
    }
}


impl MethodDefinition {
    pub fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self, MethodDefinitionErr> {
        println!("{:#?}", code_line);

        let s = NameToken::from_str("5")?;
        Ok(MethodDefinition {
            name: NameToken::from_str("hallo")?,
            return_type: NameToken::from_str("hallo")?,
            arguments: vec![],
        })
    }
}
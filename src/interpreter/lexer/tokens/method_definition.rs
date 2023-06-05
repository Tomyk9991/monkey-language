use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;
use std::str::FromStr;
use crate::interpreter::io::code_line::CodeLine;
use crate::interpreter::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::interpreter::lexer::tokens::name_token::{NameToken, NameTokenErr};

#[derive(Debug)]
pub struct MethodDefinition<'a> {
    name: NameToken,
    return_type: NameToken,
    arguments: Vec<AssignableToken>,
    stack: Vec<&'a CodeLine>
}

#[derive(Debug)]
pub enum MethodDefinitionErr {
    PatternNotMatched { target_value: String },
    NameTokenErr(NameTokenErr),
    AssignableTokenErr(AssignableTokenErr),
    EmptyIterator
}

impl From<AssignableTokenErr> for MethodDefinitionErr {
    fn from(value: AssignableTokenErr) -> Self { MethodDefinitionErr::AssignableTokenErr(value) }
}

impl From<NameTokenErr> for MethodDefinitionErr {
    fn from(value: NameTokenErr) -> Self { MethodDefinitionErr::NameTokenErr(value) }
}

impl Display for MethodDefinition<'_> {
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
            MethodDefinitionErr::EmptyIterator => String::from("Iterator is empty")
        })
    }
}


impl MethodDefinition<'_> {
    pub fn try_parse(code_lines: &mut Peekable<Iter<CodeLine>>) -> anyhow::Result<Self, MethodDefinitionErr> {
        let method_header = *code_lines.peek().ok_or(MethodDefinitionErr::EmptyIterator)?;

        let split_alloc = method_header.split(vec![' ']);
        let split = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();

        // println!("Split parameter: {:?}", split);

        println!("{:?}", method_header);

        if let ["fn", name, "(", middle @ .., ")", ":", return_type] = &split[..] {
            while let Some(line) = code_lines.next() {
                println!("{:?}", line);
            }

            // let line = code_lines.next().ok_or(MethodDefinitionErr::EmptyIterator)?;
        }

        if let ["fn", name, "(", ")", ":", return_type, "{"] = split[..] {
            println!("match parameterless");
        }


        Ok(MethodDefinition {
            name: NameToken::from_str("hallo")?,
            return_type: NameToken::from_str("hallo")?,
            arguments: vec![],
            stack: vec![],
        })
    }
}
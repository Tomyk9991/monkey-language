use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;
use std::str::FromStr;
use crate::interpreter::io::code_line::CodeLine;
use crate::interpreter::lexer::scope::{Scope, ScopeError};
use crate::interpreter::lexer::token::Token;
use crate::interpreter::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::interpreter::lexer::tokens::name_token::{NameToken, NameTokenErr};
use crate::interpreter::lexer::TryParse;

#[derive(Debug, PartialEq)]
pub struct MethodDefinition {
    name: NameToken,
    return_type: NameToken,
    arguments: Vec<AssignableToken>,
    stack: Vec<Token>
}

#[derive(Debug)]
pub enum MethodDefinitionErr {
    PatternNotMatched { target_value: String },
    NameTokenErr(NameTokenErr),
    AssignableTokenErr(AssignableTokenErr),
    ScopeErrorErr(ScopeError),
    EmptyIterator
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
            MethodDefinitionErr::EmptyIterator => String::from("Iterator is empty"),
            MethodDefinitionErr::ScopeErrorErr(a) => a.to_string()
        })
    }
}


impl MethodDefinition {
    pub fn try_parse(code_lines: &mut Peekable<Iter<CodeLine>>) -> anyhow::Result<Self, MethodDefinitionErr> {
        let method_header = *code_lines.peek().ok_or(MethodDefinitionErr::EmptyIterator)?;

        let split_alloc = method_header.split(vec![' ']);
        let split_ref = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();

        if let ["fn", name, "(", arguments @ .., ")", ":", return_type, "{"] = &split_ref[..] {
            let mut tokens = vec![];

            // consume the header
            let _ = code_lines.next();

            // consume the body
            while let Some(code_line) = code_lines.peek() {
                println!("{code_line:?}");
                let token = Scope::try_parse(code_lines).map_err(|scope_error| {
                    MethodDefinitionErr::ScopeErrorErr(scope_error)
                })?;

                println!("{token}");

                if token == Token::ScopeClosing {
                    break;
                }

                tokens.push(token);
            }

            let mut assignable_arguments = vec![];

            let arguments_String = arguments.join("");
            let arguments = arguments_String.split(",").collect::<Vec<_>>();

            for argument in arguments {
                assignable_arguments.push(AssignableToken::try_from(argument)?);
            }


            return Ok(MethodDefinition {
                name: NameToken::from_str(name, false)?,
                return_type: NameToken::from_str(return_type, true)?,
                arguments: assignable_arguments,
                stack: tokens,
            })
        }

        // if let ["fn", name, "(", ")", ":", return_type, "{", stack, "}"] = split_header[..] {
        //     println!("match parameterless");
        // }


        Ok(MethodDefinition {
            name: NameToken::from_str("hallo", false)?,
            return_type: NameToken::from_str("hallo", true)?,
            arguments: vec![],
            stack: vec![],
        })
    }
}
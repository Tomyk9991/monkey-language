use crate::interpreter::io::code_line::CodeLine;
use crate::interpreter::lexer::scope::{Scope, ScopeError};
use crate::interpreter::lexer::token::Token;
use crate::interpreter::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::interpreter::lexer::tokens::name_token::{NameToken, NameTokenErr};
use crate::interpreter::lexer::TryParse;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;
use std::str::FromStr;
use crate::interpreter::constants::FUNCTION_KEYWORD;
use crate::interpreter::lexer::errors::EmptyIteratorErr;
use crate::interpreter::lexer::tokens::scope_ending::ScopeEnding;
use crate::interpreter::lexer::levenshtein_distance::PatternedLevenshteinDistance;
use crate::interpreter::lexer::levenshtein_distance::{ArgumentsIgnoreSummarizeTransform, EmptyParenthesesExpand, PatternedLevenshteinString, QuoteSummarizeTransform};

#[derive(Debug, PartialEq)]
pub struct MethodDefinition {
    pub name: NameToken,
    pub return_type: NameToken,
    pub arguments: Vec<AssignableToken>,
    pub stack: Vec<Token>,
}

#[derive(Debug)]
pub enum MethodDefinitionErr {
    PatternNotMatched { target_value: String },
    NameTokenErr(NameTokenErr),
    AssignableTokenErr(AssignableTokenErr),
    ScopeErrorErr(ScopeError),
    EmptyIterator(EmptyIteratorErr),
}

impl From<AssignableTokenErr> for MethodDefinitionErr {
    fn from(value: AssignableTokenErr) -> Self {
        MethodDefinitionErr::AssignableTokenErr(value)
    }
}

impl From<NameTokenErr> for MethodDefinitionErr {
    fn from(value: NameTokenErr) -> Self {
        MethodDefinitionErr::NameTokenErr(value)
    }
}

impl Display for MethodDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "fn {}({}): {{Body}}",
            self.name,
            self.arguments
                .iter()
                .map(|ass| format!("{}", ass))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Error for MethodDefinitionErr {}

impl Display for MethodDefinitionErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            MethodDefinitionErr::PatternNotMatched { target_value}
            => format!("Pattern not matched for: `{target_value}`\n\t fn function_name(argument1, ..., argumentN): returnType {{ }}"),
            MethodDefinitionErr::AssignableTokenErr(a) => a.to_string(),
            MethodDefinitionErr::NameTokenErr(a) => a.to_string(),
            MethodDefinitionErr::EmptyIterator(e) => e.to_string(),
            MethodDefinitionErr::ScopeErrorErr(a) => a.to_string()
        })
    }
}

impl TryParse for MethodDefinition {
    type Output = MethodDefinition;
    type Err = MethodDefinitionErr;

    fn try_parse(code_lines_iterator: &mut Peekable<Iter<CodeLine>>) -> anyhow::Result<Self, MethodDefinitionErr> {
        let method_header = *code_lines_iterator
            .peek()
            .ok_or_else(|| MethodDefinitionErr::EmptyIterator(EmptyIteratorErr::default()))?;

        let split_alloc = method_header.split(vec![' ']);
        let split_ref = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();

        if let ["fn", name, "(", arguments @ .., ")", ":", return_type, "{"] = &split_ref[..] {
            let arguments_string = arguments.join("");
            let arguments = arguments_string.split(',').filter(|a| !a.is_empty()).collect::<Vec<_>>();
            let mut assignable_arguments = vec![];

            for argument in arguments {
                assignable_arguments.push(AssignableToken::from_str(argument)?);
            }

            let mut tokens = vec![];

            // consume the header
            let _ = code_lines_iterator.next();

            // consume the body
            while code_lines_iterator.peek().is_some() {
                let token = Scope::try_parse(code_lines_iterator)
                    .map_err(MethodDefinitionErr::ScopeErrorErr)?;

                if token == Token::ScopeClosing(ScopeEnding) {
                    break;
                }

                tokens.push(token);
            }

            return Ok(MethodDefinition {
                name: NameToken::from_str(name, false)?,
                return_type: NameToken::from_str(return_type, true)?,
                arguments: assignable_arguments,
                stack: tokens,
            });
        }

        Err(MethodDefinitionErr::PatternNotMatched {
            target_value: method_header.line.to_string()
        })
    }
}

impl PatternedLevenshteinDistance for MethodDefinition {
    fn distance_from_code_line(code_line: &CodeLine) -> usize {
        let method_header_pattern = PatternedLevenshteinString::default()
            .insert(FUNCTION_KEYWORD)
            .insert(PatternedLevenshteinString::ignore())
            .insert("(")
            .insert(PatternedLevenshteinString::ignore())
            .insert(")")
            .insert(":")
            .insert(PatternedLevenshteinString::ignore())
            .insert("{");

        <MethodDefinition as PatternedLevenshteinDistance>::distance(
            PatternedLevenshteinString::match_to(
                &code_line.line,
                &method_header_pattern,
                vec![
                    Box::new(QuoteSummarizeTransform),
                    Box::new(EmptyParenthesesExpand),
                    Box::new(ArgumentsIgnoreSummarizeTransform)
                ]
            ),
            method_header_pattern
        )
    }
}

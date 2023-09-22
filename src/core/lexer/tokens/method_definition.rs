use crate::core::io::code_line::CodeLine;
use crate::core::lexer::scope::{Scope, ScopeError};
use crate::core::lexer::token::Token;
use crate::core::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::core::lexer::tokens::name_token::{NameToken, NameTokenErr};
use crate::core::lexer::TryParse;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;
use std::str::FromStr;
use crate::core::constants::FUNCTION_KEYWORD;
use crate::core::lexer::errors::EmptyIteratorErr;
use crate::core::lexer::levenshtein_distance::PatternedLevenshteinDistance;
use crate::core::lexer::levenshtein_distance::{ArgumentsIgnoreSummarizeTransform, EmptyParenthesesExpand, PatternedLevenshteinString, QuoteSummarizeTransform};
use crate::core::lexer::type_token::{InferTypeError, TypeToken};

/// Token for method definition. Pattern is `fn function_name(argument1, ..., argumentN): returnType { }`
#[derive(Debug, PartialEq, Clone)]
pub struct MethodDefinition {
    pub name: NameToken,
    pub return_type: TypeToken,
    pub arguments: Vec<AssignableToken>,
    pub stack: Vec<Token>,
    pub code_line: CodeLine
}

#[derive(Debug)]
pub enum MethodDefinitionErr {
    PatternNotMatched { target_value: String },
    NameTokenErr(NameTokenErr),
    ReturnTokenErr(InferTypeError),
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

impl From<InferTypeError> for MethodDefinitionErr {
    fn from(value: InferTypeError) -> Self {
        MethodDefinitionErr::ReturnTokenErr(value)
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
            MethodDefinitionErr::ReturnTokenErr(a) => a.to_string(),
            MethodDefinitionErr::EmptyIterator(e) => e.to_string(),
            MethodDefinitionErr::ScopeErrorErr(a) => a.to_string(),
        })
    }
}

impl TryParse for MethodDefinition {
    type Output = MethodDefinition;
    type Err = MethodDefinitionErr;

    fn try_parse(code_lines_iterator: &mut Peekable<Iter<CodeLine>>) -> anyhow::Result<Self, MethodDefinitionErr> {
        let method_header = *code_lines_iterator
            .peek()
            .ok_or(MethodDefinitionErr::EmptyIterator(EmptyIteratorErr))?;

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

                if let Token::ScopeClosing(_) = token {
                    break;
                }

                tokens.push(token);
            }

            return Ok(MethodDefinition {
                name: NameToken::from_str(name, false)?,
                return_type: TypeToken::from_str(return_type)?,
                arguments: assignable_arguments,
                stack: tokens,
                code_line: method_header.clone(),
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

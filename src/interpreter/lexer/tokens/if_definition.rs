use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;

use crate::interpreter::constants::IF_KEYWORD;
use crate::interpreter::constants::OPENING_SCOPE;
use crate::interpreter::io::code_line::CodeLine;
use crate::interpreter::lexer::errors::EmptyIteratorErr;
use crate::interpreter::lexer::levenshtein_distance::{ArgumentsIgnoreSummarizeTransform, EmptyParenthesesExpand, PatternedLevenshteinDistance, PatternedLevenshteinString, QuoteSummarizeTransform};
use crate::interpreter::lexer::scope::{Scope, ScopeError};
use crate::interpreter::lexer::token::Token;
use crate::interpreter::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::interpreter::lexer::tokens::scope_ending::ScopeEnding;
use crate::interpreter::lexer::TryParse;

#[derive(Debug, PartialEq)]
pub struct IfDefinition {
    pub condition: AssignableToken,
    pub if_stack: Vec<Token>,
    pub else_stack: Option<Vec<Token>>,
}

#[derive(Debug)]
pub enum IfDefinitionErr {
    PatternNotMatched { target_value: String },
    AssignableTokenErr(AssignableTokenErr),
    ScopeErrorErr(ScopeError),
    EmptyIterator(EmptyIteratorErr),
}

impl From<AssignableTokenErr> for IfDefinitionErr {
    fn from(value: AssignableTokenErr) -> Self {
        IfDefinitionErr::AssignableTokenErr(value)
    }
}

impl Display for IfDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if self.else_stack.is_some() {
                format!("if ({}) {{Body}} else {{Body}}", self.condition)
            } else {
                format!("if ({}) {{Body}}", self.condition)
            }
        )
    }
}

impl Error for IfDefinitionErr {}

impl Display for IfDefinitionErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            IfDefinitionErr::PatternNotMatched { target_value }
            => format!("Pattern not matched for: `{target_value}\n\t if(condition) {{ }}`"),
            IfDefinitionErr::AssignableTokenErr(a) => a.to_string(),
            IfDefinitionErr::ScopeErrorErr(a) => a.to_string(),
            IfDefinitionErr::EmptyIterator(e) => e.to_string(),
        })
    }
}

impl TryParse for IfDefinition {
    type Output = IfDefinition;
    type Err = IfDefinitionErr;

    fn try_parse(code_lines_iterator: &mut Peekable<Iter<CodeLine>>) -> anyhow::Result<Self::Output, Self::Err> {
        let if_header = *code_lines_iterator
            .peek()
            .ok_or(IfDefinitionErr::EmptyIterator(EmptyIteratorErr::default()))?;

        let split_alloc = if_header.split(vec![' ']);
        let split_ref = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();

        let mut if_stack = vec![];

        if let ["if", "(", condition, ")", "{"] = &split_ref[..] {
            let condition = AssignableToken::try_from(condition)?;

            // consume the header
            let _ = code_lines_iterator.next();

            while code_lines_iterator.peek().is_some() {
                let token = Scope::try_parse(code_lines_iterator)
                    .map_err(IfDefinitionErr::ScopeErrorErr)?;

                if token == Token::ScopeClosing(ScopeEnding) {
                    break;
                }

                if_stack.push(token);
            }

            return Ok(IfDefinition {
                condition,
                if_stack,
                else_stack: None,
            });
        }


        Err(IfDefinitionErr::PatternNotMatched {
            target_value: if_header.line.to_string()
        })
    }
}


impl PatternedLevenshteinDistance for IfDefinition {
    fn distance_from_code_line(code_line: &CodeLine) -> usize {
        let if_header_pattern = PatternedLevenshteinString::default()
            .insert(IF_KEYWORD)
            .insert("(")
            .insert(PatternedLevenshteinString::ignore())
            .insert(")")
            .insert(&OPENING_SCOPE.to_string());

        <IfDefinition as PatternedLevenshteinDistance>::distance(
            PatternedLevenshteinString::match_to(
                &code_line.line,
                &if_header_pattern,
                vec![
                    Box::new(QuoteSummarizeTransform),
                    Box::new(EmptyParenthesesExpand),
                    Box::new(ArgumentsIgnoreSummarizeTransform),
                ],
            ),
            if_header_pattern,
        )
    }
}
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, ASMOptions, ASMResult, MetaInfo, ToASM};
use crate::core::code_generator::registers::GeneralPurposeRegister;
use crate::core::io::monkey_file::MonkeyFile;
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::errors::EmptyIteratorErr;
use crate::core::lexer::levenshtein_distance::{PatternedLevenshteinDistance, PatternedLevenshteinString, QuoteSummarizeTransform};
use crate::core::lexer::scope::PatternNotMatchedError;
use crate::core::lexer::TryParse;

#[derive(Debug, PartialEq, Clone)]
pub struct ImportToken {
    pub monkey_file: MonkeyFile,
    pub code_line: CodeLine
}

#[derive(Debug)]
pub enum ImportTokenError {
    PatternNotMatched { target_value: String },
    EmptyIterator(EmptyIteratorErr),
    MonkeyFileRead(anyhow::Error)
}

impl PatternNotMatchedError for ImportTokenError {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, ImportTokenError::PatternNotMatched {..})
    }
}

impl Display for ImportToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "import {}", self.monkey_file.path.display())
    }
}

impl Display for ImportTokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ImportTokenError::PatternNotMatched { target_value } => {
                format!("Pattern not matched for: `{}?\n\t import name;", target_value)
            },
            ImportTokenError::EmptyIterator(e) => e.to_string(),
            ImportTokenError::MonkeyFileRead(a) => format!("Cannot read the file: {a}")
        })
    }
}

impl Error for ImportTokenError { }

impl From<anyhow::Error> for ImportTokenError {
    fn from(value: anyhow::Error) -> Self {
        ImportTokenError::MonkeyFileRead(value)
    }
}

impl TryParse for ImportToken {
    type Output = ImportToken;
    type Err = ImportTokenError;

    fn try_parse(code_lines_iterator: &mut Peekable<Iter<CodeLine>>) -> anyhow::Result<Self::Output, Self::Err> {
        let code_line = *code_lines_iterator.peek().ok_or(ImportTokenError::EmptyIterator(EmptyIteratorErr))?;
        ImportToken::try_parse(code_line)
    }
}

impl ImportToken {
    pub fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self, ImportTokenError> {
        let split_alloc = code_line.split(vec![' ', ';']);
        let split = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();

        if let ["module", monkey_file, ";"] = &split[..] {
            return Ok(ImportToken {
                monkey_file: MonkeyFile::read(monkey_file)?,
                code_line: code_line.clone(),
            });
        }

        Err(ImportTokenError::PatternNotMatched {target_value: code_line.line.to_string() })
    }
}

impl PatternedLevenshteinDistance for ImportToken {
    fn distance_from_code_line(code_line: &CodeLine) -> usize {
        let pattern = PatternedLevenshteinString::default()
            .insert("import")
            .insert(PatternedLevenshteinString::ignore())
            .insert(";");

        <ImportToken as PatternedLevenshteinDistance>::distance(
            PatternedLevenshteinString::match_to(
                &code_line.line,
                &pattern,
                vec![Box::new(QuoteSummarizeTransform)]
            ),
            pattern
        )
    }
}

impl ToASM for ImportToken {
    fn to_asm<T: ASMOptions>(&self, _stack: &mut Stack, _meta: &mut MetaInfo, _options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        Ok(ASMResult::Inline(String::new()))
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        0
    }

    fn before_label(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Option<Result<String, ASMGenerateError>> {
        None
    }

    fn multi_line_asm(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Result<(bool, String, Option<GeneralPurposeRegister>), ASMGenerateError> {
        Ok((false, String::new(), None))
    }
}
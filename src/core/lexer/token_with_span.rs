use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;
use crate::core::lexer::token::Token;

/// A struct representing a span in a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilePosition {
    pub line: RangeInclusive<i32>,
    pub column: RangeInclusive<i32>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenWithSpan {
    /// The token.
    pub token: Token,
    /// The span of the token.
    pub span: FilePosition
}

impl TokenWithSpan {
    pub fn new(token: Token, min: i32, max: i32) -> TokenWithSpan {
        TokenWithSpan { token, span: FilePosition { line: min..=max, column: min..=max } }
    }
}

impl FilePosition {
    pub fn from_min_max(min: &TokenWithSpan, max: &TokenWithSpan) -> FilePosition {
        FilePosition {
            line: *min.span.line.start()..=*max.span.line.end(),
            column: *min.span.column.start()..=*max.span.column.end()
        }
    }
}

impl Default for FilePosition {
    fn default() -> Self {
        Self {
            line: 0..=0,
            column: 0..=0
        }
    }
}

impl Display for FilePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let line_break_information = if self.line.start() == self.line.end() {
            "".to_string()
        } else {
            format!(", {} line breaks", self.line.end() - self.line.start())
        };

        let column = if self.column.start() == self.column.end() {
            self.column.start().to_string()
        } else {
            format!("{} ({} chars{})", self.column.start(), (self.column.end() - self.column.start()) + 1, line_break_information)
        };


        write!(f, "{}:{}", self.line.start(), column)
    }
}

impl Display for TokenWithSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}` at line {}", self.token, self.span)
    }
}


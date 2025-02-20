use std::fmt::Display;
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

impl Display for TokenWithSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line_break_information = if self.span.line.start() == self.span.line.end() {
            "".to_string()
        } else {
            format!(", {} line breaks", self.span.line.end() - self.span.line.start())
        };

        let column = if self.span.column.start() == self.span.column.end() {
            self.span.column.start().to_string()
        } else {
            format!("{} ({} chars{})", self.span.column.start(), self.span.column.end() - self.span.column.start(), line_break_information)
        };


        write!(f, "{} at line {}:{}", self.token, self.span.line.start(), column)
    }
}


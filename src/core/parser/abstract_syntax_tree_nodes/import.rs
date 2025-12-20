use crate::core::io::monkey_file::MonkeyFile;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::abstract_syntax_tree_nodes::import::Import;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;


#[derive(Debug)]
pub enum ImportError {
    MonkeyFileRead(anyhow::Error)
}


impl Display for ImportError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ImportError::MonkeyFileRead(a) => format!("Cannot read the file: {a}")
        })
    }
}

impl Error for ImportError { }

impl From<anyhow::Error> for ImportError {
    fn from(value: anyhow::Error) -> Self {
        ImportError::MonkeyFileRead(value)
    }
}

impl Parse for Import {
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, crate::core::lexer::error::Error> where Self: Sized, Self: Default {
        if let [TokenWithSpan { token: Token::Module, .. }, TokenWithSpan { token: Token::Literal(ref literal), .. }, TokenWithSpan { token: Token::SemiColon, .. }, ..] = tokens[..] {
            return Ok(ParseResult {
                result: Import {
                    monkey_file: MonkeyFile::read(PathBuf::from(literal)).map_err(|e| {
                        eprintln!("{}", e);
                        crate::core::lexer::error::Error::UnexpectedEOF
                    })?,
                    file_position: Default::default(),
                },
                consumed: 3,
            })
        }

        Err(crate::core::lexer::error::Error::UnexpectedToken(tokens[0].clone()))
    }
}
use crate::interpreter::io::monkey_file::MonkeyFile;
use crate::interpreter::lexer::scope::Scope;
use crate::interpreter::lexer::TryParse;

pub struct Lexer {
    current_file: MonkeyFile
}

impl Lexer {
    pub fn from(file: MonkeyFile) -> Self {
        Self {
            current_file: file
        }
    }
}

impl Lexer {
    pub fn tokenize(&mut self) -> anyhow::Result<Scope> {
        let mut scope = Scope {
            tokens: vec![],
        };

        let mut iterator = self.current_file.lines.iter().peekable();

        while iterator.peek().is_some() {
            let token = Scope::try_parse(&mut iterator)?;
            scope.tokens.push(token);
        }

        Ok(scope)
    }
}
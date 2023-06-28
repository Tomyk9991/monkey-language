use crate::core::io::monkey_file::MonkeyFile;
use crate::core::lexer::scope::Scope;
use crate::core::lexer::TryParse;

pub struct Lexer {
    current_file: MonkeyFile
}

impl From<MonkeyFile> for Lexer {
    fn from(value: MonkeyFile) -> Self {
        Self {
            current_file: value
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
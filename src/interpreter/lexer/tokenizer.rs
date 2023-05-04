use crate::interpreter::io::monkey_file::MonkeyFile;
use crate::interpreter::lexer::scope::Scope;
use crate::interpreter::lexer::TryParse;

pub struct Lexer<'a> {
    current_file: &'a MonkeyFile
}

impl<'a> From<&'a MonkeyFile> for Lexer<'a> {
    fn from(file: &'a MonkeyFile) -> Self {
        Self {
            current_file: file
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn tokenize(&mut self) -> anyhow::Result<Scope> {
        let mut scope = Scope {
            tokens: vec![],
        };


        for line in &self.current_file.lines {
            let token = Scope::try_parse(line)?;
            scope.tokens.push(token);
        }

        Ok(scope)
    }
}
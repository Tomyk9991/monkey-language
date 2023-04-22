use crate::interpreter::io::monkey_file::MonkeyFile;
use crate::interpreter::lexer::scope::Scope;
use crate::interpreter::lexer::TryParse;

pub struct Lexer<'a> {
    current_file: &'a MonkeyFile
}

impl<'a> Lexer<'a> {
    pub fn new(file: &'a MonkeyFile) -> Self {
        Self {
            current_file: file
        }
    }

    pub fn tokenize(&mut self) -> anyhow::Result<Scope> {
        let mut scope = Scope {
            tokens: vec![],
        };


        for line in &self.current_file.lines {
            let token = Scope::try_parse(line)?;
        }

        Ok(scope)
    }
}
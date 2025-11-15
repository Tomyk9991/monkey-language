use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::{ASMResult};
use crate::core::io::monkey_file::{MonkeyFile, MonkeyFileNew};
use crate::core::io::code_line::CodeLine;
use crate::core::scanner::errors::EmptyIteratorErr;
use crate::core::scanner::scope::PatternNotMatchedError;
use crate::core::scanner::static_type_context::StaticTypeContext;
use crate::core::scanner::{Lines, TryParse};
use crate::core::semantics::type_checker::static_type_checker::StaticTypeCheckError;
use crate::core::semantics::type_checker::StaticTypeCheck;

#[derive(Debug, PartialEq, Clone)]
pub struct Import {
    pub monkey_file: MonkeyFileNew,
    pub code_line: CodeLine
}

impl Default for Import {
    fn default() -> Self {
        Import {
            monkey_file: MonkeyFileNew {
                path: Default::default(),
                tokens: vec![],
                size: 0,
            },
            code_line: Default::default(),
        }
    }
}


#[derive(Debug)]
pub enum ImportError {
    PatternNotMatched { target_value: String },
    EmptyIterator(EmptyIteratorErr),
    MonkeyFileRead(anyhow::Error)
}

impl PatternNotMatchedError for ImportError {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, ImportError::PatternNotMatched {..})
    }
}

impl Display for Import {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "import {}", self.monkey_file.path.display())
    }
}

impl Display for ImportError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ImportError::PatternNotMatched { target_value } => {
                format!("Pattern not matched for: `{}?\n\t import name;", target_value)
            },
            ImportError::EmptyIterator(e) => e.to_string(),
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

impl StaticTypeCheck for Import {
    fn static_type_check(&self, _type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        Ok(())
    }
}

impl TryParse for Import {
    type Output = Import;
    type Err = ImportError;

    fn try_parse(code_lines_iterator: &mut Lines<'_>) -> anyhow::Result<Self::Output, Self::Err> {
        let code_line = *code_lines_iterator.peek().ok_or(ImportError::EmptyIterator(EmptyIteratorErr))?;
        Import::try_parse(code_line)
    }
}

impl Import {
    pub fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self, ImportError> {
        todo!()
        // let split_alloc = code_line.split(vec![' ', ';']);
        // let split = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();
        //
        // if let ["module", monkey_file, ";"] = &split[..] {
        //     return Ok(Import {
        //         monkey_file: MonkeyFile::read(monkey_file)?,
        //         code_line: code_line.clone(),
        //     });
        // }
        //
        // Err(ImportError::PatternNotMatched {target_value: code_line.line.to_string() })
    }
}

impl ToASM for Import {
    fn to_asm<T: ASMOptions>(&self, _stack: &mut Stack, _meta: &mut MetaInfo, _options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        Ok(ASMResult::Inline(String::new()))
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        0
    }

}
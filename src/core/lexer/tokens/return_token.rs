use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;
use std::str::FromStr;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_result::{ASMOptions, ASMResult, ASMResultError, ASMResultVariance, InterimResultOption};
use crate::core::code_generator::conventions::return_calling_convention;
use crate::core::code_generator::registers::{ByteSize, GeneralPurposeRegister};
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::errors::EmptyIteratorErr;
use crate::core::lexer::scope::PatternNotMatchedError;
use crate::core::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::core::lexer::TryParse;

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnToken {
    pub assignable: Option<AssignableToken>,
    pub code_line: CodeLine
}

#[derive(Debug)]
pub enum ReturnTokenError {
    PatternNotMatched { target_value: String },
    AssignableError(AssignableTokenErr),
    EmptyIterator(EmptyIteratorErr)
}

impl PatternNotMatchedError for ReturnTokenError {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, ReturnTokenError::PatternNotMatched {..})
    }
}

impl Display for ReturnToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "return{}", if let Some(assignable) = &self.assignable {
            format!(" {}", assignable)
        } else {
            "".to_string()
        })
    }
}

impl Display for ReturnTokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ReturnTokenError::PatternNotMatched { target_value } => {
                format!("Pattern not matched for: `{}?\n\t return assignable;", target_value)
            }
            ReturnTokenError::AssignableError(e) => e.to_string(),
            ReturnTokenError::EmptyIterator(e) => e.to_string(),
        })
    }
}

impl Error for ReturnTokenError { }

impl From<AssignableTokenErr> for ReturnTokenError {
    fn from(value: AssignableTokenErr) -> Self {
        ReturnTokenError::AssignableError(value)
    }
}

impl From<anyhow::Error> for ReturnTokenError {
    fn from(value: anyhow::Error) -> Self {
        ReturnTokenError::PatternNotMatched { target_value: value.to_string() }
    }
}

impl TryParse for ReturnToken {
    type Output = ReturnToken;
    type Err = ReturnTokenError;

    fn try_parse(code_lines_iterator: &mut Peekable<Iter<CodeLine>>) -> anyhow::Result<Self::Output, Self::Err> {
        let code_line = *code_lines_iterator.peek().ok_or(ReturnTokenError::EmptyIterator(EmptyIteratorErr))?;
        ReturnToken::try_parse(code_line)
    }
}

impl ToASM for ReturnToken {
    fn to_asm<T: ASMOptions>(&self, stack: &mut Stack, meta: &mut MetaInfo, _options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        let mut target = String::new();
        target += &ASMBuilder::ident(&ASMBuilder::comment_line(&format!("{}", self)));

        if let Some(assignable) = &self.assignable {
            let destination_register = return_calling_convention(stack, meta)?.to_size_register_ignore_float(
                &ByteSize::try_from(meta.static_type_information.expected_return_type.as_ref().map_or(8, |t| t.return_type.byte_size()))?
            );
            let options = InterimResultOption {
                general_purpose_register: destination_register.clone(),
            };

            let source = assignable.to_asm(stack, meta, Some(options))?;

            match source {
                ASMResult::Inline(source) => target += &ASMBuilder::mov_ident_line(destination_register, source),
                ASMResult::MultilineResulted(source, r) => {
                    target += &source;

                    if let GeneralPurposeRegister::Float(f) = r {
                        target += &ASMBuilder::mov_x_ident_line(destination_register, f, Some(assignable.byte_size(meta)));
                    }
                }
                ASMResult::Multiline(_) => return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                    expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                    actual: ASMResultVariance::Multiline,
                    token: "Return".to_string(),
                }))

            }
        }

        target += &ASMBuilder::ident_line("leave");
        target += &ASMBuilder::ident_line("ret");

        Ok(ASMResult::Multiline(target))
    }

    fn is_stack_look_up(&self, stack: &mut Stack, meta: &MetaInfo) -> bool {
        if let Some(assignable) = &self.assignable {
            return assignable.is_stack_look_up(stack, meta);
        }

        false
    }

    fn byte_size(&self, meta: &mut MetaInfo) -> usize {
        if let Some(assignable) = &self.assignable {
            return assignable.byte_size(meta)
        }

        0
    }

    fn data_section(&self, stack: &mut Stack, meta: &mut MetaInfo) -> bool {
        if let Some(assignable) = &self.assignable {
            assignable.data_section(stack, meta)
        } else {
            false
        }
    }
}

impl ReturnToken {
    pub fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self, ReturnTokenError> {
        let split_alloc = code_line.split(vec![' ', ';']);
        let split = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();

        if let ["return", assignable @ .., ";"] = &split[..] {
            let joined = &assignable.join(" ");

            Ok(ReturnToken {
                assignable: Some(AssignableToken::from_str(joined)?),
                code_line: code_line.clone(),
            })
        } else if let ["return", ";"] = &split[..] {
            return Ok(ReturnToken {
                assignable: None,
                code_line: code_line.clone(),
            })
        } else {
            Err(ReturnTokenError::PatternNotMatched { target_value: code_line.line.clone() })
        }
    }
}
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;
use std::str::FromStr;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::asm_result::{ASMResult, ASMResultError, ASMResultVariance};
use crate::core::code_generator::conventions::return_calling_convention;
use crate::core::code_generator::registers::{ByteSize, GeneralPurposeRegister};
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::errors::EmptyIteratorErr;
use crate::core::lexer::scope::PatternNotMatchedError;
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::core::lexer::tokens::assignable_tokens::integer_token::IntegerToken;
use crate::core::lexer::TryParse;
use crate::core::lexer::types::integer::Integer;
use crate::core::lexer::types::type_token::{InferTypeError};
use crate::core::type_checker::static_type_checker::StaticTypeCheckError;
use crate::core::type_checker::StaticTypeCheck;

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnToken {
    pub assignable: Option<AssignableToken>,
    pub code_line: CodeLine
}

impl ReturnToken {
    /// returns a ReturnToken with an assignable, the assignable is an integer containing 0
    pub fn num_0() -> ReturnToken {
        ReturnToken {
            assignable: Some(AssignableToken::IntegerToken(IntegerToken { value: "0".to_string(), ty: Integer::I32 })),
            code_line: CodeLine::imaginary("return 0;")
        }
    }
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

impl StaticTypeCheck for ReturnToken {
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        if let Some(expected_return_type) = &type_context.expected_return_type {
            if let Some(assignable) = &self.assignable {
                let actual_type = assignable.infer_type_with_context(type_context, &self.code_line)?;

                if expected_return_type.return_type != actual_type {
                    return Err(StaticTypeCheckError::InferredError(InferTypeError::MethodReturnArgumentTypeMismatch {
                        expected: expected_return_type.return_type.clone(),
                        actual: actual_type,
                        code_line: self.code_line.clone(),
                    }));
                }
            }
        }

        Ok(())
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
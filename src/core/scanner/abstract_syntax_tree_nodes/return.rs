use std::error::Error;
use std::fmt::{Display, Formatter};
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
use crate::core::scanner::errors::EmptyIteratorErr;
use crate::core::scanner::scope::PatternNotMatchedError;
use crate::core::scanner::static_type_context::StaticTypeContext;
use crate::core::scanner::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableErr};
use crate::core::scanner::{Lines, TryParse};
use crate::core::scanner::types::r#type::{InferTypeError};
use crate::core::semantics::type_checker::static_type_checker::StaticTypeCheckError;
use crate::core::semantics::type_checker::StaticTypeCheck;

type Integer = crate::core::scanner::abstract_syntax_tree_nodes::assignables::integer::IntegerAST;
type IntegerType = crate::core::scanner::types::integer::Integer;

#[derive(Debug, PartialEq, Clone)]
pub struct Return {
    pub assignable: Option<Assignable>,
    pub code_line: CodeLine
}

impl Return {
    /// returns a `Return` with an assignable, the assignable is an integer containing 0
    pub fn num_0() -> Return {
        Return {
            assignable: Some(Assignable::Integer(Integer { value: "0".to_string(), ty: IntegerType::I32 })),
            code_line: CodeLine::imaginary("return 0;")
        }
    }
}

#[derive(Debug)]
pub enum ReturnError {
    PatternNotMatched { target_value: String },
    AssignableError(AssignableErr),
    EmptyIterator(EmptyIteratorErr)
}

impl PatternNotMatchedError for ReturnError {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, ReturnError::PatternNotMatched {..})
    }
}

impl Display for Return {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "return{}", if let Some(assignable) = &self.assignable {
            format!(" {}", assignable)
        } else {
            "".to_string()
        })
    }
}

impl Display for ReturnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ReturnError::PatternNotMatched { target_value } => {
                format!("Pattern not matched for: `{}?\n\t return assignable;", target_value)
            }
            ReturnError::AssignableError(e) => e.to_string(),
            ReturnError::EmptyIterator(e) => e.to_string(),
        })
    }
}

impl Error for ReturnError { }

impl From<AssignableErr> for ReturnError {
    fn from(value: AssignableErr) -> Self {
        ReturnError::AssignableError(value)
    }
}

impl From<anyhow::Error> for ReturnError {
    fn from(value: anyhow::Error) -> Self {
        ReturnError::PatternNotMatched { target_value: value.to_string() }
    }
}

impl StaticTypeCheck for Return {
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        if let Some(expected_return_type) = &type_context.expected_return_type {
            if let Some(assignable) = &self.assignable {
                let actual_type = assignable.infer_type_with_context(type_context, &self.code_line)?;

                if expected_return_type.return_type < actual_type {
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

impl TryParse for Return {
    type Output = Return;
    type Err = ReturnError;

    fn try_parse(code_lines_iterator: &mut Lines<'_>) -> anyhow::Result<Self::Output, Self::Err> {
        let code_line = *code_lines_iterator.peek().ok_or(ReturnError::EmptyIterator(EmptyIteratorErr))?;
        Return::try_parse(code_line)
    }
}

impl ToASM for Return {
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
                    ast_node: "Return".to_string(),
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

impl Return {
    pub fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self, ReturnError> {
        let split_alloc = code_line.split(vec![' ', ';']);
        let split = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();

        if let ["return", assignable @ .., ";"] = &split[..] {
            let joined = &assignable.join(" ");

            Ok(Return {
                assignable: Some(Assignable::from_str(joined)?),
                code_line: code_line.clone(),
            })
        } else if let ["return", ";"] = &split[..] {
            return Ok(Return {
                assignable: None,
                code_line: code_line.clone(),
            })
        } else {
            Err(ReturnError::PatternNotMatched { target_value: code_line.line.clone() })
        }
    }
}
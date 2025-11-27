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
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::model::abstract_syntax_tree_nodes::ret::Return;
use crate::core::model::types::integer::{IntegerAST, IntegerType};
use crate::core::scanner::errors::EmptyIteratorErr;
use crate::core::scanner::scope::PatternNotMatchedError;
use crate::core::scanner::static_type_context::StaticTypeContext;
use crate::core::scanner::{Lines, TryParse};
use crate::core::scanner::types::r#type::{InferTypeError};
use crate::core::semantics::type_checker::static_type_checker::StaticTypeCheckError;
use crate::core::semantics::type_checker::StaticTypeCheck;

impl Return {
    /// returns a `Return` with an assignable, the assignable is an integer containing 0
    pub fn num_0() -> Return {
        Return {
            assignable: Some(Assignable::Integer(IntegerAST { value: "0".to_string(), ty: IntegerType::I32 })),
            code_line: CodeLine::imaginary("return 0;")
        }
    }
}

#[derive(Debug)]
pub enum ReturnError {
    PatternNotMatched { target_value: String },
    AssignableError(AssignableError),
    EmptyIterator(EmptyIteratorErr)
}

impl PatternNotMatchedError for ReturnError {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, ReturnError::PatternNotMatched {..})
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

impl From<AssignableError> for ReturnError {
    fn from(value: AssignableError) -> Self {
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
use std::fmt::{Debug, Display, Formatter};

use crate::core::io::code_line::CodeLine;
use crate::core::lexer::scope::Scope;
use crate::core::lexer::static_type_context::{StaticTypeContext};
use crate::core::lexer::token::Token;
use crate::core::lexer::tokens::assignable_token::AssignableToken;
use crate::core::lexer::tokens::l_value::LValue;
use crate::core::lexer::types::type_token::{InferTypeError, MethodCallSignatureMismatchCause, TypeToken};
use crate::core::type_checker::StaticTypeCheck;

#[derive(Debug)]
pub enum StaticTypeCheckError {
    UnresolvedReference { name: LValue, code_line: CodeLine },
    NoTypePresent { name: LValue, code_line: CodeLine },
    VoidType { assignable_token: AssignableToken, code_line: CodeLine },
    ImmutabilityViolated { name: LValue, code_line: CodeLine },
    InferredError(InferTypeError),
}

impl std::error::Error for StaticTypeCheckError {}

impl Display for StaticTypeCheckError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            StaticTypeCheckError::UnresolvedReference { name, code_line } => format!("Line: {:?}\tUnresolved reference: `{name}`", code_line.actual_line_number),
            StaticTypeCheckError::InferredError(err) => err.to_string(),
            StaticTypeCheckError::NoTypePresent { name, code_line } => format!("Line: {:?}\tType not inferred: `{name}`", code_line.actual_line_number),
            StaticTypeCheckError::ImmutabilityViolated { name, code_line } => format!("Line: {:?}\tThis symbol isn't declared mutable: `{name}`", code_line.actual_line_number),
            StaticTypeCheckError::VoidType { assignable_token, code_line } => format!("Line: {:?}\tCannot assign void to a variable: `{assignable_token}`", code_line.actual_line_number),
        })
    }
}

impl From<InferTypeError> for StaticTypeCheckError {
    fn from(value: InferTypeError) -> Self {
        StaticTypeCheckError::InferredError(value)
    }
}

pub fn static_type_check(scope: &Scope) -> Result<(), StaticTypeCheckError> {
    // check if a variable, which is not a defined variable has an invalid re-assignment
    // let a = 1.0;
    // a = 5;
    let mut type_context: StaticTypeContext = StaticTypeContext::new(&scope.tokens);
    type_context.colliding_symbols()?;
    static_type_check_rec(&scope.tokens, &mut type_context)
}

pub fn static_type_check_rec(scope: &Vec<Token>, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
    if let Some(ty) = &type_context.expected_return_type {
        if scope.is_empty() && ty.return_type != TypeToken::Void {
            return Err(StaticTypeCheckError::InferredError(InferTypeError::MethodReturnSignatureMismatch {
                expected: ty.return_type.clone(),
                method_name: ty.method_name.to_string(),
                method_head_line: ty.method_header_line.clone(),
                cause: MethodCallSignatureMismatchCause::ReturnMissing,
            }));
        }
    }

    for token in scope {
        token.static_type_check(type_context)?;
    }


    Ok(())
}
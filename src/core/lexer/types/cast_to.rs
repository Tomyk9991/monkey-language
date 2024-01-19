use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::lexer::types::float::Float::{Float32, Float64};
use crate::core::lexer::types::type_token::TypeToken;



#[derive(Debug, Clone)]
pub struct CastTo {
    pub from: TypeToken,
    pub to: TypeToken
}

#[derive(Debug)]
pub enum CastToError {
    CastUnsupported(CastTo),
    CastTypesIdentical(CastTo),
}



impl ToASM for CastTo {
    /// returns the needed instruction to actually convert
    fn to_asm(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Result<String, ASMGenerateError> {
        // from, to, instruction
        let mut cast_to_matrix: HashMap<(TypeToken, TypeToken), &'static str> = HashMap::new();

        cast_to_matrix.insert((TypeToken::Float(Float32), TypeToken::Float(Float64)), "cvtss2sd");
        cast_to_matrix.insert((TypeToken::Float(Float64), TypeToken::Float(Float32)), "cvtsd2ss");

        if self.from == self.to {
            return Err(ASMGenerateError::CastUnsupported(CastToError::CastTypesIdentical(self.clone())))
        }


        if let Some(v) = cast_to_matrix.get(&(self.from.clone(), self.to.clone())) {
            return Ok(v.to_string())
        }

        Err(ASMGenerateError::CastUnsupported(CastToError::CastUnsupported(self.clone())))
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        0
    }

    fn before_label(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Option<Result<String, ASMGenerateError>> {
        None
    }
}

impl Error for CastToError { }

impl Display for CastToError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CastToError::CastUnsupported(cast_to) => format!("Cannot cast from '{}' to '{}'", cast_to.from, cast_to.to),
            CastToError::CastTypesIdentical(cast_to) => format!("Cannot cast from '{}' to '{}', types are identical", cast_to.from, cast_to.to),
        })
    }
}
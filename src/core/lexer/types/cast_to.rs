use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::core::lexer::types::float::Float;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, ASMOptions, ASMResult, MetaInfo, ToASM};
use crate::core::code_generator::registers::GeneralPurposeRegister;
use crate::core::lexer::types::boolean::Boolean;
use crate::core::lexer::types::integer::Integer;
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


pub trait Castable<T, K> {
    fn add_casts(cast_matrix: &mut HashMap<(TypeToken, TypeToken), &'static str>);
    fn cast_from_to(t1: &T, t2: &K, source: &str, stack: &mut Stack, meta: &mut MetaInfo) -> Result<ASMResult, ASMGenerateError>;
}


impl ToASM for CastTo {
    fn to_asm<T: ASMOptions>(&self, _stack: &mut Stack, meta: &mut MetaInfo, _options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        // from, to, instruction
        let mut cast_to_matrix: HashMap<(TypeToken, TypeToken), &'static str> = HashMap::new();

        <Integer as Castable<Integer, Integer>>::add_casts(&mut cast_to_matrix);
        <Integer as Castable<Integer, Float>>::add_casts(&mut cast_to_matrix);

        <Boolean as Castable<Boolean, Integer>>::add_casts(&mut cast_to_matrix);

        <Float as Castable<Float, Float>>::add_casts(&mut cast_to_matrix);
        <Float as Castable<Float, Integer>>::add_casts(&mut cast_to_matrix);

        if self.from == self.to {
            return Err(ASMGenerateError::CastUnsupported(CastToError::CastTypesIdentical(self.clone()), meta.code_line.clone()))
        }


        if let Some(v) = cast_to_matrix.get(&(self.from.clone(), self.to.clone())) {
            return Ok(ASMResult::Inline(v.to_string()))
        }

        Err(ASMGenerateError::CastUnsupported(CastToError::CastUnsupported(self.clone()), meta.code_line.clone()))
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

    fn multi_line_asm(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Result<(bool, String, Option<GeneralPurposeRegister>), ASMGenerateError> {
        Ok((false, String::new(), None))
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

impl CastTo {
    /// true, if the source type is bytewise bigger, than the destination type
    pub fn casting_down(&self) -> bool {
        self.from.byte_size() > self.to.byte_size()
    }
}
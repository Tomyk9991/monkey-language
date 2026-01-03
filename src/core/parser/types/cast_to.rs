use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::{ASMResult};
use crate::core::model::types::float::FloatType;
use crate::core::model::types::integer::IntegerType;
use crate::core::model::types::ty::Type;
use crate::core::parser::types::boolean::Boolean;



#[derive(Debug, Clone)]
pub struct CastTo {
    pub from: Type,
    pub to: Type
}

#[derive(Debug)]
pub enum CastToError {
    CastUnsupported(CastTo),
    CastTypesIdentical(CastTo),
}


pub trait Castable<T, K> {
    fn add_casts(cast_matrix: &mut HashMap<(Type, Type), &'static str>);
    fn cast_from_to(t1: &T, t2: &K, source: &str, stack: &mut Stack, meta: &mut MetaInfo) -> Result<ASMResult, ASMGenerateError>;
}


impl ToASM for CastTo {
    fn to_asm(&self, _stack: &mut Stack, meta: &mut MetaInfo, _options: Option<ASMOptions>) -> Result<ASMResult, ASMGenerateError> {
        // from, to, instruction
        let mut cast_to_matrix: HashMap<(Type, Type), &'static str> = HashMap::new();

        <IntegerType as Castable<IntegerType, IntegerType>>::add_casts(&mut cast_to_matrix);
        <IntegerType as Castable<IntegerType, FloatType>>::add_casts(&mut cast_to_matrix);

        <Boolean as Castable<Boolean, IntegerType>>::add_casts(&mut cast_to_matrix);

        <FloatType as Castable<FloatType, FloatType>>::add_casts(&mut cast_to_matrix);
        <FloatType as Castable<FloatType, IntegerType>>::add_casts(&mut cast_to_matrix);

        if self.from == self.to {
            return Err(ASMGenerateError::CastUnsupported(CastToError::CastTypesIdentical(self.clone()), meta.file_position.clone()))
        }


        if let Some(v) = cast_to_matrix.get(&(self.from.clone(), self.to.clone())) {
            return Ok(ASMResult::Inline(v.to_string()))
        }

        Err(ASMGenerateError::CastUnsupported(CastToError::CastUnsupported(self.clone()), meta.file_position.clone()))
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, _meta: &MetaInfo) -> usize {
        0
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
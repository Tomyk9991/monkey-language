use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::ASMResult;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::model::types::integer::{IntegerAST, IntegerType};
use std::fmt::{Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};


#[derive(Debug)]
pub enum NumberErr {
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError)
}

impl From<ParseIntError> for NumberErr {
    fn from(value: ParseIntError) -> Self {
        NumberErr::ParseIntError(value)
    }
}

impl From<ParseFloatError> for NumberErr {
    fn from(value: ParseFloatError) -> Self { NumberErr::ParseFloatError(value) }
}

impl std::error::Error for NumberErr { }

impl Display for NumberErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            NumberErr::ParseIntError(err) => err.to_string(),
            NumberErr::ParseFloatError(err) => err.to_string()
        })
    }
}

impl ToASM for IntegerAST {
    fn to_asm(&self, _stack: &mut Stack, _meta: &mut MetaInfo, _options: Option<ASMOptions>) -> Result<ASMResult, ASMGenerateError> {
        Ok(ASMResult::Inline(self.value.to_string()))
    }


    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, _meta: &MetaInfo) -> usize {
        match self.ty {
            IntegerType::I8 | IntegerType::U8 => 1,
            IntegerType::I16 | IntegerType::U16 => 2,
            IntegerType::I32 | IntegerType::U32 => 4,
            IntegerType::I64 | IntegerType::U64 => 8,
        }
    }
}
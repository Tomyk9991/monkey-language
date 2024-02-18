use std::any::Any;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_result::{ASMOptions, ASMResult, ASMResultError, InterimResultOption, PrepareRegisterOption};
use crate::core::code_generator::registers::{ByteSize, GeneralPurposeRegister};
use crate::core::lexer::tokens::assignable_tokens::integer_token::NumberTokenErr;
use crate::core::lexer::types::float::Float;

#[derive(Debug, PartialEq, Clone)]
pub struct FloatToken {
    // https://pastebin.com/DWcHQbT5
    // there is no need to use a string literal instead of a f64 like in the integer token, because
    // you cant have a float that's bigger than the biggest value of f64. but you can have a bigger value than a i64. consider every number that's between i64::MAX and u64::MAX
    pub value: f64,
    pub ty: Float
}

impl Display for FloatToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl ToASM for FloatToken {
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        if let Some(options) = options {
            let any_t = &options as &dyn Any;
            if let Some(concrete_type) = any_t.downcast_ref::<InterimResultOption>() {
                let value_str = if !self.value.to_string().contains('.') {
                    format!("{}.0", self.value)
                } else {
                    self.value.to_string()
                };

                return match self.ty {
                    Float::Float32 => Ok(ASMResult::MultilineResulted(
                        ASMBuilder::mov_ident_line(concrete_type.general_purpose_register.to_size_register(&ByteSize::_4), format!("__?float32?__({})", value_str)), concrete_type.general_purpose_register.clone())
                    ),
                    Float::Float64 => Ok(ASMResult::MultilineResulted(
                        ASMBuilder::mov_ident_line(concrete_type.general_purpose_register.to_size_register(&ByteSize::_8), format!("__?float64?__({})", value_str)), concrete_type.general_purpose_register.clone())
                    )
                }
            }

            if let Some(s) = any_t.downcast_ref::<PrepareRegisterOption>() {
                return s.transform(stack, meta);
            }
        }

        Err(ASMGenerateError::ASMResult(ASMResultError::NoOptionProvided("float".to_string())))
    }


    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        todo!()
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        match self.ty {
            Float::Float32 => 4,
            Float::Float64 => 8,
        }
    }

    fn before_label(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Option<Result<String, ASMGenerateError>> {
        None
    }

    fn multi_line_asm(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Result<(bool, String, Option<GeneralPurposeRegister>), ASMGenerateError> {
        Ok((false, String::new(), None))
    }
}


impl FromStr for FloatToken {
    type Err = NumberTokenErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !lazy_regex::regex_is_match!("^[+-]?(\\d+\\.\\d*|\\d*\\.\\d+)(_f64|_f32)?$", s) {
            return Err(NumberTokenErr::UnmatchedRegex);
        }

        let expected_type = match s {
            a if a.ends_with("_f64") => Float::Float64,
            a if a.ends_with("_f32") => Float::Float32,
            _ => Float::Float32
        };

        let s = s.replace("_f64", "").replace("_f32", "");

        let value = s.parse::<f64>()?;

        let final_type = if (-3.40282347e+38..=3.40282347e+38).contains(&value) {
            if matches!(expected_type, Float::Float64) {
                Float::Float64
            } else {
                Float::Float32
            }
        } else {
            if matches!(expected_type, Float::Float32) {
                return Err(NumberTokenErr::UnmatchedRegex)
            }
            Float::Float64
        };

        Ok(FloatToken {
            value,
            ty: final_type,
        })
    }
}
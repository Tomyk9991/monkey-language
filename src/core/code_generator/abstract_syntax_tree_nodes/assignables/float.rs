use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::{ASMResult, ASMResultError};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::registers::ByteSize;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::model::types::float::{FloatAST, FloatType};


impl ToASM for FloatAST {
    fn to_asm(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<ASMOptions>) -> Result<ASMResult, ASMGenerateError> {
        match options {
            Some(ASMOptions::InterimResultOption(concrete_type)) => {
                let value_str = if !self.value.to_string().contains('.') {
                    format!("{}.0", self.value)
                } else {
                    self.value.to_string()
                };

                match self.ty {
                    FloatType::Float32 => Ok(ASMResult::MultilineResulted(
                        ASMBuilder::mov_ident_line(concrete_type.general_purpose_register.to_size_register(&ByteSize::_4), format!("__?float32?__({})", value_str)), concrete_type.general_purpose_register.clone())
                    ),
                    FloatType::Float64 => Ok(ASMResult::MultilineResulted(
                        ASMBuilder::mov_ident_line(concrete_type.general_purpose_register.to_size_register(&ByteSize::_8), format!("__?float64?__({})", value_str)), concrete_type.general_purpose_register.clone())
                    )
                }
            },
            Some(ASMOptions::PrepareRegisterOption(s)) => {
                s.transform(stack, meta)
            },
            _ => Err(ASMGenerateError::ASMResult(ASMResultError::NoOptionProvided("float".to_string())))
        }
    }


    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        match self.ty {
            FloatType::Float32 => 4,
            FloatType::Float64 => 8,
        }
    }
}
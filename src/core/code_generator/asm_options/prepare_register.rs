use crate::core::code_generator::asm_result::{ASMResult, ASMResultError};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::registers::{ByteSize, GeneralPurposeRegister};
use crate::core::lexer::abstract_syntax_tree_nodes::assignable::Assignable;

/// Builds the assembly instructions to load a float AST node into a general purpose register
/// and finally to a register, where a float operation can be operated on
#[derive(Clone)]
pub struct PrepareRegisterOption {
    pub general_purpose_register: GeneralPurposeRegister,
    pub assignable: Option<Assignable>,
}

impl ASMOptions for PrepareRegisterOption {
    fn transform(&self, stack: &mut Stack, meta: &mut MetaInfo) -> Result<ASMResult, ASMGenerateError> {
        if let Some(Assignable::Float(float_node)) = &self.assignable {
            let size = float_node.byte_size(meta);
            let general_purpose_register_sized = self.general_purpose_register.to_size_register(&ByteSize::try_from(size)?);
            let float_register = &self.general_purpose_register.to_float_register();

            let mut target = match float_node.to_asm(stack, meta, Some(InterimResultOption::from(&general_purpose_register_sized)))? {
                ASMResult::Inline(t) | ASMResult::MultilineResulted(t, _) | ASMResult::Multiline(t) => t
            };

            target += &ASMBuilder::mov_x_ident_line(float_register, &general_purpose_register_sized, Some(size));
            return Ok(ASMResult::MultilineResulted(target, float_register.clone()));
        }

        if let Some(Assignable::Identifier(identifier)) = &self.assignable {
            let size = identifier.byte_size(meta);
            let general_purpose_register_sized = self.general_purpose_register.to_size_register(&ByteSize::try_from(size)?);
            let float_register = &self.general_purpose_register.to_float_register();

            let mut target = match identifier.to_asm::<InterimResultOption>(stack, meta, None)? {
                ASMResult::Inline(t) | ASMResult::MultilineResulted(t, _) | ASMResult::Multiline(t) => {
                    ASMBuilder::mov_ident_line(&general_purpose_register_sized, t)
                }
            };

            target += &ASMBuilder::mov_x_ident_line(float_register, &general_purpose_register_sized, Some(size));
            return Ok(ASMResult::MultilineResulted(target, float_register.clone()));
        }

        Err(ASMGenerateError::ASMResult(ASMResultError::NoOptionProvided("Wrong assignable in Float calculation".to_string())))
    }
}
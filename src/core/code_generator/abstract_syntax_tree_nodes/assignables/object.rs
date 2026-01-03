use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::ASMResult;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::register_destination::word_from_byte_size;
use crate::core::code_generator::registers::GeneralPurposeRegister;
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::abstract_syntax_tree_nodes::assignables::object::Object;

impl ToASM for Object {
    fn to_asm(&self, stack: &mut Stack, meta: &mut MetaInfo, _options: Option<ASMOptions>) -> Result<ASMResult, ASMGenerateError> {
        let mut target = String::new();
        target += &ASMBuilder::ident_comment_line(&format!("{}", self).replace("\n", "").replace("    ", " ").replace("}", " }"));
        let struct_def = meta.static_type_information.custom_defined_types
            .get(&self.ty)
            .cloned()
            .ok_or(ASMGenerateError::InternalError(
            format!("Struct definition for type {} not found", self.ty),
            meta.file_position.clone(),
        ))?;

        for field in &self.fields {
            let field_size = field.assignable.byte_size(meta);
            let options: Option<ASMOptions> = if field_size >= 0 && field_size <= 8 {
                Some(ASMOptions::InterimResultOption(InterimResultOption {
                    general_purpose_register: GeneralPurposeRegister::iter_from_byte_size(field.assignable.byte_size(meta))?.current().clone(),
                }))
            } else {
                None
            };

            target += &ASMBuilder::ident_comment_line(&format!("{}.{}", self.ty, field.l_value.identifier()));

            let field_asm = field.assignable.to_asm(stack, meta, options)?;
            let mut current_stack_position = stack.stack_position;

            match field_asm {
                ASMResult::Inline(source) => {
                    if field.assignable.is_stack_look_up(stack, meta) {
                        let destination_register = GeneralPurposeRegister::iter_from_byte_size(field.assignable.byte_size(meta))?.current();
                        let current_word = word_from_byte_size(field.assignable.byte_size(meta));
                        let struct_offset = format!("{}.{}", struct_def.ty, field.l_value.identifier());
                        let destination = format!("{current_word} [rsp - {current_stack_position} - {struct_offset}]");

                        target += &ASMBuilder::mov_x_ident_line(&destination_register, source, Some(destination_register.size() as usize));
                        target += &ASMBuilder::mov_ident_line(destination, &destination_register);
                    } else {
                        let current_word = word_from_byte_size(field.assignable.byte_size(meta));
                        let struct_offset = format!("{}.{}", struct_def.ty, field.l_value.identifier());
                        let destination = format!("{current_word} [rsp - {current_stack_position} - {struct_offset}]");

                        target += &ASMBuilder::mov_ident_line(destination, source);
                    }
                },
                ASMResult::MultilineResulted(t, r) => {
                    let current_word = word_from_byte_size(field.assignable.byte_size(meta));
                    let struct_offset = format!("{}.{}", struct_def.ty, field.l_value.identifier());
                    let destination = format!("{current_word} [rsp - {current_stack_position} - {struct_offset}]");

                    target += &t;
                    target += &ASMBuilder::mov_ident_line(destination, r);
                },
                ASMResult::Multiline(t) => {
                    target += &t;
                },
            }

            stack.stack_position += field_size;
        }
        
        stack.stack_position -= struct_def.byte_size(meta);

        Ok(ASMResult::Multiline(target))
    }

    fn data_section(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> bool {
        self.fields.iter().any(|f| f.assignable.data_section(_stack, _meta))
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        self.fields.iter().any(|f| f.assignable.is_stack_look_up(_stack, _meta))
    }

    fn byte_size(&self, meta: &MetaInfo) -> usize {
        meta.static_type_information.custom_defined_types.get(&self.ty)
            .cloned()
            .map(|struct_def| struct_def.byte_size(meta))
            .unwrap_or(0)
    }
}
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::ASMResult;
use crate::core::model::abstract_syntax_tree_nodes::struct_::Struct;

impl ToASM for Struct {
    fn to_asm(&self, _: &mut Stack, _: &mut MetaInfo, _: Option<ASMOptions>) -> Result<ASMResult, ASMGenerateError> {
        Ok(ASMResult::Inline(String::new()))
    }

    fn is_stack_look_up(&self, stack: &mut Stack, meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, _: &MetaInfo) -> usize {
        self.fields.iter().fold(0, |acc, field| acc + field.ty.byte_size())
    }

    fn data_section(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> bool {
        let mut target = String::new();
        target += &ASMBuilder::comment_line(&format!("{}", self).replace("\n", "").replace("    ", " ").replace("}", " }"));

        target += &ASMBuilder::line(&format!("struc {}", self.ty));

        let longest_field_name_length = self.fields.iter().map(|f| f.name.name.chars().count()).max().unwrap_or(0);

        for field in &self.fields {
            let padding = " ".repeat(longest_field_name_length - field.name.name.chars().count());
            target += &ASMBuilder::ident_line(&format!(".{}{}\tresb {}", field.name, padding, field.ty.byte_size()));
        }

        target += &ASMBuilder::line("endstruc");

        _stack.data_section.push_struct_definition(target);
        true
    }
}
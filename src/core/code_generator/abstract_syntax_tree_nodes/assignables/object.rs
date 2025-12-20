use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::ASMResult;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::model::abstract_syntax_tree_nodes::assignables::object::Object;


impl ToASM for Object {
    fn to_asm(&self, _stack: &mut Stack, _meta: &mut MetaInfo, _options: Option<ASMOptions>) -> Result<ASMResult, ASMGenerateError> {
        todo!()
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        8
    }
}
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::ASMResult;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;


impl ToASM for LValue {
    fn to_asm(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<ASMOptions>) -> Result<ASMResult, ASMGenerateError> {
        match self {
            LValue::Identifier(name) => name.to_asm(stack, meta, options),
            LValue::Expression(node) => node.to_asm(stack, meta, Some(ASMOptions::LValueExpressionOption)),
        }
    }

    fn is_stack_look_up(&self, stack: &mut Stack, meta: &MetaInfo) -> bool {
        match self {
            LValue::Identifier(a) => a.is_stack_look_up(stack, meta),
            LValue::Expression(node) => node.is_stack_look_up(stack, meta),
        }
    }

    fn byte_size(&self, meta: &MetaInfo) -> usize {
        match self {
            LValue::Identifier(a) => a.byte_size(meta),
            LValue::Expression(node) => node.byte_size(meta)
        }
    }
}
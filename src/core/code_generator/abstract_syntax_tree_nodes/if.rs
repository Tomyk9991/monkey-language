use std::fmt::{Display};
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::asm_result::{ASMResult, ASMResultError, ASMResultVariance};
use crate::core::code_generator::generator::Stack;
use crate::core::lexer::parse::{Parse};
use crate::core::model::abstract_syntax_tree_nodes::if_::{If};



impl ToASM for If {
    fn to_asm(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<ASMOptions>) -> Result<ASMResult, ASMGenerateError> {
        let mut target = String::new();

        target.push_str(&format!("    ; if condition ({})\n", self.condition));

        let continue_label = stack.create_label();
        let else_label = stack.create_label();

        let jump_label: &str = if self.else_stack.is_some() {
            &else_label
        } else {
            &continue_label
        };

        let result = format!("    cmp {}, 0\n", match &self.condition.to_asm(stack, meta, options.clone())? {
            ASMResult::Inline(t) => t.to_owned(),
            ASMResult::MultilineResulted(t, r) => {
                target += t;
                r.to_string()
            }
            ASMResult::Multiline(_) => return Err(ASMResultError::UnexpectedVariance {
                expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                actual: ASMResultVariance::Multiline,
                ast_node: "if node".to_string(),
            }.into())
        });
        target += &result;


        target.push_str(&format!("    je {}\n", jump_label));


        target.push_str(&ASMBuilder::ident_comment_line("if branch"));
        target.push_str(&stack.generate_scope(&self.if_stack, meta, options)?);
        target.push_str(&format!("    jmp {}\n", continue_label));


        if let Some(else_stack) = &self.else_stack {
            target.push_str(&format!("{}:\n", else_label));
            target.push_str(&ASMBuilder::ident_comment_line("else branch"));
            target.push_str(&stack.generate_scope(else_stack, meta, None)?);
        }

        target.push_str(&format!("{}:\n", continue_label));
        target.push_str("    ; Continue after if \n");
        Ok(ASMResult::Multiline(target))
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        true
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        0
    }

    fn data_section(&self, stack: &mut Stack, meta: &mut MetaInfo) -> bool {
        let mut has_before_label_asm = false;
        let count_before = stack.label_count;


        if self.condition.data_section(stack, meta) {
            has_before_label_asm = true;
            stack.label_count -= 1;
        }

        for node in &self.if_stack {
            if node.data_section(stack, meta) {
                has_before_label_asm = true;
                stack.label_count -= 1;
            }
        }

        if let Some(else_stack) = &self.else_stack {
            for node in else_stack {
                if node.data_section(stack, meta) {
                    has_before_label_asm = true;
                    stack.label_count -= 1;
                }
            }
        }

        stack.label_count = count_before;
        has_before_label_asm
    }
}

use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::ASMResult;
use crate::core::code_generator::generator::Stack;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::semantics::static_type_check::static_type_checker::StaticTypeCheckError;
use crate::core::semantics::static_type_check::StaticTypeCheck;

impl AbstractSyntaxTreeNode {
    pub fn scope(&self) -> Option<Vec<&Vec<AbstractSyntaxTreeNode>>> {
        match self {
            AbstractSyntaxTreeNode::Variable(_) | AbstractSyntaxTreeNode::MethodCall(_) |
            AbstractSyntaxTreeNode::Import(_) | AbstractSyntaxTreeNode::Return(_) |
            AbstractSyntaxTreeNode::StructDefinition(_) => None,
            AbstractSyntaxTreeNode::MethodDefinition(t) => Some(vec![&t.stack]),
            AbstractSyntaxTreeNode::If(t) => {
                let mut res = vec![&t.if_stack];
                if let Some(else_stack) = &t.else_stack {
                    res.push(else_stack);
                }

                Some(res)
            }
            AbstractSyntaxTreeNode::For(t) => Some(vec![&t.stack]),
            AbstractSyntaxTreeNode::While(t) => Some(vec![&t.stack]),
        }
    }
}

impl StaticTypeCheck for AbstractSyntaxTreeNode {
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        match self {
            AbstractSyntaxTreeNode::Variable(node) => node.static_type_check(type_context),
            AbstractSyntaxTreeNode::MethodCall(node) => node.static_type_check(type_context),
            AbstractSyntaxTreeNode::MethodDefinition(node) => node.static_type_check(type_context),
            AbstractSyntaxTreeNode::Import(node) => node.static_type_check(type_context),
            AbstractSyntaxTreeNode::Return(node) => node.static_type_check(type_context),
            AbstractSyntaxTreeNode::If(node) => node.static_type_check(type_context),
            AbstractSyntaxTreeNode::For(node) => node.static_type_check(type_context),
            AbstractSyntaxTreeNode::While(node) => node.static_type_check(type_context),
            AbstractSyntaxTreeNode::StructDefinition(node) => node.static_type_check(type_context),
        }
    }
}

impl ToASM for AbstractSyntaxTreeNode {
    fn to_asm(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<ASMOptions>) -> Result<ASMResult, ASMGenerateError> {
        let variables_len = meta.static_type_information.len();

        let scopes = match self {
            AbstractSyntaxTreeNode::If(if_def) => {
                let mut res = vec![&if_def.if_stack];
                if let Some(else_stack) = &if_def.else_stack {
                    res.push(else_stack)
                }

                res
            }
            AbstractSyntaxTreeNode::For(for_node) => {
                vec![&for_node.stack]
            }
            AbstractSyntaxTreeNode::While(while_node) => {
                vec![&while_node.stack]
            }
            AbstractSyntaxTreeNode::MethodDefinition(method_def) => {
                vec![&method_def.stack]
            }
            AbstractSyntaxTreeNode::Variable(_) | AbstractSyntaxTreeNode::MethodCall(_) |
            AbstractSyntaxTreeNode::Import(_) | AbstractSyntaxTreeNode::Return(_) |
            AbstractSyntaxTreeNode::StructDefinition(_)
            => vec![]
        };

        for scope in scopes {
            let scoped_checker = StaticTypeContext::new(scope);
            meta.static_type_information.merge(scoped_checker);

            let amount_pop = meta.static_type_information.len() - variables_len;

            for _ in 0..amount_pop {
                let _ = meta.static_type_information.pop();
            }
        }

        match self {
            AbstractSyntaxTreeNode::Variable(node) => node.to_asm(stack, meta, options),
            AbstractSyntaxTreeNode::MethodCall(node) => node.to_asm(stack, meta, options),
            AbstractSyntaxTreeNode::Return(node) => node.to_asm(stack, meta, options),
            AbstractSyntaxTreeNode::MethodDefinition(node) => node.to_asm(stack, meta, options),
            AbstractSyntaxTreeNode::Import(node) => node.to_asm(stack, meta, options),
            AbstractSyntaxTreeNode::If(node) => node.to_asm(stack, meta, options),
            AbstractSyntaxTreeNode::For(node) => node.to_asm(stack, meta, options),
            AbstractSyntaxTreeNode::While(node) => node.to_asm(stack, meta, options),
            AbstractSyntaxTreeNode::StructDefinition(node) => node.to_asm(stack, meta, options)
        }
    }


    fn is_stack_look_up(&self, stack: &mut Stack, meta: &MetaInfo) -> bool {
        match self {
            AbstractSyntaxTreeNode::Variable(node) => node.is_stack_look_up(stack, meta),
            AbstractSyntaxTreeNode::MethodCall(node) => node.is_stack_look_up(stack, meta),
            AbstractSyntaxTreeNode::If(node) => node.is_stack_look_up(stack, meta),
            AbstractSyntaxTreeNode::Import(node) => node.is_stack_look_up(stack, meta),
            AbstractSyntaxTreeNode::For(node) => node.is_stack_look_up(stack, meta),
            AbstractSyntaxTreeNode::While(node) => node.is_stack_look_up(stack, meta),
            AbstractSyntaxTreeNode::MethodDefinition(node) => node.is_stack_look_up(stack, meta),
            AbstractSyntaxTreeNode::Return(node) => node.is_stack_look_up(stack, meta),
            AbstractSyntaxTreeNode::StructDefinition(node) => node.is_stack_look_up(stack, meta),
        }
    }

    fn byte_size(&self, meta: &MetaInfo) -> usize {
        match self {
            AbstractSyntaxTreeNode::Variable(node) => node.byte_size(meta),
            AbstractSyntaxTreeNode::MethodCall(node) => node.byte_size(meta),
            AbstractSyntaxTreeNode::MethodDefinition(node) => node.byte_size(meta),
            AbstractSyntaxTreeNode::Import(node) => node.byte_size(meta),
            AbstractSyntaxTreeNode::For(node) => node.byte_size(meta),
            AbstractSyntaxTreeNode::While(node) => node.byte_size(meta),
            AbstractSyntaxTreeNode::If(node) => node.byte_size(meta),
            AbstractSyntaxTreeNode::Return(node) => node.byte_size(meta),
            AbstractSyntaxTreeNode::StructDefinition(node) => node.byte_size(meta),
        }
    }


    fn data_section(&self, stack: &mut Stack, meta: &mut MetaInfo) -> bool {
        match self {
            AbstractSyntaxTreeNode::Variable(node) => node.data_section(stack, meta),
            AbstractSyntaxTreeNode::MethodCall(node) => node.data_section(stack, meta),
            AbstractSyntaxTreeNode::MethodDefinition(node) => node.data_section(stack, meta),
            AbstractSyntaxTreeNode::Import(node) => node.data_section(stack, meta),
            AbstractSyntaxTreeNode::For(node) => node.data_section(stack, meta),
            AbstractSyntaxTreeNode::While(node) => node.data_section(stack, meta),
            AbstractSyntaxTreeNode::If(node) => node.data_section(stack, meta),
            AbstractSyntaxTreeNode::Return(node) => node.data_section(stack, meta),
            AbstractSyntaxTreeNode::StructDefinition(node) => node.data_section(stack, meta),
        }
    }
}


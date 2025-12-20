use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::ASMResult;
use crate::core::code_generator::generator::Stack;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::InferTypeError;
use crate::core::semantics::static_type_check::static_type_check::StaticTypeCheck;
use crate::core::semantics::static_type_check::static_type_checker::StaticTypeCheckError;

impl AbstractSyntaxTreeNode {
    pub fn scope(&self) -> Option<Vec<&Vec<AbstractSyntaxTreeNode>>> {
        match self {
            AbstractSyntaxTreeNode::Variable(_) | AbstractSyntaxTreeNode::MethodCall(_) | AbstractSyntaxTreeNode::Import(_) | AbstractSyntaxTreeNode::Return(_) => None,
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
            AbstractSyntaxTreeNode::Variable(variable) => variable.static_type_check(type_context),
            AbstractSyntaxTreeNode::MethodCall(method_call) => method_call.static_type_check(type_context),
            AbstractSyntaxTreeNode::MethodDefinition(method_definition) => method_definition.static_type_check(type_context),
            AbstractSyntaxTreeNode::Import(import) => import.static_type_check(type_context),
            AbstractSyntaxTreeNode::Return(return_node) => return_node.static_type_check(type_context),
            AbstractSyntaxTreeNode::If(if_node) => if_node.static_type_check(type_context),
            AbstractSyntaxTreeNode::For(for_node) => for_node.static_type_check(type_context),
            AbstractSyntaxTreeNode::While(while_node) => while_node.static_type_check(type_context),
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
            AbstractSyntaxTreeNode::Import(_) | AbstractSyntaxTreeNode::Return(_)
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
            AbstractSyntaxTreeNode::Variable(variable) => variable.to_asm(stack, meta, options),
            AbstractSyntaxTreeNode::MethodCall(method_call) => method_call.to_asm(stack, meta, options),
            AbstractSyntaxTreeNode::Return(return_node) => return_node.to_asm(stack, meta, options),
            AbstractSyntaxTreeNode::MethodDefinition(return_node) => return_node.to_asm(stack, meta, options),
            AbstractSyntaxTreeNode::Import(import_node) => import_node.to_asm(stack, meta, options),
            AbstractSyntaxTreeNode::If(if_node) => if_node.to_asm(stack, meta, options),
            AbstractSyntaxTreeNode::For(for_node) => for_node.to_asm(stack, meta, options),
            AbstractSyntaxTreeNode::While(while_node) => while_node.to_asm(stack, meta, options),
        }
    }


    fn is_stack_look_up(&self, stack: &mut Stack, meta: &MetaInfo) -> bool {
        match self {
            AbstractSyntaxTreeNode::Variable(a) => a.is_stack_look_up(stack, meta),
            AbstractSyntaxTreeNode::MethodCall(a) => a.is_stack_look_up(stack, meta),
            AbstractSyntaxTreeNode::If(a) => a.is_stack_look_up(stack, meta),
            AbstractSyntaxTreeNode::Import(a) => a.is_stack_look_up(stack, meta),
            AbstractSyntaxTreeNode::For(a) => a.is_stack_look_up(stack, meta),
            AbstractSyntaxTreeNode::While(a) => a.is_stack_look_up(stack, meta),
            AbstractSyntaxTreeNode::MethodDefinition(a) => a.is_stack_look_up(stack, meta),
            AbstractSyntaxTreeNode::Return(return_type) => return_type.is_stack_look_up(stack, meta),
        }
    }

    fn byte_size(&self, meta: &mut MetaInfo) -> usize {
        match self {
            AbstractSyntaxTreeNode::Variable(a) => a.byte_size(meta),
            AbstractSyntaxTreeNode::MethodCall(a) => a.byte_size(meta),
            AbstractSyntaxTreeNode::MethodDefinition(a) => a.byte_size(meta),
            AbstractSyntaxTreeNode::Import(a) => a.byte_size(meta),
            AbstractSyntaxTreeNode::For(a) => a.byte_size(meta),
            AbstractSyntaxTreeNode::While(a) => a.byte_size(meta),
            AbstractSyntaxTreeNode::If(a) => a.byte_size(meta),
            AbstractSyntaxTreeNode::Return(r) => r.byte_size(meta),
        }
    }


    fn data_section(&self, stack: &mut Stack, meta: &mut MetaInfo) -> bool {
        match self {
            AbstractSyntaxTreeNode::Variable(v) => v.data_section(stack, meta),
            AbstractSyntaxTreeNode::MethodCall(v) => v.data_section(stack, meta),
            AbstractSyntaxTreeNode::MethodDefinition(v) => v.data_section(stack, meta),
            AbstractSyntaxTreeNode::Import(v) => v.data_section(stack, meta),
            AbstractSyntaxTreeNode::For(v) => v.data_section(stack, meta),
            AbstractSyntaxTreeNode::While(v) => v.data_section(stack, meta),
            AbstractSyntaxTreeNode::If(v) => v.data_section(stack, meta),
            AbstractSyntaxTreeNode::Return(ret) => ret.data_section(stack, meta),
        }
    }
}


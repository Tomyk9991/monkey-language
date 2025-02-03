use std::fmt::{Debug, Display, Formatter};

use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::{ASMResult};
use crate::core::code_generator::generator::Stack;
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use crate::core::lexer::abstract_syntax_tree_nodes::r#for::For;
use crate::core::lexer::abstract_syntax_tree_nodes::r#if::If;
use crate::core::lexer::abstract_syntax_tree_nodes::import::Import;
use crate::core::lexer::abstract_syntax_tree_nodes::method_definition::MethodDefinition;
use crate::core::lexer::abstract_syntax_tree_nodes::r#while::While;
use crate::core::lexer::abstract_syntax_tree_nodes::r#return::Return;
use crate::core::lexer::abstract_syntax_tree_nodes::scope_ending::ScopeEnding;
use crate::core::lexer::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::lexer::types::r#type::InferTypeError;
use crate::core::type_checker::{InferType, StaticTypeCheck};
use crate::core::type_checker::static_type_checker::StaticTypeCheckError;

/// An abstract syntax tree node is a piece of code that is used to represent atomic elements of a program.
#[derive(Debug, PartialEq, Clone)]
pub enum AbstractSyntaxTreeNode {
    Variable(Variable<'=', ';'>),
    MethodCall(MethodCall),
    MethodDefinition(MethodDefinition),
    Import(Import),
    Return(Return),
    ScopeClosing(ScopeEnding),
    If(If),
    For(For),
    While(While)
}

impl AbstractSyntaxTreeNode {
    pub(crate) fn code_line(&self) -> CodeLine {
        match self {
            AbstractSyntaxTreeNode::Variable(a) => a.code_line.clone(),
            AbstractSyntaxTreeNode::MethodCall(a) => a.code_line.clone(),
            AbstractSyntaxTreeNode::MethodDefinition(a) => a.code_line.clone(),
            AbstractSyntaxTreeNode::ScopeClosing(a) => a.code_line.clone(),
            AbstractSyntaxTreeNode::If(a) => a.code_line.clone(),
            AbstractSyntaxTreeNode::Import(a) => a.code_line.clone(),
            AbstractSyntaxTreeNode::Return(a) => a.code_line.clone(),
            AbstractSyntaxTreeNode::For(a) => a.code_line.clone(),
            AbstractSyntaxTreeNode::While(a) => a.code_line.clone(),
        }
    }

    pub fn scope(&self) -> Option<Vec<&Vec<AbstractSyntaxTreeNode>>> {
        match self {
            AbstractSyntaxTreeNode::Variable(_) | AbstractSyntaxTreeNode::MethodCall(_) | AbstractSyntaxTreeNode::Import(_) | AbstractSyntaxTreeNode::Return(_) | AbstractSyntaxTreeNode::ScopeClosing(_) => None,
            AbstractSyntaxTreeNode::MethodDefinition(t) => Some(vec![&t.stack]),
            AbstractSyntaxTreeNode::If(t) => {
                let mut res = vec![&t.if_stack];
                if let Some(else_stack) = &t.else_stack {
                    res.push(else_stack);
                }

                Some(res)
            }
            AbstractSyntaxTreeNode::For(t) => Some(vec![&t.stack]),
            AbstractSyntaxTreeNode::While(t) => Some(vec![&t.stack])
        }
    }
}

impl AbstractSyntaxTreeNode {
    pub fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<(), InferTypeError> {
        match self {
            AbstractSyntaxTreeNode::Variable(variable) => {
                variable.infer_type(type_context)?;
            }
            AbstractSyntaxTreeNode::If(if_definition) => {
                if_definition.infer_type(type_context)?;
            },
            AbstractSyntaxTreeNode::For(for_loop) => {
                for_loop.infer_type(type_context)?;
            }
            AbstractSyntaxTreeNode::While(while_loop) => {
                while_loop.infer_type(type_context)?;
            }
            AbstractSyntaxTreeNode::MethodDefinition(_) | AbstractSyntaxTreeNode::MethodCall(_) | AbstractSyntaxTreeNode::ScopeClosing(_) | AbstractSyntaxTreeNode::Import(_) | AbstractSyntaxTreeNode::Return(_) => {}
        }

        Ok(())
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
            AbstractSyntaxTreeNode::ScopeClosing(scope_closing) => scope_closing.static_type_check(type_context),
            AbstractSyntaxTreeNode::If(if_node) => if_node.static_type_check(type_context),
            AbstractSyntaxTreeNode::For(for_node) => for_node.static_type_check(type_context),
            AbstractSyntaxTreeNode::While(while_node) => while_node.static_type_check(type_context)
        }
    }
}

impl ToASM for AbstractSyntaxTreeNode {
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
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
            AbstractSyntaxTreeNode::ScopeClosing(_) => vec![]
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
            AbstractSyntaxTreeNode::ScopeClosing(_) => Ok(ASMResult::Inline(String::new())),
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
            AbstractSyntaxTreeNode::ScopeClosing(_) => false,
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
            AbstractSyntaxTreeNode::ScopeClosing(_) => 0,
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
            AbstractSyntaxTreeNode::ScopeClosing(_) => false,
        }
    }
}

impl Display for AbstractSyntaxTreeNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            AbstractSyntaxTreeNode::Variable(v) => format!("{}", v),
            AbstractSyntaxTreeNode::MethodCall(m) => format!("{}", m),
            AbstractSyntaxTreeNode::MethodDefinition(m) => format!("{}", m),
            AbstractSyntaxTreeNode::ScopeClosing(m) => format!("{}", m),
            AbstractSyntaxTreeNode::If(m) => format!("{}", m),
            AbstractSyntaxTreeNode::Import(m) => format!("{}", m),
            AbstractSyntaxTreeNode::Return(m) => format!("{}", m),
            AbstractSyntaxTreeNode::While(a) => format!("{}", a),
            AbstractSyntaxTreeNode::For(m) => format!("{}", m),
        })
    }
}


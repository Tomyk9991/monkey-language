use std::fmt::{Debug, Display, Formatter};

use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::{ASMResult};
use crate::core::code_generator::generator::Stack;
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::tokens::assignable_tokens::method_call_token::MethodCallToken;
use crate::core::lexer::tokens::for_token::ForToken;
use crate::core::lexer::tokens::if_token::IfToken;
use crate::core::lexer::tokens::import_token::ImportToken;
use crate::core::lexer::tokens::method_definition::MethodDefinition;
use crate::core::lexer::tokens::r#while::WhileToken;
use crate::core::lexer::tokens::return_token::ReturnToken;
use crate::core::lexer::tokens::scope_ending::ScopeEnding;
use crate::core::lexer::tokens::variable_token::VariableToken;
use crate::core::lexer::types::type_token::InferTypeError;
use crate::core::type_checker::{InferType, StaticTypeCheck};
use crate::core::type_checker::static_type_checker::StaticTypeCheckError;

/// A token is a piece of code that is used to represent atomic elements of a program.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Variable(VariableToken<'=', ';'>),
    MethodCall(MethodCallToken),
    MethodDefinition(MethodDefinition),
    Import(ImportToken),
    Return(ReturnToken),
    ScopeClosing(ScopeEnding),
    If(IfToken),
    For(ForToken),
    While(WhileToken)
}

impl Token {
    pub(crate) fn code_line(&self) -> CodeLine {
        match self {
            Token::Variable(a) => a.code_line.clone(),
            Token::MethodCall(a) => a.code_line.clone(),
            Token::MethodDefinition(a) => a.code_line.clone(),
            Token::ScopeClosing(a) => a.code_line.clone(),
            Token::If(a) => a.code_line.clone(),
            Token::Import(a) => a.code_line.clone(),
            Token::Return(a) => a.code_line.clone(),
            Token::For(a) => a.code_line.clone(),
            Token::While(a) => a.code_line.clone(),
        }
    }

    pub fn scope(&self) -> Option<Vec<&Vec<Token>>> {
        match self {
            Token::Variable(_) | Token::MethodCall(_) | Token::Import(_) | Token::Return(_) | Token::ScopeClosing(_) => None,
            Token::MethodDefinition(t) => Some(vec![&t.stack]),
            Token::If(t) => {
                let mut res = vec![&t.if_stack];
                if let Some(else_stack) = &t.else_stack {
                    res.push(else_stack);
                }

                Some(res)
            }
            Token::For(t) => Some(vec![&t.stack]),
            Token::While(t) => Some(vec![&t.stack])
        }
    }
}

impl Token {
    pub fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<(), InferTypeError> {
        match self {
            Token::Variable(variable) => {
                variable.infer_type(type_context)?;
            }
            Token::If(if_definition) => {
                if_definition.infer_type(type_context)?;
            },
            Token::For(for_loop) => {
                for_loop.infer_type(type_context)?;
            }
            Token::While(while_loop) => {
                while_loop.infer_type(type_context)?;
            }
            Token::MethodDefinition(_) | Token::MethodCall(_) | Token::ScopeClosing(_) | Token::Import(_) | Token::Return(_) => {}
        }

        Ok(())
    }
}

impl StaticTypeCheck for Token {
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        match self {
            Token::Variable(variable_token) => variable_token.static_type_check(type_context),
            Token::MethodCall(method_call) => method_call.static_type_check(type_context),
            Token::MethodDefinition(method_definition) => method_definition.static_type_check(type_context),
            Token::Import(import_token) => import_token.static_type_check(type_context),
            Token::Return(return_token) => return_token.static_type_check(type_context),
            Token::ScopeClosing(scope_closing) => scope_closing.static_type_check(type_context),
            Token::If(if_token) => if_token.static_type_check(type_context),
            Token::For(for_token) => for_token.static_type_check(type_context),
            Token::While(while_token) => while_token.static_type_check(type_context)
        }
    }
}

impl ToASM for Token {
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        let variables_len = meta.static_type_information.len();

        let scopes = match self {
            Token::If(if_def) => {
                let mut res = vec![&if_def.if_stack];
                if let Some(else_stack) = &if_def.else_stack {
                    res.push(else_stack)
                }

                res
            }
            Token::For(for_token) => {
                vec![&for_token.stack]
            }
            Token::While(while_token) => {
                vec![&while_token.stack]
            }
            Token::MethodDefinition(method_def) => {
                vec![&method_def.stack]
            }
            Token::Variable(_) | Token::MethodCall(_) |
            Token::Import(_) | Token::Return(_) |
            Token::ScopeClosing(_) => vec![]
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
            Token::Variable(variable) => variable.to_asm(stack, meta, options),
            Token::MethodCall(method_call) => method_call.to_asm(stack, meta, options),
            Token::Return(return_token) => return_token.to_asm(stack, meta, options),
            Token::MethodDefinition(return_token) => return_token.to_asm(stack, meta, options),
            Token::Import(import_token) => import_token.to_asm(stack, meta, options),
            Token::If(if_token) => if_token.to_asm(stack, meta, options),
            Token::For(for_loop) => for_loop.to_asm(stack, meta, options),
            Token::While(while_loop) => while_loop.to_asm(stack, meta, options),
            Token::ScopeClosing(_) => Ok(ASMResult::Inline(String::new())),
        }
    }


    fn is_stack_look_up(&self, stack: &mut Stack, meta: &MetaInfo) -> bool {
        match self {
            Token::Variable(a) => a.is_stack_look_up(stack, meta),
            Token::MethodCall(a) => a.is_stack_look_up(stack, meta),
            Token::If(a) => a.is_stack_look_up(stack, meta),
            Token::Import(a) => a.is_stack_look_up(stack, meta),
            Token::For(a) => a.is_stack_look_up(stack, meta),
            Token::While(a) => a.is_stack_look_up(stack, meta),
            Token::MethodDefinition(a) => a.is_stack_look_up(stack, meta),
            Token::Return(return_type) => return_type.is_stack_look_up(stack, meta),
            Token::ScopeClosing(_) => false,
        }
    }

    fn byte_size(&self, meta: &mut MetaInfo) -> usize {
        match self {
            Token::Variable(a) => a.byte_size(meta),
            Token::MethodCall(a) => a.byte_size(meta),
            Token::MethodDefinition(a) => a.byte_size(meta),
            Token::Import(a) => a.byte_size(meta),
            Token::For(a) => a.byte_size(meta),
            Token::While(a) => a.byte_size(meta),
            Token::If(a) => a.byte_size(meta),
            Token::Return(r) => r.byte_size(meta),
            Token::ScopeClosing(_) => 0,
        }
    }


    fn data_section(&self, stack: &mut Stack, meta: &mut MetaInfo) -> bool {
        match self {
            Token::Variable(v) => v.data_section(stack, meta),
            Token::MethodCall(v) => v.data_section(stack, meta),
            Token::MethodDefinition(v) => v.data_section(stack, meta),
            Token::Import(v) => v.data_section(stack, meta),
            Token::For(v) => v.data_section(stack, meta),
            Token::While(v) => v.data_section(stack, meta),
            Token::If(v) => v.data_section(stack, meta),
            Token::Return(ret) => ret.data_section(stack, meta),
            Token::ScopeClosing(_) => false,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Token::Variable(v) => format!("{}", v),
            Token::MethodCall(m) => format!("{}", m),
            Token::MethodDefinition(m) => format!("{}", m),
            Token::ScopeClosing(m) => format!("{}", m),
            Token::If(m) => format!("{}", m),
            Token::Import(m) => format!("{}", m),
            Token::Return(m) => format!("{}", m),
            Token::While(a) => format!("{}", a),
            Token::For(m) => format!("{}", m),
        })
    }
}


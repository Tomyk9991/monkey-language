use std::fmt::{Debug, Display, Formatter};

use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_result::{ASMOptions, ASMResult};
use crate::core::code_generator::generator::Stack;
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::tokens::assignable_tokens::method_call_token::MethodCallToken;
use crate::core::lexer::tokens::if_definition::IfDefinition;
use crate::core::lexer::tokens::import::ImportToken;
use crate::core::lexer::tokens::method_definition::MethodDefinition;
use crate::core::lexer::tokens::return_token::ReturnToken;
use crate::core::lexer::tokens::scope_ending::ScopeEnding;
use crate::core::lexer::tokens::variable_token::VariableToken;
use crate::core::lexer::types::type_token::InferTypeError;
use crate::core::type_checker::InferType;

/// A token is a piece of code that is used to represent atomic elements of a program.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Variable(VariableToken<'=', ';'>),
    MethodCall(MethodCallToken),
    MethodDefinition(MethodDefinition),
    Import(ImportToken),
    Return(ReturnToken),
    ScopeClosing(ScopeEnding),
    IfDefinition(IfDefinition),
}

impl Token {
    pub(crate) fn code_line(&self) -> CodeLine {
        match self {
            Token::Variable(a) => a.code_line.clone(),
            Token::MethodCall(a) => a.code_line.clone(),
            Token::MethodDefinition(a) => a.code_line.clone(),
            Token::ScopeClosing(a) => a.code_line.clone(),
            Token::IfDefinition(a) => a.code_line.clone(),
            Token::Import(a) => a.code_line.clone(),
            Token::Return(a) => a.code_line.clone(),
        }
    }
}

impl Token {
    pub fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<(), InferTypeError> {
        match self {
            Token::Variable(variable) => {
                variable.infer_type(type_context)?;
            }
            Token::IfDefinition(if_definition) => {
                if_definition.infer_type(type_context)?;
            }
            Token::MethodDefinition(_) | Token::MethodCall(_) | Token::ScopeClosing(_) | Token::Import(_) | Token::Return(_) => {}
        }

        Ok(())
    }
}

impl ToASM for Token {
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        let variables_len = meta.static_type_information.len();

        let scopes = match self {
            Token::IfDefinition(if_def) => {
                let mut res = vec![&if_def.if_stack];
                if let Some(else_stack) = &if_def.else_stack {
                    res.push(else_stack)
                }

                res
            }
            Token::MethodDefinition(method_def) => {
                vec![&method_def.stack]
            }
            _ => { vec![] }
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
            Token::IfDefinition(if_token) => if_token.to_asm(stack, meta, options),
            Token::ScopeClosing(_) => Ok(ASMResult::Inline(String::new())),
        }
    }


    fn is_stack_look_up(&self, stack: &mut Stack, meta: &MetaInfo) -> bool {
        match self {
            Token::Variable(a) => a.is_stack_look_up(stack, meta),
            Token::MethodCall(a) => a.is_stack_look_up(stack, meta),
            Token::IfDefinition(a) => a.is_stack_look_up(stack, meta),
            Token::Import(a) => a.is_stack_look_up(stack, meta),
            Token::MethodDefinition(_) => true,
            Token::ScopeClosing(_) => false,
            Token::Return(return_type) => return_type.is_stack_look_up(stack, meta),
        }
    }

    fn byte_size(&self, meta: &mut MetaInfo) -> usize {
        match self {
            Token::Variable(a) => a.byte_size(meta),
            Token::MethodCall(a) => a.byte_size(meta),
            Token::MethodDefinition(a) => a.byte_size(meta),
            Token::Import(a) => a.byte_size(meta),
            Token::ScopeClosing(_) => 0,
            Token::IfDefinition(a) => a.byte_size(meta),
            Token::Return(r) => r.byte_size(meta),
        }
    }

    fn before_label(&self, stack: &mut Stack, meta: &mut MetaInfo) -> Option<Result<String, ASMGenerateError>> {
        match self {
            Token::Variable(v) => v.before_label(stack, meta),
            Token::MethodCall(v) => v.before_label(stack, meta),
            Token::MethodDefinition(v) => v.before_label(stack, meta),
            Token::Import(v) => v.before_label(stack, meta),
            Token::ScopeClosing(_) => None,
            Token::IfDefinition(v) => v.before_label(stack, meta),
            Token::Return(ret) => ret.before_label(stack, meta),
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
            Token::IfDefinition(m) => format!("{}", m),
            Token::Import(m) => format!("{}", m),
            Token::Return(m) => format!("{}", m)
        })
    }
}


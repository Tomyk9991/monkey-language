use std::fmt::{Debug, Display, Formatter};
use crate::core::lexer::tokens::import::ImportToken;
use crate::core::code_generator::generator::{Stack};
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::tokenizer::StaticTypeContext;
use crate::core::lexer::tokens::scope_ending::ScopeEnding;
use crate::core::lexer::tokens::method_definition::MethodDefinition;
use crate::core::lexer::tokens::assignable_tokens::method_call_token::MethodCallToken;
use crate::core::lexer::tokens::if_definition::IfDefinition;
use crate::core::lexer::tokens::name_token::NameToken;
use crate::core::lexer::tokens::variable_token::VariableToken;
use crate::core::lexer::type_token::InferTypeError;
use crate::core::type_checker::InferType;

/// A token is a piece of code that is used to represent atomic elements of a program.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Variable(VariableToken<'=', ';'>),
    MethodCall(MethodCallToken),
    MethodDefinition(MethodDefinition),
    Import(ImportToken),
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
            Token::Import(a) => a.code_line.clone()
        }
    }
}

impl Token {
    pub fn infer_type(&mut self, type_context: &mut StaticTypeContext, method_names: &[NameToken]) -> Result<(), InferTypeError> {
        match self {
            Token::Variable(variable) => {
                variable.infer_type(type_context, method_names)?;
            }
            Token::IfDefinition(if_definition) => {
                if_definition.infer_type(type_context, method_names)?;
            }
            Token::MethodDefinition(_) | Token::MethodCall(_) | Token::ScopeClosing(_) | Token::Import(_) => {}
        }

        Ok(())
    }
}

impl ToASM for Token {
    fn to_asm(&self, stack: &mut Stack, meta: &MetaInfo) -> Result<String, ASMGenerateError> {
        match self {
            Token::Variable(variable) => variable.to_asm(stack, meta),
            Token::MethodCall(method_call_token) => method_call_token.to_asm(stack, meta),
            Token::IfDefinition(if_definition) => if_definition.to_asm(stack, meta),
            Token::Import(import) => import.to_asm(stack, meta),
            rest => Err(ASMGenerateError::NotImplemented { token: format!("{}", rest) }),
            // Token::MethodDefinition(_) => {}
            // Token::ScopeClosing(_) => {}
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
        })
    }
}


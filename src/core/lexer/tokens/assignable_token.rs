use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::tokens::assignable_tokens::boolean_token::BooleanToken;
use crate::core::lexer::tokens::assignable_tokens::float_token::FloatToken;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::EquationToken;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::expression::{Expression, PrefixArithmetic};
use crate::core::lexer::tokens::assignable_tokens::integer_token::IntegerToken;
use crate::core::lexer::tokens::assignable_tokens::method_call_token::{MethodCallToken, MethodCallTokenErr};
use crate::core::lexer::tokens::assignable_tokens::object_token::ObjectToken;
use crate::core::lexer::tokens::assignable_tokens::string_token::StringToken;
use crate::core::lexer::tokens::name_token::NameToken;
use crate::core::lexer::types::type_token;
use crate::core::lexer::types::type_token::{InferTypeError, TypeToken};

/// Token for assignable tokens. Numbers, strings, method calls, other variables, objects, and arithmetic / boolean equations.
#[derive(Debug, PartialEq, Clone)]
pub enum AssignableToken {
    String(StringToken),
    IntegerToken(IntegerToken),
    FloatToken(FloatToken),
    BooleanToken(BooleanToken),
    MethodCallToken(MethodCallToken),
    NameToken(NameToken),
    Object(ObjectToken),
    ArithmeticEquation(Expression),
}


#[derive(Debug)]
pub enum AssignableTokenErr {
    PatternNotMatched { target_value: String }
}

impl AssignableToken {
    pub fn infer_type(&self, code_line: &CodeLine) -> Option<TypeToken> {
        self.infer_type_with_context(&StaticTypeContext::default(), code_line).ok()
    }

    pub fn prefix_arithmetic(&self) -> Option<PrefixArithmetic> {
        match self {
            AssignableToken::ArithmeticEquation(a) => {
                a.prefix_arithmetic.clone()
            }
            _ => None
        }
    }


    pub fn from_str_ignore(line: &str, ignore_expression: bool) -> Result<Self, AssignableTokenErr> {
        if let Ok(string_token) = StringToken::from_str(line) {
            return Ok(AssignableToken::String(string_token));
        } else if let Ok(integer_token) = IntegerToken::from_str(line) {
            return Ok(AssignableToken::IntegerToken(integer_token));
        } else if let Ok(double_token) = FloatToken::from_str(line) {
            return Ok(AssignableToken::FloatToken(double_token));
        } else if let Ok(boolean_token) = BooleanToken::from_str(line) {
            return Ok(AssignableToken::BooleanToken(boolean_token));
        }

        match MethodCallToken::from_str(line) {
            Ok(method_call_token) => return Ok(AssignableToken::MethodCallToken(method_call_token)),
            Err(err) => {
                // this counts as a not recoverable error and should return immediately
                if let MethodCallTokenErr::AssignableTokenErr(_) = err {
                    return Err(AssignableTokenErr::PatternNotMatched { target_value: line.to_string() });
                }
            }
        }
        if let Ok(variable_name) = NameToken::from_str(line, false) {
            return Ok(AssignableToken::NameToken(variable_name));
        }

        if let Ok(object_token) = ObjectToken::from_str(line) {
            return Ok(AssignableToken::Object(object_token));
        }

        if !ignore_expression {
            if let Ok(arithmetic_equation_token) = EquationToken::from_str(line) {
                return Ok(AssignableToken::ArithmeticEquation(arithmetic_equation_token));
            }
        }


        Err(AssignableTokenErr::PatternNotMatched { target_value: line.to_string() })
    }

    pub fn infer_type_with_context(&self, context: &StaticTypeContext, code_line: &CodeLine) -> Result<TypeToken, InferTypeError> {
        match self {
            AssignableToken::String(_) => Ok(type_token::common::string()),
            AssignableToken::IntegerToken(a) => Ok(TypeToken::Integer(a.ty.clone())),
            AssignableToken::FloatToken(a) => Ok(TypeToken::Float(a.ty.clone())),
            AssignableToken::BooleanToken(_) => Ok(TypeToken::Bool),
            AssignableToken::Object(object) => Ok(TypeToken::Custom(NameToken { name: object.ty.to_string() })),
            AssignableToken::ArithmeticEquation(arithmetic_expression) => Ok(arithmetic_expression.traverse_type_resulted(context, code_line)?),
            AssignableToken::MethodCallToken(method_call) => Ok(method_call.infer_type_with_context(context, code_line)?),
            AssignableToken::NameToken(var) => Ok(var.infer_type_with_context(context, code_line)?),
        }
    }
}

impl Default for AssignableToken {
    fn default() -> Self {
        AssignableToken::IntegerToken(IntegerToken::default())
    }
}

impl Display for AssignableToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            AssignableToken::String(token) => format!("{}", token),
            AssignableToken::IntegerToken(token) => format!("{}", token),
            AssignableToken::FloatToken(token) => format!("{}", token),
            AssignableToken::BooleanToken(token) => format!("{}", token),
            AssignableToken::MethodCallToken(token) => format!("{}", token),
            AssignableToken::NameToken(token) => format!("{}", token),
            AssignableToken::Object(token) => format!("{}", token),
            AssignableToken::ArithmeticEquation(token) => format!("{}", token),
        })
    }
}

impl Error for AssignableTokenErr {}

impl Display for AssignableTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            AssignableTokenErr::PatternNotMatched { target_value }
            => format!("Pattern not matched for: `{target_value}`\n\tAssignments are: string: \"String\", integer: 21, -125, double: -51.1512, 152.1521")
        })
    }
}

impl ToASM for AssignableToken {
    fn to_asm(&self, stack: &mut Stack, meta: &mut MetaInfo) -> Result<String, ASMGenerateError> {
        match &self {
            AssignableToken::IntegerToken(token) => Ok(token.to_asm(stack, meta)?),
            AssignableToken::NameToken(variable) => Ok(variable.to_asm(stack, meta)?),
            AssignableToken::ArithmeticEquation(expression) => Ok(expression.to_asm(stack, meta)?),
            AssignableToken::String(string) => Ok(string.to_asm(stack, meta)?),
            AssignableToken::FloatToken(float) => Ok(float.to_asm(stack, meta)?),
            AssignableToken::MethodCallToken(method_call) => Ok(method_call.to_asm(stack, meta)?),
            token => Err(ASMGenerateError::AssignmentNotImplemented { assignable_token: (*token).clone() })
            // AssignableToken::DoubleToken(_) => {}
            // AssignableToken::BooleanToken(_) => {}
            // AssignableToken::Object(_) => {}
            // AssignableToken::BooleanEquation(_) => {}
        }
    }

    fn is_stack_look_up(&self, stack: &mut Stack, meta: &MetaInfo) -> bool {
        match self {
            AssignableToken::String(_) =>  false,
            AssignableToken::IntegerToken(_) => false,
            AssignableToken::FloatToken(_) => false,
            AssignableToken::BooleanToken(_) => false,
            AssignableToken::MethodCallToken(_) => true,
            AssignableToken::NameToken(_) => true,
            AssignableToken::Object(_) => false,
            AssignableToken::ArithmeticEquation(a) => a.is_stack_look_up(stack, meta),
        }
    }

    fn byte_size(&self, meta: &mut MetaInfo) -> usize {
        match self {
            AssignableToken::String(a) => a.byte_size(meta),
            AssignableToken::IntegerToken(a) => a.byte_size(meta),
            AssignableToken::FloatToken(a) => a.byte_size(meta),
            AssignableToken::BooleanToken(a) => a.byte_size(meta),
            AssignableToken::MethodCallToken(a) => a.byte_size(meta),
            AssignableToken::NameToken(a) => a.byte_size(meta),
            AssignableToken::Object(a) => a.byte_size(meta),
            AssignableToken::ArithmeticEquation(a) => a.byte_size(meta),
        }
    }

    fn before_label(&self, stack: &mut Stack, meta: &mut MetaInfo) -> Option<Result<String, ASMGenerateError>> {
        match &self {
            AssignableToken::String(v) => v.before_label(stack, meta),
            AssignableToken::IntegerToken(v) => v.before_label(stack, meta),
            AssignableToken::FloatToken(v) => v.before_label(stack, meta),
            AssignableToken::BooleanToken(v) => v.before_label(stack, meta),
            AssignableToken::MethodCallToken(v) => v.before_label(stack, meta),
            AssignableToken::NameToken(v) => v.before_label(stack, meta),
            AssignableToken::Object(v) => v.before_label(stack, meta),
            AssignableToken::ArithmeticEquation(v) => v.before_label(stack, meta),
        }
    }
}

impl FromStr for AssignableToken {
    type Err = AssignableTokenErr;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Self::from_str_ignore(line, false)
    }
}
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::{ASMResult};
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::tokens::assignable_tokens::array_token::ArrayToken;
use crate::core::lexer::tokens::assignable_tokens::boolean_token::BooleanToken;
use crate::core::lexer::tokens::assignable_tokens::float_token::FloatToken;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::EquationToken;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::expression::Expression;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::prefix_arithmetic::PrefixArithmetic;
use crate::core::lexer::tokens::assignable_tokens::integer_token::IntegerToken;
use crate::core::lexer::tokens::assignable_tokens::method_call_token::{MethodCallToken, MethodCallTokenErr};
use crate::core::lexer::tokens::assignable_tokens::object_token::ObjectToken;
use crate::core::lexer::tokens::assignable_tokens::string_token::StringToken;
use crate::core::lexer::tokens::name_token::NameToken;
use crate::core::lexer::tokens::parameter_token::ParameterToken;
use crate::core::lexer::types::type_token;
use crate::core::lexer::types::type_token::{InferTypeError, Mutability, TypeToken};

/// Token for assignable tokens. Numbers, strings, method calls, other variables, objects, and arithmetic / boolean equations.
#[derive(Debug, PartialEq, Clone)]
pub enum AssignableToken {
    String(StringToken),
    IntegerToken(IntegerToken),
    FloatToken(FloatToken),
    Parameter(ParameterToken),
    BooleanToken(BooleanToken),
    MethodCallToken(MethodCallToken),
    NameToken(NameToken),
    Object(ObjectToken),
    ArrayToken(ArrayToken),
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
        } else if let Ok(array_token) = ArrayToken::from_str(line) {
            return Ok(AssignableToken::ArrayToken(array_token))
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
            AssignableToken::IntegerToken(a) => Ok(TypeToken::Integer(a.ty.clone(), Mutability::Immutable)),
            AssignableToken::ArrayToken(array_token) => Ok(array_token.infer_type_with_context(context, code_line)?),
            AssignableToken::FloatToken(a) => Ok(TypeToken::Float(a.ty.clone(), Mutability::Immutable)),
            AssignableToken::BooleanToken(_) => Ok(TypeToken::Bool(Mutability::Immutable)),
            AssignableToken::Object(object) => Ok(TypeToken::Custom(NameToken { name: object.ty.to_string() }, Mutability::Immutable)),
            AssignableToken::ArithmeticEquation(arithmetic_expression) => Ok(arithmetic_expression.traverse_type_resulted(context, code_line)?),
            AssignableToken::MethodCallToken(method_call) => Ok(method_call.infer_type_with_context(context, code_line)?),
            AssignableToken::NameToken(var) => Ok(var.infer_type_with_context(context, code_line)?),
            AssignableToken::Parameter(r) => Ok(r.ty.clone()),
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
            AssignableToken::Parameter(r) => format!("{}", r),
            AssignableToken::ArrayToken(r) => format!("{}", r),
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

    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        match &self {
            AssignableToken::IntegerToken(token) => Ok(token.to_asm(stack, meta, options)?),
            AssignableToken::NameToken(variable) => Ok(variable.to_asm(stack, meta, options)?),
            AssignableToken::ArithmeticEquation(expression) => Ok(expression.to_asm(stack, meta, options)?),
            AssignableToken::String(string) => Ok(string.to_asm(stack, meta, options)?),
            AssignableToken::FloatToken(float) => Ok(float.to_asm(stack, meta, options)?),
            AssignableToken::MethodCallToken(method_call) => Ok(method_call.to_asm(stack, meta, options)?),
            AssignableToken::BooleanToken(boolean) => Ok(boolean.to_asm(stack, meta, options)?),
            AssignableToken::ArrayToken(array) => Ok(array.to_asm(stack, meta, options)?),
            AssignableToken::Parameter(_) | AssignableToken::Object(_) => Err(ASMGenerateError::AssignmentNotImplemented { assignable_token: self.clone() })
            // token => Err(ASMGenerateError::AssignmentNotImplemented { assignable_token: (*token).clone() })
        }
    }


    fn is_stack_look_up(&self, stack: &mut Stack, meta: &MetaInfo) -> bool {
        match self {
            AssignableToken::String(s) => s.is_stack_look_up(stack, meta),
            AssignableToken::IntegerToken(s) => s.is_stack_look_up(stack, meta),
            AssignableToken::FloatToken(s) => s.is_stack_look_up(stack, meta),
            AssignableToken::BooleanToken(s) => s.is_stack_look_up(stack, meta),
            AssignableToken::MethodCallToken(s) => s.is_stack_look_up(stack, meta),
            AssignableToken::NameToken(s) => s.is_stack_look_up(stack, meta),
            AssignableToken::Object(s) => s.is_stack_look_up(stack, meta),
            AssignableToken::ArithmeticEquation(a) => a.is_stack_look_up(stack, meta),
            AssignableToken::Parameter(r) => r.is_stack_look_up(stack, meta),
            AssignableToken::ArrayToken(s) => s.is_stack_look_up(stack, meta)
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
            AssignableToken::Parameter(r) => r.ty.byte_size(),
            AssignableToken::ArrayToken(r) => r.byte_size(meta),
        }
    }

    fn data_section(&self, stack: &mut Stack, meta: &mut MetaInfo) -> bool {
        match &self {
            AssignableToken::String(v) => v.data_section(stack, meta),
            AssignableToken::IntegerToken(v) => v.data_section(stack, meta),
            AssignableToken::FloatToken(v) => v.data_section(stack, meta),
            AssignableToken::BooleanToken(v) => v.data_section(stack, meta),
            AssignableToken::MethodCallToken(v) => v.data_section(stack, meta),
            AssignableToken::NameToken(v) => v.data_section(stack, meta),
            AssignableToken::Object(v) => v.data_section(stack, meta),
            AssignableToken::ArithmeticEquation(v) => v.data_section(stack, meta),
            AssignableToken::Parameter(r) => r.data_section(stack, meta),
            AssignableToken::ArrayToken(r) => r.data_section(stack, meta),
        }
    }
}

impl FromStr for AssignableToken {
    type Err = AssignableTokenErr;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Self::from_str_ignore(line, false)
    }
}
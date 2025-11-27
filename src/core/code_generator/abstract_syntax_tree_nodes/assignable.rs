use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::{ASMResult};
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::parse::{Parse, ParseResult};
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::prefix_arithmetic::PrefixArithmetic;
use crate::core::model::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use crate::core::model::abstract_syntax_tree_nodes::assignables::object::Object;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::types::array::Array;
use crate::core::model::types::boolean::Boolean;
use crate::core::model::types::float::FloatAST;
use crate::core::model::types::integer::IntegerAST;
use crate::core::model::types::mutability::Mutability;
use crate::core::model::types::static_string::StaticString;
use crate::core::model::types::ty::Type;
use crate::core::scanner::static_type_context::StaticTypeContext;
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::equation_parser::Equation;
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::method_call::{MethodCallErr};
use crate::core::scanner::types::r#type;
use crate::core::scanner::types::r#type::{InferTypeError};


impl ToASM for Assignable {

    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        match &self {
            Assignable::Integer(integer) => Ok(integer.to_asm(stack, meta, options)?),
            Assignable::Identifier(variable) => Ok(variable.to_asm(stack, meta, options)?),
            Assignable::ArithmeticEquation(expression) => Ok(expression.to_asm(stack, meta, options)?),
            Assignable::String(string) => Ok(string.to_asm(stack, meta, options)?),
            Assignable::Float(float) => Ok(float.to_asm(stack, meta, options)?),
            Assignable::MethodCall(method_call) => Ok(method_call.to_asm(stack, meta, options)?),
            Assignable::Boolean(boolean) => Ok(boolean.to_asm(stack, meta, options)?),
            Assignable::Array(array) => Ok(array.to_asm(stack, meta, options)?),
            Assignable::Parameter(_) | Assignable::Object(_) => Err(ASMGenerateError::AssignmentNotImplemented { assignable: self.clone() })
        }
    }


    fn is_stack_look_up(&self, stack: &mut Stack, meta: &MetaInfo) -> bool {
        match self {
            Assignable::String(s) => s.is_stack_look_up(stack, meta),
            Assignable::Integer(s) => s.is_stack_look_up(stack, meta),
            Assignable::Float(s) => s.is_stack_look_up(stack, meta),
            Assignable::Boolean(s) => s.is_stack_look_up(stack, meta),
            Assignable::MethodCall(s) => s.is_stack_look_up(stack, meta),
            Assignable::Identifier(s) => s.is_stack_look_up(stack, meta),
            Assignable::Object(s) => s.is_stack_look_up(stack, meta),
            Assignable::ArithmeticEquation(a) => a.is_stack_look_up(stack, meta),
            Assignable::Parameter(r) => r.is_stack_look_up(stack, meta),
            Assignable::Array(s) => s.is_stack_look_up(stack, meta)
        }
    }

    fn byte_size(&self, meta: &mut MetaInfo) -> usize {
        match self {
            Assignable::String(a) => a.byte_size(meta),
            Assignable::Integer(a) => a.byte_size(meta),
            Assignable::Float(a) => a.byte_size(meta),
            Assignable::Boolean(a) => a.byte_size(meta),
            Assignable::MethodCall(a) => a.byte_size(meta),
            Assignable::Identifier(a) => a.byte_size(meta),
            Assignable::Object(a) => a.byte_size(meta),
            Assignable::ArithmeticEquation(a) => a.byte_size(meta),
            Assignable::Parameter(r) => r.ty.byte_size(),
            Assignable::Array(r) => r.byte_size(meta),
        }
    }

    fn data_section(&self, stack: &mut Stack, meta: &mut MetaInfo) -> bool {
        match &self {
            Assignable::String(v) => v.data_section(stack, meta),
            Assignable::Integer(v) => v.data_section(stack, meta),
            Assignable::Float(v) => v.data_section(stack, meta),
            Assignable::Boolean(v) => v.data_section(stack, meta),
            Assignable::MethodCall(v) => v.data_section(stack, meta),
            Assignable::Identifier(v) => v.data_section(stack, meta),
            Assignable::Object(v) => v.data_section(stack, meta),
            Assignable::ArithmeticEquation(v) => v.data_section(stack, meta),
            Assignable::Parameter(r) => r.data_section(stack, meta),
            Assignable::Array(r) => r.data_section(stack, meta),
        }
    }
}
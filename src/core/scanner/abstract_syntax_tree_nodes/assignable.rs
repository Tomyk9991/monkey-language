use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::{ASMResult};
use crate::core::io::code_line::CodeLine;
use crate::core::scanner::static_type_context::StaticTypeContext;
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::array::Array;
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::boolean::Boolean;
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::float::FloatAST;
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::equation_parser::Equation;
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::equation_parser::prefix_arithmetic::PrefixArithmetic;
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::integer::IntegerAST;
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::method_call::{MethodCall, MethodCallErr};
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::object::Object;
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::string::StaticString;
use crate::core::scanner::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::scanner::abstract_syntax_tree_nodes::parameter::Parameter;
use crate::core::scanner::types::r#type;
use crate::core::scanner::types::r#type::{InferTypeError, Mutability, Type};

/// AST node for assignable abstract_syntax_tree_nodes. Numbers, strings, method calls, other variables, objects, and arithmetic / boolean equations.
#[derive(Debug, PartialEq, Clone)]
pub enum Assignable {
    String(StaticString),
    Integer(IntegerAST),
    Float(FloatAST),
    Parameter(Parameter),
    Boolean(Boolean),
    MethodCall(MethodCall),
    Identifier(Identifier),
    Object(Object),
    Array(Array),
    ArithmeticEquation(Expression),
}


#[derive(Debug)]
pub enum AssignableErr {
    PatternNotMatched { target_value: String }
}

impl Assignable {
    pub fn infer_type(&self, code_line: &CodeLine) -> Option<Type> {
        self.infer_type_with_context(&StaticTypeContext::default(), code_line).ok()
    }

    pub fn identifier(&self) -> Option<String> {
        match self {
            Assignable::Identifier(identifier) => Some(identifier.name.clone()),
            Assignable::ArithmeticEquation(value) => {
                value.identifier()
            }
            _ => None
        }
    }

    pub fn prefix_arithmetic(&self) -> Option<PrefixArithmetic> {
        match self {
            Assignable::ArithmeticEquation(a) => {
                a.prefix_arithmetic.clone()
            }
            _ => None
        }
    }


    pub fn from_str_ignore(line: &str, ignore_expression: bool) -> Result<Self, AssignableErr> {
        if let Ok(string) = StaticString::from_str(line) {
            return Ok(Assignable::String(string));
        } else if let Ok(integer) = IntegerAST::from_str(line) {
            return Ok(Assignable::Integer(integer));
        } else if let Ok(double) = FloatAST::from_str(line) {
            return Ok(Assignable::Float(double));
        } else if let Ok(boolean) = Boolean::from_str(line) {
            return Ok(Assignable::Boolean(boolean));
        } else if let Ok(array) = Array::from_str(line) {
            return Ok(Assignable::Array(array))
        }


        match MethodCall::from_str(line) {
            Ok(method_call) => return Ok(Assignable::MethodCall(method_call)),
            Err(err) => {
                // this counts as a not recoverable error and should return immediately
                if let MethodCallErr::AssignableErr(_) = err {
                    return Err(AssignableErr::PatternNotMatched { target_value: line.to_string() });
                }
            }
        }
        if let Ok(variable_name) = Identifier::from_str(line, false) {
            return Ok(Assignable::Identifier(variable_name));
        }

        if let Ok(object) = Object::from_str(line) {
            return Ok(Assignable::Object(object));
        }

        if !ignore_expression {
            if let Ok(arithmetic_equation) = Equation::from_str(line) {
                return Ok(Assignable::ArithmeticEquation(arithmetic_equation));
            }
        }


        Err(AssignableErr::PatternNotMatched { target_value: line.to_string() })
    }

    pub fn infer_type_with_context(&self, context: &StaticTypeContext, code_line: &CodeLine) -> Result<Type, InferTypeError> {
        match self {
            Assignable::String(_) => Ok(r#type::common::string()),
            Assignable::Integer(a) => Ok(Type::Integer(a.ty.clone(), Mutability::Immutable)),
            Assignable::Array(array) => Ok(array.infer_type_with_context(context, code_line)?),
            Assignable::Float(a) => Ok(Type::Float(a.ty.clone(), Mutability::Immutable)),
            Assignable::Boolean(_) => Ok(Type::Bool(Mutability::Immutable)),
            Assignable::Object(object) => Ok(Type::Custom(Identifier { name: object.ty.to_string() }, Mutability::Immutable)),
            Assignable::ArithmeticEquation(arithmetic_expression) => Ok(arithmetic_expression.traverse_type_resulted(context, code_line)?),
            Assignable::MethodCall(method_call) => Ok(method_call.infer_type_with_context(context, code_line)?),
            Assignable::Identifier(var) => Ok(var.infer_type_with_context(context, code_line)?),
            Assignable::Parameter(r) => Ok(r.ty.clone()),
        }
    }
}

impl Default for Assignable {
    fn default() -> Self {
        Assignable::Integer(IntegerAST::default())
    }
}

impl Display for Assignable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Assignable::String(node) => format!("{}", node),
            Assignable::Integer(node) => format!("{}", node),
            Assignable::Float(node) => format!("{}", node),
            Assignable::Boolean(node) => format!("{}", node),
            Assignable::MethodCall(node) => format!("{}", node),
            Assignable::Identifier(node) => format!("{}", node),
            Assignable::Object(node) => format!("{}", node),
            Assignable::ArithmeticEquation(node) => format!("{}", node),
            Assignable::Parameter(node) => format!("{}", node),
            Assignable::Array(node) => format!("{}", node),
        })
    }
}

impl Error for AssignableErr {}

impl Display for AssignableErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            AssignableErr::PatternNotMatched { target_value }
            => format!("Pattern not matched for: `{target_value}`\n\tAssignments are: string: \"String\", integer: 21, -125, double: -51.1512, 152.1521")
        })
    }
}

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

impl FromStr for Assignable {
    type Err = AssignableErr;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Self::from_str_ignore(line, false)
    }
}
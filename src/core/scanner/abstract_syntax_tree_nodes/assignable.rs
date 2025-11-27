use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

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


impl TryFrom<Result<ParseResult<Self>, crate::core::lexer::error::Error>> for Assignable {
    type Error = crate::core::lexer::error::Error;

    fn try_from(value: Result<ParseResult<Self>, crate::core::lexer::error::Error>) -> Result<Self, Self::Error> {
        match value {
            Ok(value) => Ok(value.result),
            Err(err) => Err(err)
        }
    }
}


impl Parse for Assignable {
    fn parse(tokens: &[TokenWithSpan]) -> Result<ParseResult<Self>, crate::core::lexer::error::Error> where Self: Sized, Self: Default {
        return if let Ok(string) = StaticString::parse(tokens) {
            Ok(ParseResult {
                result: Assignable::String(string.result),
                consumed: string.consumed
            })
        } else if let Ok(float) = FloatAST::parse(tokens) {
            Ok(ParseResult {
                result: Assignable::Float(float.result),
                consumed: float.consumed
            })
        } else if let Ok(integer) = IntegerAST::parse(tokens) {
            Ok(ParseResult {
                result: Assignable::Integer(integer.result),
                consumed: integer.consumed
            })
        }
        else if let Ok(boolean) = Boolean::parse(tokens) {
            Ok(ParseResult {
                result: Assignable::Boolean(boolean.result),
                consumed: boolean.consumed
            })
        }
        else if let Ok(array) = Array::parse(tokens) {
            Ok(ParseResult {
                result: Assignable::Array(array.result),
                consumed: array.consumed
            })
        } else if let Ok(l_value) = LValue::parse(tokens) {
            Ok(ParseResult {
                result: match l_value.result {
                    LValue::Identifier(i) => Assignable::Identifier(i),
                    LValue::Expression(e) => Assignable::ArithmeticEquation(e),
                },
                consumed: l_value.consumed
            })
        }
        else {
            if tokens.is_empty() {
                return Err(crate::core::lexer::error::Error::UnexpectedEOF);
            }

            Err(crate::core::lexer::error::Error::UnexpectedToken(tokens[0].clone()))
        };
        // else if let Ok(array) = Array::from_str(line) {
        //     return Ok(Assignable::Array(array))
        // }
        //
        //
        // match MethodCall::from_str(line) {
        //     Ok(method_call) => return Ok(Assignable::MethodCall(method_call)),
        //     Err(err) => {
        //         // this counts as a not recoverable error and should return immediately
        //         if let MethodCallErr::AssignableErr(_) = err {
        //             return Err(AssignableErr::PatternNotMatched { target_value: line.to_string() });
        //         }
        //     }
        // }
        // if let Ok(variable_name) = Identifier::from_str(line, false) {
        //     return Ok(Assignable::Identifier(variable_name));
        // }
        //
        // if let Ok(object) = Object::from_str(line) {
        //     return Ok(Assignable::Object(object));
        // }
        //
        // if !ignore_expression {
        //     if let Ok(arithmetic_equation) = Equation::from_str(line) {
        //         return Ok(Assignable::ArithmeticEquation(arithmetic_equation));
        //     }
        // }
        //
        //
        // Err(AssignableErr::PatternNotMatched { target_value: line.to_string() })
        //
        return Ok(ParseResult {
            result: Assignable::Identifier(Identifier { name: "test".to_string() }),
            consumed: 1
        });
    }
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


    pub fn from_str_ignore(line: &str, ignore_expression: bool) -> Result<Self, AssignableError> {
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
                    return Err(AssignableError::PatternNotMatched { target_value: line.to_string() });
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


        Err(AssignableError::PatternNotMatched { target_value: line.to_string() })
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


impl FromStr for Assignable {
    type Err = AssignableError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Self::from_str_ignore(line, false)
    }
}
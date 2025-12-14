use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
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
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::abstract_syntax_tree_nodes::assignables::equation_parser::Equation;
use crate::core::parser::abstract_syntax_tree_nodes::assignables::method_call::{MethodCallErr};
use crate::core::parser::types::r#type;
use crate::core::parser::types::r#type::{InferTypeError};

impl TryFrom<Result<ParseResult<Self>, crate::core::lexer::error::Error>> for Assignable {
    type Error = crate::core::lexer::error::Error;

    fn try_from(
        value: Result<ParseResult<Self>, crate::core::lexer::error::Error>,
    ) -> Result<Self, Self::Error> {
        match value {
            Ok(value) => Ok(value.result),
            Err(err) => Err(err),
        }
    }
}

impl Parse for Assignable {
    fn parse(tokens: &[TokenWithSpan], parse_options: ParseOptions) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default {
        let mut parsers = vec![
            |tokens: &[TokenWithSpan]| StaticString::parse(tokens, ParseOptions::default()).map(|r| ParseResult { result: Assignable::String(r.result), consumed: r.consumed }),
            |tokens: &[TokenWithSpan]| FloatAST::parse(tokens, ParseOptions::default()).map(|r| ParseResult { result: Assignable::Float(r.result), consumed: r.consumed }),
            |tokens: &[TokenWithSpan]| IntegerAST::parse(tokens, ParseOptions::default()).map(|r| ParseResult { result: Assignable::Integer(r.result), consumed: r.consumed }),
            |tokens: &[TokenWithSpan]| MethodCall::parse(tokens, ParseOptions::default()).map(|r| ParseResult { result: Assignable::MethodCall(r.result), consumed: r.consumed }),
            |tokens: &[TokenWithSpan]| Boolean::parse(tokens, ParseOptions::default()).map(|r| ParseResult { result: Assignable::Boolean(r.result), consumed: r.consumed }),
            |tokens: &[TokenWithSpan]| Array::parse(tokens, ParseOptions::default()).map(|r| ParseResult { result: Assignable::Array(r.result), consumed: r.consumed }),
            |tokens: &[TokenWithSpan]| LValue::parse(tokens, ParseOptions::default()).map(|r| ParseResult { result: match r.result { LValue::Identifier(i) => Assignable::Identifier(i) }, consumed: r.consumed }),
        ];

        if !parse_options.ignore_expression {
            parsers.insert(0, |tokens: &[TokenWithSpan]| Expression::parse(tokens, ParseOptions::default()).map(|r| ParseResult { result: Assignable::Expression(r.result), consumed: r.consumed }),);
        }

        if tokens.is_empty() {
            return Err(Error::UnexpectedEOF);
        }

        for (index, parser) in parsers.iter().enumerate() {
            match parser(tokens) {
                Ok(assignable) => {
                    if !parse_options.ignore_expression && index == 0 {
                        // we parsed an expression, wrap it in an arithmetic equation
                        let expr = match &assignable.result {
                            Assignable::Expression(e) => e,
                            _ => unreachable!(),
                        };

                        if let (Some(value), None, None, None, None) = (&expr.value, &expr.prefix_arithmetic, &expr.index_operator, &expr.lhs, &expr.rhs) {
                            // it's just a single value, return that directly
                            let s = *(*value).clone();
                            return Ok(ParseResult {
                                result: s,
                                consumed: assignable.consumed,
                            });
                        }
                    }
                    return Ok(assignable)
                }
                Err(err) => match &err {
                    Error::InvalidCharacter(_) => {}
                    Error::UnexpectedToken(_) => {}
                    Error::ExpectedToken(_) => {}
                    Error::UnexpectedEOF => {}
                    Error::Callstack(t) => {
                        return Err(err)
                    }
                    Error::ErrorWithContext { error, context } => {
                        return Err(err)
                    }
                }
            }
        }

        Err(Error::UnexpectedToken(tokens[0].clone()))
    }
}

impl Assignable {
    pub fn identifier(&self) -> Option<String> {
        match self {
            Assignable::Identifier(identifier) => Some(identifier.name.clone()),
            Assignable::Expression(value) => value.identifier(),
            _ => None,
        }
    }

    pub fn prefix_arithmetic(&self) -> Option<PrefixArithmetic> {
        match self {
            Assignable::Expression(a) => a.prefix_arithmetic.clone(),
            _ => None,
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
            return Ok(Assignable::Array(array));
        }

        match MethodCall::from_str(line) {
            Ok(method_call) => return Ok(Assignable::MethodCall(method_call)),
            Err(err) => {
                // this counts as a not recoverable error and should return immediately
                if let MethodCallErr::AssignableErr(_) = err {
                    return Err(AssignableError::PatternNotMatched {
                        target_value: line.to_string(),
                    });
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
                return Ok(Assignable::Expression(arithmetic_equation));
            }
        }

        Err(AssignableError::PatternNotMatched {
            target_value: line.to_string(),
        })
    }
}

impl FromStr for Assignable {
    type Err = AssignableError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Self::from_str_ignore(line, false)
    }
}

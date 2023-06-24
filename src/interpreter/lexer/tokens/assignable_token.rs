use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use crate::interpreter::lexer::tokens::assignable_tokens::boolean_token::BooleanToken;
use crate::interpreter::lexer::tokens::assignable_tokens::double_token::DoubleToken;
use crate::interpreter::lexer::tokens::assignable_tokens::equation_parser::EquationToken;
use crate::interpreter::lexer::tokens::assignable_tokens::equation_parser::equation_token_options::{ArithmeticEquationOptions, BooleanEquationOptions};
use crate::interpreter::lexer::tokens::assignable_tokens::equation_parser::expression::Expression;
use crate::interpreter::lexer::tokens::assignable_tokens::integer_token::IntegerToken;
use crate::interpreter::lexer::tokens::assignable_tokens::method_call_token::MethodCallToken;
use crate::interpreter::lexer::tokens::assignable_tokens::object_token::ObjectToken;
use crate::interpreter::lexer::tokens::assignable_tokens::string_token::{StringToken};
use crate::interpreter::lexer::tokens::name_token::NameToken;

#[derive(Debug, PartialEq, Clone)]
pub enum AssignableToken {
    String(StringToken),
    IntegerToken(IntegerToken),
    DoubleToken(DoubleToken),
    BooleanToken(BooleanToken),
    MethodCallToken(MethodCallToken),
    Variable(NameToken),
    Object(ObjectToken),
    ArithmeticEquation(Box<Expression>),
    BooleanEquation(Box<Expression>),
}

#[derive(Debug)]
pub enum AssignableTokenErr {
    PatternNotMatched { target_value: String }
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
            AssignableToken::DoubleToken(token) => format!("{}", token),
            AssignableToken::BooleanToken(token) => format!("{}", token),
            AssignableToken::MethodCallToken(token) => format!("{}", token),
            AssignableToken::Variable(token) => format!("{}", token),
            AssignableToken::Object(token) => format!("{}", token),
            AssignableToken::ArithmeticEquation(token) => format!("{}", token.value),
            AssignableToken::BooleanEquation(token) => format!("{}", token.value),
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

impl FromStr for AssignableToken {
    type Err = AssignableTokenErr;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if let Ok(string_token) = StringToken::from_str(line) {
            return Ok(AssignableToken::String(string_token));
        } else if let Ok(integer_token) = IntegerToken::from_str(line) {
            return Ok(AssignableToken::IntegerToken(integer_token));
        } else if let Ok(double_token) = DoubleToken::from_str(line) {
            return Ok(AssignableToken::DoubleToken(double_token));
        } else if let Ok(boolean_token) = BooleanToken::from_str(line) {
            return Ok(AssignableToken::BooleanToken(boolean_token));
        } else if let Ok(method_call_token) = MethodCallToken::from_str(line) {
            return Ok(AssignableToken::MethodCallToken(method_call_token));
        } else if let Ok(variable_name) = NameToken::from_str(line, false) {
            return Ok(AssignableToken::Variable(variable_name));
        } else if let Ok(object_token) = ObjectToken::from_str(line) {
            return Ok(AssignableToken::Object(object_token));
        } else if let Ok(arithmetic_equation_token) = EquationToken::<ArithmeticEquationOptions>::from_str(line) {
            return Ok(AssignableToken::ArithmeticEquation(arithmetic_equation_token));
        } else if let Ok(boolean_equation_token) = EquationToken::<BooleanEquationOptions>::from_str(line) {
            return Ok(AssignableToken::BooleanEquation(boolean_equation_token));
        }

        Err(AssignableTokenErr::PatternNotMatched { target_value: line.to_string() })
    }
}
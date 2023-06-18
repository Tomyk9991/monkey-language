use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use crate::interpreter::lexer::tokens::assignable_tokens::equation_parser::{ArithmeticEquationOptions, EquationToken};
use crate::interpreter::lexer::tokens::assignable_tokens::double_token::DoubleToken;
use crate::interpreter::lexer::tokens::assignable_tokens::equation_parser::expression::Expression;
use crate::interpreter::lexer::tokens::assignable_tokens::integer_token::IntegerToken;
use crate::interpreter::lexer::tokens::assignable_tokens::method_call_token::MethodCallToken;
use crate::interpreter::lexer::tokens::assignable_tokens::object_token::ObjectToken;
use crate::interpreter::lexer::tokens::assignable_tokens::string_token::StringToken;
use crate::interpreter::lexer::tokens::name_token::NameToken;

#[derive(Debug, PartialEq, Clone)]
pub enum AssignableToken {
    String(StringToken),
    IntegerToken(IntegerToken),
    DoubleToken(DoubleToken),
    MethodCallToken(MethodCallToken),
    Variable(NameToken),
    Object(ObjectToken),
    Equation(Box<Expression>),
    // BooleanStatement(BooleanToken)
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
            AssignableToken::MethodCallToken(token) => format!("{}", token),
            AssignableToken::Variable(token) => format!("{}", token),
            AssignableToken::Object(token) => format!("{}", token),
            AssignableToken::Equation(_token) => "Token String".to_string()
        })
    }
}

impl Error for AssignableTokenErr { }

impl Display for AssignableTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            AssignableTokenErr::PatternNotMatched { target_value}
            => format!("Pattern not matched for: `{target_value}`\n\tAssignments are: string: \"String\", integer: 21, -125, double: -51.1512, 152.1521")
        })
    }
}

impl AssignableToken {
    pub fn try_from(line: &str) -> Result<Self, AssignableTokenErr> {
        if let Ok(string_token) = StringToken::from_str(line) {
            return Ok(AssignableToken::String(string_token))
        } else if let Ok(integer_token) = IntegerToken::from_str(line) {
            return Ok(AssignableToken::IntegerToken(integer_token))
        } else if let Ok(double_token) = DoubleToken::from_str(line) {
            return Ok(AssignableToken::DoubleToken(double_token))
        } else if let Ok(method_call_token) = MethodCallToken::from_str(line) {
            return Ok(AssignableToken::MethodCallToken(method_call_token))
        } else if let Ok(variable_name) = NameToken::from_str(line, false) {
            return Ok(AssignableToken::Variable(variable_name))
        } else if let Ok(object_token) = ObjectToken::from_str(line) {
            return Ok(AssignableToken::Object(object_token))
        } else if let Ok(equation_token) = EquationToken::<ArithmeticEquationOptions>::from_str(line) {
            return Ok(AssignableToken::Equation(equation_token))
        }
        
        Err(AssignableTokenErr::PatternNotMatched { target_value: line.to_string()})
    }
}
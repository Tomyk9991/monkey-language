use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::interpreter::lexer::tokens::assignable_tokens::double_token::DoubleToken;
use crate::interpreter::lexer::tokens::assignable_tokens::integer_token::IntegerToken;
use crate::interpreter::lexer::tokens::assignable_tokens::method_call_token::MethodCallToken;
use crate::interpreter::lexer::tokens::assignable_tokens::string_token::{StringToken, StringTokenErr};
use crate::interpreter::lexer::tokens::name_token::NameToken;

#[derive(Debug)]
pub enum AssignableToken {
    String(StringToken),
    IntegerToken(IntegerToken),
    DoubleToken(DoubleToken),
    MethodCallToken(MethodCallToken),
    Variable(NameToken),
    Object(ObjectToken),
    // Equation(EquationToken),
}

#[derive(Debug)]
pub enum AssignableTokenErr {
    PatternNotMatched { target_value: String }
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
        } else if let Ok(variable_name) = NameToken::from_str(line) {
            return Ok(AssignableToken::Variable(variable_name))
        }
        
        return Err(AssignableTokenErr::PatternNotMatched { target_value: line.to_string()});
    }
}
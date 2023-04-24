use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::interpreter::lexer::tokens::assignable_tokens::double_token::DoubleToken;
use crate::interpreter::lexer::tokens::assignable_tokens::integer_token::IntegerToken;
use crate::interpreter::lexer::tokens::assignable_tokens::string_token::StringToken;

#[derive(Debug)]
pub enum AssignableToken {
    String(StringToken),
    IntegerToken(IntegerToken),
    DoubleToken(DoubleToken)
    // Equation(EquationToken),
    // Object(ObjectToken),
    // Variable(NameToken),
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
            => format!("Pattern not matched for: \"{target_value}\"Assignments are: string: \"Hallo\", integer: 21, -125, double: -51.1512, 152.1521")
        })
    }
}

impl AssignableToken {
    pub fn try_from(line: &str) -> anyhow::Result<Self, AssignableTokenErr> {
        if let Ok(string_token) = StringToken::from_str(line) {
            return Ok(AssignableToken::String(string_token))
        } else if let Ok(integer_token) = IntegerToken::from_str(line) {
            return Ok(AssignableToken::IntegerToken(integer_token))
        } else if let Ok(double_token) = DoubleToken::from_str(line) {
            return Ok(AssignableToken::DoubleToken(double_token))
        }
        
        return Err(AssignableTokenErr::PatternNotMatched { target_value: line.to_string()});
        
    }
}
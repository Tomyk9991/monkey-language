use std::str::FromStr;
use crate::interpreter::lexer::tokens::assignable_tokens::integer_token::IntegerToken;
use crate::interpreter::lexer::tokens::assignable_tokens::string_token::StringToken;
use crate::interpreter::lexer::tokens::name_token::NameToken;

pub enum AssignableToken {
    String(StringToken),
    IntegerToken(IntegerToken),
    // DoubleToken(DoubleToken)
    // Equation(EquationToken),
    // Object(ObjectToken),
    // Variable(NameToken),
}
use std::fmt::{Debug, Display, Formatter};

use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable};
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::operator::Operator;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::prefix_arithmetic::{PointerArithmetic, PrefixArithmetic};
use crate::core::model::abstract_syntax_tree_nodes::identifier::IdentifierError;
use crate::core::model::types::ty::Type;
use crate::core::parser::types::r#type::InferTypeError;
use crate::core::parser::utils::dyck::dyck_language_generic;
use crate::pattern;

pub mod expression;
pub mod operator;
pub mod prefix_arithmetic;

#[derive(Debug, PartialEq, Clone)]
pub struct Equation<'a> {
    source_code: &'a [TokenWithSpan],
    pub syntax_tree: ParseResult<Box<Expression>>,
    pos: i32,
    ch: Option<&'a TokenWithSpan>,
}

impl PartialEq for ParseResult<Box<Expression>> {
    fn eq(&self, other: &Self) -> bool {
        self.result.eq(&other.result)
    }
}


fn contains(a: &[TokenWithSpan], b: &TokenWithSpan) -> bool {
    a.iter().any(|x| x.token == b.token)
}

#[derive(Debug)]
#[allow(unused)]
pub enum Error {
    PositionNotInRange(i32),
    UndefinedSequence(String),
    FunctionNotFound,
    SourceEmpty,
    NotAType(String),
    // Message
    TermNotParsable(String),
    ParenExpected,
    BracketExpected,
    CannotParse,
}

impl From<InferTypeError> for Error {
    fn from(value: InferTypeError) -> Self {
        match value {
            InferTypeError::TypeNotAllowed(t) => Error::NotAType(t.to_string()),
            _ => unreachable!("Cannot reach this"),
        }
    }
}

impl From<IdentifierError> for Error {
    fn from(value: IdentifierError) -> Self {
        Error::UndefinedSequence(value.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::PositionNotInRange(index) => format!("Index {index} out of range"),
                Error::ParenExpected => "Expected \")\"".to_string(),
                Error::BracketExpected => "Expected \"]\"".to_string(),
                Error::TermNotParsable(v) => v.to_string(),
                Error::UndefinedSequence(value) => value.to_string(),
                Error::FunctionNotFound => "Not a function".to_string(),
                Error::SourceEmpty => "Source code is empty".to_string(),
                Error::CannotParse => "Cannot parse".to_string(),
                Error::NotAType(f) => format!("Unexpected type: {f}"),
            }
        )
    }
}

impl std::error::Error for Error {}


#[allow(clippy::should_implement_trait)]
impl<'a> Equation<'a> {
    pub fn new(tokens: &'a [TokenWithSpan]) -> Self {
        Self {
            source_code: tokens,
            syntax_tree: *Box::default(),
            pos: -1,
            ch: None,
        }
    }

    fn next_char(&mut self) {
        self.pos += 1;
        self.ch = self.source_code.get(self.pos as usize);
    }

    fn prev_char(&mut self) {
        self.pos -= 1;
        self.ch = self.source_code.get(self.pos as usize);
    }

    // skips the provided amount of characters
    fn next_char_amount(&mut self, amount_skip: usize) {
        for _ in 0..amount_skip {
            self.next_char();
        }
    }

    fn previous_char_amount(&mut self, amount_skip: usize) {
        for _ in 0..amount_skip {
            self.prev_char();
        }
    }

    fn eat(&mut self, char_to_eat: Option<Token>) -> bool {
        if let (Some(ch), Some(token_to_eat)) = (self.ch, char_to_eat) {
            if ch.token == token_to_eat {
                self.next_char();
                return true;
            }
        } else {
            return false;
        }

        false
    }


    fn parse(&mut self) -> Result<ParseResult<Box<Expression>>, crate::core::lexer::error::Error> where Self: Sized {
        self.next_char();

        if dyck_language_generic(self.source_code, [vec!['('], vec![','], vec![')']], vec![')'], contains).is_err() {
            return Err(crate::core::lexer::error::Error::UnexpectedToken(self.source_code[0].clone()));
        }

        if self.pos < 0 || self.pos >= self.source_code.len() as i32 {
            return Err(crate::core::lexer::error::Error::UnexpectedToken(self.source_code[0].clone()));
        }

        self.syntax_tree = self.parse_logical_or()?;
        Ok(self.syntax_tree.clone())
    }


    fn parse_logical_or(&mut self) -> Result<ParseResult<Box<Expression>>, crate::core::lexer::error::Error> where Self: Sized {
        let mut x = self.parse_logical_and()?;

        loop {
            if self.eat(Some(Token::LogicalOr)) {
                let expression = self.parse_logical_and()?;
                x.result.set(Some(x.result.clone()), Operator::LogicalOr, Some(expression.result), None);
                x.consumed = x.consumed + expression.consumed + 1;
            } else {
                return Ok(x);
            }
        }
    }

    fn parse_logical_and(&mut self) -> Result<ParseResult<Box<Expression>>, crate::core::lexer::error::Error> where Self: Sized {
        let mut x = self.parse_bitwise_or()?;

        loop {
            if self.eat(Some(Token::LogicalAnd)) {
                let expression = self.parse_bitwise_or()?;
                x.result.set(Some(x.result.clone()), Operator::LogicalAnd, Some(expression.result), None);
                x.consumed = x.consumed + expression.consumed + 1;
            } else {
                return Ok(x);
            }
        }
    }

    fn parse_bitwise_or(&mut self) -> Result<ParseResult<Box<Expression>>, crate::core::lexer::error::Error> where Self: Sized {
        let mut x = self.parse_bitwise_xor()?;

        loop {
            let latest_ch = self.ch;
            let latest_pos = self.pos;

            if self.eat(Some(Token::Pipe)) {
                let expression = self.parse_bitwise_xor()?;
                x.result.set(Some(x.result.clone()), Operator::BitwiseOr, Some(expression.result), None);
                x.consumed = x.consumed + expression.consumed + 1;
            } else {
                self.ch = latest_ch;
                self.pos = latest_pos;
                return Ok(x);
            }
        }
    }

    fn parse_bitwise_xor(&mut self) -> Result<ParseResult<Box<Expression>>, crate::core::lexer::error::Error> where Self: Sized {
        let mut x = self.parse_bitwise_and()?;

        loop {
            if self.eat(Some(Token::Xor)) {
                let expression = self.parse_bitwise_and()?;
                x.result.set(Some(x.result.clone()), Operator::BitwiseXor, Some(expression.result), None);
                x.consumed = x.consumed + expression.consumed + 1;
            } else {
                return Ok(x);
            }
        }
    }

    fn parse_bitwise_and(&mut self) -> Result<ParseResult<Box<Expression>>, crate::core::lexer::error::Error> where Self: Sized {
        let mut x = self.parse_equality_expression()?;

        loop {
            let latest_ch = self.ch;
            let latest_pos = self.pos;

            if self.eat(Some(Token::Ampersand)) {
                let expression = self.parse_equality_expression()?;
                x.result.set(
                    Some(x.result.clone()),
                    Operator::BitwiseAnd,
                    Some(expression.result),
                    None,
                );
                x.consumed = x.consumed + expression.consumed + 1;
            } else {
                self.ch = latest_ch;
                self.pos = latest_pos;
                return Ok(x);
            }
        }
    }

    fn parse_equality_expression(&mut self) -> Result<ParseResult<Box<Expression>>, crate::core::lexer::error::Error> where Self: Sized {
        let mut x = self.parse_relational_expression()?;

        loop {
            if self.eat(Some(Token::EqualsEquals)) {
                let expression = self.parse_relational_expression()?;
                x.result.set(Some(x.result.clone()), Operator::Equal, Some(expression.result), None);
                x.consumed = x.consumed + expression.consumed + 1;
            } else if self.eat(Some(Token::NotEquals)) {
                let expression = self.parse_relational_expression()?;
                x.result.set(Some(x.result.clone()), Operator::NotEqual, Some(expression.result), None);
                x.consumed = x.consumed + expression.consumed + 1;
            } else {
                return Ok(x);
            }
        }
    }

    fn parse_relational_expression(&mut self) -> Result<ParseResult<Box<Expression>>, crate::core::lexer::error::Error> where Self: Sized {
        let mut x = self.parse_bitwise_shift_expression()?;

        loop {
            if self.eat(Some(Token::LessThanEquals)) {
                let expression = self.parse_bitwise_shift_expression()?;
                x.result.set(
                    Some(x.result.clone()),
                    Operator::LessThanEqual,
                    Some(expression.result),
                    None,
                );
                x.consumed = x.consumed + expression.consumed + 1;
            } else if self.eat(Some(Token::GreaterThanEquals)) {
                let expression = self.parse_bitwise_shift_expression()?;
                x.result.set(
                    Some(x.result.clone()),
                    Operator::GreaterThanEqual,
                    Some(expression.result),
                    None,
                );
                x.consumed = x.consumed + expression.consumed + 1;
            } else if self.eat(Some(Token::LessThan)) {
                let expression = self.parse_bitwise_shift_expression()?;
                x.result.set(Some(x.result.clone()), Operator::LessThan, Some(expression.result), None);
                x.consumed = x.consumed + expression.consumed + 1;
            } else if self.eat(Some(Token::GreaterThan)) {
                let expression = self.parse_bitwise_shift_expression()?;
                x.result.set(
                    Some(x.result.clone()),
                    Operator::GreaterThan,
                    Some(expression.result),
                    None,
                );
                x.consumed = x.consumed + expression.consumed + 1;
            } else {
                return Ok(x);
            }
        }
    }

    fn parse_bitwise_shift_expression(&mut self) -> Result<ParseResult<Box<Expression>>, crate::core::lexer::error::Error> where Self: Sized {
        let mut x = self.parse_expression()?;

        loop {
            if self.eat(Some(Token::LeftShift)) {
                let expression = self.parse_expression()?;
                x.result.set(Some(x.result.clone()), Operator::LeftShift, Some(expression.result), None);
                x.consumed = x.consumed + expression.consumed + 1;
            } else if self.eat(Some(Token::RightShift)) {
                let expression = self.parse_expression()?;
                x.result.set(
                    Some(x.result.clone()),
                    Operator::RightShift,
                    Some(expression.result),
                    None,
                );
                x.consumed = x.consumed + expression.consumed + 1;
            } else {
                return Ok(x);
            }
        }
    }

    fn parse_expression(&mut self) -> Result<ParseResult<Box<Expression>>, crate::core::lexer::error::Error> where Self: Sized {
        let mut x = self.parse_term()?;

        loop {
            #[allow(clippy::if_same_then_else)]
            if self.eat(Some(Token::Plus)) {
                let term = self.parse_term()?;
                x.result.set(Some(x.result.clone()), Operator::Add, Some(term.result), None);
                x.consumed = x.consumed + term.consumed + 1;
            } else if self.eat(Some(Token::Minus)) {
                let term = self.parse_term()?;
                x.result.set(Some(x.result.clone()), Operator::Sub, Some(term.result), None);
                x.consumed = x.consumed + term.consumed + 1;
            } else {
                return Ok(x);
            }
        }
    }

    fn parse_term(&mut self) -> Result<ParseResult<Box<Expression>>, crate::core::lexer::error::Error> where Self: Sized {
        let mut x = self.parse_factor()?;
        loop {
            #[allow(clippy::if_same_then_else)]
            if self.eat(Some(Token::Multiply)) {
                let term = self.parse_factor()?;
                x.result.set(Some(x.result.clone()), Operator::Mul, Some(term.result), None);
                x.consumed = x.consumed + term.consumed + 1;
            } else if self.eat(Some(Token::Divide)) {
                let term = self.parse_factor()?;
                x.result.set(Some(x.result.clone()), Operator::Div, Some(term.result), None);
                x.consumed = x.consumed + term.consumed + 1;
            } else if self.eat(Some(Token::Modulo)) {
                let term = self.parse_factor()?;
                x.result.set(Some(x.result.clone()), Operator::Mod, Some(term.result), None);
                x.consumed = x.consumed + term.consumed + 1;
            } else {
                return Ok(x);
            }
        }
    }

    fn parse_factor(&mut self) -> Result<ParseResult<Box<Expression>>, crate::core::lexer::error::Error> where Self: Sized {
        let mut x: ParseResult<Box<Expression>> = *Box::default();

        //Start Prefix
        if self.eat(Some(Token::Plus)) {
            x = self.parse_factor()?;
            x.consumed += 1;
            return Ok(x);
        } else if self.eat(Some(Token::Minus)) {
            x = self.parse_factor()?;
            x.result.flip_value();
            x.consumed += 1;
            return Ok(x);
        }

        if let Some(MatchResult::Parse(cast_type)) = pattern!(&self.source_code[self.pos as usize..], ParenthesisOpen, @parse Type, ParenthesisClose) {
            self.next_char_amount(cast_type.consumed + 2);

            if let Ok(value) = self.parse_factor() {
                x = *Box::default();
    
                x.result.value = Some(Box::new(Assignable::Expression(*value.result)));
    
                x.result.prefix_arithmetic = Some(PrefixArithmetic::Cast(cast_type.result));
                x.consumed = value.consumed + cast_type.consumed + 2;
    
                return Ok(x);
            }
            
            self.previous_char_amount(cast_type.consumed + 2);
        }

        if self.eat(Some(Token::Multiply)) {
            let value = self.parse_factor()?;
            x = *Box::default();

            x.result.value = Some(Box::new(Assignable::Expression(*value.result)));
            x.result.prefix_arithmetic = Some(PrefixArithmetic::PointerArithmetic(
                PointerArithmetic::Asterics,
            ));

            x.consumed = value.consumed + 1;

            return Ok(x);
        }

        if self.eat(Some(Token::Ampersand)) {
            let value = self.parse_factor()?;
            x = *Box::default();

            x.result.value = Some(Box::new(Assignable::Expression(*value.result)));
            x.result.prefix_arithmetic = Some(PrefixArithmetic::PointerArithmetic(
                PointerArithmetic::Ampersand,
            ));

            x.consumed = value.consumed + 1;

            return Ok(x);
        }

        //End Prefix

        let start_pos: i32 = self.pos;
        if self.eat(Some(Token::ParenthesisOpen)) {
            x = self.parse_logical_or()?;
            x.consumed += 1;

            if !self.eat(Some(Token::ParenthesisClose)) {
                return Err(crate::core::lexer::error::Error::ExpectedToken(Token::ParenthesisClose));
            }

            x.consumed += 1;
        } else if self.ch.is_some() {
            // digits only
            if let Some(mut ch) = self.ch {
                if matches!(&ch.token, Token::Numbers(_)) || matches!(&ch.token, Token::Dot) {
                    while matches!(&ch.token, Token::Numbers(_))
                        || matches!(&ch.token, Token::Dot)
                        || matches!(&ch.token, Token::Underscore)
                        || matches!(&ch.token, Token::Literal(_)) {
                        self.next_char();
                        if let Some(r) = self.ch {
                            ch = r;
                        } else {
                            break;
                        }
                    }

                    let sub_expression = &self.source_code[start_pos as usize..self.pos as usize];
                    let assignment = Assignable::parse(sub_expression, ParseOptions::builder()
                        .with_ignore_expression(true)
                        .build())?;
                    x = ParseResult {
                        result: Box::new(Expression::from(Some(Box::new(assignment.result)))),
                        consumed: assignment.consumed,
                    };
                } else if matches!(&ch.token, Token::Literal(_) | Token::True | Token::False) {
                    let mut ident = 0;
                    let mut in_brackets = false;

                    while ident != 0 || in_brackets || !self.operator_sequence() {
                        match self.ch {
                            Some(TokenWithSpan { token: Token::ParenthesisOpen, ..}) => ident += 1,
                            Some(TokenWithSpan { token: Token::ParenthesisClose, ..}) => ident -= 1,
                            Some(TokenWithSpan { token: Token::SquareBracketOpen, ..}) => in_brackets = true,
                            Some(TokenWithSpan { token: Token::SquareBracketClose, ..}) => in_brackets = false,
                            Some(TokenWithSpan { token: Token::Literal(_), .. }) => { }
                            _ if ident <= 0 && !in_brackets => break,
                            _ => { }
                        }

                        if ident == -1 {
                            break;
                        }

                        self.next_char();
                    }

                    let sub_expression = &self
                        .source_code
                        .iter()
                        .skip(start_pos as usize)
                        .take(((self.pos - start_pos) as usize).max(1))
                        .cloned()
                        .collect::<Vec<TokenWithSpan>>();


                    let (index_operation, sub_string) = if let (Some(left), Some(right)) = (
                        sub_expression.iter().position(|a| a.token == Token::SquareBracketOpen),
                        sub_expression.iter().rposition(|a| a.token == Token::SquareBracketClose),
                    ) {
                        (
                            Some(Box::new(Assignable::parse(
                                &sub_expression[left + 1..right], ParseOptions::default()
                            )?)),
                            &sub_expression[..left],
                        )
                    } else {
                        (None, sub_expression.as_slice())
                    };

                    let assignable = Assignable::parse(
                        sub_string, ParseOptions::builder().with_ignore_expression(true).build()
                    )?;

                    x = ParseResult {
                        result: Box::new(Expression::from(Some(Box::new(assignable.result)))),
                        consumed: assignable.consumed,
                    };

                    x.consumed += index_operation.clone().map(|ip| ip.consumed + 2).unwrap_or(0);
                    x.result.index_operator = index_operation.map(|s| Box::new(s.result));

                    if (self.pos - start_pos) == 0 {
                        self.next_char();
                    }
                }
            } else {
                return self.undefined_or_empty();
            }
        } else {
            return self.undefined_or_empty();
        }

        if x == *Box::default() {
            return self.undefined_or_empty();
        }

        Ok(x)
    }

    fn undefined_or_empty(&self) -> Result<ParseResult<Box<Expression>>, crate::core::lexer::error::Error> where Self: Sized {
        if let Some(token) = self.ch {
            Err(crate::core::lexer::error::Error::UnexpectedToken(token.clone()))
        } else if let Some(last_character) = self.source_code.iter().last() {
            Err(crate::core::lexer::error::Error::UnexpectedToken(last_character.clone()))
        } else {
            Err(crate::core::lexer::error::Error::UnexpectedEOF)
        }
    }
    fn operator_sequence(&mut self) -> bool {
        static OPERATORS: [Token; 18] = [
            Token::Plus, Token::Minus, Token::Multiply, Token::Modulo, Token::Divide, Token::LeftShift,
            Token::RightShift, Token::LessThan, Token::GreaterThan, Token::LessThanEquals, Token::GreaterThanEquals,
            Token::EqualsEquals, Token::NotEquals, Token::LogicalAnd, Token::LogicalOr, Token::Ampersand,
            Token::Xor, Token::Pipe
        ];

        for operator in &OPERATORS {
            let latest_ch = self.ch;
            let latest_pos = self.pos;

            if self.eat(Some(operator.clone())) {
                self.ch = latest_ch;
                self.pos = latest_pos;

                return true;
            }

            self.ch = latest_ch;
            self.pos = latest_pos;
        }

        false
    }
}

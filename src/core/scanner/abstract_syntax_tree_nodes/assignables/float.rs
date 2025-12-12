use std::any::Any;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::asm_options::prepare_register::PrepareRegisterOption;
use crate::core::code_generator::asm_result::{ASMResult, ASMResultError};
use crate::core::code_generator::registers::{ByteSize};
use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::types::float::{FloatAST, FloatType};
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::integer::{NumberErr};

impl Parse for FloatAST {
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default {
        if let Some(TokenWithSpan { token: Token::Minus, ..}) = tokens.get(0) {
            let mut parsed_float = Self::parse(&tokens[1..], Default::default())?;
            parsed_float.result.value *= -1.0;

            return Ok(ParseResult {
                result: parsed_float.result,
                consumed: parsed_float.consumed + 1,
            });
        }

        if let Some(TokenWithSpan { token: Token::Plus, ..}) = tokens.get(0) {
            let mut parsed_float = Self::parse(&tokens[1..], Default::default())?;

            return Ok(ParseResult {
                result: parsed_float.result,
                consumed: parsed_float.consumed + 1,
            });
        }

        let (float_literal, expected_type, consumed) = match tokens.iter().map(|x| x.token.clone()).collect::<Vec<Token>>().as_slice() {
            [Token::Numbers(number), Token::Literal(postfix), ..] if postfix == "_f32" => (number.to_string(), FloatType::Float32, 2),
            [Token::Numbers(number), Token::Literal(postfix), ..] if postfix == "_f64" => (number.to_string(), FloatType::Float64, 2),
            [Token::Numbers(number), Token::Dot, Token::Numbers(decimal), Token::Literal(postfix), ..] if postfix == "_f32" => (format!("{}.{}", number, decimal), FloatType::Float32, 4),
            [Token::Numbers(number), Token::Dot, Token::Numbers(decimal), Token::Literal(postfix), ..] if postfix == "_f64" => (format!("{}.{}", number, decimal), FloatType::Float64, 4),
            [Token::Numbers(number), Token::Dot, Token::Numbers(decimal), ..] => (format!("{}.{}", number, decimal), FloatType::Float32, 3),
            [Token::Dot, Token::Numbers(decimal), Token::Literal(postfix), ..] if postfix == "_f32" => (format!("0.{}", decimal), FloatType::Float32, 3),
            [Token::Dot, Token::Numbers(decimal), Token::Literal(postfix), ..] if postfix == "_f64" => (format!("0.{}", decimal), FloatType::Float64, 3),
            [Token::Dot, Token::Numbers(decimal), ..] => (format!("0.{}", decimal), FloatType::Float32, 2),
            _ => return Err(Error::UnexpectedToken(tokens[0].clone()))
        };

        let value = float_literal.parse::<f64>().map_err(|_| Error::UnexpectedToken(tokens[0].clone()))?;
        let final_type = if (-3.40282347e+38..=3.40282347e+38).contains(&value) {
            if matches!(expected_type, FloatType::Float64) {
                FloatType::Float64
            } else {
                FloatType::Float32
            }
        } else {
            if matches!(expected_type, FloatType::Float32) {
                return Err(Error::UnexpectedToken(tokens[0].clone()));
            }
            FloatType::Float64
        };

        Ok(ParseResult {
            result: FloatAST {
                value,
                ty: final_type,
            },
            consumed,
        })
    }
}

impl FromStr for FloatAST {
    type Err = NumberErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !lazy_regex::regex_is_match!("^[+-]?(\\d+\\.\\d*|\\d*\\.\\d+)(_f64|_f32)?$", s) {
            return Err(NumberErr::UnmatchedRegex);
        }

        let expected_type = match s {
            a if a.ends_with("_f64") => FloatType::Float64,
            a if a.ends_with("_f32") => FloatType::Float32,
            _ => FloatType::Float32
        };

        let s = s.replace("_f64", "").replace("_f32", "");

        let value = s.parse::<f64>()?;

        let final_type = if (-3.40282347e+38..=3.40282347e+38).contains(&value) {
            if matches!(expected_type, FloatType::Float64) {
                FloatType::Float64
            } else {
                FloatType::Float32
            }
        } else {
            if matches!(expected_type, FloatType::Float32) {
                return Err(NumberErr::UnmatchedRegex);
            }
            FloatType::Float64
        };

        Ok(FloatAST {
            value,
            ty: final_type,
        })
    }
}
use std::fmt::{Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::{ASMResult};
use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::string::StaticString;

type IntegerType = crate::core::scanner::types::integer::Integer;


#[derive(Default, Debug, Eq, PartialEq, Clone)]
pub struct IntegerAST {
    // Must be stored as a string literal, because
    // you can have a bigger value than a i64. consider every number that's between i64::MAX and u64::MAX
    pub value: String,
    pub ty: IntegerType
}

impl Parse for IntegerAST {
    fn parse(tokens: &[TokenWithSpan]) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default {
        if let [number_literal, ..] = tokens {
            if let Token::Numbers(s) = &number_literal.token {
                if lazy_regex::regex_is_match!("^[+-]?\\d+$", s) {
                    let value: i128 = s.parse::<i128>().map_err(|e| Error::UnexpectedToken(tokens[0].clone()))?;

                    let final_type = match value {
                        -2_147_483_648..=2_147_483_647 => IntegerType::I32,
                        -9_223_372_036_854_775_808..=9_223_372_036_854_775_808 => IntegerType::I64,
                        _ => return Err(Error::UnexpectedToken(tokens[0].clone()))
                    };

                    return Ok(ParseResult {
                        result: IntegerAST {
                            value: value.to_string(),
                            ty: final_type,
                        },
                        consumed: 1,
                    })
                }
            }
        }

        Err(Error::UnexpectedToken(tokens[0].clone()))
    }
}


impl Display for IntegerAST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}


#[derive(Debug)]
pub enum NumberErr {
    UnmatchedRegex,
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError)
}

impl From<ParseIntError> for NumberErr {
    fn from(value: ParseIntError) -> Self {
        NumberErr::ParseIntError(value)
    }
}

impl From<ParseFloatError> for NumberErr {
    fn from(value: ParseFloatError) -> Self { NumberErr::ParseFloatError(value) }
}

impl std::error::Error for NumberErr { }

impl Display for NumberErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            NumberErr::UnmatchedRegex => "Integer must match ^[+-]?\\d+$".to_string(),
            NumberErr::ParseIntError(err) => err.to_string(),
            NumberErr::ParseFloatError(err) => err.to_string()
        })
    }
}

impl FromStr for IntegerAST {
    type Err = NumberErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !lazy_regex::regex_is_match!("^[+-]?\\d+$", s) {
            return Err(NumberErr::UnmatchedRegex);
        }

        let value: i128 = s.parse::<i128>()?;

        let final_type = match value {
            -2_147_483_648..=2_147_483_647 => IntegerType::I32,
            -9_223_372_036_854_775_808..=9_223_372_036_854_775_808 => IntegerType::I64,
            _ => return Err(NumberErr::UnmatchedRegex)
        };

        Ok(IntegerAST {
            value: value.to_string(),
            ty: final_type,
        })
    }
}

impl ToASM for IntegerAST {
    fn to_asm<T: ASMOptions>(&self, _stack: &mut Stack, _meta: &mut MetaInfo, _options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        Ok(ASMResult::Inline(self.value.to_string()))
    }


    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        match self.ty {
            IntegerType::I8 | IntegerType::U8 => 1,
            IntegerType::I16 | IntegerType::U16 => 2,
            IntegerType::I32 | IntegerType::U32 => 4,
            IntegerType::I64 | IntegerType::U64 => 8,
        }
    }
}
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
use crate::core::lexer::parse::{Parse, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::integer::{IntegerAST, NumberErr};

type FloatType = crate::core::scanner::types::float::Float;


#[derive(Debug, PartialEq, Clone, Default)]
pub struct FloatAST {
    // https://pastebin.com/DWcHQbT5
    // there is no need to use a string literal instead of a f64 like in the integerASTNode, because
    // you cant have a float that's bigger than the biggest value of f64. but you can have a bigger value than a i64. consider every number that's between i64::MAX and u64::MAX
    pub value: f64,
    pub ty: FloatType,
}

impl Display for FloatAST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl ToASM for FloatAST {
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        if let Some(options) = options {
            let any_t = &options as &dyn Any;
            if let Some(concrete_type) = any_t.downcast_ref::<InterimResultOption>() {
                let value_str = if !self.value.to_string().contains('.') {
                    format!("{}.0", self.value)
                } else {
                    self.value.to_string()
                };

                return match self.ty {
                    FloatType::Float32 => Ok(ASMResult::MultilineResulted(
                        ASMBuilder::mov_ident_line(concrete_type.general_purpose_register.to_size_register(&ByteSize::_4), format!("__?float32?__({})", value_str)), concrete_type.general_purpose_register.clone())
                    ),
                    FloatType::Float64 => Ok(ASMResult::MultilineResulted(
                        ASMBuilder::mov_ident_line(concrete_type.general_purpose_register.to_size_register(&ByteSize::_8), format!("__?float64?__({})", value_str)), concrete_type.general_purpose_register.clone())
                    )
                };
            }

            if let Some(s) = any_t.downcast_ref::<PrepareRegisterOption>() {
                return s.transform(stack, meta);
            }
        }

        Err(ASMGenerateError::ASMResult(ASMResultError::NoOptionProvided("float".to_string())))
    }


    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        match self.ty {
            FloatType::Float32 => 4,
            FloatType::Float64 => 8,
        }
    }
}


impl Parse for FloatAST {
    fn parse(tokens: &[TokenWithSpan]) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default {
        let (float_literal, expected_type, consumed) = match tokens.iter().map(|x| x.token.clone()).collect::<Vec<Token>>().as_slice() {
            [Token::Numbers(number), Token::Literal(postfix), ..] if postfix == "_f32" => (number.to_string(), FloatType::Float32, 2),
            [Token::Numbers(number), Token::Literal(postfix), ..] if postfix == "_f64" => (number.to_string(), FloatType::Float64, 2),
            [Token::Numbers(number), Token::Dot, Token::Numbers(decimal), Token::Literal(postfix), ..] if postfix == "_f32" => (format!("{}.{}", number, decimal), FloatType::Float32, 4),
            [Token::Numbers(number), Token::Dot, Token::Numbers(decimal), Token::Literal(postfix), ..] if postfix == "_f64" => (format!("{}.{}", number, decimal), FloatType::Float64, 4),
            [Token::Numbers(number), Token::Dot, Token::Numbers(decimal), ..] => (format!("{}.{}", number, decimal), FloatType::Float32, 3),
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
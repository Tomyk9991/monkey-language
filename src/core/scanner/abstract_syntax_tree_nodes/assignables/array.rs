use std::any::Any;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::core::code_generator::asm_result::{ASMResult};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, register_destination, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_options::identifier_present::IdentifierPresent;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::registers::{Bit64, ByteSize, GeneralPurposeRegister};
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::collect_tokens_until_scope_close::CollectTokensFromUntil;
use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::types::array::Array;
use crate::core::model::types::mutability::Mutability;
use crate::core::model::types::ty::Type;
use crate::core::scanner::static_type_context::StaticTypeContext;
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::method_call::{dyck_language, dyck_language_generic};
use crate::core::scanner::types::r#type::{InferTypeError};
use crate::pattern;

#[derive(Debug)]
pub enum ArrayErr {
    UnmatchedRegex,
}

fn contains(a: &[TokenWithSpan], b: &TokenWithSpan) -> bool {
    a.iter().any(|x| x.token == b.token)
}

impl Parse for Array {
    fn parse(tokens: &[TokenWithSpan]) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default {
        let slice = tokens.iter().map(|x| x.token.clone()).collect::<Vec<Token>>();

        if let [Token::SquareBracketOpen, Token::SquareBracketClose] = &slice[..] {
            return Ok(ParseResult {
                result: Array {
                    values: vec![]
                },
                consumed: 2,
            })
        }

        if let Some(MatchResult::Collect(array_content)) = pattern!(tokens, SquareBracketOpen, @ parse CollectTokensFromUntil<'[', ']'>, SquareBracketClose) {
            let array_elements = dyck_language_generic(&array_content, [vec!['{', '('], vec![','], vec!['}', ')']], contains)
                .map_err(|_| Error::UnexpectedToken(tokens[0].clone()))?;

            if array_elements.is_empty() {
                return Err(Error::UnexpectedToken(tokens[0].clone()));
            }

            let mut values = vec![];

            for array_element in &array_elements {
                values.push(Assignable::parse(array_element)?);
            }


            let tokens_consumed_square_brackets = 2;
            let tokens_consumed_assign = array_elements.iter().fold(0, |acc, x| acc + x.len());
            let tokens_consumed_separator = array_elements.len() - 1;

            return Ok(ParseResult {
                result: Array {
                    values: values.iter().map(|x| x.result.clone()).collect::<Vec<Assignable>>(),
                },
                consumed: tokens_consumed_square_brackets + tokens_consumed_assign + tokens_consumed_separator,
            })
        }

        Err(Error::UnexpectedToken(tokens[0].clone()))
    }
}

impl std::error::Error for ArrayErr { }

impl Array {
    pub fn infer_type_with_context(&self, context: &StaticTypeContext, code_line: &CodeLine) -> Result<Type, InferTypeError> {
        if self.values.is_empty() {
            return Err(InferTypeError::NoTypePresent(LValue::Identifier(Identifier { name: "Array".to_string() }), code_line.clone()))
        }

        if let Ok(ty) = self.values[0].infer_type_with_context(context, code_line) {
            return Ok(Type::Array(Box::new(ty), self.values.len(), Mutability::Immutable));
        }

        Err(InferTypeError::NoTypePresent(LValue::Identifier(Identifier { name: "Array".to_string() }), code_line.clone()))
    }
}

impl Display for ArrayErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ArrayErr::UnmatchedRegex => "Array must match: [type, size]"
        })
    }
}


impl FromStr for Array {
    type Err = ArrayErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !lazy_regex::regex_is_match!(r"^\[.*\]$", s) {
            return Err(ArrayErr::UnmatchedRegex);
        }

        if s.replace(" ", "") == "[]" {
            return Ok(Array {
                values: vec![]
            })
        }

        if let ["[ ", array_content @ .., "]"] = &s.split_inclusive(' ').collect::<Vec<_>>()[..] {
            let array_elements_str = dyck_language(&array_content.join(" "), [vec!['{', '('], vec![','], vec!['}', ')']])
                .map_err(|_| ArrayErr::UnmatchedRegex)?;

            if array_elements_str.is_empty() {
                return Err(ArrayErr::UnmatchedRegex);
            }

            let mut values = vec![];

            for array_element in &array_elements_str {
                values.push(Assignable::from_str(array_element).map_err(|_| ArrayErr::UnmatchedRegex)?);
            }

            return Ok(Array {
                values,
            })
        }

        Err(ArrayErr::UnmatchedRegex)
    }
}
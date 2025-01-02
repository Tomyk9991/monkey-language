use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_result::ASMResult;
use crate::core::code_generator::register_destination::word_from_byte_size;
use crate::core::code_generator::registers::{GeneralPurposeRegister};
use crate::core::lexer::tokens::assignable_token::AssignableToken;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::EquationToken;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::expression::Expression;
use crate::core::lexer::tokens::name_token::{NameToken, NameTokenErr};
use crate::core::lexer::tokens::assignable_tokens::equation_parser::prefix_arithmetic::{PointerArithmetic, PrefixArithmetic};

#[derive(Debug, PartialEq, Clone)]
pub enum LValue {
    Name(NameToken),
    PrefixArithmetic(NameToken, Vec<PointerArithmetic>)
}

#[derive(Debug)]
pub enum LValueErr {
    KeywordReserved(String),
}

impl Display for LValueErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            LValueErr::KeywordReserved(value) => {
                format!("The variable name \"{}\" variable name can't have the same name as a reserved keyword", value)
            }
        };
        write!(f, "{}", message)
    }
}

impl Error for LValueErr { }

impl Display for LValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            LValue::Name(name) => name.to_string(),
            LValue::PrefixArithmetic(name, prefixes) => format!("{}{}", prefixes.iter().map(|p| p.to_string()).collect::<String>(), name.to_string())
        })
    }
}

impl FromStr for LValue {
    type Err = LValueErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(name) = NameToken::from_str(s, false) {
            Ok(LValue::Name(name))
        } else if let Ok((name_token, arithmetic)) = Self::arithmetic(s, false) {
            Ok(LValue::PrefixArithmetic(name_token, arithmetic))
        } else {
            return Err(LValueErr::KeywordReserved(s.to_string()))
        }
    }
}

impl LValue {
    pub fn identifier(&self) -> &str {
        match self {
            LValue::Name(name) => name.name.as_str(),
            LValue::PrefixArithmetic(name, _) => name.name.as_str()
        }
    }

    fn arithmetic(s: &str, is_attribute: bool) -> Result<(NameToken, Vec<PointerArithmetic>), NameTokenErr> {
        let mut arithmetic = vec![];
        let mut target = s;
        while let Some(char) = target.chars().nth(0) {
            let a = match char {
                '&' => Some(PointerArithmetic::Ampersand),
                '*' => Some(PointerArithmetic::Asterics),
                _ => None
            };
            if let Some(a) = a {
                arithmetic.push(a);
                target = &target[1..];
            } else {
                break;
            }
        }

        let name = NameToken::from_str(target, is_attribute)?;
        Ok((name, arithmetic))
    }
}

impl ToASM for LValue {
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        match self {
            LValue::Name(name) => name.to_asm(stack, meta, options),
            LValue::PrefixArithmetic(name, arithmetic) => {
                let str = format!("{}{}", arithmetic.iter().map(|p| p.to_string()).collect::<String>(), name.to_string());
                let mut l_value_equation = EquationToken::from_str(&str).map_err(|_| ASMGenerateError::LValueAssignment(self.clone(), meta.code_line.clone()))?;
                let resulting_type = l_value_equation.traverse_type(meta).ok_or(ASMGenerateError::LValueAssignment(self.clone(), meta.code_line.clone()))?;


                if let (Some(prefix_arithmetic), Some(inner_value)) = (&l_value_equation.prefix_arithmetic, l_value_equation.value) {
                    let last_register = stack.register_to_use.last().ok_or(ASMGenerateError::LValueAssignment(self.clone(), meta.code_line.clone()))?;
                    let mut result = Expression::prefix_arithmetic_to_asm(
                        prefix_arithmetic,
                        &inner_value,
                        &GeneralPurposeRegister::Memory(format!("{} [{}]", word_from_byte_size(resulting_type.byte_size()), last_register.to_64_bit_register())),
                        stack, meta);

                    if let Ok(mut result) = result {
                        result.remove_latest_line();
                        Ok(result)
                    } else {
                        result
                    }
                } else {
                    Err(ASMGenerateError::LValueAssignment(self.clone(), meta.code_line.clone()))
                }
            },
        }
    }

    fn is_stack_look_up(&self, stack: &mut Stack, meta: &MetaInfo) -> bool {
        match self {
            LValue::Name(a) => a.is_stack_look_up(stack, meta),
            LValue::PrefixArithmetic(a, _) => a.is_stack_look_up(stack, meta)
        }
    }

    fn byte_size(&self, meta: &mut MetaInfo) -> usize {
        match self {
            LValue::Name(a) => a.byte_size(meta),
            LValue::PrefixArithmetic(a, _) => a.byte_size(meta)
        }
    }
}
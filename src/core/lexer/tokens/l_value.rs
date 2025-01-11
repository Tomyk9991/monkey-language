use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::asm_result::ASMResult;
use crate::core::code_generator::register_destination::word_from_byte_size;
use crate::core::code_generator::registers::{GeneralPurposeRegister};
use crate::core::lexer::tokens::assignable_tokens::equation_parser::EquationToken;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::expression::Expression;
use crate::core::lexer::tokens::name_token::{NameToken, NameTokenErr};
use crate::core::lexer::tokens::assignable_tokens::equation_parser::prefix_arithmetic::PointerArithmetic;

#[derive(Debug, PartialEq, Clone)]
pub enum LValue {
    Name(NameToken),
    Expression(Expression),
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
            LValue::Expression(e) => e.to_string()
        })
    }
}

impl FromStr for LValue {
    type Err = LValueErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(name) = NameToken::from_str(s, false) {
            Ok(LValue::Name(name))
        } else if let Ok(equation) = EquationToken::from_str(s) {
            Ok(LValue::Expression(equation))
        } else {
            return Err(LValueErr::KeywordReserved(s.to_string()))
        }
    }
}

impl LValue {
    pub fn identifier(&self) -> String {
        match self {
            LValue::Name(name) => name.name.clone(),
            LValue::Expression(e) => e.identifier().unwrap_or(e.to_string())
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
            LValue::Expression(l_value_equation) => {
                let resulting_type = l_value_equation.traverse_type(meta).ok_or(ASMGenerateError::LValueAssignment(self.clone(), meta.code_line.clone()))?;

                if let (Some(prefix_arithmetic), Some(inner_value)) = (&l_value_equation.prefix_arithmetic, &l_value_equation.value) {
                    let last_register = stack.register_to_use.last().ok_or(ASMGenerateError::LValueAssignment(self.clone(), meta.code_line.clone()))?;
                    let result = Expression::prefix_arithmetic_to_asm(
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
                } else if let (Some(index_operation), Some(inner_value)) = (&l_value_equation.index_operator, &l_value_equation.value) {
                    let i = index_operation.to_asm::<InterimResultOption>(stack, meta, None)?;
                    stack.indexing = Some(i);
                    let result = inner_value.to_asm::<InterimResultOption>(stack, meta, None);
                    stack.indexing = None;

                    result
                    // let mut target = String::new();
                    // target += &ASMBuilder::ident_comment_line(&format!("LValue: {}", self));
                    //
                    // let mut last_register = stack.register_to_use.last().ok_or(ASMGenerateError::LValueAssignment(self.clone(), meta.code_line.clone()))?.clone();
                    //
                    // match index_operation.to_asm::<InterimResultOption>(stack, meta, None)? {
                    //     ASMResult::Inline(a) => {
                    //         target += &ASMBuilder::mov_ident_line(last_register.to_64_bit_register(), a);
                    //     }
                    //     ASMResult::MultilineResulted(asm, resulting_register) => {
                    //         last_register = resulting_register;
                    //         target.push_str(&asm);
                    //     }
                    //     ASMResult::Multiline(_) => { unreachable!() }
                    // }
                    //
                    // let destination = match inner_value.to_asm::<InterimResultOption>(stack, meta, None)? {
                    //     ASMResult::Inline(a) => {
                    //         GeneralPurposeRegister::Memory(a)
                    //     }
                    //     ASMResult::MultilineResulted(asm, resulting_register) => {
                    //         target.push_str(&asm);
                    //         resulting_register
                    //     }
                    //     ASMResult::Multiline(_) => { unreachable!() }
                    // };
                    //
                    //
                    // return Ok(ASMResult::MultilineResulted(target, GeneralPurposeRegister::Memory(format!("{} [{}]", word_from_byte_size(resulting_type.byte_size()), last_register.to_64_bit_register()))))
                }
                else {
                    Err(ASMGenerateError::LValueAssignment(self.clone(), meta.code_line.clone()))
                }
            }
        }
    }

    fn is_stack_look_up(&self, stack: &mut Stack, meta: &MetaInfo) -> bool {
        match self {
            LValue::Name(a) => a.is_stack_look_up(stack, meta),
            LValue::Expression(e) => e.is_stack_look_up(stack, meta)
        }
    }

    fn byte_size(&self, meta: &mut MetaInfo) -> usize {
        match self {
            LValue::Name(a) => a.byte_size(meta),
            LValue::Expression(e) => e.byte_size(meta)
        }
    }
}
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::asm_result::ASMResult;
use crate::core::code_generator::register_destination::word_from_byte_size;
use crate::core::code_generator::registers::{GeneralPurposeRegister};
use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseResult};
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::equation_parser::Equation;
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use crate::core::scanner::abstract_syntax_tree_nodes::identifier::Identifier;

#[derive(Debug, PartialEq, Clone)]
pub enum LValue {
    Identifier(Identifier),
    Expression(Expression),
}

impl Default for LValue {
    fn default() -> Self {
        LValue::Identifier(Identifier::default())
    }
}


impl Parse for LValue {
    fn parse(tokens: &[TokenWithSpan]) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default {
        if let Ok(identifier) = Identifier::from_str(&format!("{}", tokens[0].token), false) {
            Ok(ParseResult {
                consumed: 1,
                result: LValue::Identifier(identifier)
            })
        } else {
            Err(Error::UnexpectedToken(tokens[0].clone()))
        }
    }
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

impl std::error::Error for LValueErr { }

impl Display for LValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            LValue::Identifier(name) => name.to_string(),
            LValue::Expression(e) => e.to_string()
        })
    }
}

impl FromStr for LValue {
    type Err = LValueErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(name) = Identifier::from_str(s, false) {
            Ok(LValue::Identifier(name))
        } else if let Ok(equation) = Equation::from_str(s) {
            Ok(LValue::Expression(equation))
        } else {
            return Err(LValueErr::KeywordReserved(s.to_string()))
        }
    }
}

impl LValue {
    pub fn identifier(&self) -> String {
        match self {
            LValue::Identifier(name) => name.name.clone(),
            LValue::Expression(e) => e.identifier().unwrap_or(e.to_string())
        }
    }
}

impl ToASM for LValue {
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        match self {
            LValue::Identifier(name) => name.to_asm(stack, meta, options),
            LValue::Expression(l_value_equation) => {
                let resulting_type = l_value_equation.traverse_type(meta).ok_or(ASMGenerateError::LValueAssignment(self.clone(), meta.code_line.clone()))?;

                if let (Some(prefix_arithmetic), Some(inner_value)) = (&l_value_equation.prefix_arithmetic, &l_value_equation.value) {
                    let last_register = stack.register_to_use.last().ok_or(ASMGenerateError::LValueAssignment(self.clone(), meta.code_line.clone()))?;
                    let result = Expression::prefix_arithmetic_to_asm(
                        prefix_arithmetic,
                        inner_value,
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
                }
                else {
                    Err(ASMGenerateError::LValueAssignment(self.clone(), meta.code_line.clone()))
                }
            }
        }
    }

    fn is_stack_look_up(&self, stack: &mut Stack, meta: &MetaInfo) -> bool {
        match self {
            LValue::Identifier(a) => a.is_stack_look_up(stack, meta),
            LValue::Expression(e) => e.is_stack_look_up(stack, meta)
        }
    }

    fn byte_size(&self, meta: &mut MetaInfo) -> usize {
        match self {
            LValue::Identifier(a) => a.byte_size(meta),
            LValue::Expression(e) => e.byte_size(meta)
        }
    }
}
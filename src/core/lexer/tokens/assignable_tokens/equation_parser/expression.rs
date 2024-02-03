use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use crate::core::lexer::types::cast_to::{Castable, CastToError};
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::generator::{LastUnchecked, Stack};
use crate::core::code_generator::register_destination::from_byte_size;
use crate::core::code_generator::registers::{ByteSize, FloatRegister, GeneralPurposeRegister, GeneralPurposeRegisterIterator};
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::tokens::assignable_token::AssignableToken;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::{Operator};
use crate::core::lexer::tokens::name_token::NameToken;
use crate::core::lexer::types::boolean::Boolean;
use crate::core::lexer::types::float::Float;
use crate::core::lexer::types::integer::Integer;
use crate::core::lexer::types::type_token::{InferTypeError, TypeToken};

#[derive(Clone, PartialEq, Debug)]
pub enum PointerArithmetic {
    /// *
    Asterics,
    /// &
    Ampersand,
}

#[derive(Clone, PartialEq, Debug)]
pub enum PrefixArithmetic {
    #[allow(unused)]
    Operation(Operator),
    // For example the "-" like let a = -5;
    PointerArithmetic(PointerArithmetic),
    Cast(TypeToken),
}

#[derive(Clone, PartialEq)]
#[allow(unused)]
pub struct Expression {
    pub lhs: Option<Box<Expression>>,
    pub rhs: Option<Box<Expression>>,
    pub operator: Operator,
    pub prefix_arithmetic: Option<PrefixArithmetic>,
    pub value: Option<Box<AssignableToken>>,
    pub positive: bool,
}

impl Expression {
    pub fn pointers(&self) -> Vec<PointerArithmetic> {
        let mut pointer_arithmetic = vec![];
        for prefix in &self.prefix_arithmetic {
            if let PrefixArithmetic::PointerArithmetic(p) = &prefix {
                pointer_arithmetic.push(p.clone());
            }
        }

        pointer_arithmetic
    }

    fn iterator_from_type(&self, meta: &mut MetaInfo, lhs_size: usize) -> Result<(GeneralPurposeRegisterIterator, Option<Float>), ASMGenerateError> {
        if let Some(lhs) = &self.lhs {
            let ty = &lhs.traverse_type(meta).ok_or(ASMGenerateError::InternalError("Could not traverse type".to_string()))?;
            return Ok(if let TypeToken::Float(f) = ty {
                (GeneralPurposeRegister::iter_float_register()?, Some(f.clone()))
            } else {
                (GeneralPurposeRegister::iter_from_byte_size(lhs_size)?, None)
            })
        }

        Err(ASMGenerateError::InternalError("Internal error".to_string()))
    }

    fn latest_used_destination_register(&self, meta: &mut MetaInfo, target: &str, lhs_size: usize) -> Result<GeneralPurposeRegister, ASMGenerateError> {
        let pushing_register: GeneralPurposeRegister = if let Some(last_instruction) = extract_last_general_purpose_instruction(target) {
            let (mut i, _) = self.iterator_from_type(meta, lhs_size)?;

            if let Some(mut r) = i.nth(2) {
                if let Some(space_index) = last_instruction.chars().position(|a| a == ' ') {
                    if let Some(comma_index) = last_instruction.chars().position(|a| a == ',') {
                        r = GeneralPurposeRegister::from_str(&last_instruction[space_index + 1..comma_index])?;
                    }
                }

                r
            } else {
                unreachable!()
            }
        } else {
            let (mut i, _) = self.iterator_from_type(meta, lhs_size)?;
            if let Some(r) = i.nth(2) {
                r
            } else {
                unreachable!()
            }
        };
        Ok(pushing_register)
    }

    fn cut_last_register_to_size(stack: &mut Stack, float_type: &Option<Float>) -> Result<GeneralPurposeRegister, ASMGenerateError> {
        let last = if let Some(f) = &float_type {
            match f.byte_size() {
                8 => stack.register_to_use.last()?.to_64_bit_register(),
                4 => stack.register_to_use.last()?.to_32_bit_register(),
                _ => stack.register_to_use.last()?,
            }
        } else {
            stack.register_to_use.last()?
        };
        Ok(last)
    }

    fn generate_lhs(&self, stack: &mut Stack, meta: &mut MetaInfo, target: &mut String, lhs: &Expression, lhs_size: usize) -> Result<String, ASMGenerateError> {
        let mut r = lhs.to_asm(stack, meta)?;


        Ok(if let Some(PrefixArithmetic::Cast(_)) = lhs.prefix_arithmetic {
            target.push_str(&r);
            self.latest_used_destination_register(meta, &r, lhs_size)?.to_string()
        } else {

            if let Ok((is_multi_line, code, general_purpose_register)) = lhs.multi_line_asm(stack, meta) {
                if is_multi_line {
                    if let Some(gpr) = general_purpose_register {
                        let target_register = stack.register_to_use.last()?;
                        println!("{}", target_register);

                        target.push_str(&ASMBuilder::push_registers(&[&target_register]));

                        target.push_str(&code);

                        target.push_str(&ASMBuilder::mov_ident_line(&target_register, gpr.to_size_register(&target_register.size()).to_string()));
                        target.push_str(&ASMBuilder::pop_registers(&[&target_register]));

                        r = gpr.to_size_register(&ByteSize::try_from(lhs_size)?).to_string()
                    }
                }
            }

            r
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn generate_rhs(&self, stack: &mut Stack, meta: &mut MetaInfo, target: &mut String, rhs: &Expression, lhs_size: usize, destination_register: &GeneralPurposeRegister, target_register: &GeneralPurposeRegister) -> Result<(), ASMGenerateError> {
        let source = if let Some(AssignableToken::FloatToken(f)) = &rhs.value.as_deref() {
            let destination_register = from_byte_size(f.byte_size(meta));
            target.push_str(&ASMBuilder::mov_ident_line(&destination_register, rhs.to_asm(stack, meta)?));
            target.push_str(&ASMBuilder::mov_x_ident_line(target_register, destination_register, Some(f.byte_size(meta))));
            target_register.to_string()
        } else {
            let mut r = rhs.to_asm(stack, meta)?;

            if let Ok((is_multi_line, code, general_purpose_register)) = rhs.multi_line_asm(stack, meta) {
                if is_multi_line {
                    if let Some(gpr) = general_purpose_register {
                        target.push_str(&ASMBuilder::push_registers(&[target_register]));

                        target.push_str(&code);

                        target.push_str(&ASMBuilder::mov_ident_line(target_register, gpr.to_size_register(&destination_register.size()).to_string()));
                        target.push_str(&ASMBuilder::pop_registers(&[target_register]));

                        r = target_register.to_string()
                    }
                }
            }

            if let Some(PrefixArithmetic::Cast(_)) = rhs.prefix_arithmetic {
                target.push_str(&r);
                let register = self.latest_used_destination_register(meta, &r, lhs_size)?.to_string();
                target.push_str(&ASMBuilder::mov_x_ident_line(target_register, register, Some(lhs_size)));
                target_register.to_string()
            } else {
                r
            }
        };

        let ty = &rhs.traverse_type(meta).ok_or(ASMGenerateError::InternalError("Could not traverse type".to_string()))?;
        let operation = &self.operator.specific_operation(ty, &[destination_register.to_string(), source], stack, meta)?.inject_registers();
        target.push_str(&ASMBuilder::ident_line(operation));
        Ok(())
    }

    fn move_result(&self, _meta: &mut MetaInfo, stack: &mut Stack, target: &mut String, lhs_size: usize, float_type: &Option<Float>, source_register: &GeneralPurposeRegister) -> Result<(), ASMGenerateError> {
        let last = Self::cut_last_register_to_size(stack, float_type)?;
        let source_register = if float_type.is_none() {
            source_register.to_size_register(&last.size())
        } else {
            source_register.clone()
        };

        if stack.register_to_use.len() == 1 && !matches!(stack.register_to_use.last()?, GeneralPurposeRegister::Float(_)) {
            let bool_destination;
            let source_register = if last.size() == 1 {
                bool_destination = true;
                source_register.to_64_bit_register().to_size_register(&ByteSize::_1)
            } else {
                bool_destination = false;
                source_register
            };
            target.push_str(&ASMBuilder::mov_x_ident_line(last, source_register, if float_type.is_some() && !bool_destination { Some(lhs_size) } else { None }));
        } else {
            let bool_destination;
            let source_register = if stack.register_to_use.last()?.size() == 1 {
                bool_destination = true;
                source_register.to_64_bit_register().to_size_register(&ByteSize::_1)
            } else {
                bool_destination = false;
                source_register
            };
            target.push_str(&ASMBuilder::mov_x_ident_line(stack.register_to_use.last()?, source_register, if float_type.is_some() && !bool_destination { Some(lhs_size) } else { None }));
        }
        Ok(())
    }

    fn pop_to_register(target: &mut String, float_type: &Option<Float>, register_target: &GeneralPurposeRegister) -> Result<(), ASMGenerateError> {
        if let Some(f) = &float_type {
            target.push_str(&ASMBuilder::ident_line(&format!("pop {}", register_target.to_64_bit_register())));
            target.push_str(&ASMBuilder::mov_x_ident_line(register_target, register_target.to_size_register(&ByteSize::try_from(f.byte_size())?), Some(f.byte_size())));
        } else {
            target.push_str(&ASMBuilder::ident_line(&format!("pop {}", register_target.to_64_bit_register())));
        }
        Ok(())
    }
}

impl Default for Expression {
    fn default() -> Self {
        Self {
            lhs: None,
            rhs: None,
            operator: Operator::Noop,
            value: None,
            positive: true,
            prefix_arithmetic: None,
        }
    }
}

impl Debug for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct_formatter = f.debug_struct("Expression");

        if let Some(lhs) = &self.lhs {
            debug_struct_formatter.field("lhs", lhs);
        }

        debug_struct_formatter.field("operator", &self.operator);

        if let Some(rhs) = &self.rhs {
            debug_struct_formatter.field("rhs", rhs);
        }

        if let Some(value) = &self.value {
            debug_struct_formatter.field("value", value);
        }

        debug_struct_formatter.field("positive", &self.positive);
        let prefix_arithmetic = self.prefix_arithmetic.iter().map(|a| a.to_string()).collect::<String>();

        debug_struct_formatter.field("prefix_arithmetic", &prefix_arithmetic);
        debug_struct_formatter.finish()
    }
}

impl Display for PrefixArithmetic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PrefixArithmetic::Operation(operation) => operation.to_string(),
            PrefixArithmetic::PointerArithmetic(p) => p.to_string(),
            PrefixArithmetic::Cast(c) => format!("({c})")
        })
    }
}

impl Display for PointerArithmetic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PointerArithmetic::Asterics => "*".to_string(),
            PointerArithmetic::Ampersand => "&".to_string()
        })
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let sign = if self.positive { "".to_string() } else { "-".to_string() };
        let prefix_arithmetic = self.prefix_arithmetic.iter().rev().map(|a| a.to_string()).collect::<String>();

        match (&self.lhs, &self.rhs) {
            (Some(lhs), Some(rhs)) => {
                write!(f, "{}{}({} {} {})", prefix_arithmetic, sign, lhs, &self.operator, rhs)
            }
            _ => {
                if let Some(ass) = &self.value {
                    write!(f, "{}{}{}", prefix_arithmetic, sign, ass)
                } else {
                    write!(f, "Some error. No lhs and rhs and no value found")
                }
            }
        }
    }
}


impl From<Option<Box<AssignableToken>>> for Expression {
    fn from(value: Option<Box<AssignableToken>>) -> Self {
        Expression {
            value,
            ..Default::default()
        }
    }
}

impl ToASM for Expression {
    fn to_asm(&self, stack: &mut Stack, meta: &mut MetaInfo) -> Result<String, ASMGenerateError> {
        if let Some(value) = &self.value { // no lhs and rhs
            let mut target = String::new();

            if let Some(prefix_arithmetic) = &self.prefix_arithmetic {
                if stack.register_to_use.is_empty() {
                    let assignable_type = self.traverse_type_resulted(&meta.static_type_information, &meta.code_line)?;
                    let iterator = GeneralPurposeRegister::iter_from_byte_size(assignable_type.byte_size())?;

                    stack.register_to_use.push(iterator.current());
                }
                target += &ASMBuilder::push(&Self::prefix_arithmetic_to_asm(prefix_arithmetic, value, &stack.register_to_use.last().ok(), stack, meta)?);
                if stack.register_to_use.len() == 1 { stack.register_to_use.pop(); }
            } else {
                if let Ok((is_multi_line, code, _register)) = &value.multi_line_asm(stack, meta) {
                    if *is_multi_line {
                        target += &ASMBuilder::push(code);
                    } else {
                        target += &value.to_asm(stack, meta)?;
                    }
                } else {
                    target += &value.to_asm(stack, meta)?;
                }
            }

            return Ok(target);
        }

        let mut target = String::new();
        target += &ASMBuilder::ident(&ASMBuilder::comment_line(&format!("{}", self)));

        match (&self.lhs, &self.rhs) {
            (Some(lhs), Some(rhs)) => {
                // first optimization. use every register
                if let (Some(a), Some(b), Some(c), Some(d)) = (&lhs.lhs, &lhs.rhs, &rhs.lhs, &rhs.rhs) {
                    if let (Some(_), Some(_), Some(_), Some(_)) = (&a.value, &b.value, &c.value, &d.value) {
                        // two expressions containing two values
                        let (lhs_size, _) = lhs_rhs_byte_sizes(lhs, rhs, meta)?;
                        let (mut register_iterator, float_type) = self.iterator_from_type(meta, lhs_size)?;

                        let register_a = register_iterator.current();
                        let register_b = register_iterator.next().ok_or(ASMGenerateError::InternalError("No next register found".to_string()))?;
                        let register_c = register_iterator.next().ok_or(ASMGenerateError::InternalError("No next register found".to_string()))?;

                        stack.register_to_use.push(register_b.clone());
                        target += &ASMBuilder::push(&lhs.to_asm(stack, meta)?.to_string());
                        stack.register_to_use.pop();

                        stack.register_to_use.push(register_c.clone());
                        target += &ASMBuilder::push(&rhs.to_asm(stack, meta)?.to_string());
                        stack.register_to_use.pop();

                        let ty = &self.traverse_type(meta).ok_or(ASMGenerateError::InternalError("Could not traverse type".to_string()))?;
                        target += &ASMBuilder::ident_line(&self.operator.specific_operation(ty, &[&register_b, &register_c], stack, meta)?.inject_registers());
                        target += &ASMBuilder::mov_x_ident_line(register_a, register_b, if float_type.is_some() { Some(lhs_size) } else { None });

                        return Ok(target);
                    }
                }
                match (&lhs.value, &rhs.value) {
                    (Some(_), Some(_)) => { // 2 + 3
                        let (lhs_size, _) = lhs_rhs_byte_sizes(lhs, rhs, meta)?;
                        let (mut register_iterator, float_type) = self.iterator_from_type(meta, lhs_size)?;
                        let next_register = register_iterator.current();


                        // pushing twice. the last pop will move the arithmetic result into this register,
                        // basically eax or rax or anything similar where a result is expected
                        if stack.register_to_use.is_empty() {
                            stack.register_to_use.push(next_register.clone());
                        }

                        stack.register_to_use.push(next_register.clone());
                        let destination_register = stack.register_to_use.last()?;
                        if lhs.is_pointer() {
                            target += &ASMBuilder::push(&lhs.to_asm(stack, meta)?);
                        } else {
                            let source = if let Some(AssignableToken::FloatToken(f)) = &lhs.value.as_deref() {
                                let destination_register = from_byte_size(f.byte_size(meta));
                                target += &ASMBuilder::mov_ident_line(&destination_register, lhs.to_asm(stack, meta)?);
                                destination_register
                            } else {
                                self.generate_lhs(stack, meta, &mut target, lhs, lhs_size)?
                            };

                            target += &ASMBuilder::mov_x_ident_line(&destination_register, source, if float_type.is_some() { Some(lhs_size) } else { None });
                        };
                        stack.register_to_use.pop();

                        let next_register = register_iterator.nth(2).ok_or(ASMGenerateError::InternalError("No next register found".to_string()))?;

                        stack.register_to_use.push(next_register);
                        let target_register = stack.register_to_use.last()?;
                        if rhs.is_pointer() {
                            target += &ASMBuilder::push(&rhs.to_asm(stack, meta)?);
                            let ty = &self.traverse_type(meta).ok_or(ASMGenerateError::InternalError("Could not traverse type".to_string()))?;
                            target += &ASMBuilder::ident_line(&self.operator.specific_operation(ty, &[&destination_register, &target_register], stack, meta)?.inject_registers());
                        } else {
                            self.generate_rhs(stack, meta, &mut target, rhs, lhs_size, &destination_register, &target_register)?;
                        };
                        stack.register_to_use.pop();

                        self.move_result(meta, stack, &mut target, lhs_size, &float_type, &destination_register)?;

                        if stack.register_to_use.len() == 1 {
                            stack.register_to_use.pop();
                        }
                    }
                    (None, Some(_)) => { // (3 + 2) + 5
                        let (lhs_size, _) = lhs_rhs_byte_sizes(lhs, rhs, meta)?;
                        let (mut register_iterator, float_type) = self.iterator_from_type(meta, lhs_size)?;
                        let register_a = register_iterator.current();
                        let register_b = register_iterator.nth(2).ok_or(ASMGenerateError::InternalError("No next register found".to_string()))?;


                        if stack.register_to_use.is_empty() {
                            stack.register_to_use.push(register_a.clone());
                        }

                        stack.register_to_use.push(register_a);
                        let destination_register = stack.register_to_use.last()?;
                        target += &ASMBuilder::push(&lhs.to_asm(stack, meta)?.to_string());
                        stack.register_to_use.pop();

                        stack.register_to_use.push(register_b);
                        let target_register = stack.register_to_use.last()?;
                        if rhs.is_pointer() {
                            target += &ASMBuilder::push(&rhs.to_asm(stack, meta)?);
                            let ty = &self.traverse_type(meta).ok_or(ASMGenerateError::InternalError("Could not traverse type".to_string()))?;
                            target += &ASMBuilder::ident_line(&self.operator.specific_operation(ty, &[&destination_register, &target_register], stack, meta)?.inject_registers());
                        } else {
                            self.generate_rhs(stack, meta, &mut target, rhs, lhs_size, &destination_register, &target_register)?;
                        };
                        stack.register_to_use.pop();
                        self.move_result(meta, stack, &mut target, lhs_size, &float_type, &destination_register)?;


                        if stack.register_to_use.len() == 1 {
                            stack.register_to_use.pop();
                        }
                    }
                    (Some(_), None) => { // 5 + (3 + 2)
                        let (lhs_size, _) = lhs_rhs_byte_sizes(lhs, rhs, meta)?;
                        let (mut register_iterator, float_type) = self.iterator_from_type(meta, lhs_size)?;

                        let register_a = register_iterator.current();
                        let register_b = register_iterator.nth(2).ok_or(ASMGenerateError::InternalError("No next register found".to_string()))?;

                        if stack.register_to_use.is_empty() {
                            stack.register_to_use.push(register_a.clone());
                        }

                        stack.register_to_use.push(register_b.clone());
                        let target_register = stack.register_to_use.last()?;
                        target += &ASMBuilder::push(&rhs.to_asm(stack, meta)?.to_string());
                        stack.register_to_use.pop();

                        stack.register_to_use.push(register_a.clone());
                        let destination_register = stack.register_to_use.last()?;
                        if lhs.is_pointer() {
                            target += &ASMBuilder::push(&lhs.to_asm(stack, meta)?);
                            let ty = &self.traverse_type(meta).ok_or(ASMGenerateError::InternalError("Could not traverse type".to_string()))?;
                            target += &ASMBuilder::ident_line(&self.operator.specific_operation(ty, &[&register_a, &register_b], stack, meta)?.inject_registers());
                        } else {
                            let source = if let Some(AssignableToken::FloatToken(f)) = &lhs.value.as_deref() {
                                let destination_register = from_byte_size(f.byte_size(meta));
                                target += &ASMBuilder::mov_ident_line(&destination_register, lhs.to_asm(stack, meta)?);
                                let register_c = register_iterator.next().ok_or(ASMGenerateError::InternalError("No next register found".to_string()))?;
                                target += &ASMBuilder::mov_x_ident_line(&register_c, destination_register, Some(f.byte_size(meta)));
                                register_c.to_string()
                            } else {
                                self.generate_lhs(stack, meta, &mut target, lhs, lhs_size)?
                            };

                            target += &ASMBuilder::mov_x_ident_line(&destination_register, source, if float_type.is_some() { Some(lhs_size) } else { None });
                            let ty = &self.traverse_type(meta).ok_or(ASMGenerateError::InternalError("Could not traverse type".to_string()))?;
                            target += &ASMBuilder::ident_line(&self.operator.specific_operation(ty, &[&destination_register, &target_register], stack, meta)?.inject_registers());
                        };
                        stack.register_to_use.pop();
                        self.move_result(meta, stack, &mut target, lhs_size, &float_type, &register_a)?;

                        if stack.register_to_use.len() == 1 {
                            stack.register_to_use.pop();
                        }
                    }
                    (None, None) => { // ((1 + 2) + (3 + 4)) + ((5 + 6) + (7 + 8)) // any depth
                        let (lhs_size, _) = lhs_rhs_byte_sizes(lhs, rhs, meta)?;
                        let (mut register_iterator, float_type) = self.iterator_from_type(meta, lhs_size)?;

                        let register_a = register_iterator.current();
                        let register_b = register_iterator.nth(1).ok_or(ASMGenerateError::InternalError("No next register found".to_string()))?;

                        stack.register_to_use.push(register_b.clone());
                        target += &ASMBuilder::push(&lhs.to_asm(stack, meta)?.to_string());
                        stack.register_to_use.pop();

                        let pushing_register = self.latest_used_destination_register(meta, &target, lhs_size)?;

                        if float_type.is_some() {
                            target += &ASMBuilder::mov_x_ident_line(pushing_register.to_64_bit_register(), &pushing_register, Some(8));
                        }

                        target += &ASMBuilder::ident_line(&format!("push {}", pushing_register.to_64_bit_register()));
                        target += &ASMBuilder::ident_line(&format!("xor {}, {}", pushing_register.to_64_bit_register(), pushing_register.to_64_bit_register()));

                        stack.register_to_use.push(register_a.clone());
                        target += &ASMBuilder::push(&rhs.to_asm(stack, meta)?.to_string());
                        stack.register_to_use.pop();

                        if float_type.is_some() {
                            let pushing_register = self.latest_used_destination_register(meta, &target, lhs_size)?;
                            target += &ASMBuilder::mov_x_ident_line(register_a.to_64_bit_register(), pushing_register, Some(8));
                        }

                        target += &ASMBuilder::ident_line(&format!("push {}", register_a.to_64_bit_register()));
                        target += &ASMBuilder::ident_line(&format!("xor {}, {}", register_a.to_64_bit_register(), register_a.to_64_bit_register()));

                        Self::pop_to_register(&mut target, &float_type, &register_b)?;
                        Self::pop_to_register(&mut target, &float_type, &register_a)?;

                        let ty = &self.traverse_type(meta).ok_or(ASMGenerateError::InternalError("Could not traverse type".to_string()))?;
                        target += &ASMBuilder::ident_line(&self.operator.specific_operation(ty, &[&register_a, &register_b], stack, meta)?.inject_registers());
                    }
                }
            }
            (_, _) => return Err(ASMGenerateError::NotImplemented { token: "Something went wrong. Neither rhs nor lhs are valid".to_string() })
        }

        Ok(target)
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        true
    }

    fn byte_size(&self, meta: &mut MetaInfo) -> usize {
        if let Some(ty) = self.traverse_type(meta) {
            return ty.byte_size();
        }

        0
    }

    fn before_label(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Option<Result<String, ASMGenerateError>> {
        None
    }

    fn multi_line_asm(&self, stack: &mut Stack, meta: &mut MetaInfo) -> Result<(bool, String, Option<GeneralPurposeRegister>), ASMGenerateError> {
        if let Some(value) = &self.value {
            return value.multi_line_asm(stack, meta);
        }

        Ok((false, String::new(), None))
    }
}


fn lhs_rhs_byte_sizes(a: &Expression, b: &Expression, meta: &mut MetaInfo) -> Result<(usize, usize), ASMGenerateError> {
    let lhs_size = a.byte_size(meta);
    let rhs_size = b.byte_size(meta);

    if lhs_size != rhs_size {
        return Err(ASMGenerateError::NotImplemented { token: format!("Expected both types to be the same byte size. lhs: {}, rhs: {}", lhs_size, rhs_size) });
    }

    Ok((lhs_size, rhs_size))
}

fn extract_last_general_purpose_instruction(current_asm: &str) -> Option<String> {
    for line in current_asm.lines().rev() {
        let line = line.trim();

        if line.starts_with(';') || line.contains("r12") || line.contains("r13") || line.contains("r14") || line.starts_with(".label") {
            continue;
        }

        return Some(line.to_string());
    }

    None
}

#[allow(unused)]
impl Expression {
    pub fn new(lhs: Option<Box<Expression>>, operator: Operator, rhs: Option<Box<Expression>>, value: Option<Box<AssignableToken>>) -> Self {
        Self {
            lhs,
            rhs,
            operator,
            prefix_arithmetic: None,
            value,
            positive: true,
        }
    }

    pub fn pointer(&self) -> Option<PointerArithmetic> {
        if let Some(PrefixArithmetic::PointerArithmetic(a)) = &self.prefix_arithmetic {
            return Some(a.clone());
        }

        None
    }


    fn prefix_arithmetic_to_asm(prefix_arithmetic: &PrefixArithmetic, value: &AssignableToken, float_register: &Option<GeneralPurposeRegister>, stack: &mut Stack, meta: &mut MetaInfo) -> Result<String, ASMGenerateError> {
        let mut target = String::new();
        let mut inner_source = String::new();
        let register_to_use = stack.register_to_use.last()?;
        let register_64 = register_to_use.to_64_bit_register();


        let mut child_has_pointer_arithmetic = false;
        let mut float_type = None;

        if let Some(prefix_arithmetic) = value.prefix_arithmetic() {
            if let AssignableToken::ArithmeticEquation(a) = value {
                if let Some(child) = &a.value {
                    target += &ASMBuilder::push(&Self::prefix_arithmetic_to_asm(&prefix_arithmetic, child, float_register, stack, meta)?);

                    inner_source = match prefix_arithmetic {
                        PrefixArithmetic::PointerArithmetic(_) => {
                            child_has_pointer_arithmetic = true;
                            format!("QWORD [{}]", register_64)
                        },
                        PrefixArithmetic::Cast(_) => {
                            GeneralPurposeRegister::Float(FloatRegister::Xmm7).to_string()
                        },
                        _ => register_64.to_string()
                    };
                }
            }
        } else {
            inner_source = value.to_asm(stack, meta)?;
            float_type = value.infer_type_with_context(&meta.static_type_information, &meta.code_line).ok();
        }


        match prefix_arithmetic {
            PrefixArithmetic::PointerArithmetic(PointerArithmetic::Asterics) => {
                target += &ASMBuilder::mov_ident_line(&register_64, inner_source);
                if !child_has_pointer_arithmetic {
                    target += &ASMBuilder::mov_ident_line(&register_64, format!("QWORD [{}]", register_64));

                    if let (Some(GeneralPurposeRegister::Float(destination_float_register)), Some(f)) = (float_register, &float_type) {
                        target += &ASMBuilder::mov_x_ident_line(destination_float_register, &register_64, Some(f.byte_size()));
                    }
                }
            }
            PrefixArithmetic::PointerArithmetic(PointerArithmetic::Ampersand) => {
                target += &ASMBuilder::ident_line(
                    &format!("lea {}, {}", register_64, inner_source.replace("QWORD ", "").replace("DWORD ", ""))
                );
            }
            PrefixArithmetic::Cast(ty) => {
                let assignable_type = value.infer_type_with_context(&meta.static_type_information, &meta.code_line)?;
                let cast_to = assignable_type.cast_to(ty);

                if child_has_pointer_arithmetic {
                    inner_source = register_64.to_string();
                }


                target += &match (&cast_to.from, &cast_to.to) {
                    (TypeToken::Float(f1), TypeToken::Float(f2)) => Float::cast_from_to(f1, f2, &inner_source, stack, meta)?,
                    (TypeToken::Integer(i1), TypeToken::Float(f2)) => Integer::cast_from_to(i1, f2, &inner_source, stack, meta)?,
                    (TypeToken::Bool, TypeToken::Integer(i2)) => Boolean::cast_from_to(&Boolean::True, i2, &inner_source, stack, meta)?,
                    (TypeToken::Float(f1), TypeToken::Integer(i2)) => Float::cast_from_to(f1, i2, &inner_source, stack, meta)?,
                    (TypeToken::Integer(i1), TypeToken::Integer(i2)) => Integer::cast_from_to(i1, i2, &inner_source, stack, meta)?,
                    _ => return Err(ASMGenerateError::CastUnsupported(CastToError::CastUnsupported(cast_to.clone()), meta.code_line.clone()))
                };

                return Ok(target);
            }
            PrefixArithmetic::Operation(_) => {
                unimplemented!("Prefix operations are not supported yet (-+)")
            }
        }


        Ok(target)
    }

    pub fn is_pointer(&self) -> bool {
        !self.pointers().is_empty()
    }

    pub fn traverse_type(&self, meta: &MetaInfo) -> Option<TypeToken> {
        self.traverse_type_resulted(&meta.static_type_information, &meta.code_line).ok()
    }

    pub fn traverse_type_resulted(&self, context: &StaticTypeContext, code_line: &CodeLine) -> Result<TypeToken, InferTypeError> {
        if let Some(value) = &self.value {
            let value_type = value.infer_type_with_context(context, code_line);
            let has_prefix_arithmetics = self.prefix_arithmetic.is_some();

            return if let (true, Ok(value_type)) = (has_prefix_arithmetics, &value_type) {
                let mut current_pointer_arithmetic: String = match value_type {
                    TypeToken::Custom(name) if name.name.starts_with(['*', '&']) => {
                        if let Some(index) = name.name.chars().position(|m| m.is_ascii_alphanumeric()) {
                            name.name[..index].to_string()
                        } else {
                            "".to_string()
                        }
                    }
                    _ => "".to_string()
                };

                let mut value_type = value_type.clone();

                if let Some(prefix_arithmetic) = &self.prefix_arithmetic {
                    match prefix_arithmetic {
                        PrefixArithmetic::PointerArithmetic(PointerArithmetic::Asterics) if current_pointer_arithmetic.ends_with('*') => {
                            if let Some(new_ty) = value_type.pop_pointer() {
                                value_type = new_ty;
                                current_pointer_arithmetic = current_pointer_arithmetic.chars().collect::<Vec<char>>()[..current_pointer_arithmetic.len() - 1].iter().collect::<String>();
                            } else {
                                return Err(InferTypeError::IllegalDereference(*value.clone(), code_line.clone()));
                            }
                        }
                        PrefixArithmetic::PointerArithmetic(PointerArithmetic::Ampersand) => {
                            value_type = value_type.push_pointer();
                        }
                        PrefixArithmetic::PointerArithmetic(PointerArithmetic::Asterics) => {
                            // just using & in front of non pointer types is illegal. Dereferencing non pointers doesnt make any sense
                            return Err(InferTypeError::IllegalDereference(*value.clone(), code_line.clone()));
                        }
                        PrefixArithmetic::Cast(casting_to) => {
                            value_type = TypeToken::from_str(&casting_to.to_string())?;
                        }
                        PrefixArithmetic::Operation(_) => {}
                    }
                }

                if value_type.is_pointer() {
                    Ok(TypeToken::Custom(NameToken { name: format!("{}", value_type) }))
                } else {
                    Ok(value_type)
                }
            } else {
                value_type
            };
        }

        Self::check_operator_compatibility(self.to_string(), &self.lhs, self.operator.clone(), &self.rhs, context, code_line)
    }

    fn check_operator_compatibility(error_message: String, lhs: &Option<Box<Expression>>, operator: Operator, rhs: &Option<Box<Expression>>, context: &StaticTypeContext, code_line: &CodeLine) -> Result<TypeToken, InferTypeError> {
        if let Some(lhs) = &lhs {
            if let Some(rhs) = &rhs {
                let lhs_type = lhs.traverse_type_resulted(context, code_line)?;
                let rhs_type = rhs.traverse_type_resulted(context, code_line)?;

                let mut base_type_matrix: HashMap<(TypeToken, Operator, TypeToken), TypeToken> = HashMap::new();
                base_type_matrix.insert((TypeToken::Custom(NameToken { name: "string".to_string() }), Operator::Add, TypeToken::Custom(NameToken { name: "string".to_string() })), TypeToken::Custom(NameToken { name: "*string".to_string() }));

                Integer::operation_matrix(&mut base_type_matrix);
                Float::operation_matrix(&mut base_type_matrix);
                Boolean::operation_matrix(&mut base_type_matrix);

                if let Some(result_type) = base_type_matrix.get(&(lhs_type.clone(), operator.clone(), rhs_type.clone())) {
                    return Ok(result_type.clone());
                }

                return Err(InferTypeError::TypesNotCalculable(lhs_type, operator, rhs_type, code_line.clone()));
            }
        }

        Err(InferTypeError::UnresolvedReference(error_message, code_line.clone()))
    }

    pub fn set(&mut self, lhs: Option<Box<Expression>>, operation: Operator, rhs: Option<Box<Expression>>, value: Option<Box<AssignableToken>>) {
        self.lhs = lhs;
        self.rhs = rhs;
        self.operator = operation;
        self.value = value;
        self.prefix_arithmetic = None;
    }

    pub fn flip_value(&mut self) {
        if let Some(AssignableToken::IntegerToken(i)) = &mut self.value.as_deref_mut() {
            i.value = "-".to_string() + &i.value;
        }

        if let Some(AssignableToken::FloatToken(f)) = &mut self.value.as_deref_mut() {
            f.value *= -1.0;
        }

        if let Some(v) = &mut self.value {
            self.positive = !self.positive;
        }
    }
}
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_options::in_expression_method_call::InExpressionMethodCall;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::asm_options::prepare_register::PrepareRegisterOption;
use crate::core::code_generator::asm_result::{ASMResult, ASMResultError, ASMResultVariance};
use crate::core::code_generator::generator::{LastUnchecked, Stack};
use crate::core::code_generator::registers::{ByteSize, FloatRegister, GeneralPurposeRegister, GeneralPurposeRegisterIterator};
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::tokens::assignable_token::AssignableToken;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::prefix_arithmetic::{PointerArithmetic, PrefixArithmetic, PrefixArithmeticOptions};
use crate::core::lexer::tokens::name_token::NameToken;
use crate::core::lexer::types::boolean::Boolean;
use crate::core::lexer::types::float::Float;
use crate::core::lexer::types::integer::Integer;
use crate::core::lexer::types::type_token::{InferTypeError, Mutability, TypeToken};

#[derive(Clone, PartialEq)]
#[allow(unused)]
pub struct Expression {
    pub lhs: Option<Box<Expression>>,
    pub rhs: Option<Box<Expression>>,
    pub operator: Operator,
    pub prefix_arithmetic: Option<PrefixArithmetic>,
    pub value: Option<Box<AssignableToken>>,
    pub index_operator: Option<Box<AssignableToken>>,
    pub positive: bool,
}

impl Expression {
    fn iterator_from_type(&self, meta: &mut MetaInfo, lhs_size: usize) -> Result<(GeneralPurposeRegisterIterator, Option<Float>), ASMGenerateError> {
        if let Some(lhs) = &self.lhs {
            let ty = &lhs.traverse_type(meta).ok_or(ASMGenerateError::InternalError("Could not traverse type".to_string()))?;

            return Ok(if let TypeToken::Float(f, _) = ty {
                (GeneralPurposeRegister::iter_from_byte_size(lhs_size)?, Some(f.clone()))
            } else {
                (GeneralPurposeRegister::iter_from_byte_size(lhs_size)?, None)
            });
        }

        Err(ASMGenerateError::InternalError("Internal error".to_string()))
    }

    /// identifier expects a variable name. Expressions per se dont have variables names, but the identifier function is called on a l_value
    pub fn identifier(&self) -> Option<String> {
        if let Some(value) = &self.value {
            return value.identifier();
        }

        None
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

    fn pop_to_register(target: &mut String, float_type: &Option<Float>, register_target: &GeneralPurposeRegister) -> Result<(), ASMGenerateError> {
        if let Some(f) = &float_type {
            target.push_str(&ASMBuilder::ident_line(&format!("pop {}", register_target.to_64_bit_register())));
            target.push_str(&ASMBuilder::mov_x_ident_line(register_target, register_target.to_size_register(&ByteSize::try_from(f.byte_size())?), Some(f.byte_size())));
        } else {
            target.push_str(&ASMBuilder::ident_line(&format!("pop {}", register_target.to_64_bit_register())));
        }
        Ok(())
    }

    fn expression_some_some_some_some(&self, stack: &mut Stack, meta: &mut MetaInfo, lhs: &Expression, rhs: &Expression) -> Result<Option<ASMResult>, ASMGenerateError> {
        if let (Some(a), Some(b), Some(c), Some(d)) = (&lhs.lhs, &lhs.rhs, &rhs.lhs, &rhs.rhs) {
            if let (Some(_), Some(_), Some(_), Some(_)) = (&a.value, &b.value, &c.value, &d.value) {
                // two expressions containing two values
                let mut target = String::new();
                let (lhs_size, _) = lhs_rhs_byte_sizes(lhs, rhs, meta)?;
                let (mut register_iterator, _) = self.iterator_from_type(meta, lhs_size)?;

                let register_b = register_iterator.next().ok_or(ASMGenerateError::InternalError("No next register found".to_string()))?;
                let register_c = register_iterator.next().ok_or(ASMGenerateError::InternalError("No next register found".to_string()))?;

                stack.register_to_use.push(register_b.clone());
                let mut destination_register = stack.register_to_use.last()?;

                match lhs.to_asm(stack, meta, Some(PrepareRegisterOption {
                    general_purpose_register: destination_register.clone(),
                    assignable_token: lhs.value.clone().map(|value| value.as_ref().clone()),
                }))? {
                    ASMResult::Inline(inline) => target += &ASMBuilder::mov_x_ident_line(&destination_register, inline, Some(lhs.byte_size(meta))),
                    ASMResult::MultilineResulted(s, r) => {
                        target += &s;
                        destination_register = if r.is_float_register() { destination_register.to_float_register() } else { destination_register };
                        target += &ASMBuilder::mov_x_ident_line(&destination_register, &r, Some(r.size() as usize));
                    }
                    ASMResult::Multiline(_) => return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                        expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                        actual: ASMResultVariance::Multiline,
                        token: "Expression".to_string(),
                    }))
                }
                stack.register_to_use.pop();

                stack.register_to_use.push(register_c.clone());
                let mut target_register = stack.register_to_use.last()?;
                match rhs.to_asm(stack, meta, Some(PrepareRegisterOption {
                    general_purpose_register: target_register.clone(),
                    assignable_token: rhs.value.clone().map(|value| value.as_ref().clone()),
                }))? {
                    ASMResult::Inline(inline) => target += &ASMBuilder::mov_x_ident_line(&target_register, inline, Some(rhs.byte_size(meta))),
                    ASMResult::MultilineResulted(s, r) => {
                        target += &s;
                        target_register = if r.is_float_register() { target_register.to_float_register() } else { target_register };
                        target += &ASMBuilder::mov_x_ident_line(&target_register, &r, Some(r.size() as usize));
                    }
                    ASMResult::Multiline(_) => return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                        expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                        actual: ASMResultVariance::Multiline,
                        token: "Expression".to_string(),
                    }))
                }
                stack.register_to_use.pop();

                let ty = &rhs.traverse_type(meta).ok_or(ASMGenerateError::InternalError("Could not traverse type".to_string()))?;
                let operation = self.operator.specific_operation(ty, &[&destination_register, &target_register], stack, meta)?.inject_registers();
                target += &ASMBuilder::ident_line(&operation.0);
                return Ok(Some(ASMResult::MultilineResulted(target, operation.1)));
            }
        }

        Ok(None)
    }

    fn expression_some_some(&self, stack: &mut Stack, meta: &mut MetaInfo, lhs: &Expression, rhs: &Expression) -> Result<ASMResult, ASMGenerateError> {
        let mut target = String::new();
        target += &ASMBuilder::ident(&ASMBuilder::comment_line(&format!("{}", self)));
        let (lhs_size, _) = lhs_rhs_byte_sizes(lhs, rhs, meta)?;
        let (mut register_iterator, _) = self.iterator_from_type(meta, lhs_size)?;
        let next_register = register_iterator.current();


        // pushing twice. the last pop will move the arithmetic result into this register,
        // basically eax or rax or anything similar where a result is expected
        if stack.register_to_use.is_empty() {
            stack.register_to_use.push(next_register.clone());
        }

        stack.register_to_use.push(next_register.clone());
        let mut destination_register = stack.register_to_use.last()?;

        match lhs.to_asm(stack, meta, Some(PrepareRegisterOption {
            general_purpose_register: destination_register.clone(),
            assignable_token: lhs.value.clone().map(|value| value.as_ref().clone()),
        }))? {
            ASMResult::Inline(inline) => target += &ASMBuilder::mov_x_ident_line(&destination_register, inline, Some(rhs.byte_size(meta))),
            ASMResult::MultilineResulted(s, new_register) => {
                target += &s;
                let final_ty = self.traverse_type_resulted(&meta.static_type_information, &meta.code_line)?;
                let maybe_new_register = if final_ty.is_float() { new_register.to_float_register() } else { new_register.clone() };

                target += &ASMBuilder::mov_x_ident_line(&maybe_new_register, new_register, Some(final_ty.byte_size()));
                destination_register = maybe_new_register;
            }
            ASMResult::Multiline(_) => return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                actual: ASMResultVariance::Multiline,
                token: "Expression".to_string(),
            }))
        }

        let next_register = register_iterator.nth(2).ok_or(ASMGenerateError::InternalError("No next register found".to_string()))?;

        stack.register_to_use.push(next_register);
        let target_register = stack.register_to_use.last()?;

        match rhs.to_asm(stack, meta, Some(PrepareRegisterOption {
            general_purpose_register: target_register.clone(),
            assignable_token: rhs.value.clone().map(|value| value.as_ref().clone()),
        }))? {
            ASMResult::Inline(inline) => {
                let ty = rhs.traverse_type(meta).ok_or(ASMGenerateError::InternalError("Could not traverse type".to_string()))?;
                let operation = self.operator.specific_operation(&ty, &[destination_register.to_string(), inline.to_string()], stack, meta)?.inject_registers();
                target += &ASMBuilder::ident_line(&operation.0);
                destination_register = operation.1;
            }
            ASMResult::MultilineResulted(s, mut new_register) => {
                target += &s;
                let final_ty = self.traverse_type_resulted(&meta.static_type_information, &meta.code_line)?;
                let maybe_new_register = if final_ty.is_float() { new_register.to_float_register() } else { new_register.clone() };


                target += &ASMBuilder::mov_x_ident_line(&maybe_new_register, &new_register, Some(final_ty.byte_size()));
                new_register = maybe_new_register;


                let ty = rhs.traverse_type(meta).ok_or(ASMGenerateError::InternalError("Could not traverse type".to_string()))?;
                let operation = self.operator.specific_operation(&ty, &[destination_register.to_string(), new_register.to_string()], stack, meta)?.inject_registers();
                target += &ASMBuilder::ident_line(&operation.0);
                destination_register = operation.1;
            }
            ASMResult::Multiline(_) => return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                actual: ASMResultVariance::Multiline,
                token: "Expression".to_string(),
            }))
        }
        stack.register_to_use.pop();

        if stack.register_to_use.len() == 1 {
            stack.register_to_use.pop();
        }

        Ok(ASMResult::MultilineResulted(target, destination_register))
    }

    fn expression_none_some(&self, stack: &mut Stack, meta: &mut MetaInfo, lhs: &Expression, rhs: &Expression) -> Result<ASMResult, ASMGenerateError> {
        let mut target = String::new();
        target += &ASMBuilder::ident(&ASMBuilder::comment_line(&format!("{}", self)));
        let (lhs_size, _) = lhs_rhs_byte_sizes(lhs, rhs, meta)?;
        let (mut register_iterator, _) = self.iterator_from_type(meta, lhs_size)?;
        let register_a = register_iterator.current();
        let register_b = register_iterator.nth(2).ok_or(ASMGenerateError::InternalError("No next register found".to_string()))?;

        if stack.register_to_use.is_empty() {
            stack.register_to_use.push(register_a.clone());
        }


        stack.register_to_use.push(register_a);
        let mut destination_register = stack.register_to_use.last()?;
        match lhs.to_asm(stack, meta, Some(PrepareRegisterOption {
            general_purpose_register: destination_register.clone(),
            assignable_token: lhs.value.clone().map(|value| value.as_ref().clone()),
        }))? {
            ASMResult::Inline(inline) => target += &ASMBuilder::mov_x_ident_line(&destination_register, inline, Some(rhs.byte_size(meta))),
            ASMResult::MultilineResulted(s, new_register) => {
                target += &s;
                let final_ty = self.traverse_type_resulted(&meta.static_type_information, &meta.code_line)?;
                let maybe_new_register = if final_ty.is_float() { new_register.to_float_register() } else { new_register.clone() };

                target += &ASMBuilder::mov_x_ident_line(&maybe_new_register, new_register, Some(final_ty.byte_size()));
                destination_register = maybe_new_register;
            }
            ASMResult::Multiline(_) => return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                actual: ASMResultVariance::Multiline,
                token: "Expression".to_string(),
            }))
        }

        stack.register_to_use.pop();

        stack.register_to_use.push(register_b);
        let target_register = stack.register_to_use.last()?;
        match rhs.to_asm(stack, meta, Some(PrepareRegisterOption {
            general_purpose_register: target_register.clone(),
            assignable_token: rhs.value.clone().map(|value| value.as_ref().clone()),
        }))? {
            ASMResult::Inline(inline) => {
                let ty = rhs.traverse_type(meta).ok_or(ASMGenerateError::InternalError("Could not traverse type".to_string()))?;
                let operation = self.operator.specific_operation(&ty, &[destination_register.to_string(), inline.to_string()], stack, meta)?.inject_registers();
                target += &ASMBuilder::ident_line(&operation.0);
                destination_register = operation.1;
            }
            ASMResult::MultilineResulted(s, mut new_register) => {
                target += &s;
                let final_ty = self.traverse_type_resulted(&meta.static_type_information, &meta.code_line)?;
                let maybe_new_register = if final_ty.is_float() { new_register.to_float_register() } else { new_register.clone() };

                target += &ASMBuilder::mov_x_ident_line(&maybe_new_register, &new_register, Some(final_ty.byte_size()));
                new_register = maybe_new_register;


                let ty = rhs.traverse_type(meta).ok_or(ASMGenerateError::InternalError("Could not traverse type".to_string()))?;
                let operation = self.operator.specific_operation(&ty, &[destination_register.to_string(), new_register.to_string()], stack, meta)?.inject_registers();
                target += &ASMBuilder::ident_line(&operation.0);
                destination_register = operation.1;
            }
            ASMResult::Multiline(_) => return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                actual: ASMResultVariance::Multiline,
                token: "Expression".to_string(),
            }))
        }
        stack.register_to_use.pop();

        Ok(ASMResult::MultilineResulted(target, destination_register))
    }

    fn expression_some_none(&self, stack: &mut Stack, meta: &mut MetaInfo, lhs: &Expression, rhs: &Expression) -> Result<ASMResult, ASMGenerateError> {
        let mut target = String::new();
        target += &ASMBuilder::ident(&ASMBuilder::comment_line(&format!("{}", self)));
        let (lhs_size, _) = lhs_rhs_byte_sizes(lhs, rhs, meta)?;
        let (mut register_iterator, _) = self.iterator_from_type(meta, lhs_size)?;

        let register_a = register_iterator.current();
        let register_b = register_iterator.nth(2).ok_or(ASMGenerateError::InternalError("No next register found".to_string()))?;

        if stack.register_to_use.is_empty() {
            stack.register_to_use.push(register_a.clone());
        }


        stack.register_to_use.push(register_b);
        let mut target_register = stack.register_to_use.last()?;
        match rhs.to_asm(stack, meta, Some(PrepareRegisterOption {
            general_purpose_register: target_register.clone(),
            assignable_token: rhs.value.clone().map(|value| value.as_ref().clone()),
        }))? {
            ASMResult::Inline(inline) => target += &ASMBuilder::mov_x_ident_line(&target_register, inline, Some(rhs.byte_size(meta))),
            ASMResult::MultilineResulted(s, new_register) => {
                target += &s;

                target_register = if new_register.is_float_register() { target_register.to_float_register() } else { target_register };
                target += &ASMBuilder::mov_x_ident_line(&target_register, &new_register, Some(new_register.size() as usize));
            }
            ASMResult::Multiline(_) => return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                actual: ASMResultVariance::Multiline,
                token: "Expression".to_string(),
            }))
        }

        stack.register_to_use.pop();

        stack.register_to_use.push(register_a);
        let mut destination_register = stack.register_to_use.last()?;
        match lhs.to_asm(stack, meta, Some(PrepareRegisterOption {
            general_purpose_register: destination_register.clone(),
            assignable_token: lhs.value.clone().map(|value| value.as_ref().clone()),
        }))? {
            ASMResult::Inline(inline) => {
                let ty = rhs.traverse_type(meta).ok_or(ASMGenerateError::InternalError("Could not traverse type".to_string()))?;
                let operation = self.operator.specific_operation(&ty, &[destination_register.to_string(), target_register.to_string()], stack, meta)?.inject_registers();
                target += &ASMBuilder::mov_x_ident_line(&destination_register, inline, Some(ty.byte_size()));
                target += &ASMBuilder::ident_line(&operation.0);
                destination_register = operation.1;
            }
            ASMResult::MultilineResulted(s, mut new_register) => {
                target += &s;
                let final_ty = self.traverse_type_resulted(&meta.static_type_information, &meta.code_line)?;
                let maybe_new_register = if final_ty.is_float() { new_register.to_float_register() } else { new_register.clone() };

                target += &ASMBuilder::mov_x_ident_line(&maybe_new_register, &new_register, Some(final_ty.byte_size()));
                new_register = maybe_new_register;

                let ty = rhs.traverse_type(meta).ok_or(ASMGenerateError::InternalError("Could not traverse type".to_string()))?;
                let operation = self.operator.specific_operation(&ty, &[new_register, target_register], stack, meta)?.inject_registers();
                target += &ASMBuilder::ident_line(&operation.0);
                destination_register = operation.1;
            }
            ASMResult::Multiline(_) => return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                actual: ASMResultVariance::Multiline,
                token: "Expression".to_string(),
            }))
        }
        stack.register_to_use.pop();

        Ok(ASMResult::MultilineResulted(target, destination_register))
    }

    fn expression_none_none(&self, stack: &mut Stack, meta: &mut MetaInfo, lhs: &Expression, rhs: &Expression) -> Result<ASMResult, ASMGenerateError> {
        let mut target = String::new();
        target += &ASMBuilder::ident(&ASMBuilder::comment_line(&format!("{}", self)));
        let (lhs_size, _) = lhs_rhs_byte_sizes(lhs, rhs, meta)?;
        let (mut register_iterator, float_type) = self.iterator_from_type(meta, lhs_size)?;

        let register_a = register_iterator.current();
        let register_b = register_iterator.nth(1).ok_or(ASMGenerateError::InternalError("No next register found".to_string()))?;

        stack.register_to_use.push(register_b.clone());
        let mut destination_register = stack.register_to_use.last()?;
        match lhs.to_asm(stack, meta, Some(PrepareRegisterOption {
            general_purpose_register: destination_register.clone(),
            assignable_token: lhs.value.clone().map(|value| value.as_ref().clone()),
        }))? {
            ASMResult::Inline(inline) => target += &ASMBuilder::mov_x_ident_line(&destination_register, inline, Some(lhs.byte_size(meta))),
            ASMResult::MultilineResulted(s, new_register) => {
                target += &s;
                destination_register = if new_register.is_float_register() { destination_register.to_float_register() } else { destination_register };
                target += &ASMBuilder::mov_x_ident_line(&destination_register, &new_register, Some(new_register.size() as usize));
            }
            ASMResult::Multiline(_) => return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                actual: ASMResultVariance::Multiline,
                token: "Expression".to_string(),
            }))
        }
        stack.register_to_use.pop();

        let pushing_register = self.latest_used_destination_register(meta, &target, lhs_size)?;

        if destination_register.is_float_register() {
            target += &ASMBuilder::mov_x_ident_line(pushing_register.to_64_bit_register(), &destination_register, Some(8));
        }

        target += &ASMBuilder::ident_line(&format!("push {}", pushing_register.to_64_bit_register()));
        target += &ASMBuilder::ident_line(&format!("xor {}, {}", pushing_register.to_64_bit_register(), pushing_register.to_64_bit_register()));

        stack.register_to_use.push(register_a.clone());
        let mut target_register = stack.register_to_use.last()?;
        match rhs.to_asm(stack, meta, Some(PrepareRegisterOption {
            general_purpose_register: target_register.clone(),
            assignable_token: rhs.value.clone().map(|value| value.as_ref().clone()),
        }))? {
            ASMResult::Inline(inline) => target += &ASMBuilder::mov_x_ident_line(&target_register, inline, Some(rhs.byte_size(meta))),
            ASMResult::MultilineResulted(s, r) => {
                target += &s;
                target_register = if r.is_float_register() { target_register.to_float_register() } else { target_register };
                target += &ASMBuilder::mov_x_ident_line(&target_register, &r, Some(r.size() as usize));
            }
            ASMResult::Multiline(_) => return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                actual: ASMResultVariance::Multiline,
                token: "Expression".to_string(),
            }))
        }
        stack.register_to_use.pop();

        if target_register.is_float_register() {
            target += &ASMBuilder::mov_x_ident_line(register_a.to_64_bit_register(), &target_register, Some(8));
        }

        target += &ASMBuilder::ident_line(&format!("push {}", register_a.to_64_bit_register()));
        target += &ASMBuilder::ident_line(&format!("xor {}, {}", register_a.to_64_bit_register(), register_a.to_64_bit_register()));

        Self::pop_to_register(&mut target, &float_type, &register_b)?;
        Self::pop_to_register(&mut target, &float_type, &register_a)?;

        if target_register.is_float_register() {
            target += &ASMBuilder::mov_x_ident_line(&target_register, &register_a, Some(register_a.size() as usize));
        }

        if destination_register.is_float_register() {
            target += &ASMBuilder::mov_x_ident_line(&destination_register, &register_b, Some(register_b.size() as usize));
        }

        let ty = &rhs.traverse_type(meta).ok_or(ASMGenerateError::InternalError("Could not traverse type".to_string()))?;
        let operation = self.operator.specific_operation(ty, &[&target_register, &destination_register], stack, meta)?.inject_registers();
        target += &ASMBuilder::ident_line(&operation.0);
        target_register = operation.1;

        Ok(ASMResult::MultilineResulted(target, target_register))
    }
}

impl Default for Expression {
    fn default() -> Self {
        Self {
            lhs: None,
            rhs: None,
            operator: Operator::Noop,
            value: None,
            index_operator: None,
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
        if let Some(index_operator) = &self.index_operator {
            debug_struct_formatter.field("index_operator", index_operator);
        }
        debug_struct_formatter.finish()
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let sign = if self.positive { "".to_string() } else { "-".to_string() };
        let prefix_arithmetic = self.prefix_arithmetic.iter().rev().map(|a| a.to_string()).collect::<String>();

        let index_operator = if let Some(index_operator) = &self.index_operator {
            format!("[{}]", index_operator)
        } else {
            "".to_string()
        };
        match (&self.lhs, &self.rhs) {
            (Some(lhs), Some(rhs)) => {
                write!(f, "{prefix_arithmetic}{sign}({lhs} {operator} {rhs}){index_operator}", operator = &self.operator)
            }
            _ => {
                if let Some(ass) = &self.value {
                    write!(f, "{}{}{}{}", prefix_arithmetic, sign, ass, index_operator)
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
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        let mut target = String::new();

        if let Some(value) = &self.value { // no lhs and rhs
            if stack.register_to_use.is_empty() {
                let assignable_type = self.traverse_type_resulted(&meta.static_type_information, &meta.code_line)?;
                let iterator = GeneralPurposeRegister::iter_from_byte_size(assignable_type.byte_size())?;
                stack.register_to_use.push(iterator.current());
            }

            if let Some(index_operator) = &self.index_operator {
                let index_asm_operation = index_operator.to_asm(stack, meta, options.clone())?;
                stack.indexing = Some(index_asm_operation.clone());
            }


            let s = if let Some(prefix_arithmetic) = &self.prefix_arithmetic {
                Self::prefix_arithmetic_to_asm(prefix_arithmetic, value, &stack.register_to_use.last()?, stack, meta)
            } else if matches!(value.as_ref(), AssignableToken::MethodCallToken(_)) {
                value.to_asm(stack, meta, Some(InExpressionMethodCall))
            } else {
                value.to_asm(stack, meta, options)
            };

            stack.indexing = None;

            if stack.register_to_use.len() == 1 {
                stack.register_to_use.pop();
            }

            return s;
        }

        target += &ASMBuilder::ident(&ASMBuilder::comment_line(&format!("{}", self)));

        match (&self.lhs, &self.rhs) {
            (Some(lhs), Some(rhs)) => {
                // Optimization. Use every register. (Some(Some, Some), Some(Some, Some))
                if let Some(t) = &self.expression_some_some_some_some(stack, meta, lhs, rhs)? {
                    if let Ok(Some(new_register)) = t.apply_with(&mut target)
                        .allow(ASMResultVariance::MultilineResulted)
                        .token("Expression")
                        .finish() {
                        return Ok(ASMResult::MultilineResulted(target, new_register.clone()));
                    }
                }

                match (&lhs.value, &rhs.value) {
                    (Some(_), Some(_)) => self.expression_some_some(stack, meta, lhs, rhs), // 2 + 3
                    (None, Some(_)) => self.expression_none_some(stack, meta, lhs, rhs), // (3 + 2) + 5
                    (Some(_), None) => self.expression_some_none(stack, meta, lhs, rhs), // 5 + (3 + 2)
                    (None, None) => self.expression_none_none(stack, meta, lhs, rhs), // ((1 + 2) + (3 + 4)) + ((5 + 6) + (7 + 8)) // any depth
                }
            }
            (_, _) => Err(ASMGenerateError::NotImplemented { token: "Something went wrong. Neither rhs nor lhs are valid".to_string() })
        }
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

impl Expression {
    pub fn prefix_arithmetic_to_asm(prefix_arithmetic: &PrefixArithmetic, value: &AssignableToken, target_register: &GeneralPurposeRegister, stack: &mut Stack, meta: &mut MetaInfo) -> Result<ASMResult, ASMGenerateError> {
        let mut target = String::new();
        let register_to_use = stack.register_to_use.last()?;
        let register_64 = register_to_use.to_64_bit_register();
        let mut child_has_pointer_arithmetic = false;
        let mut register_or_stack_address = String::new();

        if let Some(prefix_arithmetic) = value.prefix_arithmetic() {
            if let AssignableToken::ArithmeticEquation(a) = value {
                if let Some(child) = &a.value {
                    Self::prefix_arithmetic_to_asm(&prefix_arithmetic, child, target_register, stack, meta)?
                        .apply_with(&mut target)
                        .allow(ASMResultVariance::MultilineResulted)
                        .allow(ASMResultVariance::MultilineResulted)
                        .token("Expression")
                        .finish()?;


                    register_or_stack_address = match prefix_arithmetic {
                        PrefixArithmetic::PointerArithmetic(_) => {
                            child_has_pointer_arithmetic = true;
                            format!("QWORD [{}]", register_64)
                        }
                        PrefixArithmetic::Cast(_) => {
                            GeneralPurposeRegister::Float(FloatRegister::Xmm7).to_string()
                        }
                        _ => register_64.to_string(),
                    };
                }
            }
        } else {
            match value.to_asm::<InterimResultOption>(stack, meta, None)? {
                ASMResult::Inline(t) => register_or_stack_address = t,
                ASMResult::MultilineResulted(s, g) => {
                    target += &s;
                    register_or_stack_address = g.to_string();
                }
                ASMResult::Multiline(s) => {
                    target += &s;
                }
            }
        }

        prefix_arithmetic.to_asm::<PrefixArithmeticOptions>(stack, meta, Some(PrefixArithmeticOptions {
            value: value.clone(),
            register_or_stack_address,
            register_64,
            target_register: target_register.clone(),
            child_has_pointer_arithmetic,
            target,
        }))
    }

    pub fn traverse_type(&self, meta: &MetaInfo) -> Option<TypeToken> {
        self.traverse_type_resulted(&meta.static_type_information, &meta.code_line).ok()
    }

    pub fn traverse_type_resulted(&self, context: &StaticTypeContext, code_line: &CodeLine) -> Result<TypeToken, InferTypeError> {
        if let Some(value) = &self.value {
            let mut value_type = value.infer_type_with_context(context, code_line);
            let has_prefix_arithmetics = self.prefix_arithmetic.is_some();
            let has_index_operation = self.index_operator.is_some();

            if has_index_operation {
                if let Some(index_operator) = &self.index_operator {
                    let index_type = index_operator.infer_type_with_context(context, code_line)?;
                    if !matches!(index_type, TypeToken::Integer(_, _)) {
                        return Err(InferTypeError::IllegalIndexOperation(index_type, code_line.clone()));
                    }
                }
                let value_type_cloned = value_type?.clone();
                if let Some(element_type) = value_type_cloned.pop_array() {
                    value_type = Ok(element_type);
                } else {
                    return Err(InferTypeError::IllegalArrayTypeLookup(value_type_cloned, code_line.clone()));
                }
            }

            return if let (true, Ok(value_type)) = (has_prefix_arithmetics, &value_type) {
                let current_pointer_arithmetic: String = match value_type {
                    TypeToken::Custom(name, _) if name.name.starts_with(['*', '&']) => {
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
                            } else {
                                return Err(InferTypeError::IllegalDereference(*value.clone(), value_type, code_line.clone()));
                            }
                        }
                        PrefixArithmetic::PointerArithmetic(PointerArithmetic::Ampersand) => {
                            value_type = value_type.push_pointer();
                        }
                        PrefixArithmetic::PointerArithmetic(PointerArithmetic::Asterics) => {
                            // just using & in front of non pointer types is illegal. Dereferencing non pointers doesnt make any sense
                            return Err(InferTypeError::IllegalDereference(*value.clone(), value_type, code_line.clone()));
                        }
                        PrefixArithmetic::Cast(casting_to) => {
                            value_type = TypeToken::from_str(&casting_to.to_string(), Mutability::Immutable)?;
                        }
                        PrefixArithmetic::Operation(_) => {}
                    }
                }

                if value_type.is_pointer() {
                    Ok(TypeToken::Custom(NameToken { name: format!("{}", value_type) }, Mutability::from(value_type.mutable())))
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
                base_type_matrix.insert((TypeToken::Custom(NameToken { name: "string".to_string() }, Mutability::Immutable), Operator::Add, TypeToken::Custom(NameToken { name: "string".to_string() }, Mutability::Immutable)), TypeToken::Custom(NameToken { name: "*string".to_string() }, Mutability::Immutable));

                Integer::operation_matrix(&mut base_type_matrix);
                Float::operation_matrix(&mut base_type_matrix);
                Boolean::operation_matrix(&mut base_type_matrix);

                // I do not care, if the expression is mutable or not. The type is the relevant factor
                let mut lhs_clone = lhs_type.clone();
                lhs_clone.set_mutability(Mutability::Immutable);
                let mut rhs_clone = rhs_type.clone();
                rhs_clone.set_mutability(Mutability::Immutable);

                if let Some(result_type) = base_type_matrix.get(&(lhs_clone, operator.clone(), rhs_clone)) {
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

        if self.value.is_some() {
            self.positive = !self.positive;
        }
    }
}
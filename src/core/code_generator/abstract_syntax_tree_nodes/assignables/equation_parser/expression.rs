use std::str::FromStr;

use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::in_expression_method_call::InExpressionMethodCall;
use crate::core::code_generator::asm_options::prepare_register::PrepareRegisterOption;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::{ASMResult, ASMResultError, ASMResultVariance};
use crate::core::code_generator::generator::{LastUnchecked, Stack};
use crate::core::code_generator::registers::{ByteSize, FloatRegister, GeneralPurposeRegister, GeneralPurposeRegisterIterator};
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::prefix_arithmetic::PrefixArithmetic;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::types::float::FloatType;
use crate::core::model::types::ty::Type;
use crate::core::parser::abstract_syntax_tree_nodes::assignables::equation_parser::prefix_arithmetic::PrefixArithmeticOptions;
use crate::core::parser::types::r#type::InferTypeError;

impl Expression {
    fn iterator_from_type(&self, meta: &mut MetaInfo, lhs_size: usize) -> Result<(GeneralPurposeRegisterIterator, Option<FloatType>), ASMGenerateError> {
        if let Some(lhs) = &self.lhs {
            let ty = &lhs.get_type(&meta.static_type_information).ok_or(Box::new(InferTypeError::NoTypePresent(
                LValue::Identifier(Identifier { name: "Expression".to_string() }), meta.file_position.clone()
            )))?;

            return Ok(if let Type::Float(f, _) = ty {
                (GeneralPurposeRegister::iter_from_byte_size(lhs_size)?, Some(f.clone()))
            } else {
                (GeneralPurposeRegister::iter_from_byte_size(lhs_size)?, None)
            });
        }

        Err(ASMGenerateError::InternalError("Internal error".to_string(), meta.file_position.clone()))
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

    fn pop_to_register(target: &mut String, float_type: &Option<FloatType>, register_target: &GeneralPurposeRegister) -> Result<(), ASMGenerateError> {
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

                let register_b = register_iterator.next().ok_or(ASMGenerateError::InternalError("No next register found".to_string(), meta.file_position.clone()))?;
                let register_c = register_iterator.next().ok_or(ASMGenerateError::InternalError("No next register found".to_string(), meta.file_position.clone()))?;

                stack.register_to_use.push(register_b.clone());
                let mut destination_register = stack.register_to_use.last(&meta.file_position)?;

                match lhs.to_asm(stack, meta, Some(ASMOptions::PrepareRegisterOption(PrepareRegisterOption {
                    general_purpose_register: destination_register.clone(),
                    assignable: lhs.value.clone().map(|value| value.as_ref().clone()),
                })))? {
                    ASMResult::Inline(inline) => target += &ASMBuilder::mov_x_ident_line(&destination_register, inline, Some(lhs.byte_size(meta))),
                    ASMResult::MultilineResulted(s, r) => {
                        target += &s;
                        destination_register = if r.is_float_register() { destination_register.to_float_register() } else { destination_register };
                        target += &ASMBuilder::mov_x_ident_line(&destination_register, &r, Some(r.size() as usize));
                    }
                    ASMResult::Multiline(_) => return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                        expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                        actual: ASMResultVariance::Multiline,
                        ast_node: "Expression".to_string(),
                    }))
                }
                stack.register_to_use.pop();

                stack.register_to_use.push(register_c.clone());
                let mut target_register = stack.register_to_use.last(&meta.file_position)?;
                match rhs.to_asm(stack, meta, Some(ASMOptions::PrepareRegisterOption(PrepareRegisterOption {
                    general_purpose_register: target_register.clone(),
                    assignable: rhs.value.clone().map(|value| value.as_ref().clone()),
                })))? {
                    ASMResult::Inline(inline) => target += &ASMBuilder::mov_x_ident_line(&target_register, inline, Some(rhs.byte_size(meta))),
                    ASMResult::MultilineResulted(s, r) => {
                        target += &s;
                        target_register = if r.is_float_register() { target_register.to_float_register() } else { target_register };
                        target += &ASMBuilder::mov_x_ident_line(&target_register, &r, Some(r.size() as usize));
                    }
                    ASMResult::Multiline(_) => return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                        expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                        actual: ASMResultVariance::Multiline,
                        ast_node: "Expression".to_string(),
                    }))
                }
                stack.register_to_use.pop();

                let ty = &rhs.get_type(&meta.static_type_information).ok_or(Box::new(InferTypeError::NoTypePresent(
                    LValue::Identifier(Identifier { name: "Expression".to_string() }), meta.file_position.clone()
                )))?;
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
        let mut destination_register = stack.register_to_use.last(&meta.file_position)?;

        match lhs.to_asm(stack, meta, Some(ASMOptions::PrepareRegisterOption(PrepareRegisterOption {
            general_purpose_register: destination_register.clone(),
            assignable: lhs.value.clone().map(|value| value.as_ref().clone()),
        })))? {
            ASMResult::Inline(inline) => target += &ASMBuilder::mov_x_ident_line(&destination_register, inline, Some(rhs.byte_size(meta))),
            ASMResult::MultilineResulted(s, new_register) => {
                target += &s;
                let final_ty = self.get_type(&meta.static_type_information).ok_or(Box::new(InferTypeError::NoTypePresent(
                    LValue::Identifier(Identifier { name: "Expression".to_string() }), meta.file_position.clone()
                )))?;
                let maybe_new_register = if final_ty.is_float() { new_register.to_float_register() } else { new_register.clone() };

                target += &ASMBuilder::mov_x_ident_line(&maybe_new_register, new_register, Some(final_ty.byte_size()));
                destination_register = maybe_new_register;
            }
            ASMResult::Multiline(_) => return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                actual: ASMResultVariance::Multiline,
                ast_node: "Expression".to_string(),
            }))
        }

        let next_register = register_iterator.nth(2).ok_or(ASMGenerateError::InternalError("No next register found".to_string(), meta.file_position.clone()))?;

        stack.register_to_use.push(next_register);
        let target_register = stack.register_to_use.last(&meta.file_position)?;

        match rhs.to_asm(stack, meta, Some(ASMOptions::PrepareRegisterOption(PrepareRegisterOption {
            general_purpose_register: target_register.clone(),
            assignable: rhs.value.clone().map(|value| value.as_ref().clone()),
        })))? {
            ASMResult::Inline(inline) => {
                let ty = rhs.get_type(&meta.static_type_information).ok_or(Box::new(InferTypeError::NoTypePresent(
                    LValue::Identifier(Identifier { name: "Expression".to_string() }), meta.file_position.clone()
                )))?;
                let operation = self.operator.specific_operation(&ty, &[destination_register.to_string(), inline.to_string()], stack, meta)?.inject_registers();
                target += &ASMBuilder::ident_line(&operation.0);
                destination_register = operation.1;
            }
            ASMResult::MultilineResulted(s, mut new_register) => {
                target += &s;
                let final_ty = self.get_type(&meta.static_type_information).ok_or(Box::new(InferTypeError::NoTypePresent(
                    LValue::Identifier(Identifier { name: "Expression".to_string() }), meta.file_position.clone()
                )))?;
                let maybe_new_register = if final_ty.is_float() { new_register.to_float_register() } else { new_register.clone() };


                target += &ASMBuilder::mov_x_ident_line(&maybe_new_register, &new_register, Some(final_ty.byte_size()));
                new_register = maybe_new_register;


                let ty = rhs.get_type(&meta.static_type_information).ok_or(Box::new(InferTypeError::NoTypePresent(
                    LValue::Identifier(Identifier { name: "Expression".to_string() }), meta.file_position.clone()
                )))?;
                let operation = self.operator.specific_operation(&ty, &[destination_register.to_string(), new_register.to_string()], stack, meta)?.inject_registers();
                target += &ASMBuilder::ident_line(&operation.0);
                destination_register = operation.1;
            }
            ASMResult::Multiline(_) => return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                actual: ASMResultVariance::Multiline,
                ast_node: "Expression".to_string(),
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
        let register_b = register_iterator.nth(2).ok_or(ASMGenerateError::InternalError("No next register found".to_string(), meta.file_position.clone()))?;

        if stack.register_to_use.is_empty() {
            stack.register_to_use.push(register_a.clone());
        }


        stack.register_to_use.push(register_a);
        let mut destination_register = stack.register_to_use.last(&meta.file_position)?;
        match lhs.to_asm(stack, meta, Some(ASMOptions::PrepareRegisterOption(PrepareRegisterOption {
            general_purpose_register: destination_register.clone(),
            assignable: lhs.value.clone().map(|value| value.as_ref().clone()),
        })))? {
            ASMResult::Inline(inline) => target += &ASMBuilder::mov_x_ident_line(&destination_register, inline, Some(rhs.byte_size(meta))),
            ASMResult::MultilineResulted(s, new_register) => {
                target += &s;
                let final_ty = self.get_type(&meta.static_type_information).ok_or(Box::new(InferTypeError::NoTypePresent(
                    LValue::Identifier(Identifier { name: "Expression".to_string() }), meta.file_position.clone()
                )))?;
                let maybe_new_register = if final_ty.is_float() { new_register.to_float_register() } else { new_register.clone() };

                target += &ASMBuilder::mov_x_ident_line(&maybe_new_register, new_register, Some(final_ty.byte_size()));
                destination_register = maybe_new_register;
            }
            ASMResult::Multiline(_) => return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                actual: ASMResultVariance::Multiline,
                ast_node: "Expression".to_string(),
            }))
        }

        stack.register_to_use.pop();

        stack.register_to_use.push(register_b);
        let target_register = stack.register_to_use.last(&meta.file_position)?;
        match rhs.to_asm(stack, meta, Some(ASMOptions::PrepareRegisterOption(PrepareRegisterOption {
            general_purpose_register: target_register.clone(),
            assignable: rhs.value.clone().map(|value| value.as_ref().clone()),
        })))? {
            ASMResult::Inline(inline) => {
                let ty = rhs.get_type(&meta.static_type_information).ok_or(ASMGenerateError::InternalError("Could not traverse type".to_string(), meta.file_position.clone()))?;
                let operation = self.operator.specific_operation(&ty, &[destination_register.to_string(), inline.to_string()], stack, meta)?.inject_registers();
                target += &ASMBuilder::ident_line(&operation.0);
                destination_register = operation.1;
            }
            ASMResult::MultilineResulted(s, mut new_register) => {
                target += &s;
                let final_ty = self.get_type(&meta.static_type_information).ok_or(Box::new(InferTypeError::NoTypePresent(
                    LValue::Identifier(Identifier { name: "Expression".to_string() }), meta.file_position.clone()
                )))?;
                let maybe_new_register = if final_ty.is_float() { new_register.to_float_register() } else { new_register.clone() };

                target += &ASMBuilder::mov_x_ident_line(&maybe_new_register, &new_register, Some(final_ty.byte_size()));
                new_register = maybe_new_register;


                let ty = rhs.get_type(&meta.static_type_information).ok_or(ASMGenerateError::InternalError("Could not traverse type".to_string(), meta.file_position.clone()))?;
                let operation = self.operator.specific_operation(&ty, &[destination_register.to_string(), new_register.to_string()], stack, meta)?.inject_registers();
                target += &ASMBuilder::ident_line(&operation.0);
                destination_register = operation.1;
            }
            ASMResult::Multiline(_) => return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                actual: ASMResultVariance::Multiline,
                ast_node: "Expression".to_string(),
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
        let register_b = register_iterator.nth(2).ok_or(ASMGenerateError::InternalError("No next register found".to_string(), meta.file_position.clone()))?;

        if stack.register_to_use.is_empty() {
            stack.register_to_use.push(register_a.clone());
        }


        stack.register_to_use.push(register_b);
        let mut target_register = stack.register_to_use.last(&meta.file_position)?;
        match rhs.to_asm(stack, meta, Some(ASMOptions::PrepareRegisterOption(PrepareRegisterOption {
            general_purpose_register: target_register.clone(),
            assignable: rhs.value.clone().map(|value| value.as_ref().clone()),
        })))? {
            ASMResult::Inline(inline) => target += &ASMBuilder::mov_x_ident_line(&target_register, inline, Some(rhs.byte_size(meta))),
            ASMResult::MultilineResulted(s, new_register) => {
                target += &s;

                target_register = if new_register.is_float_register() { target_register.to_float_register() } else { target_register };
                target += &ASMBuilder::mov_x_ident_line(&target_register, &new_register, Some(new_register.size() as usize));
            }
            ASMResult::Multiline(_) => return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                actual: ASMResultVariance::Multiline,
                ast_node: "Expression".to_string(),
            }))
        }

        stack.register_to_use.pop();

        stack.register_to_use.push(register_a);
        let mut destination_register = stack.register_to_use.last(&meta.file_position)?;
        match lhs.to_asm(stack, meta, Some(ASMOptions::PrepareRegisterOption(PrepareRegisterOption {
            general_purpose_register: destination_register.clone(),
            assignable: lhs.value.clone().map(|value| value.as_ref().clone()),
        })))? {
            ASMResult::Inline(inline) => {
                let ty = rhs.get_type(&meta.static_type_information).ok_or(ASMGenerateError::InternalError("Could not traverse type".to_string(), meta.file_position.clone()))?;
                let operation = self.operator.specific_operation(&ty, &[destination_register.to_string(), target_register.to_string()], stack, meta)?.inject_registers();
                target += &ASMBuilder::mov_x_ident_line(&destination_register, inline, Some(ty.byte_size()));
                target += &ASMBuilder::ident_line(&operation.0);
                destination_register = operation.1;
            }
            ASMResult::MultilineResulted(s, mut new_register) => {
                target += &s;
                let final_ty = self.get_type(&meta.static_type_information).ok_or(Box::new(InferTypeError::NoTypePresent(
                    LValue::Identifier(Identifier { name: "Expression".to_string() }), meta.file_position.clone()
                )))?;
                let maybe_new_register = if final_ty.is_float() { new_register.to_float_register() } else { new_register.clone() };

                target += &ASMBuilder::mov_x_ident_line(&maybe_new_register, &new_register, Some(final_ty.byte_size()));
                new_register = maybe_new_register;

                let ty = rhs.get_type(&meta.static_type_information).ok_or(ASMGenerateError::InternalError("Could not traverse type".to_string(), meta.file_position.clone()))?;
                let operation = self.operator.specific_operation(&ty, &[new_register, target_register], stack, meta)?.inject_registers();
                target += &ASMBuilder::ident_line(&operation.0);
                destination_register = operation.1;
            }
            ASMResult::Multiline(_) => return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                actual: ASMResultVariance::Multiline,
                ast_node: "Expression".to_string(),
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
        let register_b = register_iterator.nth(1).ok_or(ASMGenerateError::InternalError("No next register found".to_string(), meta.file_position.clone()))?;

        stack.register_to_use.push(register_b.clone());
        let mut destination_register = stack.register_to_use.last(&meta.file_position)?;
        match lhs.to_asm(stack, meta, Some(ASMOptions::PrepareRegisterOption(PrepareRegisterOption {
            general_purpose_register: destination_register.clone(),
            assignable: lhs.value.clone().map(|value| value.as_ref().clone()),
        })))? {
            ASMResult::Inline(inline) => target += &ASMBuilder::mov_x_ident_line(&destination_register, inline, Some(lhs.byte_size(meta))),
            ASMResult::MultilineResulted(s, new_register) => {
                target += &s;
                destination_register = if new_register.is_float_register() { destination_register.to_float_register() } else { destination_register };
                target += &ASMBuilder::mov_x_ident_line(&destination_register, &new_register, Some(new_register.size() as usize));
            }
            ASMResult::Multiline(_) => return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                actual: ASMResultVariance::Multiline,
                ast_node: "Expression".to_string(),
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
        let mut target_register = stack.register_to_use.last(&meta.file_position)?;
        match rhs.to_asm(stack, meta, Some(ASMOptions::PrepareRegisterOption(PrepareRegisterOption {
            general_purpose_register: target_register.clone(),
            assignable: rhs.value.clone().map(|value| value.as_ref().clone()),
        })))? {
            ASMResult::Inline(inline) => target += &ASMBuilder::mov_x_ident_line(&target_register, inline, Some(rhs.byte_size(meta))),
            ASMResult::MultilineResulted(s, r) => {
                target += &s;
                target_register = if r.is_float_register() { target_register.to_float_register() } else { target_register };
                target += &ASMBuilder::mov_x_ident_line(&target_register, &r, Some(r.size() as usize));
            }
            ASMResult::Multiline(_) => return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                actual: ASMResultVariance::Multiline,
                ast_node: "Expression".to_string(),
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

        let ty = &rhs.get_type(&meta.static_type_information).ok_or(ASMGenerateError::InternalError("Could not traverse type".to_string(), meta.file_position.clone()))?;
        let operation = self.operator.specific_operation(ty, &[&target_register, &destination_register], stack, meta)?.inject_registers();
        target += &ASMBuilder::ident_line(&operation.0);
        target_register = operation.1;

        Ok(ASMResult::MultilineResulted(target, target_register))
    }
}


impl ToASM for Expression {
    fn to_asm(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<ASMOptions>) -> Result<ASMResult, ASMGenerateError> {
        let mut target = String::new();

        if let Some(value) = &self.value { // no lhs and rhs
            if stack.register_to_use.is_empty() {
                let assignable_type = self.get_type(&meta.static_type_information).ok_or(Box::new(InferTypeError::NoTypePresent(
                    LValue::Identifier(Identifier { name: value.identifier().unwrap_or("Expression".to_string() )}), meta.file_position.clone()
                )))?;
                let iterator = GeneralPurposeRegister::iter_from_byte_size(assignable_type.byte_size())?;
                stack.register_to_use.push(iterator.current());
            }

            if let Some(index_operator) = &self.index_operator {
                let index_asm_operation = index_operator.to_asm(stack, meta, options.clone())?;
                stack.indexing = Some(index_asm_operation.clone());
            }


            let s = if let Some(prefix_arithmetic) = &self.prefix_arithmetic {
                Self::prefix_arithmetic_to_asm(prefix_arithmetic, value, &stack.register_to_use.last(&meta.file_position)?, stack, meta, options)
            } else if matches!(value.as_ref(), Assignable::MethodCall(_)) {
                value.to_asm(stack, meta, Some(ASMOptions::InExpressionMethodCall(InExpressionMethodCall)))
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
                        .ast_node("Expression")
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
            (_, _) => Err(ASMGenerateError::NotImplemented { ast_node: "Something went wrong. Neither rhs nor lhs are valid".to_string() })
        }
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        true
    }

    fn byte_size(&self, meta: &MetaInfo) -> usize {
        if let Some(ty) = self.get_type(&meta.static_type_information) {
            return ty.byte_size();
        }

        0
    }
}


fn lhs_rhs_byte_sizes(a: &Expression, b: &Expression, meta: &mut MetaInfo) -> Result<(usize, usize), ASMGenerateError> {
    let lhs_size = a.byte_size(meta);
    let rhs_size = b.byte_size(meta);

    if lhs_size != rhs_size {
        return Err(ASMGenerateError::NotImplemented { ast_node: format!("Expected both types to be the same byte size. lhs: {}, rhs: {}", lhs_size, rhs_size) });
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
    pub fn prefix_arithmetic_to_asm(prefix_arithmetic: &PrefixArithmetic, value: &Assignable, target_register: &GeneralPurposeRegister, stack: &mut Stack, meta: &mut MetaInfo, options: Option<ASMOptions>) -> Result<ASMResult, ASMGenerateError> {
        let mut target = String::new();
        let register_to_use = stack.register_to_use.last(&meta.file_position)?;
        let register_64 = register_to_use.to_64_bit_register();
        let mut child_has_pointer_arithmetic = false;
        let mut register_or_stack_address = String::new();

        if let Some(prefix_arithmetic) = value.prefix_arithmetic() {
            if let Assignable::Expression(a) = value {
                if let Some(child) = &a.value {
                    Self::prefix_arithmetic_to_asm(&prefix_arithmetic, child, target_register, stack, meta, options.clone())?
                        .apply_with(&mut target)
                        .allow(ASMResultVariance::MultilineResulted)
                        .allow(ASMResultVariance::MultilineResulted)
                        .ast_node("Expression")
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
            match value.to_asm(stack, meta, None)? {
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

        prefix_arithmetic.to_asm(stack, meta, Some(ASMOptions::PrefixArithmeticOptions(PrefixArithmeticOptions {
            value: value.clone(),
            register_or_stack_address,
            register_64,
            target_register: target_register.clone(),
            child_has_pointer_arithmetic,
            is_lvalue: matches!(options, Some(ASMOptions::LValueExpressionOption)),
            target,
        })))
    }
}
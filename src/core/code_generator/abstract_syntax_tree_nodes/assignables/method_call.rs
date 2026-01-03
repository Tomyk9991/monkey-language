use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::{ASMResult, ASMResultError, ASMResultVariance};
use crate::core::code_generator::conventions::CallingRegister;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::register_destination::byte_size_from_word;
use crate::core::code_generator::registers::{Bit64, ByteSize, GeneralPurposeRegister};
use crate::core::code_generator::ToASM;
use crate::core::code_generator::{conventions, ASMGenerateError, MetaInfo};
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::types::ty::Type;
use crate::core::parser::types::r#type::InferTypeError;
use std::fmt::Debug;

impl ToASM for MethodCall {
    fn to_asm(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<ASMOptions>) -> Result<ASMResult, ASMGenerateError> {
        let mut calling_convention = conventions::calling_convention(stack, meta, &self.arguments, &self.identifier.identifier())?;
        calling_convention.reverse();

        let method_defs = conventions::method_definitions(&meta.static_type_information, &self.arguments, &self.identifier.identifier())?;

        if method_defs.is_empty() {
            return Err(ASMGenerateError::TypeNotInferrable(Box::new(InferTypeError::UnresolvedReference(self.identifier.to_string(), meta.file_position.clone()))));
        }

        if method_defs.len() > 1 {
            return Err(ASMGenerateError::TypeNotInferrable(Box::new(InferTypeError::MethodCallSignatureMismatch {
                signatures: meta.static_type_information.methods
                    .iter().filter(|m| m.identifier.identifier() == self.identifier.identifier())
                    .map(|m| m.arguments.iter().map(|a| a.ty.clone()).collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
                method_name: self.identifier.clone(),
                file_position: meta.file_position.clone(),
                provided: self.arguments.iter().filter_map(|a| a.get_type(&meta.static_type_information)).collect::<Vec<_>>(),
            })));
        }

        let method_def = &method_defs[0];
        let resulting_register = GeneralPurposeRegister::Bit64(Bit64::Rax);

        // represents the register where the final result must lay in, and where it is expected, after call
        let register_to_move_result = stack.register_to_use.last().unwrap_or(&GeneralPurposeRegister::Bit64(Bit64::Rax)).clone();
        let register_to_move_result_64bit = register_to_move_result.to_64_bit_register();
        let mut target = String::new();
        let mut registers_push_ignore = vec![];


        let is_direct_method_call = !matches!(options, Some(ASMOptions::InExpressionMethodCall(_)));
        
        if method_def.return_type != Type::Void && register_to_move_result_64bit == resulting_register {
            registers_push_ignore.push(&resulting_register);
        }

        if !register_to_move_result.is_float_register() && (register_to_move_result_64bit != resulting_register) {
            registers_push_ignore.push(&register_to_move_result_64bit);
        }

        if !is_direct_method_call {
            target += &ASMBuilder::push_registers(&registers_push_ignore);
        }

        #[derive(Debug)]
        enum RegisterResult {
            Assign(String),
            Stack,
        }

        let zipped = calling_convention.iter().zip(self.arguments.iter().rev().collect::<Vec<_>>());
        let mut parameters = vec![];

        for (conventions, argument) in zipped {
            let provided_type = argument.get_type(&meta.static_type_information).ok_or(Box::new(InferTypeError::NoTypePresent(
                LValue::Identifier(Identifier { name: self.identifier.identifier() },), self.file_position.clone()
            )))?;
            let result_from_eval = GeneralPurposeRegister::Bit64(Bit64::Rax)
                .to_size_register(&ByteSize::try_from(provided_type.byte_size())?);

            let mut inline = false;
            let mut assign = String::new();

            match argument.to_asm(stack, meta, Some(ASMOptions::InterimResultOption(InterimResultOption::from(&result_from_eval))))? {
                ASMResult::Inline(source) => {
                    inline = true;
                    assign = source;
                }
                ASMResult::MultilineResulted(source, r) => {
                    target += &source;

                    if let Assignable::Float(_) = argument {
                        inline = true;
                        assign = r.to_string();
                    } else {
                        if r.is_float_register() {
                            target += &ASMBuilder::mov_x_ident_line(r.to_64_bit_register(), &r, Some(r.size() as usize));
                        }

                        if let GeneralPurposeRegister::Memory(stack_position) = &r {
                            let inline_stack_word_size = byte_size_from_word(stack_position.split(" ")
                                .next()
                                .ok_or(ASMGenerateError::InternalError(format!("Could not parse {stack_position} as a byte size"), self.file_position.clone()))?);

                            let destination_register = stack
                                .register_to_use
                                .last()
                                .unwrap_or(&GeneralPurposeRegister::Bit64(Bit64::Rax)).to_size_register(&ByteSize::try_from(inline_stack_word_size)?);

                            target += &ASMBuilder::mov_ident_line(&destination_register, &r);
                            target += &ASMBuilder::ident_line(&format!("push {}", destination_register.to_64_bit_register()));
                        } else {
                            target += &ASMBuilder::ident_line(&format!("push {}", r.to_64_bit_register()));
                        }
                    }
                }
                ASMResult::Multiline(_) => return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                    expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                    actual: ASMResultVariance::Multiline,
                    ast_node: "Method call".to_string(),
                }))
            }


            let mut variadic_parameters = vec![];
            for convention in conventions {
                match convention {
                    CallingRegister::Register(register_convention) => {
                        let register_convention_sized = register_convention.to_size_register_ignore_float(&ByteSize::try_from(provided_type.byte_size())?);
                        variadic_parameters.push((register_convention_sized, if inline { RegisterResult::Assign(assign.clone()) } else { RegisterResult::Stack }, Some(provided_type.byte_size())));
                    }
                    CallingRegister::Stack => {}
                }
            }

            parameters.push(variadic_parameters);
        }


        // due to variadic function calls and windows calling conventions
        // float parameters need to have the value in the general purpose register AND in the xmm register accordingly
        // since multiple pops result in unexpected or even crashing behavior. just one pop is needed
        let mut popped_into = GeneralPurposeRegister::Bit64(Bit64::Rax);
        for all_conventions in parameters.iter().rev() {
            for (index, (register_convention_sized, assign, size)) in all_conventions.iter().enumerate() {
                if index == 0 {
                    match assign {
                        RegisterResult::Assign(assign) => {
                            target += &ASMBuilder::mov_x_ident_line(register_convention_sized, assign, *size)
                        }
                        RegisterResult::Stack => {
                            target += &ASMBuilder::ident_line(&format!("pop {}", register_convention_sized.to_64_bit_register()));

                            if let GeneralPurposeRegister::Float(float_register) = register_convention_sized {
                                target += &ASMBuilder::mov_x_ident_line(float_register, register_convention_sized.to_64_bit_register(), Some(register_convention_sized.size() as usize));
                                popped_into = GeneralPurposeRegister::Float(float_register.clone());
                            } else {
                                popped_into = register_convention_sized.to_64_bit_register();
                            }
                        }
                    }
                } else {
                    match assign {
                        RegisterResult::Assign(assign) => {
                            target += &ASMBuilder::mov_x_ident_line(register_convention_sized, assign, *size);
                        }
                        RegisterResult::Stack => {
                            target += &ASMBuilder::mov_x_ident_line(register_convention_sized, popped_into.to_size_register_ignore_float(&ByteSize::try_from(size.unwrap_or(8))?), *size);
                        }
                    }
                }
            }
        }

        target += &ASMBuilder::ident(&ASMBuilder::comment_line(&self.to_string()));
        target += &ASMBuilder::ident_line(&format!("call {}", if method_def.is_extern { method_def.identifier.identifier() } else { method_def.method_label_name() }));

        if method_def.return_type != Type::Void {
            target += &ASMBuilder::mov_x_ident_line(
                &register_to_move_result,
                GeneralPurposeRegister::Bit64(Bit64::Rax).to_size_register(&ByteSize::try_from(method_def.return_type.byte_size())?),
                Some(method_def.return_type.byte_size()),
            );
        }

        if !is_direct_method_call {
            target += &ASMBuilder::pop_registers(&registers_push_ignore);
        }


        if method_def.return_type != Type::Void {
            Ok(ASMResult::MultilineResulted(target, register_to_move_result.to_size_register(&ByteSize::try_from(method_def.return_type.byte_size())?)))
        } else {
            Ok(ASMResult::Multiline(target))
        }
    }


    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        true
    }

    fn byte_size(&self, meta: &MetaInfo) -> usize {
        if let Some(method_def) = meta.static_type_information.methods.iter().find(|m| m.identifier == self.identifier) {
            method_def.return_type.byte_size()
        } else {
            0
        }
    }

    fn data_section(&self, stack: &mut Stack, meta: &mut MetaInfo) -> bool {
        let mut has_before_label_asm = false;
        let count_before = stack.label_count;

        for argument in self.arguments.iter().rev() {
            if argument.data_section(stack, meta) {
                has_before_label_asm = true;
                stack.label_count -= 1;
            }
        }

        stack.label_count = count_before;
        has_before_label_asm
    }
}
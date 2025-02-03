use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use crate::core::code_generator::registers::{Bit8, ByteSize, NibbleRegister};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::asm_result::{ASMResult};
use crate::core::code_generator::register_destination::word_from_byte_size;
use crate::core::code_generator::registers::{Bit32, GeneralPurposeRegister};
use crate::core::lexer::abstract_syntax_tree_nodes::assignables::equation_parser::operator::{AssemblerOperation, Operator, OperatorToASM};
use crate::core::lexer::types::cast_to::{Castable, CastTo};
use crate::core::lexer::types::integer::Integer;
use crate::core::lexer::types::r#type::{Mutability, Type};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Boolean {
    True,
    False,
}

impl OperatorToASM for Boolean {
    fn operation_to_asm<T: Display>(&self, operator: &Operator, registers: &[T], stack: &mut Stack, meta: &mut MetaInfo) -> Result<AssemblerOperation, ASMGenerateError> {
        fn no_operation(operation: &str) -> Result<AssemblerOperation, ASMGenerateError> {
            Err(ASMGenerateError::InternalError(format!("No operation `{}` on booleans", operation)))
        }

        match operator {
            Operator::Noop => no_operation("noop"),
            Operator::Add => no_operation("add"),
            Operator::Sub => no_operation("sub"),
            Operator::Div => no_operation("div"),
            Operator::Mul => no_operation("mul"),
            Operator::LeftShift => no_operation("left shift"),
            Operator::RightShift => no_operation("right shift"),
            Operator::LessThan => no_operation("less than"),
            Operator::GreaterThan => no_operation("greater than"),
            Operator::LessThanEqual => no_operation("less than equal"),
            Operator::GreaterThanEqual => no_operation("greater than equal"),
            Operator::BitwiseXor => no_operation("bitwise xor"),
            Operator::Mod => no_operation("modulo"),
            Operator::Equal | Operator::NotEqual => Ok(AssemblerOperation {
                prefix: None,
                operation: AssemblerOperation::compare(&operator.to_asm::<InterimResultOption>(&mut Default::default(), &mut Default::default(), None)?.to_string(), &registers[0], &registers[1])?,
                postfix: None,
                result_expected: GeneralPurposeRegister::from_str(&registers[0].to_string()).map_err(|_| ASMGenerateError::InternalError(format!("Cannot build {} from register", &registers[0])))?,
            }),
            Operator::BitwiseAnd | Operator::BitwiseOr => {
                AssemblerOperation::two_operands(&operator.to_asm::<InterimResultOption>(stack, meta, None)?.to_string(), &registers[0], &registers[1])
            }
            Operator::LogicalAnd => {
                // the actual data of register[0] is moved to rcx
                // the actual data of register[1] is moved to rax
                let lhs = GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::CL));
                let rhs = GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::AL));

                let eax = GeneralPurposeRegister::Bit32(Bit32::Eax);
                let mut target = String::new();
                let label1 = stack.create_label();
                let label2 = stack.create_label();

                let jump_instruction = operator.to_asm::<InterimResultOption>(stack, meta, None)?.to_string();
                target += &ASMBuilder::line(&format!("cmp {}, 0", lhs));
                target += &ASMBuilder::ident_line(&format!("{} {label1}", jump_instruction));

                // if literal, put in register first
                if !Self::is_stack_variable(&registers[1].to_string()) {
                    target += &ASMBuilder::mov_ident_line(eax.to_size_register(&ByteSize::_1), &registers[1]);
                    target += &ASMBuilder::ident_line(&format!("cmp {}, 0", eax.to_size_register(&ByteSize::_1)));
                } else {
                    target += &ASMBuilder::ident_line(&format!("cmp {}, 0", rhs));
                }

                target += &ASMBuilder::ident_line(&format!("{} {label1}", jump_instruction));
                target += &ASMBuilder::mov_ident_line(&eax, 1);
                target += &ASMBuilder::ident_line(&format!("jmp {label2}"));

                target += &ASMBuilder::line(&format!("{label1}:"));
                target += &ASMBuilder::mov_ident_line(eax, 0);

                target += &format!("{label2}:");

                Ok(AssemblerOperation {
                    prefix: Some(AssemblerOperation::save_rax_rcx_rdx(1, registers)?),
                    operation: target,
                    postfix: Some(AssemblerOperation::load_rax_rcx_rdx(1, registers)?),
                    result_expected: GeneralPurposeRegister::from_str(&registers[0].to_string()).map_err(|_| ASMGenerateError::InternalError(format!("Cannot build register from {}", registers[0])))?,
                })
            },
            Operator::LogicalOr => {
                let eax = GeneralPurposeRegister::Bit32(Bit32::Eax);
                let mut target = String::new();
                let label1 = stack.create_label();
                let label2 = stack.create_label();
                let label3 = stack.create_label();

                let lhs = GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::CL));
                let rhs = GeneralPurposeRegister::Bit8(Bit8::Single(NibbleRegister::AL));

                let jump_instruction = operator.to_asm::<InterimResultOption>(stack, meta, None)?.to_string();
                target += &ASMBuilder::line(&format!("cmp {}, 0", lhs));
                target += &ASMBuilder::ident_line(&format!("{} {label1}", jump_instruction));

                // if literal, put in register first
                if !Self::is_stack_variable(&registers[1].to_string()) {
                    target += &ASMBuilder::mov_ident_line(eax.to_size_register(&ByteSize::_1), &registers[1]);
                    target += &ASMBuilder::ident_line(&format!("cmp {}, 0", eax.to_size_register(&ByteSize::_1)));
                } else {
                    target += &ASMBuilder::ident_line(&format!("cmp {}, 0", rhs));
                }

                target += &ASMBuilder::ident_line(&format!("je {label2}"));

                target += &ASMBuilder::line(&format!("{label1}:"));
                target += &ASMBuilder::mov_ident_line(&eax, 1);
                target += &ASMBuilder::ident_line(&format!("jmp {label3}"));

                target += &ASMBuilder::line(&format!("{label2}:"));
                target += &ASMBuilder::mov_ident_line(eax, 0);

                target += &format!("{label3}:");

                Ok(AssemblerOperation {
                    prefix: Some(AssemblerOperation::save_rax_rcx_rdx(1, registers)?),
                    operation: target,
                    postfix: Some(AssemblerOperation::load_rax_rcx_rdx(1, registers)?),
                    result_expected: GeneralPurposeRegister::from_str(&registers[0].to_string()).map_err(|_| ASMGenerateError::InternalError(format!("Cannot build register from {}", registers[0])))?,
                })
            }
        }
    }
}

impl Castable<Boolean, Integer> for Boolean {
    fn add_casts(cast_matrix: &mut HashMap<(Type, Type), &'static str>) {
        cast_matrix.insert((Type::Bool(Mutability::Immutable), Type::Integer(Integer::I32, Mutability::Immutable)), "movzx");
    }

    fn cast_from_to(_: &Boolean, t2: &Integer, source: &str, stack: &mut Stack, meta: &mut MetaInfo) -> Result<ASMResult, ASMGenerateError> {
        let cast_to = CastTo {
            from: Type::Bool(Mutability::Immutable),
            to: Type::Integer(t2.clone(), Mutability::Immutable),
        };

        // no instruction is needed. just pretend the bool is an u8
        let mut target = String::new();
        target += &ASMBuilder::ident_comment_line(&format!("Cast: ({}) -> ({})", cast_to.from, cast_to.to));
        if Self::is_stack_variable(source) || source == "0" || source == "1" || GeneralPurposeRegister::from_str(source).is_ok() {
            let source = if let Ok(general_purpose_register) = GeneralPurposeRegister::from_str(source) {
                general_purpose_register.to_size_register(&ByteSize::_1).to_string()
            } else {
                source.to_string()
            };

            match Integer::cast_from_to(&Integer::U8, t2, &source, stack, meta)? {
                ASMResult::Inline(r) => {
                    target += &r;
                    Ok(ASMResult::Inline(target))
                }
                ASMResult::MultilineResulted(r, g) => {
                    target += &r;
                    Ok(ASMResult::MultilineResulted(target, g))
                }
                ASMResult::Multiline(r) => {
                    target += &r;
                    Ok(ASMResult::Multiline(target))
                }
            }
        } else {
            target += source;
            match Integer::cast_from_to(&Integer::U8, t2, "al", stack, meta)? {
                ASMResult::Inline(r) => {
                    target += &r;
                    Ok(ASMResult::Inline(target))
                }
                ASMResult::MultilineResulted(r, g) => {
                    target += &r;
                    Ok(ASMResult::MultilineResulted(target, g))
                }
                ASMResult::Multiline(r) => {
                    target += &r;
                    Ok(ASMResult::Multiline(target))
                }
            }
        }
    }
}

impl Boolean {
    pub fn operation_matrix(base_type_matrix: &mut HashMap<(Type, Operator, Type), Type>) {
        base_type_matrix.insert((Type::Bool(Mutability::Immutable), Operator::BitwiseAnd, Type::Bool(Mutability::Immutable)), Type::Bool(Mutability::Immutable));
        base_type_matrix.insert((Type::Bool(Mutability::Immutable), Operator::BitwiseOr, Type::Bool(Mutability::Immutable)), Type::Bool(Mutability::Immutable));

        base_type_matrix.insert((Type::Bool(Mutability::Immutable), Operator::LogicalAnd, Type::Bool(Mutability::Immutable)), Type::Bool(Mutability::Immutable));
        base_type_matrix.insert((Type::Bool(Mutability::Immutable), Operator::LogicalOr, Type::Bool(Mutability::Immutable)), Type::Bool(Mutability::Immutable));
    }

    fn is_stack_variable(value: &str) -> bool {
        for (_, word) in [8, 4, 2, 1].map(|a| (a, word_from_byte_size(a))) {
            if value.starts_with(&word) {
                return true;
            }
        }

        false
    }
}
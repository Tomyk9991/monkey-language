use crate::core::code_generator::{register_destination, ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_options::identifier_present::IdentifierPresent;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::asm_result::{ASMResult, ASMResultError, ASMResultVariance};
use crate::core::code_generator::generator::{Stack, StackLocation};
use crate::core::code_generator::registers::{Bit64, ByteSize, GeneralPurposeRegister};
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable};
use crate::core::model::abstract_syntax_tree_nodes::variable::{Variable};
use crate::core::model::types::ty::Type;

impl<const ASSIGNMENT: char, const SEPARATOR: char> ToASM for Variable<ASSIGNMENT, SEPARATOR> {
    fn to_asm(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<ASMOptions>) -> Result<ASMResult, ASMGenerateError> {
        let mut target = String::new();
        target += &ASMBuilder::ident(&ASMBuilder::comment_line(&format!("{}", self)));

        let result = match &self.assignable {
            Assignable::Array(_) => {
                let i = IdentifierPresent {
                    identifier: self.l_value.clone(),
                };
                self.assignable.to_asm(stack, meta, (!self.define).then_some(ASMOptions::IdentifierPresent(i)))?
            },
            _ => {
                let interim_options = InterimResultOption {
                    general_purpose_register: GeneralPurposeRegister::iter_from_byte_size(self.assignable.byte_size(meta))?.current().clone(),
                };
                self.assignable.to_asm(stack, meta, Some(ASMOptions::InterimResultOption(interim_options)))?
            },
        };

        let destination = if self.define {
            let byte_size = self.assignable.byte_size(meta);

            let elements = match &self.assignable {
                Assignable::Array(array) if array.values.len() > 1 => array.values.len(),
                _ => 1
            };

            stack.variables.push(StackLocation { position: stack.stack_position, size: byte_size, name: self.l_value.clone(), elements });

            stack.stack_position += byte_size;

            let offset = stack.stack_position;
            if !matches!(result, ASMResult::Multiline(_)) {
                format!("{} [rbp - {}]", register_destination::word_from_byte_size(byte_size), offset)
            } else {
                String::new()
            }
        } else {
            stack.register_to_use.push(GeneralPurposeRegister::Bit64(Bit64::Rdx));
            let result = match self.l_value.to_asm(stack, meta, options)? {
                ASMResult::Inline(r) => r,
                ASMResult::MultilineResulted(t, r) => {
                    target += &t;
                    r.to_string()
                }
                ASMResult::Multiline(_) => {
                    return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                        expected: vec![ASMResultVariance::MultilineResulted, ASMResultVariance::Inline],
                        actual: ASMResultVariance::Multiline,
                        ast_node: "variable node".to_string(),
                    }))
                }
            };

            stack.register_to_use.pop();

            result
        };

        match result {
            ASMResult::Inline(source) => {
                if self.assignable.is_stack_look_up(stack, meta) {
                    let destination_register = GeneralPurposeRegister::iter_from_byte_size(self.assignable.byte_size(meta))?.current();
                    target += &ASMBuilder::mov_x_ident_line(&destination_register, source, Some(destination_register.size() as usize));
                    target += &ASMBuilder::mov_ident_line(destination, &destination_register);
                } else {
                    target += &ASMBuilder::mov_ident_line(destination, source);
                }
            },
            ASMResult::MultilineResulted(source, mut register) => {
                target += &source;

                if let Assignable::Expression(expr) = &self.assignable {
                    let final_type = expr.get_type(&meta.static_type_information).ok_or(ASMGenerateError::InternalError("Cannot infer type".to_string(), meta.file_position.clone()))?;
                    let r = GeneralPurposeRegister::Bit64(Bit64::Rax).to_size_register(&ByteSize::try_from(final_type.byte_size())?);

                    if let GeneralPurposeRegister::Memory(memory) = &register {
                        target += &ASMBuilder::mov_x_ident_line(&r, memory, None);
                        register = r.clone();
                    }


                    if let Type::Float(s, _) = final_type {
                        target += &ASMBuilder::mov_x_ident_line(&r, register, Some(s.byte_size()));
                        register = r;
                    }
                }

                target += &ASMBuilder::mov_ident_line(destination, register);
            },
            ASMResult::Multiline(source) => {
                target += &source;
            }
        }

        Ok(ASMResult::Multiline(target))
    }


    fn is_stack_look_up(&self, stack: &mut Stack, meta: &MetaInfo) -> bool {
        self.assignable.is_stack_look_up(stack, meta)
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        self.ty.as_ref().map_or(0, |ty| ty.byte_size())
    }

    fn data_section(&self, stack: &mut Stack, meta: &mut MetaInfo) -> bool {
        self.assignable.data_section(stack, meta)
    }
}
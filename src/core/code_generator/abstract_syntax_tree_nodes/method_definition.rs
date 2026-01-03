use crate::core::code_generator::conventions::calling_convention_from;

use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::{ASMResult, ASMResultVariance};
use crate::core::code_generator::conventions::CallingRegister;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::registers::ByteSize;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::model::abstract_syntax_tree_nodes::method_definition::{MethodDefinition};
use crate::core::model::types::ty::Type;
use crate::core::parser::static_type_context::{CurrentMethodInfo, StaticTypeContext};
use crate::utils::math;

impl MethodDefinition {
    pub fn method_label_name(&self) -> String {
        if self.identifier.identifier() == "main" {
            return "main".to_string();
        }

        let parameters = if self.arguments.is_empty() {
            "void".to_string()
        } else {
            self.arguments.iter().map(|a| a.ty.to_string()).collect::<Vec<String>>().join("_")
        }.replace('*', "ptr");


        let return_type = self.return_type.to_string().replace('*', "ptr");

        format!(".{}_{}~{}", self.identifier, parameters, return_type)
    }
}

impl ToASM for MethodDefinition {
    fn to_asm(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<ASMOptions>) -> Result<ASMResult, ASMGenerateError> {
        let mut label_header: String = String::new();

        label_header += &ASMBuilder::line(&format!("{}:", self.method_label_name()));
        label_header += &ASMBuilder::ident_line("push rbp");
        label_header += &ASMBuilder::mov_ident_line("rbp", "rsp");


        label_header += &ASMBuilder::ident(&ASMBuilder::comment_line("Reserve stack space as MS convention. Shadow stacking"));
        let mut stack_allocation = 32; // per default microsoft convention requires 32 byte as a shadow stack
        let mut method_scope: String = String::new();

        let calling_convention = calling_convention_from(self, &meta.target_os);

        for (index, argument) in self.arguments.iter().enumerate() {
            if let Some(stack_location) = stack.variables.iter().rfind(|v| v.name.identifier() == argument.identifier.identifier()) {
                let destination = stack_location.name.clone().to_asm(stack, meta, options.clone())?;
                let source = match &calling_convention[index][0] {
                    CallingRegister::Register(r) => {
                        if matches!(argument.ty, Type::Float(_, _)) {
                            r.to_string()
                        } else {
                            r.to_size_register(&ByteSize::try_from(argument.ty.byte_size())?).to_string()
                        }
                    }
                    CallingRegister::Stack => "popppp".to_string()
                };

                method_scope.push_str(&ASMBuilder::mov_x_ident_line(destination, source, if let Type::Float(f, _) = &argument.ty {
                    Some(f.byte_size())
                } else {
                    None
                }));
            } else {
                return Err(ASMGenerateError::UnresolvedReference { name: argument.identifier.identifier().to_string(), file_position: self.file_position.clone()})
            }
        }

        meta.static_type_information.expected_return_type = Some(CurrentMethodInfo {
            return_type: self.return_type.clone(),
            method_header_line: self.file_position.clone(),
            method_name: self.identifier.identifier(),
        });

        for node in &self.stack {
            meta.file_position = node.file_position();
            stack_allocation += node.byte_size(meta);


            let variables_len = meta.static_type_information.len();

            if let Some(scope_stacks) = node.scope() {
                for scope_stack in scope_stacks {
                    meta.static_type_information.merge(StaticTypeContext::new(scope_stack));
                }
            }

            let _ = node.to_asm(stack, meta, None)?
                .apply_with(&mut method_scope)
                .allow(ASMResultVariance::Inline)
                .allow(ASMResultVariance::MultilineResulted)
                .allow(ASMResultVariance::Multiline)
                .ast_node("method definition")
                .finish()?;

            node.data_section(stack, meta);

            let amount_pop = meta.static_type_information.len() - variables_len;

            for _ in 0..amount_pop {
                let _ = meta.static_type_information.pop();
            }
        }

        meta.static_type_information.expected_return_type = None;

        let stack_allocation_asm = ASMBuilder::ident_line(&format!("sub rsp, {}", math::lowest_power_of_2_gt_n(stack_allocation)));
        let leave_statement = if self.return_type == Type::Void { "    leave\n    ret\n".to_string() } else { String::new() };

        Ok(ASMResult::Multiline(format!("{}{}{}{}", label_header, stack_allocation_asm, method_scope, leave_statement)))
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        true
    }

    fn byte_size(&self, _meta: &MetaInfo) -> usize {
        0
    }
}

use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::core::code_generator::conventions::calling_convention_from;

use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::asm_result::{ASMResult, ASMResultVariance};
use crate::core::code_generator::conventions::CallingRegister;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::registers::ByteSize;
use crate::core::io::code_line::CodeLine;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::model::abstract_syntax_tree_nodes::identifier::{Identifier, IdentifierError};
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::abstract_syntax_tree_nodes::method_definition::{MethodArgument, MethodDefinition};
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::scope::Scope;
use crate::core::model::types::mutability::Mutability;
use crate::core::model::types::ty::Type;
use crate::core::scanner::errors::EmptyIteratorErr;
use crate::core::scanner::scope::{PatternNotMatchedError, ScopeError};
use crate::core::scanner::static_type_context::{CurrentMethodInfo, StaticTypeContext};
use crate::core::scanner::{Lines, TryParse};
use crate::core::scanner::types::r#type::{InferTypeError, MethodCallSignatureMismatchCause};
use crate::core::semantics::type_checker::static_type_checker::{static_type_check_rec, StaticTypeCheckError};
use crate::core::semantics::type_checker::StaticTypeCheck;
use crate::utils::math;


#[derive(Debug)]
pub enum MethodDefinitionErr {
    PatternNotMatched { target_value: String },
    IdentifierErr(IdentifierError),
    ReturnErr(InferTypeError),
    AssignableErr(AssignableError),
    ScopeErrorErr(ScopeError),
    EmptyIterator(EmptyIteratorErr),
}

impl PatternNotMatchedError for MethodDefinitionErr {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, MethodDefinitionErr::PatternNotMatched {..})
    }
}

impl From<AssignableError> for MethodDefinitionErr {
    fn from(value: AssignableError) -> Self {
        MethodDefinitionErr::AssignableErr(value)
    }
}

impl From<IdentifierError> for MethodDefinitionErr {
    fn from(value: IdentifierError) -> Self {
        MethodDefinitionErr::IdentifierErr(value)
    }
}

impl From<InferTypeError> for MethodDefinitionErr {
    fn from(value: InferTypeError) -> Self {
        MethodDefinitionErr::ReturnErr(value)
    }
}


impl Error for MethodDefinitionErr {}

impl Display for MethodDefinitionErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            MethodDefinitionErr::PatternNotMatched { target_value }
            => format!("Pattern not matched for: `{target_value}`\n\t fn function_name(argument1, ..., argumentN): returnType {{ }}"),
            MethodDefinitionErr::AssignableErr(a) => a.to_string(),
            MethodDefinitionErr::IdentifierErr(a) => a.to_string(),
            MethodDefinitionErr::ReturnErr(a) => a.to_string(),
            MethodDefinitionErr::EmptyIterator(e) => e.to_string(),
            MethodDefinitionErr::ScopeErrorErr(a) => a.to_string(),
        })
    }
}

impl StaticTypeCheck for MethodDefinition {
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        // add the parameters to the type information
        for argument in &self.arguments {
            type_context.context.push(Variable {
                l_value: LValue::Identifier(argument.name.clone()),
                mutability: argument.ty.mutable(),
                ty: Some(argument.ty.clone()),
                define: true,
                assignable: Assignable::default(),
                code_line: Default::default(),
            });
        }

        let variables_len = type_context.context.len();
        type_context.expected_return_type = Some(CurrentMethodInfo {
            return_type: self.return_type.clone(),
            method_header_line: self.code_line.actual_line_number.clone(),
            method_name: self.identifier.name.to_string(),
        });


        static_type_check_rec(&self.stack, type_context)?;

        if self.return_type != Type::Void {
            if let [.., last] = &self.stack[..] {
                let mut method_return_signature_mismatch = false;
                let mut cause = MethodCallSignatureMismatchCause::ReturnMissing;

                if let AbstractSyntaxTreeNode::If(if_definition) = &last {
                    method_return_signature_mismatch = !if_definition.ends_with_return_in_each_branch();
                    if method_return_signature_mismatch {
                        cause = MethodCallSignatureMismatchCause::IfCondition;
                    }
                } else if !matches!(last, AbstractSyntaxTreeNode::Return(_)) {
                    method_return_signature_mismatch = true;
                }

                if method_return_signature_mismatch {
                    if let Some(expected_return_type) = &type_context.expected_return_type {
                        return Err(StaticTypeCheckError::InferredError(InferTypeError::MethodReturnSignatureMismatch {
                            expected: expected_return_type.return_type.clone(),
                            method_name: expected_return_type.method_name.to_string(),
                            method_head_line: expected_return_type.method_header_line.to_owned(),
                            cause,
                        }));
                    }
                }
            }
        }


        let amount_pop = (type_context.context.len() - variables_len) + self.arguments.len();

        for _ in 0..amount_pop {
            let _ = type_context.context.pop();
        }

        type_context.expected_return_type = None;
        Ok(())
    }
}

impl TryParse for MethodDefinition {
    type Output = MethodDefinition;
    type Err = MethodDefinitionErr;

    fn try_parse(code_lines_iterator: &mut Lines<'_>) -> anyhow::Result<Self, MethodDefinitionErr> {
        let method_header = *code_lines_iterator
            .peek()
            .ok_or(MethodDefinitionErr::EmptyIterator(EmptyIteratorErr))?;

        let split_alloc = method_header.split(vec![' ']);
        let split_ref = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();

        let (is_extern, fn_name, _generic_type, arguments, return_type) = match &split_ref[..] {
            ["extern", "fn", name, "<", generic_type, ">", "(", arguments @ .., ")", ":", return_type, ";"] => (true, name, Some(generic_type), arguments, return_type),
            ["extern", "fn", name, "(", arguments @ .., ")", ":", return_type, ";"] => (true, name, None, arguments, return_type),
            ["fn", name, "(", arguments @ .., ")", ":", return_type, "{"] => (false, name, None, arguments, return_type),

            ["extern", "fn", name, "<", generic_type, ">", "(", arguments @ .., ")", ":", ";"] => (true, name, Some(generic_type), arguments, &"void"),
            ["extern", "fn", name, "(", arguments @ .., ")", ";"] => (true, name, None, arguments, &"void"),
            ["fn", name, "(", arguments @ .., ")", "{"] => (false, name, None, arguments, &"void"),
            _ => return Err(MethodDefinitionErr::PatternNotMatched { target_value: method_header.line.to_string() })
        };

        let mut nodes = vec![];
        // consume the header
        let _ = code_lines_iterator.next();

        // consume the body
        if !is_extern {
            while code_lines_iterator.peek().is_some() {
                let node = Scope::try_parse(code_lines_iterator).map_err(MethodDefinitionErr::ScopeErrorErr)?;

                if let AbstractSyntaxTreeNode::ScopeEnding(_) = node {
                    break;
                }

                nodes.push(node);
            }
        }

        Ok(MethodDefinition {
            identifier: Identifier::from_str(fn_name, false)?,
            return_type: Type::from_str(return_type, Mutability::Immutable)?,
            arguments: Self::type_arguments(method_header, arguments)?,
            stack: nodes,
            is_extern,
            code_line: method_header.clone(),
        })
    }
}

impl MethodDefinition {
    fn type_arguments(method_header: &CodeLine, arguments: &[&str]) -> Result<Vec<MethodArgument>, MethodDefinitionErr> {
        let arguments_string = arguments.join(" ");
        let arguments = arguments_string.split(',').filter(|a| !a.is_empty()).map(|a| a.trim()).collect::<Vec<_>>();
        let mut type_arguments = vec![];

        for argument in arguments {
            let (name, mut ty) = match &argument.split(':').collect::<Vec<&str>>()[..] {
                [name, ty] => (name.trim(), ty.trim()),
                _ => return Err(MethodDefinitionErr::PatternNotMatched { target_value: method_header.line.clone() })
            };

            let mutability = if let ["mut", t] = ty.split_whitespace().collect::<Vec<&str>>()[..] {
                ty = t.trim();
                true
            } else {
                false
            };

            type_arguments.push(MethodArgument {
                name: Identifier::from_str(name.trim(), false)?,
                ty: Type::from_str(ty.trim(), Mutability::from(mutability))?
            })
        }

        Ok(type_arguments)
    }
}
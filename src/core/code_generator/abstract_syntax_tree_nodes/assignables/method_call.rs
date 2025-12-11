use std::any::Any;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use crate::core::code_generator::{ASMGenerateError, conventions, MetaInfo};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_options::in_expression_method_call::InExpressionMethodCall;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::asm_result::{ASMResult, ASMResultError, ASMResultVariance};
use crate::core::code_generator::conventions::CallingRegister;
use crate::core::code_generator::generator::{Stack};
use crate::core::code_generator::register_destination::byte_size_from_word;
use crate::core::code_generator::registers::{Bit64, ByteSize, GeneralPurposeRegister};
use crate::core::code_generator::ToASM;
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::token::Token;
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::model::abstract_syntax_tree_nodes::assignables::method_call::{MethodCall, MethodCallErr};
use crate::core::model::abstract_syntax_tree_nodes::identifier::{Identifier, IdentifierError};
use crate::core::model::types::ty::Type;
use crate::core::scanner::errors::EmptyIteratorErr;
use crate::core::scanner::scope::PatternNotMatchedError;
use crate::core::scanner::static_type_context::StaticTypeContext;
use crate::core::scanner::{Lines, TryParse};
use crate::core::scanner::types::r#type::{InferTypeError, MethodCallArgumentTypeMismatch};
use crate::core::semantics::type_checker::static_type_checker::StaticTypeCheckError;
use crate::core::semantics::type_checker::StaticTypeCheck;


impl PatternNotMatchedError for MethodCallErr {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, MethodCallErr::PatternNotMatched {..}) || matches!(self, MethodCallErr::IdentifierErr(_))
    }
}

impl std::error::Error for MethodCallErr {}

impl From<IdentifierError> for MethodCallErr {
    fn from(value: IdentifierError) -> Self {
        MethodCallErr::IdentifierErr(value)
    }
}

impl From<AssignableError> for MethodCallErr {
    fn from(value: AssignableError) -> Self { MethodCallErr::AssignableErr(value) }
}

impl From<DyckError> for MethodCallErr {
    fn from(s: DyckError) -> Self {
        MethodCallErr::DyckLanguageErr { target_value: s.target_value, ordering: s.ordering }
    }
}

impl Display for MethodCallErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            MethodCallErr::PatternNotMatched { target_value } => format!("\"{target_value}\" must match: methodName(assignable1, ..., assignableN)"),
            MethodCallErr::AssignableErr(a) => a.to_string(),
            MethodCallErr::IdentifierErr(a) => a.to_string(),
            MethodCallErr::DyckLanguageErr { target_value, ordering } =>
                {
                    let error: String = match ordering {
                        Ordering::Less => String::from("Expected `)`"),
                        Ordering::Equal => String::from("Expected expression between `,`"),
                        Ordering::Greater => String::from("Expected `(`")
                    };
                    format!("\"{target_value}\": {error}")
                }
            MethodCallErr::EmptyIterator(e) => e.to_string()
        };

        write!(f, "{}", message)
    }
}


impl MethodCall {
    pub fn method_label_name(&self, static_type_context: &StaticTypeContext) -> String {
        let method_definition = static_type_context.methods.iter().filter(|m| m.identifier == self.identifier)
            .map(|m| (m, m.arguments.iter().map(|a| a.ty.clone())))
            .filter(|(_, a)| a.clone().collect::<Vec<_>>() == self.arguments.iter().filter_map(|a| a.infer_type(&self.code_line)).collect::<Vec<_>>())
            .map(|(m, _)| m)
            .collect::<Vec<_>>();

        if let Some(method_definition) = method_definition.first() {
            method_definition.method_label_name()
        } else {
            "EMPTY".to_string()
        }
    }
}

impl ToASM for MethodCall {
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        let mut calling_convention = conventions::calling_convention(stack, meta, &self.arguments, &self.identifier.identifier())?;
        calling_convention.reverse();

        let method_defs = conventions::method_definitions(&meta.static_type_information, &meta.code_line, &self.arguments, &self.identifier.identifier())?;

        if method_defs.is_empty() {
            return Err(ASMGenerateError::TypeNotInferrable(InferTypeError::UnresolvedReference(self.identifier.to_string(), meta.code_line.clone())));
        }

        if method_defs.len() > 1 {
            return Err(ASMGenerateError::TypeNotInferrable(InferTypeError::MethodCallSignatureMismatch {
                signatures: meta.static_type_information.methods
                    .iter().filter(|m| m.identifier.identifier() == self.identifier.identifier())
                    .map(|m| m.arguments.iter().map(|a| a.ty.clone()).collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
                method_name: self.identifier.clone(),
                code_line: meta.code_line.clone(),
                provided: self.arguments.iter().filter_map(|a| a.infer_type_with_context(&meta.static_type_information, &meta.code_line).ok()).collect::<Vec<_>>(),
            }));
        }

        let method_def = &method_defs[0];
        let resulting_register = GeneralPurposeRegister::Bit64(Bit64::Rax);

        // represents the register where the final result must lay in, and where it is expected, after call
        let register_to_move_result = stack.register_to_use.last().unwrap_or(&GeneralPurposeRegister::Bit64(Bit64::Rax)).clone();
        let register_to_move_result_64bit = register_to_move_result.to_64_bit_register();
        let mut target = String::new();
        let mut registers_push_ignore = vec![];


        let is_direct_method_call = if let Some(options) = options {
            let any_t = &options as &dyn Any;
            any_t.downcast_ref::<InExpressionMethodCall>().is_none()
        } else {
            true
        };


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
            let provided_type = argument.infer_type_with_context(&meta.static_type_information, &meta.code_line)?;
            let result_from_eval = GeneralPurposeRegister::Bit64(Bit64::Rax)
                .to_size_register(&ByteSize::try_from(provided_type.byte_size())?);

            let mut inline = false;
            let mut assign = String::new();

            match argument.to_asm(stack, meta, Some(InterimResultOption::from(&result_from_eval)))? {
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
                                .ok_or(ASMGenerateError::InternalError(format!("Could not parse {stack_position} as a byte size")))?);

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
        // since multiple pops result in unexpected or even crashing behaviour. just one pop is needed
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

    fn byte_size(&self, meta: &mut MetaInfo) -> usize {
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

#[derive(Debug)]
pub struct DyckError {
    pub target_value: String,
    pub ordering: Ordering,
}

pub trait ArrayOrObject<T> {
    fn list(&self) -> Vec<T>;
}

impl ArrayOrObject<char> for char {
    fn list(&self) -> Vec<char> {
        vec![*self]
    }
}

impl ArrayOrObject<char> for Vec<char> {
    fn list(&self) -> Vec<char> {
        self.clone()
    }
}

impl ArrayOrObject<TokenWithSpan> for Vec<char> {
    fn list(&self) -> Vec<TokenWithSpan> {
        self.iter().map(|c| TokenWithSpan {
            token: Token::from(*c),
            span: FilePosition::default(),
        }).collect::<Vec<_>>()
    }
}

/// # Formal definition
/// Let Σ = {( ) [a-z A-Z]}
///
/// {u ∈ Σ* | all prefixes of u contain no more )'s than ('s and the number of ('s in equals the number of )'s }
pub fn dyck_language_generic<T: ArrayOrObject<K>, K, F>(sequence: &[K], values: [T; 3], contains: F) -> Result<Vec<Vec<K>>, DyckError>
where
    K: Clone + Debug,
    F: Fn(&[K], &K) -> bool {
    let mut individual_parameters: Vec<Vec<K>> = Vec::new();
    let mut counter = 0;
    let mut current_start_index = 0;

    let openings = values[0].list();
    let separators = values[1].list();
    let closings = values[2].list();

    for (index, c) in sequence.iter().enumerate() {
        if contains(&openings, &c) { // opening
            counter += 1;
        } else if contains(&closings, &c) { // closing
            counter -= 1;
        } else if contains(&separators, &c) && counter == 0 { // seperator
            let value = &sequence[current_start_index..index];

            if value.is_empty() {
                return Err(DyckError {
                    target_value: format!("{:?}", sequence),
                    ordering: Ordering::Equal,
                });
            }

            individual_parameters.push(value.to_vec());
            current_start_index = index + 1;
        }

        if counter < 0 {
            return Err(DyckError {
                target_value: format!("{:?}", sequence),
                ordering: Ordering::Less,
            });
        }
    }

    match counter {
        number if number > 0 => Err(DyckError {
            target_value: format!("{:?}", sequence),
            ordering: Ordering::Less,
        }),
        number if number < 0 => Err(DyckError {
            target_value: format!("{:?}", sequence),
            ordering: Ordering::Greater,
        }),
        _ => {
            let s = &sequence[current_start_index..sequence.len()];
            if !s.is_empty() {
                individual_parameters.push(sequence[current_start_index..sequence.len()].to_vec());
            }

            Ok(individual_parameters)
        }
    }
}


/// # Formal definition
/// Let Σ = {( ) [a-z A-Z]}
///
/// {u ∈ Σ* | all prefixes of u contain no more )'s than ('s and the number of ('s in equals the number of )'s }
pub fn dyck_language<T: ArrayOrObject<char>>(parameter_string: &str, values: [T; 3]) -> Result<Vec<String>, DyckError> {
    let mut individual_parameters: Vec<String> = Vec::new();
    let mut counter = 0;
    let mut current_start_index = 0;

    for (index, c) in parameter_string.chars().enumerate() {
        if values[0].list().contains(&c) { // opening
            counter += 1;
        } else if values[2].list().contains(&c) { // closing
            counter -= 1;
        } else if values[1].list().contains(&c) && counter == 0 { // seperator
            let value = &parameter_string[current_start_index..index].trim();

            if value.is_empty() {
                return Err(DyckError {
                    target_value: parameter_string.to_string(),
                    ordering: Ordering::Equal,
                });
            }

            individual_parameters.push(value.to_string());
            current_start_index = index + 1;
        }

        if counter < 0 {
            return Err(DyckError {
                target_value: parameter_string.to_string(),
                ordering: Ordering::Less,
            });
        }
    }

    match counter {
        number if number > 0 => Err(DyckError {
            target_value: parameter_string.to_string(),
            ordering: Ordering::Less,
        }),
        number if number < 0 => Err(DyckError {
            target_value: parameter_string.to_string(),
            ordering: Ordering::Greater,
        }),
        _ => {
            let s = parameter_string[current_start_index..parameter_string.len()].trim().to_string();
            if !s.is_empty() {
                individual_parameters.push(parameter_string[current_start_index..parameter_string.len()].trim().to_string());
            }

            Ok(individual_parameters)
        }
    }
}

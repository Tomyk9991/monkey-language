use std::any::Any;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;
use std::str::FromStr;

use crate::core::code_generator::{ASMGenerateError, conventions, MetaInfo};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_result::{ASMOptions, ASMResult, ASMResultError, ASMResultVariance, InExpressionMethodCall, InterimResultOption};
use crate::core::code_generator::conventions::CallingRegister;
use crate::core::code_generator::generator::{Stack};
use crate::core::code_generator::registers::{Bit64, ByteSize, GeneralPurposeRegister};
use crate::core::code_generator::ToASM;
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::errors::EmptyIteratorErr;
use crate::core::lexer::scope::PatternNotMatchedError;
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::core::lexer::tokens::name_token::{NameToken, NameTokenErr};
use crate::core::lexer::TryParse;
use crate::core::lexer::types::type_token::{InferTypeError, MethodCallArgumentTypeMismatch, TypeToken};

#[derive(Debug, PartialEq, Clone)]
pub struct MethodCallToken {
    pub name: NameToken,
    pub arguments: Vec<AssignableToken>,
    pub code_line: CodeLine,
}

impl Display for MethodCallToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.arguments
            .iter()
            .map(|ass| format!("{}", ass))
            .collect::<Vec<String>>()
            .join(", "))
    }
}

#[derive(Debug)]
pub enum MethodCallTokenErr {
    PatternNotMatched { target_value: String },
    NameTokenErr(NameTokenErr),
    DyckLanguageErr { target_value: String, ordering: Ordering },
    AssignableTokenErr(AssignableTokenErr),
    EmptyIterator(EmptyIteratorErr),
}

impl PatternNotMatchedError for MethodCallTokenErr {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, MethodCallTokenErr::PatternNotMatched {..}) || matches!(self, MethodCallTokenErr::NameTokenErr(_))
    }
}

impl std::error::Error for MethodCallTokenErr {}

impl From<NameTokenErr> for MethodCallTokenErr {
    fn from(value: NameTokenErr) -> Self {
        MethodCallTokenErr::NameTokenErr(value)
    }
}

impl From<AssignableTokenErr> for MethodCallTokenErr {
    fn from(value: AssignableTokenErr) -> Self { MethodCallTokenErr::AssignableTokenErr(value) }
}

impl From<DyckError> for MethodCallTokenErr {
    fn from(s: DyckError) -> Self {
        MethodCallTokenErr::DyckLanguageErr { target_value: s.target_value, ordering: s.ordering }
    }
}

impl Display for MethodCallTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            MethodCallTokenErr::PatternNotMatched { target_value } => format!("\"{target_value}\" must match: methodName(assignable1, ..., assignableN)"),
            MethodCallTokenErr::AssignableTokenErr(a) => a.to_string(),
            MethodCallTokenErr::NameTokenErr(a) => a.to_string(),
            MethodCallTokenErr::DyckLanguageErr { target_value, ordering } =>
                {
                    let error: String = match ordering {
                        Ordering::Less => String::from("Expected `)`"),
                        Ordering::Equal => String::from("Expected expression between `,`"),
                        Ordering::Greater => String::from("Expected `(`")
                    };
                    format!("\"{target_value}\": {error}")
                }
            MethodCallTokenErr::EmptyIterator(e) => e.to_string()
        };

        write!(f, "{}", message)
    }
}

impl FromStr for MethodCallToken {
    type Err = MethodCallTokenErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut code_line = CodeLine::imaginary(s);

        if !s.ends_with(';') {
            code_line.line += " ;";
        }

        MethodCallToken::try_parse(&code_line)
    }
}

impl TryParse for MethodCallToken {
    type Output = MethodCallToken;
    type Err = MethodCallTokenErr;

    fn try_parse(code_lines_iterator: &mut Peekable<Iter<CodeLine>>) -> anyhow::Result<Self::Output, Self::Err> {
        let code_line = *code_lines_iterator.peek().ok_or(MethodCallTokenErr::EmptyIterator(EmptyIteratorErr))?;
        MethodCallToken::try_parse(code_line)
    }
}

impl MethodCallToken {
    pub fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self, MethodCallTokenErr> {
        let split_alloc = code_line.split(vec![' ', ';']);
        let split = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();

        if let [name, "(", ")", ";"] = &split[..] {
            Ok(MethodCallToken {
                name: NameToken::from_str(name, false)?,
                arguments: vec![],
                code_line: code_line.clone(),
            })
        } else if let [name, "(", argument_segments @ .., ")", ";"] = &split[..] {
            let name = NameToken::from_str(name, false)?;
            let joined = &argument_segments.join(" ");
            let argument_strings = dyck_language(joined, [vec!['{', '('], vec![','], vec!['}', ')']])?;

            let arguments = argument_strings
                .iter()
                .map(|s| AssignableToken::from_str(s))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(MethodCallToken {
                name,
                arguments,
                code_line: code_line.clone(),
            })
        } else {
            Err(MethodCallTokenErr::PatternNotMatched { target_value: code_line.line.to_string() })
        }
    }

    pub fn infer_type_with_context(&self, context: &StaticTypeContext, code_line: &CodeLine) -> Result<TypeToken, InferTypeError> {
        if let Some(method_def) = context.methods.iter().find(|method_def| {
            method_def.name == self.name
        }) {
            self.type_check(context, code_line)?;
            return Ok(method_def.return_type.clone());
        }

        Err(InferTypeError::UnresolvedReference(self.to_string(), code_line.clone()))
    }

    /// Type checks the symbol calling, by checking if the passed parameters are expected by the method definition
    pub fn type_check(&self, context: &StaticTypeContext, code_line: &CodeLine) -> Result<(), InferTypeError> {
        let method_defs = context.methods.iter().filter(|m| m.name == self.name).collect::<Vec<_>>();

        'outer: for method_def in &method_defs {
            if method_def.arguments.len() != self.arguments.len() {
                if method_defs.len() == 1 {
                    return Err(InferTypeError::MethodCallArgumentAmountMismatch {
                        expected: method_def.arguments.len(),
                        actual: self.arguments.len(),
                        code_line: self.code_line.clone(),
                    });
                }

                continue;
            }

            let zipped = method_def.arguments
                .iter()
                .zip(&self.arguments);

            for (index, (argument_def, argument_call)) in zipped.enumerate() {
                let def_type = argument_def.1.clone();
                let call_type = argument_call.infer_type_with_context(context, code_line)?;

                if def_type != call_type {
                    if method_defs.len() == 1 {
                        return Err(InferTypeError::MethodCallArgumentTypeMismatch {
                            info: Box::new(MethodCallArgumentTypeMismatch {
                                expected: def_type,
                                actual: call_type,
                                nth_parameter: index + 1,
                                code_line: code_line.clone(),
                            })
                        });
                    }

                    continue 'outer;
                }
            }

            return Ok(());
        }

        if method_defs.is_empty() {
            return Err(InferTypeError::UnresolvedReference(self.name.name.clone(), code_line.clone()));
        }

        let signatures = method_defs
            .iter()
            .map(|m| m.arguments.iter().map(|a| a.1.clone()).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Err(InferTypeError::MethodCallSignatureMismatch {
            signatures,
            method_name: self.name.clone(),
            code_line: code_line.clone(),
            provided: self.arguments.iter().filter_map(|a| a.infer_type_with_context(context, code_line).ok()).collect::<Vec<_>>(),
        })
    }
}

impl ToASM for MethodCallToken {
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        let calling_convention = conventions::calling_convention(stack, meta, &self.arguments, &self.name.name)?;
        let method_defs = conventions::method_definitions(meta, &self.arguments, &self.name.name)?;

        if method_defs.is_empty() {
            return Err(ASMGenerateError::TypeNotInferrable(InferTypeError::UnresolvedReference(self.name.to_string(), meta.code_line.clone())))
        }

        if method_defs.len() > 1 {
            return Err(ASMGenerateError::TypeNotInferrable(InferTypeError::MethodCallSignatureMismatch {
                signatures: meta.static_type_information.methods
                    .iter().filter(|m| m.name.name == self.name.name)
                    .map(|m| m.arguments.iter().map(|a| a.1.clone()).collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
                method_name: self.name.clone(),
                code_line: meta.code_line.clone(),
                provided: self.arguments.iter().filter_map(|a| a.infer_type_with_context(&meta.static_type_information, &meta.code_line).ok()).collect::<Vec<_>>(),
            }))
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


        if method_def.return_type != TypeToken::Void && register_to_move_result_64bit == resulting_register {
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
            Stack
        }

        let zipped = calling_convention.iter().zip(&self.arguments);
        let mut parameters = vec![];
        for (conventions, argument) in zipped.rev() {
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

                    if let AssignableToken::FloatToken(_) = argument {
                        inline = true;
                        assign = r.to_string();
                    } else {
                        if r.is_float_register() {
                            target += &ASMBuilder::mov_x_ident_line(&r.to_64_bit_register(), &r, Some(r.size() as usize));
                        }

                        target += &ASMBuilder::ident_line(&format!("push {}", r.to_64_bit_register()));
                    }

                }
                ASMResult::Multiline(_) => return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                    expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                    actual: ASMResultVariance::Multiline,
                    token: "Method call".to_string(),
                }))
            }


            let mut variadic_parameters = vec![];
            for convention in conventions {
                match convention {
                    CallingRegister::Register(register_convention) => {
                        let register_convention_sized = register_convention.to_size_register_ignore_float(&ByteSize::try_from(provided_type.byte_size())?);
                        variadic_parameters.push((register_convention_sized, if inline { RegisterResult::Assign(assign.clone()) } else { RegisterResult::Stack }, Some(provided_type.byte_size()), argument));
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
        for all_conventions in parameters {
            for (index, (register_convention_sized, assign, size, argument)) in all_conventions.iter().enumerate() {
                if index == 0 {
                    target += &ASMBuilder::ident(&ASMBuilder::comment_line(&format!("Parameter ({})", argument)));
                    match assign {
                        RegisterResult::Assign(assign) => {
                            target += &ASMBuilder::mov_x_ident_line(register_convention_sized, assign, *size)
                        }
                        RegisterResult::Stack => {
                            target += &ASMBuilder::ident_line(&format!("pop {}", register_convention_sized.to_64_bit_register()));
                            popped_into = register_convention_sized.to_64_bit_register();
                        }
                    }
                } else {
                    match assign {
                        RegisterResult::Assign(assign) => {
                            target += &ASMBuilder::mov_x_ident_line(register_convention_sized, assign, *size);
                        }
                        RegisterResult::Stack => {
                            target += &ASMBuilder::mov_x_ident_line(register_convention_sized, &popped_into, *size);
                        }
                    }
                }
            }
        }

        target += &ASMBuilder::ident(&ASMBuilder::comment_line(&self.to_string()));
        target += &ASMBuilder::ident_line(&format!("call {}", if method_def.is_extern { method_def.name.name.to_string() } else { method_def.method_label_name() }));

        if method_def.return_type != TypeToken::Void {
            target += &ASMBuilder::mov_x_ident_line(
                &register_to_move_result,
                GeneralPurposeRegister::Bit64(Bit64::Rax).to_size_register(&ByteSize::try_from(method_def.return_type.byte_size())?),
                Some(method_def.return_type.byte_size())
            );
        }

        if !is_direct_method_call {
            target += &ASMBuilder::pop_registers(&registers_push_ignore);
        }


        if method_def.return_type != TypeToken::Void {
            Ok(ASMResult::MultilineResulted(target, register_to_move_result.to_size_register(&ByteSize::try_from(method_def.return_type.byte_size())?)))
        } else {
            Ok(ASMResult::Multiline(target))
        }
    }


    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        true
    }

    fn byte_size(&self, meta: &mut MetaInfo) -> usize {
        return if let Some(method_def) = meta.static_type_information.methods.iter().find(|m| m.name == self.name) {
            method_def.return_type.byte_size()
        } else {
            0
        };
    }

    fn before_label(&self, stack: &mut Stack, meta: &mut MetaInfo) -> Option<Result<String, ASMGenerateError>> {
        let mut target = String::new();
        let mut has_before_label_asm = false;

        let count_before = stack.label_count;

        for argument in self.arguments.iter().rev() {
            if let Some(before_label) = argument.before_label(stack, meta) {
                match before_label {
                    Ok(before_label) => {
                        target += &ASMBuilder::line(&(before_label));
                        has_before_label_asm = true;
                    }
                    Err(err) => return Some(Err(err))
                }
                stack.label_count -= 1;
            }
        }

        stack.label_count = count_before;

        if has_before_label_asm { Some(Ok(target)) } else { None }
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

    return match counter {
        number if number > 0 => Err(DyckError {
            target_value: parameter_string.to_string(),
            ordering: Ordering::Less,
        }),
        number if number < 0 => return Err(DyckError {
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
    };
}

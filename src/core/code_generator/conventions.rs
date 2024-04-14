use std::fmt::{Display, Formatter};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::MetaInfo;
use crate::core::code_generator::registers::{Bit64, FloatRegister, GeneralPurposeRegister};
use crate::core::code_generator::target_os::TargetOS;
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::tokens::assignable_token::AssignableToken;
use crate::core::lexer::tokens::method_definition::MethodDefinition;
use crate::core::lexer::tokens::name_token::NameToken;
use crate::core::lexer::types::type_token::{InferTypeError, TypeToken};

/// An enum representing the destination register. If its a register it contains the register
/// For floats its for example a "rcx" or "rdx" for windows calling convention
#[derive(Clone, Debug, PartialEq)]
pub enum CallingRegister {
    Register(GeneralPurposeRegister),
    Stack,
}

impl Display for CallingRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CallingRegister::Register(r) => r.to_string(),
            CallingRegister::Stack => "[]".to_string()
        })
    }
}

pub fn calling_convention(stack: &mut Stack, meta: &MetaInfo, calling_arguments: &[AssignableToken], method_name: &str) -> Result<Vec<Vec<CallingRegister>>, InferTypeError> {
    match meta.target_os {
        TargetOS::Windows => windows_calling_convention(stack, meta, calling_arguments, method_name),
        TargetOS::Linux | TargetOS::WindowsSubsystemLinux => {
            unimplemented!("Linux calling convention not implemented yet");
        }
    }
}

pub fn calling_convention_from(method_definition: &MethodDefinition, target_os: &TargetOS) -> Vec<Vec<CallingRegister>> {
    match target_os {
        TargetOS::Windows => windows_calling_convention_from(method_definition),
        TargetOS::Linux | TargetOS::WindowsSubsystemLinux => {
            unimplemented!("Linux calling convention not implemented yet");
        }
    }
}

pub fn return_calling_convention(_stack: &mut Stack, meta: &MetaInfo) -> Result<GeneralPurposeRegister, InferTypeError> {
    match meta.target_os {
        TargetOS::Windows => Ok(GeneralPurposeRegister::Bit64(Bit64::Rax)),
        TargetOS::Linux | TargetOS::WindowsSubsystemLinux => {
            unimplemented!("Linux returning convention not implemented yet");
        }
    }
}

fn windows_calling_convention(_stack: &mut Stack, meta: &MetaInfo, calling_arguments: &[AssignableToken], method_name: &str) -> Result<Vec<Vec<CallingRegister>>, InferTypeError> {
    static FLOAT_ORDER: [CallingRegister; 4] = [
        CallingRegister::Register(GeneralPurposeRegister::Float(FloatRegister::Xmm0)),
        CallingRegister::Register(GeneralPurposeRegister::Float(FloatRegister::Xmm1)),
        CallingRegister::Register(GeneralPurposeRegister::Float(FloatRegister::Xmm2)),
        CallingRegister::Register(GeneralPurposeRegister::Float(FloatRegister::Xmm3))
    ];
    static POINTER_ORDER: [CallingRegister; 4] = [
        CallingRegister::Register(GeneralPurposeRegister::Bit64(Bit64::Rcx)),
        CallingRegister::Register(GeneralPurposeRegister::Bit64(Bit64::Rdx)),
        CallingRegister::Register(GeneralPurposeRegister::Bit64(Bit64::R8)),
        CallingRegister::Register(GeneralPurposeRegister::Bit64(Bit64::R9))
    ];

    let mut result = vec![];


    let method_defs = method_definitions(&meta.static_type_information, &meta.code_line, calling_arguments, method_name)?;

    if method_defs.is_empty() {
        return Err(InferTypeError::UnresolvedReference(method_name.to_string(), meta.code_line.clone()))
    }

    if method_defs.len() > 1 {
        return Err(InferTypeError::MethodCallSignatureMismatch {
            signatures: meta.static_type_information.methods
                .iter().filter(|m| m.name.name == method_name)
                .map(|m| m.arguments.iter().map(|a| a.1.clone()).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            method_name: NameToken { name: method_name.to_string() },
            code_line: meta.code_line.clone(),
            provided: calling_arguments.iter().filter_map(|a| a.infer_type_with_context(&meta.static_type_information, &meta.code_line).ok()).collect::<Vec<_>>(),
        })
    }

    let method_def = if let Some(method_def) = meta.static_type_information.methods.iter().find(|m| m.name.name == method_name) {
        method_def.clone()
    } else {
        return Err(InferTypeError::UnresolvedReference(method_name.to_string(), meta.code_line.clone()));
    };

    for (index, calling_argument) in calling_arguments.iter().enumerate() {
        let calling_ty: TypeToken = calling_argument.infer_type_with_context(&meta.static_type_information, &meta.code_line)?;

        match calling_ty {
            TypeToken::Integer(_) | TypeToken::Bool | TypeToken::Custom(_) | TypeToken::Array(_, _) => {
                if index < 4 {
                    result.push(vec![POINTER_ORDER[index].clone()]);
                } else {
                    result.push(vec![CallingRegister::Stack]);
                }
            }
            TypeToken::Float(_) => {
                let mut r = vec![];
                if method_def.is_extern {
                    r.push(POINTER_ORDER[index].clone());
                }

                if index < 4 {
                    r.push(FLOAT_ORDER[index].clone());
                } else {
                    r.push(CallingRegister::Stack);
                }

                result.push(r);
            }
            TypeToken::Void => {}
        }
    }

    Ok(result)
}

/// Returns every possible method definition based on the argument signature and method name
pub fn method_definitions(meta: &StaticTypeContext, code_line: &CodeLine, arguments: &[AssignableToken], method_name: &str) -> Result<Vec<MethodDefinition>, InferTypeError> {
    let mut method_definitions = vec![];

    'outer: for method in &meta.methods {
        if method.name.name != method_name || method.arguments.len() != arguments.len() {
            continue;
        }

        for (index, (_, argument_type)) in method.arguments.iter().enumerate() {
            let calling_type = arguments[index].infer_type_with_context(meta, code_line)?;
            if *argument_type != calling_type {
                continue 'outer;
            }
        }

        method_definitions.push(method.clone());
    }

    Ok(method_definitions)
}


fn windows_calling_convention_from(method_definition: &MethodDefinition) -> Vec<Vec<CallingRegister>> {
    static FLOAT_ORDER: [CallingRegister; 4] = [
        CallingRegister::Register(GeneralPurposeRegister::Float(FloatRegister::Xmm0)),
        CallingRegister::Register(GeneralPurposeRegister::Float(FloatRegister::Xmm1)),
        CallingRegister::Register(GeneralPurposeRegister::Float(FloatRegister::Xmm2)),
        CallingRegister::Register(GeneralPurposeRegister::Float(FloatRegister::Xmm3))
    ];
    static POINTER_ORDER: [CallingRegister; 4] = [
        CallingRegister::Register(GeneralPurposeRegister::Bit64(Bit64::Rcx)),
        CallingRegister::Register(GeneralPurposeRegister::Bit64(Bit64::Rdx)),
        CallingRegister::Register(GeneralPurposeRegister::Bit64(Bit64::R8)),
        CallingRegister::Register(GeneralPurposeRegister::Bit64(Bit64::R9))
    ];

    let mut result = vec![];

    for (index, (_, calling_type)) in method_definition.arguments.iter().enumerate() {
        match calling_type {
            TypeToken::Integer(_) | TypeToken::Bool | TypeToken::Custom(_) | TypeToken::Array(_, _) => {
                if index < 4 {
                    result.push(vec![POINTER_ORDER[index].clone()]);
                } else {
                    result.push(vec![CallingRegister::Stack]);
                }
            }
            TypeToken::Float(_) => {
                let mut r = vec![];
                if index < 4 {
                    r.push(FLOAT_ORDER[index].clone());
                } else {
                    r.push(CallingRegister::Stack);
                }

                result.push(r);
            }
            TypeToken::Void => {}
        }
    }

    result
}
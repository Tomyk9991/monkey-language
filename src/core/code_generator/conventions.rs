use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::MetaInfo;
use crate::core::code_generator::registers::{Bit64, FloatRegister, GeneralPurposeRegister};
use crate::core::code_generator::target_os::TargetOS;
use crate::core::lexer::tokens::assignable_token::AssignableToken;
use crate::core::lexer::types::type_token::{InferTypeError, TypeToken};

/// An enum representing the destination register. If its a register it contains the register
/// For floats its for example a "rcx" or "rdx" for windows calling convention
#[derive(Clone)]
pub enum CallingRegister {
    Register(GeneralPurposeRegister),
    Stack,
}

pub fn calling_convention(stack: &mut Stack, meta: &MetaInfo, calling_arguments: &[AssignableToken], method_name: &str) -> Result<Vec<Vec<CallingRegister>>, InferTypeError> {
    match meta.target_os {
        TargetOS::Windows => windows_calling_convention(stack, meta, calling_arguments, method_name),
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

    let method_def = if let Some(method_def) = meta.static_type_information.methods.iter().find(|m| m.name.name == method_name) {
        method_def.clone()
    } else {
        return Err(InferTypeError::UnresolvedReference(method_name.to_string(), meta.code_line.clone()));
    };

    for (index, calling_argument) in calling_arguments.iter().enumerate() {
        let calling_ty: TypeToken = calling_argument.infer_type_with_context(&meta.static_type_information, &meta.code_line)?;

        match calling_ty {
            TypeToken::Integer(_) | TypeToken::Bool | TypeToken::Custom(_) => {
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

                if method_def.is_extern {
                    r.push(POINTER_ORDER[index].clone());
                }

                result.push(r);
            }
            TypeToken::Void => {}
        }
    }

    Ok(result)
}
use crate::core::code_generator::MetaInfo;
use crate::core::code_generator::target_os::TargetOS;
use crate::core::lexer::tokens::assignable_token::AssignableToken;
use crate::core::lexer::type_token::{InferTypeError, TypeToken};

/// An enum representing the destination register. If its a register it contains the register
/// For floats its for example a "rcx" or "rdx" for windows calling convention
#[derive(Clone)]
pub enum CallingRegister {
    Register(&'static str),
    Stack
}

pub fn calling_convention(meta: &MetaInfo, calling_arguments: &[AssignableToken]) -> Result<Vec<CallingRegister>, InferTypeError> {
    match meta.target_os {
        TargetOS::Windows => windows_calling_convention(meta, calling_arguments),
        TargetOS::Linux | TargetOS::WindowsSubsystemLinux => {
            unimplemented!("Linux calling convention not implemented yet");
        }
    }
}

fn windows_calling_convention(meta: &MetaInfo, calling_arguments: &[AssignableToken]) -> Result<Vec<CallingRegister>, InferTypeError> {
    static FLOAT_ORDER: [CallingRegister; 4] = [CallingRegister::Register("xmm0"), CallingRegister::Register("xmm1"), CallingRegister::Register("xmm2"), CallingRegister::Register("xmm3")];
    static POINTER_ORDER: [CallingRegister; 4] = [CallingRegister::Register("rcx"), CallingRegister::Register("rdx"), CallingRegister::Register("r8"), CallingRegister::Register("r9")];

    let mut result = vec![];

    for (index, calling_argument) in calling_arguments.iter().enumerate() {
        let calling_ty: TypeToken = calling_argument.infer_type_with_context(&meta.static_type_information, &meta.code_line)?;

        match calling_ty {
            TypeToken::I32 | TypeToken::Bool | TypeToken::Custom(_) => {
                if index < 4 {
                    result.push(POINTER_ORDER[index].clone());
                } else {
                    result.push(CallingRegister::Stack);
                }
            }
            TypeToken::F32 => {
                if index < 4 {
                    result.push(FLOAT_ORDER[index].clone());
                } else {
                    result.push(CallingRegister::Stack);
                }
            }
            TypeToken::Void => {}
        }
    }

    Ok(result)
}
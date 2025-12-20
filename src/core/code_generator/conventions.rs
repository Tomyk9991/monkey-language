use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::registers::{Bit64, FloatRegister, GeneralPurposeRegister};
use crate::core::code_generator::target_os::TargetOS;
use crate::core::code_generator::MetaInfo;
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::abstract_syntax_tree_nodes::method_definition::MethodDefinition;
use crate::core::model::types::ty::Type;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::InferTypeError;
use std::fmt::{Display, Formatter};

/// An enum representing the destination register. If its a register it contains the register
/// For floats its for example a "rcx" or "rdx" for windows calling convention
#[derive(Clone, PartialOrd, Debug, PartialEq)]
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

pub fn calling_convention(stack: &mut Stack, meta: &mut MetaInfo, calling_arguments: &[Assignable], method_name: &str) -> Result<Vec<Vec<CallingRegister>>, Box<InferTypeError>> {
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

pub fn return_calling_convention(_stack: &mut Stack, meta: &MetaInfo) -> Result<GeneralPurposeRegister, Box<InferTypeError>> {
    match meta.target_os {
        TargetOS::Windows => Ok(GeneralPurposeRegister::Bit64(Bit64::Rax)),
        TargetOS::Linux | TargetOS::WindowsSubsystemLinux => {
            unimplemented!("Linux returning convention not implemented yet");
        }
    }
}

fn windows_calling_convention(_stack: &mut Stack, meta: &mut MetaInfo, calling_arguments: &[Assignable], method_name: &str) -> Result<Vec<Vec<CallingRegister>>, Box<InferTypeError>> {
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


    let method_defs = method_definitions(&mut meta.static_type_information, calling_arguments, method_name)?;

    if method_defs.is_empty() {
        return Err(Box::new(InferTypeError::UnresolvedReference(method_name.to_string(), meta.file_position.clone())))
    }

    if method_defs.len() > 1 {
        return Err(Box::new(InferTypeError::MethodCallSignatureMismatch {
            signatures: meta.static_type_information.methods
                .iter().filter(|m| m.identifier.identifier() == method_name)
                .map(|m| m.arguments.iter().map(|a| a.ty.clone()).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            method_name: LValue::Identifier(Identifier { name: method_name.to_string() }),
            file_position: meta.file_position.clone(),
            provided: calling_arguments.iter().filter_map(|a| a.get_type(&meta.static_type_information)).collect::<Vec<_>>(),
        }))
    }

    let method_def = if let Some(method_def) = meta.static_type_information.methods.iter().find(|m| m.identifier.identifier() == method_name) {
        method_def.clone()
    } else {
        return Err(Box::new(InferTypeError::UnresolvedReference(method_name.to_string(), meta.file_position.clone())));
    };

    for (index, calling_argument) in calling_arguments.iter().enumerate() {
        let calling_ty: Type = calling_argument.get_type(&meta.static_type_information).ok_or(InferTypeError::NoTypePresent(
            LValue::Identifier(Identifier { name: "Argument".to_string() }), meta.file_position.clone()
        ))?;

        match calling_ty {
            Type::Integer(_, _) | Type::Bool(_) | Type::Custom(_, _) | Type::Array(_, _, _) => {
                if index < 4 {
                    result.push(vec![POINTER_ORDER[index].clone()]);
                } else {
                    result.push(vec![CallingRegister::Stack]);
                }
            }
            Type::Float(_, _) => {
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
            Type::Void => {}
            Type::Statement => {}
        }
    }

    Ok(result)
}

/// Returns every possible method definition based on the argument signature and method name
pub fn method_definitions(type_context: &mut StaticTypeContext, arguments: &[Assignable], method_name: &str) -> Result<Vec<MethodDefinition>, Box<InferTypeError>> {
    let mut method_definitions = vec![];

    'outer: for method in &type_context.methods {
        if method.identifier.identifier() != method_name || method.arguments.len() != arguments.len() {
            continue;
        }

        for (index, argument) in method.arguments.iter().enumerate() {
            let calling_type = arguments[index].get_type(type_context);
            if let Some(calling_type) = calling_type {
                if argument.ty < calling_type {
                    continue 'outer;
                }
            } else {
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

    for (index, argument) in method_definition.arguments.iter().enumerate() {
        match argument.ty {
            Type::Integer(_, _) | Type::Bool(_) | Type::Custom(_, _) | Type::Array(_, _, _) => {
                if index < 4 {
                    result.push(vec![POINTER_ORDER[index].clone()]);
                } else {
                    result.push(vec![CallingRegister::Stack]);
                }
            }
            Type::Float(_, _) => {
                let mut r = vec![];
                if index < 4 {
                    r.push(FLOAT_ORDER[index].clone());
                } else {
                    r.push(CallingRegister::Stack);
                }

                result.push(r);
            }
            Type::Void | Type::Statement => {}
        }
    }

    result
}
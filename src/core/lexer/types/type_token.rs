use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::Range;
use std::str::FromStr;
use crate::core::code_generator::{ASMGenerateError, MetaInfo};
use crate::core::code_generator::generator::Stack;

use crate::core::io::code_line::CodeLine;
use crate::core::lexer::tokens::assignable_token::AssignableToken;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::{AssemblerOperation, Operator, OperatorToASM};
use crate::core::lexer::tokens::l_value::LValue;
use crate::core::lexer::tokens::name_token::{NameToken, NameTokenErr};
use crate::core::lexer::tokens::variable_token::VariableToken;
use crate::core::lexer::types::boolean::Boolean;
use crate::core::lexer::types::cast_to::CastTo;
use crate::core::lexer::types::float::Float;
use crate::core::lexer::types::integer::Integer;

pub mod common {
    use crate::core::lexer::tokens::name_token::NameToken;
    use crate::core::lexer::types::type_token::{Mutability, TypeToken};

    #[allow(unused)]
    pub fn string() -> TypeToken { TypeToken::Custom(NameToken { name: "*string".to_string() }, Mutability::Immutable)}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Mutability {
    Mutable,
    Immutable,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TypeToken {
    Integer(Integer, Mutability),
    Float(Float, Mutability),
    Bool(Mutability),
    Void,
    Array(Box<TypeToken>, usize, Mutability),
    Custom(NameToken, Mutability),
}


#[derive(Debug)]
pub enum InferTypeError {
    TypesNotCalculable(TypeToken, Operator, TypeToken, CodeLine),
    UnresolvedReference(String, CodeLine),
    TypeNotAllowed(NameTokenErr),
    MultipleTypesInArray{expected: TypeToken, unexpected_type: TypeToken, unexpected_type_index: usize, code_line: CodeLine},
    IllegalDereference(AssignableToken, TypeToken, CodeLine),
    IllegalArrayTypeLookup(TypeToken, CodeLine),
    IllegalIndexOperation(TypeToken, CodeLine),
    NoTypePresent(LValue, CodeLine),
    DefineNotAllowed(VariableToken<'=', ';'>, CodeLine),
    IntegerTooSmall { ty: TypeToken, literal: String ,code_line: CodeLine },
    FloatTooSmall { ty: TypeToken, float: f64, code_line: CodeLine },
    MethodCallArgumentAmountMismatch { expected: usize, actual: usize, code_line: CodeLine },
    MethodCallArgumentTypeMismatch { info: Box<MethodCallArgumentTypeMismatch> },
    MethodReturnArgumentTypeMismatch { expected: TypeToken, actual: TypeToken, code_line: CodeLine },
    MethodReturnSignatureMismatch { expected: TypeToken, method_name: String, method_head_line: Range<usize>, cause: MethodCallSignatureMismatchCause },
    MethodCallSignatureMismatch { signatures: Vec<Vec<TypeToken>>, method_name: NameToken, code_line: CodeLine, provided: Vec<TypeToken> },
    NameCollision(String, CodeLine),
    MismatchedTypes { expected: TypeToken, actual: TypeToken, code_line: CodeLine },
}

#[derive(Debug)]
pub enum MethodCallSignatureMismatchCause {
    ReturnMissing,
    IfCondition
}

impl From<bool> for Mutability {
    fn from(value: bool) -> Self {
        if value {
            Mutability::Mutable
        } else {
            Mutability::Immutable
        }
    }
}

impl Display for Mutability {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Mutability::Mutable => "mut ",
            Mutability::Immutable => ""
        })
    }
}

impl Display for MethodCallSignatureMismatchCause {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            MethodCallSignatureMismatchCause::ReturnMissing => "",
            MethodCallSignatureMismatchCause::IfCondition => "Every branch of an if statement must end with a return statement"
        })
    }
}

#[derive(Debug)]
pub struct MethodCallArgumentTypeMismatch {
    pub expected: TypeToken,
    pub actual: TypeToken,
    pub nth_parameter: usize,
    pub code_line: CodeLine,
}

impl PartialOrd for TypeToken {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (equal_types, m1, m2) = match (self, other) {
            (TypeToken::Float(f1, m1), TypeToken::Float(f2, m2)) if f1 == f2 => (true, m1, m2),
            (TypeToken::Integer(i1, m1), TypeToken::Integer(i2, m2)) if i1 == i2 => (true, m1, m2),
            (TypeToken::Bool(m1), TypeToken::Bool(m2)) => (true, m1, m2),
            (TypeToken::Array(t1, s1, m1), TypeToken::Array(t2, s2, m2)) if t1.partial_cmp(t2)?.is_le() && s1 == s2 => (true, m1, m2),
            (TypeToken::Custom(n1, m1), TypeToken::Custom(n2, m2)) if n1 == n2 => (true, m1, m2),
            _ => return Some(Ordering::Less)
        };

        if equal_types {
            match (m1, m2) {
                (Mutability::Mutable, Mutability::Immutable) => Some(Ordering::Less),
                (Mutability::Immutable, Mutability::Mutable) => Some(Ordering::Greater),
                _ => Some(Ordering::Equal)
            }
        } else {
            Some(Ordering::Less)
        }
    }
}

impl OperatorToASM for TypeToken {
    fn operation_to_asm<T: Display>(&self, operator: &Operator, registers: &[T], stack: &mut Stack, meta: &mut MetaInfo) -> Result<AssemblerOperation, ASMGenerateError> {

        match self {
            TypeToken::Integer(t, _) => t.operation_to_asm(operator, registers, stack, meta),
            TypeToken::Float(t, _) => t.operation_to_asm(operator, registers, stack, meta),
            TypeToken::Bool(_) => Boolean::True.operation_to_asm(operator, registers, stack, meta),
            TypeToken::Void => Err(ASMGenerateError::InternalError("Void cannot be operated on".to_string())),
            TypeToken::Array(_, _, _) | TypeToken::Custom(_, _) => todo!(),
        }
    }
}


impl From<NameTokenErr> for InferTypeError {
    fn from(value: NameTokenErr) -> Self {
        InferTypeError::TypeNotAllowed(value)
    }
}

impl Error for InferTypeError {}

impl Display for InferTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InferTypeError::TypesNotCalculable(a, o, b, code_line) => write!(f, "Line: {:?}: \tCannot {} between types {} and {}", code_line.actual_line_number, o, a, b),
            InferTypeError::UnresolvedReference(s, code_line) => write!(f, "Line: {:?}: \tUnresolved reference: {s}", code_line.actual_line_number),
            InferTypeError::MismatchedTypes { expected, actual, code_line } => write!(f, "Line: {:?}: \tMismatched types: Expected `{expected}` but found `{actual}`", code_line.actual_line_number),
            InferTypeError::NameCollision(name, code_line) => write!(f, "Line: {:?}: \tTwo symbols share the same name: `{name}`", code_line.actual_line_number),
            InferTypeError::TypeNotAllowed(ty) => write!(f, "This type is not allowed due to: {}", ty),
            InferTypeError::MethodCallArgumentAmountMismatch { expected, actual, code_line } => write!(f, "Line: {:?}: \tThe method expects {} parameter, but {} are provided", code_line.actual_line_number, expected, actual),
            InferTypeError::MethodCallArgumentTypeMismatch { info } => write!(f, "Line: {:?}: \t The {}. argument must be of type: `{}` but `{}` is provided", info.code_line.actual_line_number, info.nth_parameter, info.expected, info.actual),
            InferTypeError::MethodReturnArgumentTypeMismatch { expected, actual, code_line } => write!(f, "Line: {:?}: \t The return type is: `{}` but `{}` is provided", code_line.actual_line_number, expected, actual),
            InferTypeError::MethodReturnSignatureMismatch { expected, method_name, method_head_line, cause } => write!(f, "Line: {method_head_line:?}: \tA return statement with type: `{expected}` is expected for the method: {method_name}\n\t{cause}"),
            InferTypeError::MethodCallSignatureMismatch { signatures, method_name, code_line, provided } => {
                let provided_arguments = provided.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(", ");
                let mut signatures = signatures
                    .iter()
                    .map(|v| format!("\t - ({})", v.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(", ")))
                    .collect::<Vec<String>>();
                signatures.sort();
                let signatures = signatures.join(",\n");

                write!(f, "Line: {:?}: Arguments `({})` to the function `{}` are incorrect: Possible signatures are:\n{}", code_line.actual_line_number, provided_arguments, method_name.name, signatures)
            }
            InferTypeError::MultipleTypesInArray { expected, unexpected_type, unexpected_type_index, code_line } => {
                write!(f, "Line: {:?}: Expected `{expected}` in array but found `{unexpected_type}` at position: `{unexpected_type_index}`", code_line.actual_line_number)
            }
            InferTypeError::NoTypePresent(name, code_line) => write!(f, "Line: {:?}\tType not inferred: `{name}`", code_line.actual_line_number),
            InferTypeError::IllegalDereference(assignable, ty, code_line) => write!(f, "Line: {:?}\tType `{ty}` cannot be dereferenced: {assignable}", code_line.actual_line_number),
            InferTypeError::IllegalArrayTypeLookup(ty, code_line) => write!(f, "Line: {:?}\tType `{ty}` cannot be indexed", code_line.actual_line_number),
            InferTypeError::IllegalIndexOperation(ty, code_line) => write!(f, "Line: {:?}\tType `{ty}` cannot be used as an index", code_line.actual_line_number),
            InferTypeError::IntegerTooSmall { ty, literal: integer, code_line } => write!(f, "Line: {:?}\t`{integer}` doesn't fit into the type `{ty}`", code_line.actual_line_number),
            InferTypeError::FloatTooSmall { ty, float, code_line } =>
                write!(f, "Line: {:?}\t`{float}` doesn't fit into the type `{ty}`", code_line.actual_line_number),
            InferTypeError::DefineNotAllowed(variable_token, code_line) => {
                write!(f, "Line: {:?}\t`{}` is not allowed to be defined here", code_line.actual_line_number, variable_token.l_value)
            }
        }
    }
}

impl Display for TypeToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TypeToken::Integer(int, _) => format!("{}", int),
            TypeToken::Float(float, _) => format!("{}", float),
            TypeToken::Bool(_) => "bool".to_string(),
            TypeToken::Void => "void".to_string(),
            TypeToken::Array(array_type, size, _) => format!("[{}; {size}]", array_type),
            TypeToken::Custom(name, _) => name.name.clone().to_string(),
        })
    }
}

impl TypeToken {
    pub fn from_str(s: &str, mutability: Mutability) -> Result<Self, InferTypeError> {
        if let ["[ ", type_str, ", ", type_size, "]"] = &s.split_inclusive(' ').collect::<Vec<_>>()[..] {
            return Ok(TypeToken::Array(Box::new(TypeToken::from_str(type_str.trim(), mutability.clone())?), type_size.trim().parse::<usize>()
                .map_err(|_| InferTypeError::TypeNotAllowed(NameTokenErr::UnmatchedRegex {
                    target_value: s.to_string(),
                }))?, mutability));
        }

        Ok(match s {
            "bool" => TypeToken::Bool(Mutability::Immutable),
            "void" => TypeToken::Void,
            custom => {
                if let Ok(int) = Integer::from_str(custom) {
                    return Ok(TypeToken::Integer(int, mutability));
                }

                if let Ok(float) = Float::from_str(custom) {
                    return Ok(TypeToken::Float(float, mutability));
                }

                if !lazy_regex::regex_is_match!(r"^[\*&]*[a-zA-Z_$][a-zA-Z_$0-9]*[\*&]*$", s) {
                    return Err(InferTypeError::TypeNotAllowed(NameTokenErr::UnmatchedRegex { target_value: String::from(custom) }));
                }

                TypeToken::Custom(NameToken { name: custom.to_string() }, mutability)
            }
        })
    }
    pub fn set_mutability(&mut self, m: Mutability) {
        match self {
            TypeToken::Integer(_, mutability) => *mutability = m,
            TypeToken::Float(_, mutability) => *mutability = m,
            TypeToken::Bool(mutability) => *mutability = m,
            TypeToken::Void => {},
            TypeToken::Array(_, _, mutability) => *mutability = m,
            TypeToken::Custom(_, mutability) => *mutability = m,
        }
    }

    pub fn cast_to(&self, to: &TypeToken) -> CastTo {
        CastTo {
            from: self.clone(),
            to: to.clone(),
        }
    }

    pub fn is_float(&self) -> bool {
        matches!(self, TypeToken::Float(_, _))
    }

    // takes the element array type and returns the type of the array
    pub fn pop_array(&self) -> Option<TypeToken> {
        if let TypeToken::Array(array_type, _, _) = self {
            return Some(*array_type.clone());
        }

        None
    }
    /// removes * from type
    pub fn pop_pointer(&self) -> Option<TypeToken> {
        if let TypeToken::Custom(name_token, _) = self {
            if name_token.name.starts_with('*') {
                let new_name_token = name_token.name.replacen('*', "", 1);

                if let Ok(ty_token) = TypeToken::from_str(&new_name_token, Mutability::Immutable) {
                    return Some(ty_token);
                }
            }
        }

        None
    }


    pub fn is_pointer(&self) -> bool {
        if let TypeToken::Custom(name, _) = self {
            return name.name.starts_with('*');
        }

        false
    }

    /// adds * from type
    pub fn push_pointer(&self) -> Self {
        match self {
            TypeToken::Integer(int, mutability) => TypeToken::Custom(NameToken { name: format!("*{}", int) }, mutability.clone()),
            TypeToken::Float(float, mutability) => TypeToken::Custom(NameToken { name: format!("*{}", float) }, mutability.clone()),
            TypeToken::Bool(mutability) => TypeToken::Custom(NameToken { name: "*bool".to_string() }, mutability.clone()),
            TypeToken::Void => TypeToken::Custom(NameToken { name: format!("*{}", TypeToken::Void) }, Mutability::Immutable),
            TypeToken::Array(array_type, _, mutability) => TypeToken::Custom(NameToken { name: format!("*{}", array_type)}, mutability.clone()),
            TypeToken::Custom(custom, mutability) => TypeToken::Custom(NameToken { name: format!("*{}", custom) }, mutability.clone()),
        }
    }

    pub fn implicit_cast_to(&self, assignable_token: &mut AssignableToken, desired_type: &TypeToken, code_line: &CodeLine) -> Result<Option<TypeToken>, InferTypeError> {
        match (self, desired_type) {
            (TypeToken::Integer(_, _), TypeToken::Integer(desired, _)) => {
                if let AssignableToken::IntegerToken(integer_token) = assignable_token {
                    match desired {
                        Integer::I8 if Self::in_range(-127_i8, 127_i8, Integer::from_number_str(&integer_token.value)) => {
                            integer_token.ty = desired.clone();
                            return Ok(Some(desired_type.clone()))
                        },
                        Integer::U8 => if Self::in_range(0_u8, 255_u8, Integer::from_number_str(&integer_token.value)) {
                            integer_token.ty = desired.clone();
                            return Ok(Some(desired_type.clone()))
                        },
                        Integer::I16 => if Self::in_range(-32768_i16, 32767_i16, Integer::from_number_str(&integer_token.value)) {
                            integer_token.ty = desired.clone();
                            return Ok(Some(desired_type.clone()))
                        },
                        Integer::U16 => if Self::in_range(0_u16, 65535_u16, Integer::from_number_str(&integer_token.value)) {
                            integer_token.ty = desired.clone();
                            return Ok(Some(desired_type.clone()))
                        },
                        Integer::I32 => if Self::in_range(-2_147_483_648_i32, 2_147_483_647_i32, Integer::from_number_str(&integer_token.value)) {
                            integer_token.ty = desired.clone();
                            return Ok(Some(desired_type.clone()))
                        },
                        Integer::U32 => if Self::in_range(0_u32, 4_294_967_295_u32, Integer::from_number_str(&integer_token.value)) {
                            integer_token.ty = desired.clone();
                            return Ok(Some(desired_type.clone()))
                        },
                        Integer::I64 => if Self::in_range(-9_223_372_036_854_775_808_i64, 9_223_372_036_854_775_807_i64, Integer::from_number_str(&integer_token.value)) {
                            integer_token.ty = desired.clone();
                            return Ok(Some(desired_type.clone()))
                        },
                        Integer::U64 => if Self::in_range(0_u64, 18_446_744_073_709_551_615_u64, Integer::from_number_str(&integer_token.value)) {
                            integer_token.ty = desired.clone();
                            return Ok(Some(desired_type.clone()))
                        },
                        _ => return Err(InferTypeError::IntegerTooSmall { ty: desired_type.clone() , literal: integer_token.value.to_string(), code_line: code_line.clone() })
                    }
                }
            }
            (TypeToken::Float(_, _), TypeToken::Float(desired, _)) => {
                if let AssignableToken::FloatToken(float_token) = assignable_token {
                    return match desired {
                        Float::Float32 if Self::in_range(-3.40282347e+38, 3.40282347e+38, Ok(float_token.value)) => {
                            float_token.ty = desired.clone();
                            Ok(Some(desired_type.clone()))
                        },
                        Float::Float64 if Self::in_range(-1.797_693_134_862_315_7e308, 1.797_693_134_862_315_7e308, Ok(float_token.value)) => {
                            float_token.ty = desired.clone();
                            Ok(Some(desired_type.clone()))
                        },
                        _ => Err(InferTypeError::FloatTooSmall { ty: desired_type.clone(), float: float_token.value, code_line: code_line.clone() })
                    }
                }
            },
            (TypeToken::Array(array_type, size, mutability), TypeToken::Array(_desired_inner_type, desired_size, _)) => {
                //todo: check desired_inner_type with actual array_type
                if let AssignableToken::ArrayToken(_) = assignable_token {
                    if size != desired_size {
                        return Err(InferTypeError::MismatchedTypes {
                            expected: desired_type.clone(),
                            actual: TypeToken::Array(array_type.clone(), *size, mutability.clone()),
                            code_line: Default::default(),
                        });
                    }

                    return Ok(Some(desired_type.clone()));
                }
            },
            _ => { }
        }

        Ok(None)
    }

    fn in_range<T: PartialEq<P>, P: PartialOrd<T>>(min: T, max: T, value: Result<P, InferTypeError>) -> bool {
        if let Ok(value) = value {
            value >= min && value <= max
        } else {
            false
        }
    }

    pub fn byte_size(&self) -> usize {
        match self {
            TypeToken::Integer(int, _) => int.byte_size(),
            TypeToken::Float(float, _) => float.byte_size(),
            TypeToken::Array(array_type, _, _) => array_type.byte_size(),
            TypeToken::Bool(_) => 1,
            TypeToken::Void => 0,
            TypeToken::Custom(_, _) => 8, // todo: calculate custom data types recursively
        }
    }
}
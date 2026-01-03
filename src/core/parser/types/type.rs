use crate::core::code_generator::abstract_syntax_tree_nodes::assignables::equation_parser::operator::{AssemblerOperation, OperatorToASM};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::operator::Operator;
use crate::core::model::abstract_syntax_tree_nodes::identifier::{Identifier};
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::types::float::FloatType;
use crate::core::model::types::integer::{IntegerAST, IntegerType};
use crate::core::model::types::mutability::Mutability;
use crate::core::model::types::ty::Type;
use crate::core::parser::types::boolean::Boolean;
use crate::core::parser::types::cast_to::CastTo;
use crate::pattern;
use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub mod common {
    use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
    use crate::core::model::types::mutability::Mutability;
    use crate::core::parser::types::r#type::Type;

    #[allow(unused)]
    pub fn string() -> Type { Type::Custom(Identifier { name: "*string".to_string() }, Mutability::Immutable)}
}

#[derive(Debug)]
pub enum InferTypeError {
    TypesNotCalculable(Type, Operator, Type, FilePosition),
    UnresolvedReference(String, FilePosition),
    MultipleTypesInArray{expected: Type, unexpected_type: Type, unexpected_type_index: usize, file_position: FilePosition},
    IllegalDereference(Assignable, Type, FilePosition),
    IllegalArrayTypeLookup(Type, FilePosition),
    IllegalType(String, FilePosition),
    IllegalIndexOperation(Type, FilePosition),
    NoTypePresent(LValue, FilePosition),
    DefineNotAllowed(Variable<'=', ';'>, FilePosition),
    IntegerTooSmall { ty: Type, literal: String , file_position: FilePosition },
    FloatTooSmall { ty: Type, float: f64, file_position: FilePosition },
    MethodCallArgumentAmountMismatch { expected: usize, actual: usize, file_position: FilePosition },
    MethodCallArgumentTypeMismatch { info: Box<MethodCallArgumentTypeMismatch> },
    MethodReturnArgumentTypeMismatch { expected: Type, actual: Type, file_position: FilePosition },
    MethodReturnSignatureMismatch { expected: Type, method_name: String, file_position: FilePosition, cause: MethodCallSignatureMismatchCause },
    MethodCallSignatureMismatch { signatures: Vec<Vec<Type>>, method_name: LValue, file_position: FilePosition, provided: Vec<Type> },
    NameCollision(String, FilePosition),
    MismatchedTypes { expected: Type, actual: Type, file_position: FilePosition },
}

#[derive(Debug)]
pub enum MethodCallSignatureMismatchCause {
    ReturnMismatch,
    IfCondition,
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
            MethodCallSignatureMismatchCause::ReturnMismatch => "",
            MethodCallSignatureMismatchCause::IfCondition => "Every branch of an if statement must end with a return statement",
        })
    }
}

#[derive(Debug)]
pub struct MethodCallArgumentTypeMismatch {
    pub expected: Type,
    pub actual: Type,
    pub nth_parameter: usize,
    pub file_position: FilePosition,
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (equal_types, m1, m2) = match (self, other) {
            (Type::Float(f1, m1), Type::Float(f2, m2)) if f1 == f2 => (true, m1, m2),
            (Type::Integer(i1, m1), Type::Integer(i2, m2)) if i1 == i2 => (true, m1, m2),
            (Type::Bool(m1), Type::Bool(m2)) => (true, m1, m2),
            (Type::Array(t1, s1, m1), Type::Array(t2, s2, m2)) if t1.partial_cmp(t2)?.is_le() && s1 == s2 => (true, m1, m2),
            (Type::Custom(n1, m1), Type::Custom(n2, m2)) if n1 == n2 => (true, m1, m2),
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

impl OperatorToASM for Type {
    fn operation_to_asm<T: Display>(&self, operator: &Operator, registers: &[T], stack: &mut Stack, meta: &mut MetaInfo) -> Result<AssemblerOperation, ASMGenerateError> {

        match self {
            Type::Integer(t, _) => t.operation_to_asm(operator, registers, stack, meta),
            Type::Float(t, _) => t.operation_to_asm(operator, registers, stack, meta),
            Type::Bool(_) => Boolean::True.operation_to_asm(operator, registers, stack, meta),
            Type::Void => Err(ASMGenerateError::InternalError("Void cannot be operated on".to_string(), meta.file_position.clone())),
            Type::Statement => Err(ASMGenerateError::InternalError("Statements cannot be operated on".to_string(), meta.file_position.clone())),
            Type::Array(_, _, _) | Type::Custom(_, _) => todo!(),
        }
    }
}

impl Error for InferTypeError {}

impl Display for InferTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InferTypeError::TypesNotCalculable(a, o, b, file_position) => write!(f, "Line: {}: \tCannot {} between types {} and {}", file_position, o, a, b),
            InferTypeError::UnresolvedReference(s, file_position) => write!(f, "Line: {}: \tUnresolved reference: {s}", file_position),
            InferTypeError::MismatchedTypes { expected, actual, file_position } => write!(f, "Line: {}: \tMismatched types: Expected `{expected}` but found `{actual}`", file_position),
            InferTypeError::NameCollision(name, file_position) => write!(f, "Line: {}: \tTwo symbols share the same name: `{name}`", file_position),
            InferTypeError::MethodCallArgumentAmountMismatch { expected, actual, file_position } => write!(f, "Line: {:?}: \tThe method expects {} parameter, but {} are provided", file_position, expected, actual),
            InferTypeError::MethodCallArgumentTypeMismatch { info } => write!(f, "Line: {}: \t The {}. argument must be of type: `{}` but `{}` is provided", info.file_position, info.nth_parameter, info.expected, info.actual),
            InferTypeError::MethodReturnArgumentTypeMismatch { expected, actual, file_position } => write!(f, "Line: {}: \t The return type is: `{}` but `{}` is provided", file_position, expected, actual),
            InferTypeError::MethodReturnSignatureMismatch { expected, method_name, file_position, cause } => write!(f, "Line: {file_position}: \tA return statement with type: `{expected}` is expected for the method: {method_name}\n\t{cause}"),
            InferTypeError::MethodCallSignatureMismatch { signatures, method_name, file_position, provided } => {
                let provided_arguments = provided.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(", ");
                let mut signatures = signatures
                    .iter()
                    .map(|v| format!("\t - ({})", v.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(", ")))
                    .collect::<Vec<String>>();
                signatures.sort();
                let signatures = signatures.join(",\n");

                write!(f, "Line: {}: Arguments `({})` to the function `{}` are incorrect: Possible signatures are:\n{}", file_position, provided_arguments, method_name, signatures)
            }
            InferTypeError::MultipleTypesInArray { expected, unexpected_type, unexpected_type_index, file_position } => {
                write!(f, "Line: {}: Expected `{expected}` in array but found `{unexpected_type}` at position: `{unexpected_type_index}`", file_position)
            }
            InferTypeError::NoTypePresent(name, file_position) => write!(f, "Line: {}\tType not inferred: `{name}`", file_position),
            InferTypeError::IllegalDereference(assignable, ty, file_position) => write!(f, "Line: {}\tType `{ty}` cannot be dereferenced: {assignable}", file_position),
            InferTypeError::IllegalArrayTypeLookup(ty, file_position) => write!(f, "Line: {}\tType `{ty}` cannot be indexed", file_position),
            InferTypeError::IllegalIndexOperation(ty, file_position) => write!(f, "Line: {}\tType `{ty}` cannot be used as an index", file_position),
            InferTypeError::IntegerTooSmall { ty, literal: integer, file_position } => write!(f, "Line: {}\t`{integer}` doesn't fit into the type `{ty}`", file_position),
            InferTypeError::FloatTooSmall { ty, float, file_position } =>
                write!(f, "Line: {}\t`{float}` doesn't fit into the type `{ty}`", file_position),
            InferTypeError::DefineNotAllowed(variable, code_line) => {
                write!(f, "Line: {}\t`{}` is not allowed to be defined here", code_line, variable.l_value)
            }
            InferTypeError::IllegalType(illegal_type, file_position) => {
                write!(f, "Line: {}\t`{illegal_type}` is not a valid type", file_position)
            }
        }
    }
}

impl Parse for Type {
    fn parse(tokens: &[TokenWithSpan], options: ParseOptions) -> Result<ParseResult<Self>, crate::core::lexer::error::Error> where Self: Sized, Self: Default {
        // mutable
        if options.can_be_mutable { // prevent parse with mut mut type
            if let [TokenWithSpan { token: Token::Mut, ..}, ..] = tokens {
                let mut ty = Self::parse(&tokens[1..], ParseOptions::builder().with_can_be_mutable(false).build())?;
                ty.result.set_mutability(Mutability::Mutable);
                return Ok(ParseResult {
                    result: ty.result,
                    consumed: ty.consumed + 1,
                });
            };
        }


        // array
        if let Some(MatchResult::Parse(inner_type)) = pattern!(tokens, SquareBracketOpen, @ parse Type, Comma,) {
            if let Some(MatchResult::Parse(size_assignable)) = pattern!(&tokens[inner_type.consumed + 2..], @ parse Assignable, SquareBracketClose) {
                return if let ParseResult { result: Assignable::Integer(IntegerAST { value, .. }), .. } = size_assignable {
                    let array_size = value.parse::<usize>().map_err(|_| crate::core::lexer::error::Error::ExpectedToken(tokens[inner_type.consumed + 2].token.clone()))?;
                    Ok(ParseResult {
                        result: Type::Array(Box::new(inner_type.result), array_size, Mutability::Immutable),
                        consumed: inner_type.consumed + size_assignable.consumed + 3,
                    })
                } else {
                    Err(crate::core::lexer::error::Error::UnexpectedToken(tokens[inner_type.consumed + 2].clone()))
                }
            }
        }

        // base case
        if let [TokenWithSpan { token: Token::Literal(ty), .. }, ..] = tokens {
            if let Ok(parsed_type) = Type::from_str(ty, Mutability::Immutable) {
                return Ok(ParseResult {
                    result: parsed_type,
                    consumed: 1,
                });
            }
        }

        // reference
        if let Some(MatchResult::Parse(mut parsed_type)) = pattern!(tokens, Ampersand, @ parse Type,) {
            return if let Some(p) = parsed_type.result.pop_pointer() {
                parsed_type.result = p;
                Ok(ParseResult {
                    result: parsed_type.result,
                    consumed: parsed_type.consumed + 1,
                })
            } else {
                Err(crate::core::lexer::error::Error::UnexpectedToken(tokens[0].clone()))
            }
        }

        // pointer
        if let Some(MatchResult::Parse(mut parsed_type)) = pattern!(tokens, Multiply, @ parse Type,) {
            parsed_type.result = parsed_type.result.push_pointer();
            return Ok(ParseResult {
                result: parsed_type.result,
                consumed: parsed_type.consumed + 1,
            });
        }


        Err(crate::core::lexer::error::Error::UnexpectedToken(tokens[0].clone()))
    }
}

impl Type {
    pub fn mutable(&self) -> bool {
        match self {
            Type::Integer(_, a) |
            Type::Float(_, a) |
            Type::Bool(a) |
            Type::Array(_, _, a) |
            Type::Custom(_, a) => match a {
                Mutability::Mutable => true,
                Mutability::Immutable => false
            }
            Type::Void => false,
            Type::Statement => false
        }
    }

    pub fn from_str(s: &str, mutability: Mutability) -> Result<Self, Box<InferTypeError>> {
        Ok(match s {
            "bool" => Type::Bool(Mutability::Immutable),
            "void" => Type::Void,
            custom => {
                if let Ok(int) = IntegerType::from_str(custom) {
                    return Ok(Type::Integer(int, mutability));
                }

                if let Ok(float) = FloatType::from_str(custom) {
                    return Ok(Type::Float(float, mutability));
                }

                // if list of tokens contains the custom string, its an invalid type
                if let Some(a) = Token::iter().find(|a| a.matches(custom)) {
                    if !matches!(a.token, Token::Literal(_)) {
                        return Err(Box::new(InferTypeError::IllegalType(String::from(custom), FilePosition::default())));
                    }
                }

                if !lazy_regex::regex_is_match!(r"^[\*&]*[a-zA-Z_$][a-zA-Z_$0-9]*[\*&]*$", s) {
                    return Err(Box::new(InferTypeError::IllegalType(String::from(custom), FilePosition::default())));
                }

                Type::Custom(Identifier { name: custom.to_string() }, mutability)
            }
        })
    }

    pub fn set_mutability(&mut self, m: Mutability) {
        match self {
            Type::Integer(_, mutability) => *mutability = m,
            Type::Float(_, mutability) => *mutability = m,
            Type::Bool(mutability) => *mutability = m,
            Type::Void => {},
            Type::Statement => {}
            Type::Array(_, _, mutability) => *mutability = m,
            Type::Custom(_, mutability) => *mutability = m,
        }
    }

    pub fn cast_to(&self, to: &Type) -> CastTo {
        CastTo {
            from: self.clone(),
            to: to.clone(),
        }
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Type::Float(_, _))
    }

    // takes the element array type and returns the type of the array
    pub fn pop_array(&self) -> Option<Type> {
        if let Type::Array(array_type, _, _) = self {
            return Some(*array_type.clone());
        }

        None
    }
    /// removes * from type
    pub fn pop_pointer(&self) -> Option<Type> {
        if let Type::Custom(identifier, _) = self {
            if identifier.name.starts_with('*') {
                let new_identifier_node = identifier.name.replacen('*', "", 1);

                if let Ok(ty) = Type::from_str(&new_identifier_node, Mutability::Immutable) {
                    return Some(ty);
                }
            }
        }

        None
    }


    pub fn is_pointer(&self) -> bool {
        if let Type::Custom(name, _) = self {
            return name.name.starts_with('*');
        }

        false
    }

    /// adds * from type
    pub fn push_pointer(&self) -> Self {
        match self {
            Type::Integer(int, mutability) => Type::Custom(Identifier { name: format!("*{}", int) }, mutability.clone()),
            Type::Float(float, mutability) => Type::Custom(Identifier { name: format!("*{}", float) }, mutability.clone()),
            Type::Bool(mutability) => Type::Custom(Identifier { name: "*bool".to_string() }, mutability.clone()),
            Type::Void => Type::Custom(Identifier { name: format!("*{}", Type::Void) }, Mutability::Immutable),
            Type::Statement => Type::Custom(Identifier { name: format!("*{}", Type::Statement) }, Mutability::Immutable),
            Type::Array(array_type, _, mutability) => Type::Custom(Identifier { name: format!("*{}", array_type)}, mutability.clone()),
            Type::Custom(custom, mutability) => Type::Custom(Identifier { name: format!("*{}", custom) }, mutability.clone()),
        }
    }

    pub fn implicit_cast_to(&self, assignable: &mut Assignable, desired_type: &Type, file_position: &FilePosition) -> Result<Option<Type>, Box<InferTypeError>> {
        match (self, desired_type) {
            (Type::Integer(_, _), Type::Integer(desired, _)) => {
                if let Assignable::Integer(integer) = assignable {
                    match desired {
                        IntegerType::I8 if Self::in_range(-127_i8, 127_i8, IntegerType::from_number_str(&integer.value, file_position)) => {
                            integer.ty = desired.clone();
                            return Ok(Some(desired_type.clone()))
                        },
                        IntegerType::U8 => if Self::in_range(0_u8, 255_u8, IntegerType::from_number_str(&integer.value, file_position)) {
                            integer.ty = desired.clone();
                            return Ok(Some(desired_type.clone()))
                        },
                        IntegerType::I16 => if Self::in_range(-32768_i16, 32767_i16, IntegerType::from_number_str(&integer.value, file_position)) {
                            integer.ty = desired.clone();
                            return Ok(Some(desired_type.clone()))
                        },
                        IntegerType::U16 => if Self::in_range(0_u16, 65535_u16, IntegerType::from_number_str(&integer.value, file_position)) {
                            integer.ty = desired.clone();
                            return Ok(Some(desired_type.clone()))
                        },
                        IntegerType::I32 => if Self::in_range(-2_147_483_648_i32, 2_147_483_647_i32, IntegerType::from_number_str(&integer.value, file_position)) {
                            integer.ty = desired.clone();
                            return Ok(Some(desired_type.clone()))
                        },
                        IntegerType::U32 => if Self::in_range(0_u32, 4_294_967_295_u32, IntegerType::from_number_str(&integer.value, file_position)) {
                            integer.ty = desired.clone();
                            return Ok(Some(desired_type.clone()))
                        },
                        IntegerType::I64 => if Self::in_range(-9_223_372_036_854_775_808_i64, 9_223_372_036_854_775_807_i64, IntegerType::from_number_str(&integer.value, file_position)) {
                            integer.ty = desired.clone();
                            return Ok(Some(desired_type.clone()))
                        },
                        IntegerType::U64 => if Self::in_range(0_u64, 18_446_744_073_709_551_615_u64, IntegerType::from_number_str(&integer.value, file_position)) {
                            integer.ty = desired.clone();
                            return Ok(Some(desired_type.clone()))
                        },
                        _ => return Err(Box::new(InferTypeError::IntegerTooSmall { ty: desired_type.clone() , literal: integer.value.to_string(), file_position: file_position.clone() }))
                    }
                }
            }
            (Type::Float(_, _), Type::Float(desired, _)) => {
                if let Assignable::Float(float) = assignable {
                    return match desired {
                        FloatType::Float32 if Self::in_range(-3.40282347e+38, 3.40282347e+38, Ok(float.value)) => {
                            float.ty = desired.clone();
                            Ok(Some(desired_type.clone()))
                        },
                        FloatType::Float64 if Self::in_range(-1.797_693_134_862_315_7e308, 1.797_693_134_862_315_7e308, Ok(float.value)) => {
                            float.ty = desired.clone();
                            Ok(Some(desired_type.clone()))
                        },
                        _ => Err(Box::new(InferTypeError::FloatTooSmall { ty: desired_type.clone(), float: float.value, file_position: file_position.clone() }))
                    }
                }
            },
            (Type::Array(array_type, size, mutability), Type::Array(_desired_inner_type, desired_size, _)) => {
                //todo: check desired_inner_type with actual array_type
                if let Assignable::Array(_) = assignable {
                    if size != desired_size {
                        return Err(Box::new(InferTypeError::MismatchedTypes {
                            expected: desired_type.clone(),
                            actual: Type::Array(array_type.clone(), *size, mutability.clone()),
                            file_position: Default::default(),
                        }));
                    }

                    return Ok(Some(desired_type.clone()));
                }
            },
            _ => { }
        }

        Ok(None)
    }

    fn in_range<T: PartialEq<P>, P: PartialOrd<T>>(min: T, max: T, value: Result<P, Box<InferTypeError>>) -> bool {
        if let Ok(value) = value {
            value >= min && value <= max
        } else {
            false
        }
    }

    pub fn byte_size(&self) -> usize {
        match self {
            Type::Integer(int, _) => int.byte_size(),
            Type::Float(float, _) => float.byte_size(),
            Type::Array(array_type, _, _) => array_type.byte_size(),
            Type::Bool(_) => 1,
            Type::Void => 0,
            Type::Statement => 0,
            Type::Custom(_, _) => 8
        }
    }

    pub fn byte_size_with_meta(&self, meta: &MetaInfo) -> usize {
        match self {
            Type::Integer(int, _) => int.byte_size(),
            Type::Float(float, _) => float.byte_size(),
            Type::Array(array_type, _, _) => array_type.byte_size(),
            Type::Bool(_) => 1,
            Type::Void => 0,
            Type::Statement => 0,
            Type::Custom(identifier_type, _) => {
                if identifier_type.name.starts_with('*') {
                    return 8; // pointer size
                }

                if let Some(struct_def) = meta.static_type_information.custom_defined_types.get(&self).cloned() {
                    struct_def.byte_size(meta)
                } else {
                    eprintln!("Warning: Cannot calculate byte size of custom type: {}", identifier_type);
                    8
                }
            }
        }
    }
}
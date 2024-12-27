use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;
use std::str::FromStr;

use anyhow::Context;

use crate::core::code_generator::{ASMGenerateError, MetaInfo, register_destination, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_result::{ASMOptions, ASMResult, ASMResultError, ASMResultVariance, InterimResultOption};
use crate::core::code_generator::generator::{Stack, StackLocation};
use crate::core::code_generator::registers::{Bit64, ByteSize, GeneralPurposeRegister};
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::errors::EmptyIteratorErr;
use crate::core::lexer::scope::PatternNotMatchedError;
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::core::lexer::tokens::name_token::{NameToken, NameTokenErr};
use crate::core::lexer::TryParse;
use crate::core::lexer::types::type_token::{InferTypeError, TypeToken};
use crate::core::type_checker::{InferType, StaticTypeCheck};
use crate::core::type_checker::static_type_checker::StaticTypeCheckError;

/// Token for a variable. Pattern is defined as: name <Assignment> assignment <Separator>
/// # Examples
/// - `name = assignment;`
/// - `name: assignment,`
#[derive(Debug, PartialEq, Clone)]
pub struct VariableToken<const ASSIGNMENT: char, const SEPARATOR: char> {
    pub name_token: NameToken,
    // flag defining if the variable is mutable or not
    pub mutability: bool,
    /// type of the variable. It's None, when the type is unknown
    pub ty: Option<TypeToken>,
    /// flag defining if the variable is a new definition or a re-assignment
    pub define: bool,
    pub assignable: AssignableToken,
    pub code_line: CodeLine,
}


impl<const ASSIGNMENT: char, const SEPARATOR: char> InferType for VariableToken<ASSIGNMENT, SEPARATOR> {
    fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<(), InferTypeError> {
        if type_context.methods.iter().filter(|a| a.name == self.name_token).count() > 0 {
            return Err(InferTypeError::NameCollision(self.name_token.name.clone(), self.code_line.clone()));
        }

        if !self.define {
            return Ok(());
        }

        match &self.ty {
            // validity check. is the assignment really the type the programmer used
            // example: let a: i32 = "Hallo"; is not valid since you're assigning a string to an integer

            // if type is present. check, if the type matches the assignment
            // else infer the type with a context
            Some(ty) => {
                let inferred_type = self.assignable.infer_type_with_context(type_context, &self.code_line)?;

                if ty != &inferred_type {
                    // let a: i64 = 5; instead of let a: i32 = 5;
                    if let Some(implicit_cast) = inferred_type.implicit_cast_to(&mut self.assignable, ty, &self.code_line)? {
                        self.ty = Some(implicit_cast);
                    } else {
                        return Err(InferTypeError::MismatchedTypes { expected: ty.clone(), actual: inferred_type.clone(), code_line: self.code_line.clone() });
                    }
                }

                Ok(())
            }
            None => {
                let ty = self.infer_with_context(type_context, &self.code_line)?;
                self.ty = Some(ty.clone());
                type_context.push(VariableToken {
                    name_token: self.name_token.clone(),
                    ty: Some(ty.clone()),
                    define: self.define,
                    assignable: self.assignable.clone(),
                    mutability: self.mutability,
                    code_line: self.code_line.clone(),
                });

                Ok(())
            }
        }
    }
}

impl StaticTypeCheck for VariableToken<'=', ';'> {
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        if self.define {
            if let AssignableToken::ArrayToken(array_token) = &self.assignable {
                // check if all types are equal, where the first type is the expected type
                let all_types = array_token.values
                    .iter()
                    .map(|a| a.infer_type_with_context(type_context, &self.code_line.clone()))
                    .collect::<Vec<Result<TypeToken, InferTypeError>>>();

                if !all_types.is_empty() {
                    let first_type = &all_types[0];
                    if let Ok(first_type) = first_type {
                        for (index, current_type) in all_types.iter().enumerate() {
                            if let Ok(current_type) = current_type {
                                if current_type != first_type {
                                    return Err(StaticTypeCheckError::InferredError(InferTypeError::MultipleTypesInArray {
                                        expected: first_type.clone(),
                                        unexpected_type: current_type.clone(),
                                        unexpected_type_index: index,
                                        code_line: Default::default(),
                                    }))
                                }
                            }
                        }
                    }
                }
            }

            if self.ty.is_some() {
                type_context.context.push(self.clone());
                return Ok(());
            }
        }

        if !self.define {
            if let Some(found_variable) = type_context.iter().rfind(|v| v.name_token == self.name_token) {
                let inferred_type = self.assignable.infer_type_with_context(type_context, &self.code_line)?;

                if let Some(ty) = &found_variable.ty {
                    if ty != &inferred_type {
                        return Err(InferTypeError::MismatchedTypes { expected: ty.clone(), actual: inferred_type.clone(), code_line: self.code_line.clone() }.into());
                    }

                    if !found_variable.mutability {
                        return Err(StaticTypeCheckError::ImmutabilityViolated {
                            name: self.name_token.clone(),
                            code_line: self.code_line.clone(),
                        });
                    }
                } else {
                    return Err(StaticTypeCheckError::NoTypePresent { name: self.name_token.clone(), code_line: self.code_line.clone() });
                }
            } else {
                return Err(StaticTypeCheckError::UnresolvedReference { name: self.name_token.clone(), code_line: self.code_line.clone() });
            }
        }

        Ok(())
    }
}

impl<const ASSIGNMENT: char, const SEPARATOR: char> Display for VariableToken<ASSIGNMENT, SEPARATOR> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let t = self.ty.as_ref().map_or(String::new(), |ty| format!(": {ty}"));

        write!(
            f,
            "{}{}{} {} {}",
            if self.define { "let " } else { "" },      // definition
            self.name_token,                            // name
            &t,                                         // type
            ASSIGNMENT,                                 // assignment token
            self.assignable                             // assignment
        )
    }
}

#[derive(Debug)]
pub enum ParseVariableTokenErr {
    PatternNotMatched { target_value: String },
    NameTokenErr(NameTokenErr),
    AssignableTokenErr(AssignableTokenErr),
    InferType(InferTypeError),
    EmptyIterator(EmptyIteratorErr),
}

impl PatternNotMatchedError for ParseVariableTokenErr {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, ParseVariableTokenErr::PatternNotMatched {..})
    }
}

impl Error for ParseVariableTokenErr {}

impl From<InferTypeError> for ParseVariableTokenErr {
    fn from(value: InferTypeError) -> Self {
        ParseVariableTokenErr::InferType(value)
    }
}

impl From<NameTokenErr> for ParseVariableTokenErr {
    fn from(a: NameTokenErr) -> Self { ParseVariableTokenErr::NameTokenErr(a) }
}

impl From<anyhow::Error> for ParseVariableTokenErr {
    fn from(value: anyhow::Error) -> Self {
        let mut buffer = String::new();
        buffer += &value.to_string();
        buffer += "\n";

        if let Some(e) = value.downcast_ref::<AssignableTokenErr>() {
            buffer += &e.to_string();
        }
        ParseVariableTokenErr::PatternNotMatched { target_value: buffer }
    }
}

impl From<AssignableTokenErr> for ParseVariableTokenErr {
    fn from(a: AssignableTokenErr) -> Self { ParseVariableTokenErr::AssignableTokenErr(a) }
}

impl Display for ParseVariableTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ParseVariableTokenErr::PatternNotMatched { target_value } => format!("`{target_value}`\n\tThe pattern for a variable is defined as: name = assignment;"),
            ParseVariableTokenErr::NameTokenErr(a) => a.to_string(),
            ParseVariableTokenErr::AssignableTokenErr(a) => a.to_string(),
            ParseVariableTokenErr::EmptyIterator(e) => e.to_string(),
            ParseVariableTokenErr::InferType(err) => format!("{err}")
        })
    }
}

impl<const ASSIGNMENT: char, const SEPARATOR: char> ToASM for VariableToken<ASSIGNMENT, SEPARATOR> {
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        let mut target = String::new();
        target += &ASMBuilder::ident(&ASMBuilder::comment_line(&format!("{}", self)));


        let general_purpose_options  = match &self.assignable {
            AssignableToken::ArrayToken(arr_token) if arr_token.values.len() > 1 => { None },
            _ => Some(InterimResultOption {
                general_purpose_register: GeneralPurposeRegister::iter_from_byte_size(self.assignable.byte_size(meta))?.current().clone(),
            })
        };

        let result = self.assignable.to_asm(stack, meta, general_purpose_options)?;

        let destination = if self.define {
            let byte_size = self.assignable.byte_size(meta);

            let elements = match &self.assignable {
                AssignableToken::ArrayToken(arr_token) if arr_token.values.len() > 1 => arr_token.values.len(),
                _ => 1
            };

            stack.variables.push(StackLocation { position: stack.stack_position, size: byte_size, name: self.name_token.clone(), elements });
            stack.stack_position += byte_size;

            let offset = stack.stack_position;
            if !matches!(result, ASMResult::Multiline(_)) {
                format!("{} [rbp - {}]", register_destination::word_from_byte_size(byte_size), offset)
            } else {
                String::new()
            }
        } else {
            match self.name_token.to_asm(stack, meta, options)? {
                ASMResult::Inline(r) => r,
                ASMResult::MultilineResulted(t, r) => {
                    target += &t;
                    r.to_string()
                }
                ASMResult::Multiline(_) => {
                    return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                        expected: vec![ASMResultVariance::MultilineResulted, ASMResultVariance::Inline],
                        actual: ASMResultVariance::Multiline,
                        token: "variable token".to_string(),
                    }))
                }
            }
        };

        match result {
            ASMResult::Inline(source) => {
                if self.assignable.is_stack_look_up(stack, meta) {
                    let destination_register = GeneralPurposeRegister::iter_from_byte_size(self.assignable.byte_size(meta))?.current();
                    target += &ASMBuilder::mov_x_ident_line(&destination_register, source, Some(destination_register.size() as usize));
                    target += &ASMBuilder::mov_ident_line(destination, &destination_register);
                } else {
                    target += &ASMBuilder::mov_ident_line(destination, source);
                }
            },
            ASMResult::MultilineResulted(source, mut register) => {
                target += &source;

                if let AssignableToken::ArithmeticEquation(expr) = &self.assignable {
                    let final_type = expr.traverse_type(meta).ok_or(ASMGenerateError::InternalError("Cannot infer type".to_string()))?;
                    let r = GeneralPurposeRegister::Bit64(Bit64::Rax).to_size_register(&ByteSize::try_from(final_type.byte_size())?);

                    if let GeneralPurposeRegister::Memory(memory) = &register {
                        target += &ASMBuilder::mov_x_ident_line(&r, memory, None);
                        register = r.clone();
                    }


                    if let TypeToken::Float(s) = final_type {
                        target += &ASMBuilder::mov_x_ident_line(&r, register, Some(s.byte_size()));
                        register = r;
                    }
                }

                target += &ASMBuilder::mov_ident_line(destination, register);
            },
            ASMResult::Multiline(source) => {
                target += &source;
            }
        }

        Ok(ASMResult::Multiline(target))
    }


    fn is_stack_look_up(&self, stack: &mut Stack, meta: &MetaInfo) -> bool {
        self.assignable.is_stack_look_up(stack, meta)
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        self.ty.as_ref().map_or(0, |ty| ty.byte_size())
    }

    fn data_section(&self, stack: &mut Stack, meta: &mut MetaInfo) -> bool {
        self.assignable.data_section(stack, meta)
    }
}

impl<const ASSIGNMENT: char, const SEPARATOR: char> TryParse for VariableToken<ASSIGNMENT, SEPARATOR> {
    type Output = VariableToken<ASSIGNMENT, SEPARATOR>;
    type Err = ParseVariableTokenErr;

    fn try_parse(code_lines_iterator: &mut Peekable<Iter<CodeLine>>) -> anyhow::Result<Self::Output, Self::Err> {
        let code_line = *code_lines_iterator.peek().ok_or(ParseVariableTokenErr::EmptyIterator(EmptyIteratorErr))?;
        VariableToken::try_parse(code_line)
    }
}

impl<const ASSIGNMENT: char, const SEPARATOR: char> VariableToken<ASSIGNMENT, SEPARATOR> {
    pub fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self, ParseVariableTokenErr> {
        let split_alloc = code_line.split(vec![' ', ';']);
        let split = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();


        let assignment = ASSIGNMENT.to_string();
        let separator = SEPARATOR.to_string();

        let let_used;
        let mut_used;

        let final_variable_name: &str;
        let assignable: AssignableToken;
        let type_token: Option<TypeToken>;

        match &split[..] {
            // [let] [mut] name[: i32] = 5;
            ["let", name, assignment_token, middle @ .., separator_token] if assignment_token == &assignment && separator_token == &separator => {
                final_variable_name = name;
                assignable = AssignableToken::from_str(middle.join(" ").as_str()).context(code_line.line.clone())?;
                // type token is not specified by the programmer, so it must be inferred
                type_token = assignable.infer_type(code_line);

                let_used = true;
                mut_used = false;
            }
            ["let", name, ":", type_str, assignment_token, middle @ .., separator_token] if assignment_token == &assignment && separator_token == &separator => {
                final_variable_name = name;
                assignable = AssignableToken::from_str(middle.join(" ").as_str()).context(code_line.line.clone())?;
                type_token = Some(TypeToken::from_str(type_str)?);

                let_used = true;
                mut_used = false;
            },
            ["let", name, ":", "[", type_str, ",", type_size, "]", assignment_token, middle @ .., separator_token] if assignment_token == &assignment && separator_token == &separator => {
                final_variable_name = name;
                assignable = AssignableToken::from_str(middle.join(" ").as_str()).context(code_line.line.clone())?;
                type_token = Some(TypeToken::from_str(&format!("[ {} , {} ]", type_str, type_size))?);

                let_used = true;
                mut_used = false;
            }
            ["let", "mut", name, assignment_token, middle @ .., separator_token] if assignment_token == &assignment && separator_token == &separator => {
                final_variable_name = name;
                assignable = AssignableToken::from_str(middle.join(" ").as_str()).context(code_line.line.clone())?;
                // type token is not specified by the programmer, so it must be inferred
                type_token = assignable.infer_type(code_line);

                let_used = true;
                mut_used = true;
            }
            ["let", "mut", name, ":", type_str, assignment_token, middle @ .., separator_token] if assignment_token == &assignment && separator_token == &separator => {
                final_variable_name = name;
                assignable = AssignableToken::from_str(middle.join(" ").as_str()).context(code_line.line.clone())?;
                type_token = Some(TypeToken::from_str(type_str)?);

                let_used = true;
                mut_used = true;
            },
            ["let", "mut", name, ":", "[", type_str, ",", type_size, "]", assignment_token, middle @ .., separator_token] if assignment_token == &assignment && separator_token == &separator => {
                final_variable_name = name;
                assignable = AssignableToken::from_str(middle.join(" ").as_str()).context(code_line.line.clone())?;
                type_token = Some(TypeToken::from_str(&format!("[ {} , {} ]", type_str, type_size))?);

                let_used = true;
                mut_used = true;
            }
            [name, assignment_token, middle @ .., separator_token] if assignment_token == &assignment && separator_token == &separator => {
                final_variable_name = name;
                assignable = AssignableToken::from_str(middle.join(" ").as_str()).context(code_line.line.clone())?;
                type_token = assignable.infer_type(code_line);

                let_used = false;
                mut_used = false;
            }
            _ => {
                return Err(ParseVariableTokenErr::PatternNotMatched { target_value: code_line.line.to_string() });
            }
        }

        Ok(VariableToken {
            name_token: NameToken::from_str(final_variable_name, false)?,
            mutability: mut_used,
            ty: type_token,
            define: let_used,
            assignable,
            code_line: code_line.clone(),
        })
    }

    pub fn infer_with_context(&self, context: &mut StaticTypeContext, code_line: &CodeLine) -> Result<TypeToken, InferTypeError> {
        match &self.assignable {
            AssignableToken::MethodCallToken(method_call) => {
                if let Some(method_def) = context.methods.iter().find(|method_def| {
                    method_def.name == method_call.name
                }) {
                    return Ok(method_def.return_type.clone());
                }
            }
            AssignableToken::NameToken(variable) => {
                if let Some(v) = context.iter().rfind(|v| {
                    v.name_token == *variable
                }) {
                    return if let Some(ty) = &v.ty {
                        Ok(ty.clone())
                    } else {
                        Err(InferTypeError::NoTypePresent(v.name_token.clone(), self.code_line.clone()))
                    };
                }
            }
            AssignableToken::ArithmeticEquation(expression) => {
                return expression.traverse_type_resulted(context, code_line);
            }
            a => unreachable!("{}", format!("The type {a} should have been inferred or directly parsed. Something went wrong"))
        }

        Err(InferTypeError::UnresolvedReference(self.assignable.to_string(), self.code_line.clone()))
    }
}

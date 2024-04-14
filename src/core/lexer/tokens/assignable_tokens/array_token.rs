use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::core::code_generator::asm_result::{ASMOptions, ASMResult, InterimResultOption};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, register_destination, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::registers::{Bit64, ByteSize, GeneralPurposeRegister};
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::tokens::assignable_token::AssignableToken;
use crate::core::lexer::tokens::assignable_tokens::method_call_token::dyck_language;
use crate::core::lexer::tokens::name_token::NameToken;
use crate::core::lexer::types::type_token::{InferTypeError, TypeToken};

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayToken {
    pub values: Vec<AssignableToken>,
}

#[derive(Debug)]
pub enum ArrayTokenErr {
    UnmatchedRegex,
}

impl Error for ArrayTokenErr { }

impl ArrayToken {
    pub fn infer_type_with_context(&self, context: &StaticTypeContext, code_line: &CodeLine) -> Result<TypeToken, InferTypeError> {
        if self.values.is_empty() {
            return Err(InferTypeError::NoTypePresent(NameToken { name: "Array".to_string() }, code_line.clone()))
        }

        if let Ok(ty) = self.values[0].infer_type_with_context(context, code_line) {
            return Ok(TypeToken::Array(Box::new(ty), self.values.len()));
        }

        return Err(InferTypeError::NoTypePresent(NameToken { name: "Array".to_string() }, code_line.clone()))
    }
}

impl Display for ArrayTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ArrayTokenErr::UnmatchedRegex => "Array must match: [type, size]"
        })
    }
}


impl Display for ArrayToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let a = self.values.iter().map(|a| format!("{}", a)).collect::<Vec<_>>();
        write!(f, "[{}]", a.join(", "))
    }
}

impl FromStr for ArrayToken {
    type Err = ArrayTokenErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !lazy_regex::regex_is_match!(r"^\[.*\]$", s) {
            return Err(ArrayTokenErr::UnmatchedRegex);
        }

        if s.replace(" ", "") == "[]" {
            return Ok(ArrayToken {
                values: vec![]
            })
        }

        if let ["[ ", array_content @ .., "]"] = &s.split_inclusive(' ').collect::<Vec<_>>()[..] {
            let array_elements_str = dyck_language(&array_content.join(" "), [vec!['{', '('], vec![','], vec!['}', ')']])
                .map_err(|_| ArrayTokenErr::UnmatchedRegex)?;

            if array_elements_str.is_empty() {
                return Err(ArrayTokenErr::UnmatchedRegex);
            }

            let mut values = vec![];

            for array_element in &array_elements_str {
                values.push(AssignableToken::from_str(array_element).map_err(|_| ArrayTokenErr::UnmatchedRegex)?);
            }

            return Ok(ArrayToken {
                values,
            })
        }

        Err(ArrayTokenErr::UnmatchedRegex)
    }
}

impl ToASM for ArrayToken {
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, _options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        let mut target = String::new();
        target += &ASMBuilder::ident(&ASMBuilder::comment_line(&format!("{}", self)));

        let mut offset = stack.stack_position;

        for (i, assignable) in self.values.iter().enumerate() {
            let first_register = GeneralPurposeRegister::iter_from_byte_size(assignable.byte_size(meta))?.current();
            let result = assignable.to_asm(stack, meta, Some(InterimResultOption {
                general_purpose_register: first_register.clone(),
            }))?;

            let byte_size = assignable.byte_size(meta);
            let destination = format!("{} [rbp - {}]", register_destination::word_from_byte_size(byte_size), offset);

            match result {
                ASMResult::Inline(source) => {
                    if assignable.is_stack_look_up(stack, meta) {
                        target += &ASMBuilder::mov_x_ident_line(&first_register, source, Some(first_register.size() as usize));
                        target += &ASMBuilder::mov_ident_line(destination, &first_register);
                    } else {
                        target += &ASMBuilder::mov_ident_line(destination, source);
                    }
                }
                ASMResult::MultilineResulted(source, mut register) => {
                    target += &source;

                    if let AssignableToken::ArithmeticEquation(expr) = assignable {
                        let final_type = expr.traverse_type(meta).ok_or(ASMGenerateError::InternalError("Cannot infer type".to_string()))?;
                        let r = GeneralPurposeRegister::Bit64(Bit64::Rax).to_size_register(&ByteSize::try_from(final_type.byte_size())?);

                        if let TypeToken::Float(s) = final_type {
                            target += &ASMBuilder::mov_x_ident_line(&r, register, Some(s.byte_size()));
                            register = r;
                        }
                    }

                    target += &ASMBuilder::mov_ident_line(destination, register);
                }
                ASMResult::Multiline(source) => {
                    target += &source;
                }
            }

            offset += (i + 1) * byte_size;
        }

        Ok(ASMResult::Multiline(target))
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, meta: &mut MetaInfo) -> usize {
        let mut sum = 0;

        for assignable in &self.values {
            sum += assignable.byte_size(meta);
        }

        sum
    }

    fn data_section(&self, stack: &mut Stack, meta: &mut MetaInfo) -> bool {
        let mut has_before_label_asm = false;
        let count_before = stack.label_count;

        for value in &self.values {
            if value.data_section(stack, meta) {
                has_before_label_asm = true;
                stack.label_count -= 1;
            }
        }

        stack.label_count = count_before;
        has_before_label_asm
    }
}
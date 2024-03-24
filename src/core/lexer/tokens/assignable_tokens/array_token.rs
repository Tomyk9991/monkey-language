use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::core::code_generator::asm_result::{ASMOptions, ASMResult};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::tokens::assignable_token::AssignableToken;
use crate::core::lexer::tokens::assignable_tokens::method_call_token::dyck_language;
use crate::core::lexer::tokens::assignable_tokens::string_token::StringTokenErr;
use crate::core::lexer::types::type_token::TypeToken;

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayToken {
    pub values: Vec<AssignableToken>,
    pub ty: TypeToken,
}

#[derive(Debug)]
pub enum ArrayTokenErr {
    UnmatchedRegex,
}

impl Error for ArrayTokenErr { }


impl Display for ArrayTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ArrayTokenErr::UnmatchedRegex => "Array must match: [type, size]"
        })
    }
}


impl Display for ArrayToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.values).finish()
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
                values: vec![],
                ty: TypeToken::Undefined,
            })
        }

        let k = &s.split_inclusive(' ').collect::<Vec<_>>()[..];
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

            if let Some(ty) = values[0].infer_type(&CodeLine::imaginary(&array_elements_str[0])) {
                return Ok(ArrayToken {
                    values,
                    ty,
                })
            }
        }

        Err(ArrayTokenErr::UnmatchedRegex)
    }
}

impl ToASM for ArrayToken {
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        todo!()
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        return self.ty.byte_size() * self.values.len();
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
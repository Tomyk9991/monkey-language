use std::any::Any;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::core::code_generator::asm_result::{ASMResult};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, register_destination, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_options::identifier_present::IdentifierPresent;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::registers::{Bit64, ByteSize, GeneralPurposeRegister};
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::collect_tokens_until_scope_close::CollectTokensUntilScopeClose;
use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::scanner::static_type_context::StaticTypeContext;
use crate::core::scanner::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::method_call::{dyck_language, dyck_language_generic};
use crate::core::scanner::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::scanner::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::scanner::types::r#type::{InferTypeError, Mutability, Type};
use crate::pattern;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Array {
    pub values: Vec<Assignable>,
}

#[derive(Debug)]
pub enum ArrayErr {
    UnmatchedRegex,
}

impl Parse for Array {
    fn parse(tokens: &[TokenWithSpan]) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default {
        let slice = tokens.iter().map(|x| x.token.clone()).collect::<Vec<Token>>();

        if let [Token::SquareBracketOpen, Token::SquareBracketClose] = &slice[..] {
            return Ok(ParseResult {
                result: Array {
                    values: vec![]
                },
                consumed: 2,
            })
        }

        // if let Some(MatchResult::Collect(array_content)) = pattern!(tokens, SquareBracketOpen, @ parse CollectTokensUntil::<'[', ']'>, SquareBracketClose) {
        //
        // }
        if let [Token::SquareBracketOpen, array_content @ .., Token::SquareBracketClose] = &slice[..] {

        }
        //     let array_elements_str = dyck_language_generic(&array_content.join(" "), [vec!['{', '('], vec![','], vec!['}', ')']])
        //         .map_err(|_| ArrayErr::UnmatchedRegex)?;
        //
        //     if array_elements_str.is_empty() {
        //         return Err(ArrayErr::UnmatchedRegex);
        //     }
        //
        //     let mut values = vec![];
        //
        //     for array_element in &array_elements_str {
        //         values.push(Assignable::parse(array_element)?);
        //     }
        //
        //     return Ok(ParseResult {
        //         result: Array {
        //             values,
        //         },
        //         consumed: tokens.len(),
        //     })
        // }
        //
        // if let ["[ ", array_content @ .., "]"] = &s.split_inclusive(' ').collect::<Vec<_>>()[..] {
        //     let array_elements_str = dyck_language(&array_content.join(" "), [vec!['{', '('], vec![','], vec!['}', ')']])
        //         .map_err(|_| ArrayErr::UnmatchedRegex)?;
        //
        //     if array_elements_str.is_empty() {
        //         return Err(ArrayErr::UnmatchedRegex);
        //     }
        //
        //     let mut values = vec![];
        //
        //     for array_element in &array_elements_str {
        //         values.push(Assignable::from_str(array_element).map_err(|_| ArrayErr::UnmatchedRegex)?);
        //     }
        //
        //     return Ok(Array {
        //         values,
        //     })
        // }

        Err(Error::UnexpectedToken(tokens[0].clone()))
    }
}

impl std::error::Error for ArrayErr { }

impl Array {
    pub fn infer_type_with_context(&self, context: &StaticTypeContext, code_line: &CodeLine) -> Result<Type, InferTypeError> {
        if self.values.is_empty() {
            return Err(InferTypeError::NoTypePresent(LValue::Identifier(Identifier { name: "Array".to_string() }), code_line.clone()))
        }

        if let Ok(ty) = self.values[0].infer_type_with_context(context, code_line) {
            return Ok(Type::Array(Box::new(ty), self.values.len(), Mutability::Immutable));
        }

        Err(InferTypeError::NoTypePresent(LValue::Identifier(Identifier { name: "Array".to_string() }), code_line.clone()))
    }
}

impl Display for ArrayErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ArrayErr::UnmatchedRegex => "Array must match: [type, size]"
        })
    }
}


impl Display for Array {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let a = self.values.iter().map(|a| format!("{}", a)).collect::<Vec<_>>();
        write!(f, "[{}]", a.join(", "))
    }
}

impl FromStr for Array {
    type Err = ArrayErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !lazy_regex::regex_is_match!(r"^\[.*\]$", s) {
            return Err(ArrayErr::UnmatchedRegex);
        }

        if s.replace(" ", "") == "[]" {
            return Ok(Array {
                values: vec![]
            })
        }

        if let ["[ ", array_content @ .., "]"] = &s.split_inclusive(' ').collect::<Vec<_>>()[..] {
            let array_elements_str = dyck_language(&array_content.join(" "), [vec!['{', '('], vec![','], vec!['}', ')']])
                .map_err(|_| ArrayErr::UnmatchedRegex)?;

            if array_elements_str.is_empty() {
                return Err(ArrayErr::UnmatchedRegex);
            }

            let mut values = vec![];

            for array_element in &array_elements_str {
                values.push(Assignable::from_str(array_element).map_err(|_| ArrayErr::UnmatchedRegex)?);
            }

            return Ok(Array {
                values,
            })
        }

        Err(ArrayErr::UnmatchedRegex)
    }
}

impl ToASM for Array {
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        let mut target = String::new();
        target += &ASMBuilder::ident(&ASMBuilder::comment_line(&format!("{}", self)));

        let initial_position = match options {
            Some(options) => {
                let any_t = &options as &dyn Any;
                if let Some(concrete_type) = any_t.downcast_ref::<IdentifierPresent>() {
                    let stack_variable = stack.variables.iter().rfind(|v| v.name.name == concrete_type.identifier.name).ok_or(ASMGenerateError::InternalError("Cannot find variable".to_string()))?;
                    stack_variable.position
                } else {
                    stack.stack_position
                }
            }
            None => {
                stack.stack_position
            }
        };

        let mut offset = if let [first, ..] = &self.values[..] {
            initial_position + first.byte_size(meta) * self.values.len()
        } else {
            initial_position
        };

        for assignable in self.values.iter() {
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

                    if let Assignable::ArithmeticEquation(expr) = assignable {
                        let final_type = expr.traverse_type(meta).ok_or(ASMGenerateError::InternalError("Cannot infer type".to_string()))?;
                        let r = GeneralPurposeRegister::Bit64(Bit64::Rax).to_size_register(&ByteSize::try_from(final_type.byte_size())?);

                        if let Type::Float(s, _) = final_type {
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

            offset -= byte_size;
        }

        Ok(ASMResult::Multiline(target))
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, meta: &mut MetaInfo) -> usize {
        self.values.iter().map(|a| a.byte_size(meta)).sum::<usize>()
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
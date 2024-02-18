use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_result::{ASMOptions, ASMResult};
use crate::core::code_generator::registers::GeneralPurposeRegister;
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::tokens::assignable_token::AssignableTokenErr;
use crate::core::lexer::tokens::assignable_tokens::method_call_token::{dyck_language, DyckError};
use crate::core::lexer::tokens::name_token::{NameToken, NameTokenErr};
use crate::core::lexer::tokens::variable_token::{ParseVariableTokenErr, VariableToken};
use crate::core::lexer::types::type_token::TypeToken;

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectToken {
    pub variables: Vec<VariableToken<':', ','>>,
    pub ty: TypeToken
}

impl Display for ObjectToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}}}", self.variables.iter().map(|var| format!("{}", var)).collect::<Vec<String>>().join(", "))
    }
}

#[derive(Debug)]
pub enum ObjectTokenErr {
    PatternNotMatched { target_value: String },
    NameTokenErr(NameTokenErr),
    DyckLanguageErr { target_value: String, ordering : Ordering },
    AssignableTokenErr(AssignableTokenErr),
    ParseVariableTokenErr(ParseVariableTokenErr)
}

impl Error for ObjectTokenErr { }

impl From<NameTokenErr> for ObjectTokenErr {
    fn from(err: NameTokenErr) -> Self { ObjectTokenErr::NameTokenErr(err) }
}

impl From<AssignableTokenErr> for ObjectTokenErr {
    fn from(value: AssignableTokenErr) -> Self { ObjectTokenErr::AssignableTokenErr(value) }
}

impl From<ParseVariableTokenErr> for ObjectTokenErr {
    fn from(s: ParseVariableTokenErr) -> Self {
        ObjectTokenErr::ParseVariableTokenErr(s)
    }
}

impl From<DyckError> for ObjectTokenErr {
    fn from(s: DyckError) -> Self {
        ObjectTokenErr::DyckLanguageErr { target_value: s.target_value, ordering: s.ordering }
    }
}

impl Display for ObjectTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ObjectTokenErr::PatternNotMatched { target_value } => format!("\"{target_value}\" must match: methodName(assignable1, ..., assignableN)"),
            ObjectTokenErr::AssignableTokenErr(a) => a.to_string(),
            ObjectTokenErr::NameTokenErr(a) => a.to_string(),
            ObjectTokenErr::DyckLanguageErr { target_value, ordering } =>
                {
                    let error: String = match ordering {
                        Ordering::Less => String::from("Expected `)`"),
                        Ordering::Equal => String::from("Expected expression between `,`"),
                        Ordering::Greater => String::from("Expected `(`")
                    };
                    format!("\"{target_value}\": {error}")
                }
            ObjectTokenErr::ParseVariableTokenErr(err) => err.to_string()
        };

        write!(f, "{}", message)
    }
}

impl FromStr for ObjectToken {
    type Err = ObjectTokenErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut code_line = CodeLine::imaginary(s);
        
        if !s.ends_with(';') {
            code_line.line += " ;";
        }
        
        ObjectToken::try_parse(&code_line)
    }
}

impl ToASM for ObjectToken {
    fn to_asm<T: ASMOptions>(&self, _stack: &mut Stack, _meta: &mut MetaInfo, _options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        todo!()
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        todo!()
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        8
    }

    fn before_label(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Option<Result<String, ASMGenerateError>> {
        None
    }

    fn multi_line_asm(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Result<(bool, String, Option<GeneralPurposeRegister>), ASMGenerateError> {
        todo!()
    }
}

impl ObjectToken {
    pub fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self, ObjectTokenErr> {
        let split_alloc = code_line.split(vec![' ', ';']);
        let split = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();

        return if let [object_type, "{", arguments_segments @ .., "}", ";"] = &split[..] {
            let mut argument_strings = dyck_language(&arguments_segments.join(" "),[vec!['{', '('], vec![','], vec!['}', ')']])?;
            argument_strings.iter_mut().for_each(|s|
                if !s.ends_with(',') {
                    s.push_str(" ,")
                }
            );

            let arguments = argument_strings
                .iter()
                .map(|s| VariableToken::try_parse(&CodeLine::imaginary(s)))
                .collect::<Result<Vec<_>, _>>()?;

            let type_token = TypeToken::Custom(NameToken::from_str(object_type, false)?);
            
            Ok(ObjectToken {
                variables: arguments,
                ty: type_token,
            })
        } else {
            Err(ObjectTokenErr::PatternNotMatched { target_value: code_line.line.to_string() })
        };
    }
}
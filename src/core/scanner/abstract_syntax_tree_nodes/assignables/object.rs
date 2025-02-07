use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::{ASMResult};
use crate::core::io::code_line::CodeLine;
use crate::core::scanner::abstract_syntax_tree_nodes::assignable::AssignableErr;
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::method_call::{dyck_language, DyckError};
use crate::core::scanner::abstract_syntax_tree_nodes::identifier::{Identifier, IdentifierErr};
use crate::core::scanner::abstract_syntax_tree_nodes::variable::{ParseVariableErr, Variable};
use crate::core::scanner::types::r#type::{Mutability, Type};

#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    pub variables: Vec<Variable<':', ','>>,
    pub ty: Type
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}}}", self.variables.iter().map(|var| format!("{}", var)).collect::<Vec<String>>().join(", "))
    }
}

#[derive(Debug)]
pub enum ObjectErr {
    PatternNotMatched { target_value: String },
    IdentifierErr(IdentifierErr),
    DyckLanguageErr { target_value: String, ordering : Ordering },
    AssignableErr(AssignableErr),
    ParseVariableErr(ParseVariableErr)
}

impl Error for ObjectErr { }

impl From<IdentifierErr> for ObjectErr {
    fn from(err: IdentifierErr) -> Self { ObjectErr::IdentifierErr(err) }
}

impl From<AssignableErr> for ObjectErr {
    fn from(value: AssignableErr) -> Self { ObjectErr::AssignableErr(value) }
}

impl From<ParseVariableErr> for ObjectErr {
    fn from(s: ParseVariableErr) -> Self {
        ObjectErr::ParseVariableErr(s)
    }
}

impl From<DyckError> for ObjectErr {
    fn from(s: DyckError) -> Self {
        ObjectErr::DyckLanguageErr { target_value: s.target_value, ordering: s.ordering }
    }
}

impl Display for ObjectErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ObjectErr::PatternNotMatched { target_value } => format!("\"{target_value}\" must match: methodName(assignable1, ..., assignableN)"),
            ObjectErr::AssignableErr(a) => a.to_string(),
            ObjectErr::IdentifierErr(a) => a.to_string(),
            ObjectErr::DyckLanguageErr { target_value, ordering } =>
                {
                    let error: String = match ordering {
                        Ordering::Less => String::from("Expected `)`"),
                        Ordering::Equal => String::from("Expected expression between `,`"),
                        Ordering::Greater => String::from("Expected `(`")
                    };
                    format!("\"{target_value}\": {error}")
                }
            ObjectErr::ParseVariableErr(err) => err.to_string()
        };

        write!(f, "{}", message)
    }
}

impl FromStr for Object {
    type Err = ObjectErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut code_line = CodeLine::imaginary(s);
        
        if !s.ends_with(';') {
            code_line.line += " ;";
        }
        
        Object::try_parse(&code_line)
    }
}

impl ToASM for Object {
    fn to_asm<T: ASMOptions>(&self, _stack: &mut Stack, _meta: &mut MetaInfo, _options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        todo!()
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        8
    }
}

impl Object {
    pub fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self, ObjectErr> {
        let split_alloc = code_line.split(vec![' ', ';']);
        let split = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();

        if let [object_type, "{", arguments_segments @ .., "}", ";"] = &split[..] {
            let mut argument_strings = dyck_language(&arguments_segments.join(" "),[vec!['{', '('], vec![','], vec!['}', ')']])?;
            argument_strings.iter_mut().for_each(|s|
                if !s.ends_with(',') {
                    s.push_str(" ,")
                }
            );

            let arguments = argument_strings
                .iter()
                .map(|s| Variable::try_parse(&CodeLine::imaginary(s)))
                .collect::<Result<Vec<_>, _>>()?;

            let ty = Type::Custom(Identifier::from_str(object_type, false)?, Mutability::Immutable);
            
            Ok(Object {
                variables: arguments,
                ty,
            })
        } else {
            Err(ObjectErr::PatternNotMatched { target_value: code_line.line.to_string() })
        }
    }
}
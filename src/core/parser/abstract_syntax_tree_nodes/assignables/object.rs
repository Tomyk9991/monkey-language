use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::{ASMResult};
use crate::core::model::abstract_syntax_tree_nodes::assignable::AssignableError;
use crate::core::model::abstract_syntax_tree_nodes::assignables::object::{Object, ObjectErr};
use crate::core::model::abstract_syntax_tree_nodes::identifier::{Identifier, IdentifierError};
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::types::mutability::Mutability;
use crate::core::model::types::ty::Type;
use crate::core::parser::abstract_syntax_tree_nodes::assignables::method_call::{dyck_language, DyckError};
use crate::core::parser::abstract_syntax_tree_nodes::variable::{ParseVariableErr};


impl From<IdentifierError> for ObjectErr {
    fn from(err: IdentifierError) -> Self { ObjectErr::IdentifierErr(err) }
}

impl From<AssignableError> for ObjectErr {
    fn from(value: AssignableError) -> Self { ObjectErr::AssignableErr(value) }
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
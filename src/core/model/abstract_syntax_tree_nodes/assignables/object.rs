use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::core::model::abstract_syntax_tree_nodes::assignable::AssignableError;
use crate::core::model::abstract_syntax_tree_nodes::identifier::IdentifierError;
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::types::ty::Type;
use crate::core::parser::abstract_syntax_tree_nodes::variable::ParseVariableErr;

#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    pub variables: Vec<Variable<':', ','>>,
    pub ty: Type
}

#[derive(Debug)]
pub enum ObjectErr {
    PatternNotMatched { target_value: String },
    IdentifierErr(IdentifierError),
    DyckLanguageErr { target_value: String, ordering : Ordering },
    AssignableErr(AssignableError),
    ParseVariableErr(ParseVariableErr)
}

impl std::error::Error for ObjectErr { }

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}}}", self.variables.iter().map(|var| format!("{}", var)).collect::<Vec<String>>().join(", "))
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
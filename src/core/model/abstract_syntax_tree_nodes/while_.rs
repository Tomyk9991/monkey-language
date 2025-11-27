use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::core::io::code_line::CodeLine;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::scanner::errors::EmptyIteratorErr;
use crate::core::scanner::scope::ScopeError;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct While {
    pub condition: Assignable,
    pub stack: Vec<AbstractSyntaxTreeNode>,
    pub code_line: CodeLine
}

#[derive(Debug)]
pub enum WhileErr {
    PatternNotMatched { target_value: String },
    AssignableErr(AssignableError),
    ScopeErrorErr(ScopeError),
    DyckLanguageErr { target_value: String, ordering: Ordering },
    EmptyIterator(EmptyIteratorErr)
}

impl Display for While {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "while ({}) {{Body}}", self.condition)
    }
}

impl Error for WhileErr { }

impl Display for WhileErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            WhileErr::PatternNotMatched { target_value } =>
                format!("Pattern not matched for: `{target_value}`\n\t while (condition) {{}}"),
            WhileErr::AssignableErr(a) => a.to_string(),
            WhileErr::EmptyIterator(e) => e.to_string(),
            WhileErr::ScopeErrorErr(a) => a.to_string(),
            WhileErr::DyckLanguageErr { target_value, ordering } => {
                let error: String = match ordering {
                    Ordering::Less => String::from("Expected `)`"),
                    Ordering::Equal => String::from("Expected expression between `,`"),
                    Ordering::Greater => String::from("Expected `(`")
                };
                format!("\"{target_value}\": {error}")
            }
        })
    }
}
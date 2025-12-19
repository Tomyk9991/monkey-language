use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::core::lexer::token_with_span::FilePosition;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::parser::errors::EmptyIteratorErr;
use crate::core::parser::scope::ScopeError;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct While {
    pub condition: Assignable,
    pub stack: Vec<AbstractSyntaxTreeNode>,
    pub file_position: FilePosition
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
        write!(f, "{}while ({}) {{", " ".repeat(f.width().unwrap_or(0)), self.condition)?;

        for a in &self.stack {
            write!(f, "\n{:width$}{}", "", a, width = f.width().unwrap_or(0) + 4)?;
        }
        
        write!(f, "\n{}}}", " ".repeat(f.width().unwrap_or(0)))?;
        Ok(())
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
use crate::core::lexer::token_with_span::FilePosition;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable};
use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct While {
    pub condition: Assignable,
    pub stack: Vec<AbstractSyntaxTreeNode>,
    pub file_position: FilePosition
}

#[derive(Debug)]
pub enum WhileErr {
    DyckLanguageErr { target_value: String, ordering: Ordering },
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
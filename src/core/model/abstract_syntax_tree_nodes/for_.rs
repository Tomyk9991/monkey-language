use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::core::io::code_line::CodeLine;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::scanner::abstract_syntax_tree_nodes::variable::ParseVariableErr;
use crate::core::scanner::errors::EmptyIteratorErr;
use crate::core::scanner::scope::ScopeError;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct For {
    pub initialization: Variable<'=', ';'>,
    pub condition: Assignable,
    pub update: Variable<'=', ';'>,
    pub stack: Vec<AbstractSyntaxTreeNode>,
    pub code_line: CodeLine,
}

#[derive(Debug)]
pub enum ForErr {
    PatternNotMatched { target_value: String },
    AssignableErr(AssignableError),
    ParseVariableErr(ParseVariableErr),
    ScopeErrorErr(ScopeError),
    DyckLanguageErr { target_value: String, ordering: Ordering },
    EmptyIterator(EmptyIteratorErr),
}

impl Display for For {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut scope = String::new();
        self.stack.iter().for_each(|a| scope += &format!("\t{}\n", a));
        write!(f, "for ({}; {}; {}) \n{scope}", self.initialization, self.condition, self.update)
    }
}

impl Error for ForErr {}

impl Display for ForErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ForErr::PatternNotMatched { target_value } =>
                format!("Pattern mot matched for: `{target_value}`\n\t for (initializiation; condition; update) {{}}"),
            ForErr::AssignableErr(a) => a.to_string(),
            ForErr::ParseVariableErr(a) => a.to_string(),
            ForErr::ScopeErrorErr(a) => a.to_string(),
            ForErr::EmptyIterator(e) => e.to_string(),
            ForErr::DyckLanguageErr { target_value, ordering } =>
                {
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
use std::fmt::{Display, Formatter};
use crate::core::lexer::token_with_span::FilePosition;
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::model::abstract_syntax_tree_nodes::identifier::IdentifierError;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::types::ty::Type;
use crate::core::parser::abstract_syntax_tree_nodes::l_value::LValueErr;
use crate::core::parser::errors::EmptyIteratorErr;
use crate::core::parser::types::r#type::InferTypeError;

/// AST node for a variable. Pattern is defined as: name <Assignment> assignment <Separator>
/// # Examples
/// - `name = assignment;`
/// - `name: assignment,`
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Variable<const ASSIGNMENT: char, const SEPARATOR: char> {
    pub l_value: LValue,
    // flag defining if the variable is mutable or not
    pub mutability: bool,
    /// type of the variable. It's None, when the type is unknown
    pub ty: Option<Type>,
    /// flag defining if the variable is a new definition or a re-assignment
    pub define: bool,
    pub assignable: Assignable,
    pub file_position: FilePosition,
}

#[derive(Debug)]
pub enum ParseVariableErr {
    PatternNotMatched { target_value: String },
    IdentifierErr(IdentifierError),
    AssignableErr(AssignableError),
    LValue(LValueErr),
    InferType(InferTypeError),
    EmptyIterator(EmptyIteratorErr),
}

impl Display for ParseVariableErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ParseVariableErr::PatternNotMatched { target_value } => format!("`{target_value}`\n\tThe pattern for a variable is defined as: lvalue = assignment;"),
            ParseVariableErr::IdentifierErr(a) => a.to_string(),
            ParseVariableErr::AssignableErr(a) => a.to_string(),
            ParseVariableErr::EmptyIterator(e) => e.to_string(),
            ParseVariableErr::InferType(err) => err.to_string(),
            ParseVariableErr::LValue(err) => err.to_string(),
        })
    }
}

impl<const ASSIGNMENT: char, const SEPARATOR: char> Display for Variable<ASSIGNMENT, SEPARATOR> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let t = self.ty.as_ref().map_or(String::new(), |ty| format!(": {ty}"));

        write!(
            f,
            "{}{}{}{}{} {} {}",
            " ".repeat(f.width().unwrap_or(0)),  // indentation
            if self.define { "let " } else { "" },      // definition
            if self.mutability { "mut " } else { "" },  // mutability
            self.l_value,                               // name
            &t,                                         // type
            ASSIGNMENT,                                 // assignment literal
            self.assignable                             // assignment
        )
    }
}


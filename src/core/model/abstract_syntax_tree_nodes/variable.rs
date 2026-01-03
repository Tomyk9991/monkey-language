use std::fmt::{Display, Formatter};
use crate::core::lexer::token_with_span::FilePosition;
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable};
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::types::ty::Type;

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

impl Display for Variable<'=', ';'> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let t = self.ty.as_ref().map_or(String::new(), |ty| format!(": {ty}"));

        write!(
            f,
            "{}{}{}{}{} = {}",
            " ".repeat(f.width().unwrap_or(0)),  // indentation
            if self.define { "let " } else { "" },      // definition
            if self.mutability { "mut " } else { "" },  // mutability
            self.l_value,                               // name
            &t,                                         // type
            self.assignable                             // assignment
        )
    }
}

impl Display for Variable<':', ','> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let t = self.ty.as_ref().map_or(String::new(), |ty| format!(": {ty}"));

        write!(
            f,
            "{}{}{}{}{}: {}",
            " ".repeat(f.width().unwrap_or(0)),  // indentation
            if self.define { "let " } else { "" },      // definition
            if self.mutability { "mut " } else { "" },  // mutability
            self.l_value,                               // name
            &t,                                         // type
            self.assignable                             // assignment
        )
    }
}


use std::fmt::{Display, Formatter};
use crate::core::lexer::token_with_span::FilePosition;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::types::ty::Type;


/// AST node for Field
/// # Pattern
/// - `field_name: Type`
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Field {
    pub name: Identifier,
    pub ty: Type,
}


/// AST node for struct definition
/// # Pattern
/// - `struct StructName { field1: Type1, field2: Type2, ... }`
///   where field is represented by [Field] struct
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Struct {
    pub ty: Type,
    pub fields: Vec<Field>,
    pub file_position: FilePosition,
}

impl Display for Struct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ident: usize = f.width().unwrap_or(0);

        writeln!(f, "{}struct {} {{", " ".repeat(ident), self.ty)?;

        for (i, field) in self.fields.iter().enumerate() {
            if i < self.fields.len() - 1 {
                writeln!(f, "{:width$}{}," , "", field, width = ident + 4)?;
            } else {
                writeln!(f, "{:width$}{}", "", field, width = ident + 4)?;
            }
        }

        write!(f, "{}}}", " ".repeat(ident))?;

        Ok(())
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ident: usize = f.width().unwrap_or(0);
        
        write!(f, "{}{}: {}", " ".repeat(ident), self.name, self.ty)
    }
}
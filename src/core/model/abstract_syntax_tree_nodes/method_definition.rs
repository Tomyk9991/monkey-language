use crate::core::lexer::token_with_span::FilePosition;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::identifier::IdentifierError;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::types::ty::Type;
use crate::core::parser::types::r#type::InferTypeError;
use std::error::Error;
use std::fmt::{Display, Formatter};

/// AST node for method definition. Pattern is `fn function_name(argument1, ..., argumentN): returnType { }`
#[derive(Debug, PartialEq, Clone, Default)]
pub struct MethodDefinition {
    pub identifier: LValue,
    pub return_type: Type,
    pub arguments: Vec<MethodArgument>,
    pub stack: Vec<AbstractSyntaxTreeNode>,
    pub is_extern: bool,
    pub file_position: FilePosition
}

#[derive(Debug)]
pub enum MethodDefinitionErr {
    IdentifierErr(IdentifierError),
    ReturnErr(Box<InferTypeError>),
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MethodArgument {
    pub identifier: LValue,
    pub ty: Type,
}


impl Display for MethodDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ident = f.width().unwrap_or(0);

        write!(f, "{}{}fn {}({}): {}{}",
               " ".repeat(ident),
               if self.is_extern { "extern " } else { "" },
               self.identifier,
               self.arguments
                   .iter()
                   .map(|argument| format!("{}: {}{}", argument.identifier, if argument.ty.mutable() { "mut" } else { "" }, argument.ty))
                   .collect::<Vec<String>>()
                   .join(", "),
               self.return_type,
               if self.is_extern { ";" } else { "" }
        )?;

        if self.is_extern {
            return Ok(());
        }


        writeln!(f, " {{")?;

        for node in &self.stack {
            writeln!(f, "{}{:width$}", " ".repeat(ident), node, width = ident + 4)?;
        }

        write!(f, "}}")?;

        Ok(())
    }
}

impl Error for MethodDefinitionErr {}

impl Display for MethodDefinitionErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            MethodDefinitionErr::IdentifierErr(a) => a.to_string(),
            MethodDefinitionErr::ReturnErr(a) => a.to_string(),
        })
    }
}
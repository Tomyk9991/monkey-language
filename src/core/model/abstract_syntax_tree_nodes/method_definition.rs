use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::core::lexer::token_with_span::FilePosition;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignable::AssignableError;
use crate::core::model::abstract_syntax_tree_nodes::identifier::{Identifier, IdentifierError};
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::types::ty::Type;
use crate::core::parser::errors::EmptyIteratorErr;
use crate::core::parser::scope::ScopeError;
use crate::core::parser::types::r#type::InferTypeError;

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
    PatternNotMatched { target_value: String },
    IdentifierErr(IdentifierError),
    ReturnErr(InferTypeError),
    AssignableErr(AssignableError),
    ScopeErrorErr(ScopeError),
    EmptyIterator(EmptyIteratorErr),
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MethodArgument {
    pub identifier: LValue,
    pub ty: Type,
}


impl Display for MethodDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut stack_buffer = String::new();
        stack_buffer.push_str(" {\n");

        for a in &self.stack {
            stack_buffer.push_str(&format!("    {};\n", a));
        }
        stack_buffer.push_str("}\n");

        write!(
            f,
            "{}fn {}({}): {}{}",
            if self.is_extern { "extern " } else { "" },
            self.identifier,
            self.arguments
                .iter()
                .map(|argument| format!("{}: {}{}", argument.identifier, if argument.ty.mutable() { "mut" } else { "" }, argument.ty))
                .collect::<Vec<String>>()
                .join(", "),
            self.return_type,
            if self.is_extern { ";" } else { &stack_buffer }
        )
    }
}

impl Error for MethodDefinitionErr {}

impl Display for MethodDefinitionErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            MethodDefinitionErr::PatternNotMatched { target_value }
            => format!("Pattern not matched for: `{target_value}`\n\t fn function_name(argument1, ..., argumentN): returnType {{ }}"),
            MethodDefinitionErr::AssignableErr(a) => a.to_string(),
            MethodDefinitionErr::IdentifierErr(a) => a.to_string(),
            MethodDefinitionErr::ReturnErr(a) => a.to_string(),
            MethodDefinitionErr::EmptyIterator(e) => e.to_string(),
            MethodDefinitionErr::ScopeErrorErr(a) => a.to_string(),
        })
    }
}
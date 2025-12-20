use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::semantics::static_type_check::static_type_checker::StaticTypeCheckError;

pub mod static_type_checker;
mod abstract_syntax_tree_nodes;


pub trait StaticTypeCheck {
    /// This function is used to check the static type of AST node and its assignment.
    #[allow(clippy::result_large_err)]
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError>;
}
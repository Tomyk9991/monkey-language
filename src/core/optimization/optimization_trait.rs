use crate::core::parser::ast_parser::ASTParser;
use crate::core::parser::static_type_context::StaticTypeContext;

#[derive(Debug)]
pub struct OptimizationContext {
    pub program: ASTParser,
}

impl From<ASTParser> for OptimizationContext {
    fn from(program: ASTParser) -> Self {
        OptimizationContext {
            program,
        }
    }
}

pub trait Optimization {
    /// Apply O1 optimization to the AST node. Returns an updated OptimizationContext.
    fn o1(&mut self, static_type_context: &mut StaticTypeContext, optimization: OptimizationContext) -> OptimizationContext;
}
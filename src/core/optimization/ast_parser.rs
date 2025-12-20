use crate::core::optimization::optimization_trait::{Optimization, OptimizationContext};
use crate::core::parser::ast_parser::ASTParser;
use crate::core::parser::static_type_context::StaticTypeContext;

impl Optimization for ASTParser {
    fn o1(&mut self, static_type_context: &mut StaticTypeContext, optimization: OptimizationContext) -> OptimizationContext {
        let mut current_optimization_context: OptimizationContext = optimization;

        for ast_node in &mut self.program {
            current_optimization_context = ast_node.o1(static_type_context, current_optimization_context);
        }

        current_optimization_context
    }
}
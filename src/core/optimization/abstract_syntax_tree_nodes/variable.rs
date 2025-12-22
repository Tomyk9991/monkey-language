use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::optimization::optimization_trait::{Optimization, OptimizationContext};
use crate::core::parser::static_type_context::StaticTypeContext;

impl<const ASSIGNMENT: char, const SEPARATOR: char> Optimization for Variable<ASSIGNMENT, SEPARATOR> {
    fn o1(&mut self, static_type_context: &mut StaticTypeContext, optimization: OptimizationContext) -> OptimizationContext {
        self.assignable.o1(static_type_context, optimization)
    }
}
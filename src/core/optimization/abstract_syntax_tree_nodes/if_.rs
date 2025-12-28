use crate::core::model::abstract_syntax_tree_nodes::if_::If;
use crate::core::optimization::optimization_trait::{ConstFoldable, Optimization, OptimizationContext};
use crate::core::parser::static_type_context::StaticTypeContext;

impl Optimization for If {
    fn o1(&mut self, static_type_context: &mut StaticTypeContext, optimization: OptimizationContext) -> OptimizationContext {
        let mut current_optimization_context = optimization;

        current_optimization_context = self.condition.o1(static_type_context, current_optimization_context);

        for node in &mut self.if_stack {
            current_optimization_context = node.o1(static_type_context, current_optimization_context);
        }

        if let Some(else_body) = &mut self.else_stack {
            for node in else_body {
                current_optimization_context = node.o1(static_type_context, current_optimization_context);
            }
        }

        current_optimization_context
    }
}

impl ConstFoldable for If {
    fn is_const(&self) -> bool {
        false
    }

    fn const_fold(&self, _static_type_context: &StaticTypeContext, _optimization_context: &OptimizationContext) -> Option<Self> {
        None
    }
}
use crate::core::model::abstract_syntax_tree_nodes::for_::For;
use crate::core::optimization::optimization_trait::{ConstFoldable, Optimization, OptimizationContext};
use crate::core::parser::static_type_context::StaticTypeContext;

impl Optimization for For {
    fn o1(&mut self, static_type_context: &mut StaticTypeContext, optimization: OptimizationContext) -> OptimizationContext {
        let mut context = self.initialization.o1(static_type_context, optimization);
        context = self.condition.o1(static_type_context, context);

        for statement in &mut self.stack {
            context = statement.o1(static_type_context, context);
        }

        context = self.update.o1(static_type_context, context);
        context
    }
}

impl ConstFoldable for For {
    fn is_const(&self) -> bool {
        if self.initialization.is_const() && self.condition.is_const() && self.update.is_const() {
            for statement in &self.stack {
                if !statement.is_const() {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    fn const_fold(&self, _static_type_context: &StaticTypeContext, _optimization_context: &OptimizationContext) -> Option<Self> {
        todo!()
    }
}
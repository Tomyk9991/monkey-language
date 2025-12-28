use crate::core::model::abstract_syntax_tree_nodes::ret::Return;
use crate::core::optimization::optimization_trait::{ConstFoldable, Optimization, OptimizationContext};
use crate::core::parser::static_type_context::StaticTypeContext;

impl Optimization for Return {
    fn o1(&mut self, static_type_context: &mut StaticTypeContext, optimization: OptimizationContext) -> OptimizationContext {
        if let Some(assignable) = &mut self.assignable {
            assignable.o1(static_type_context, optimization)
        } else {
            optimization
        }
    }
}

impl ConstFoldable for Return {
    fn is_const(&self) -> bool {
        if let Some(assignable) = &self.assignable {
            assignable.is_const()
        } else {
            true
        }
    }

    fn const_fold(&self, static_type_context: &StaticTypeContext, optimization_context: &OptimizationContext) -> Option<Self> {
        if let Some(assignable) = &self.assignable {
            Some(Return {
                assignable: Some(assignable.const_fold(static_type_context, optimization_context)?),
                file_position: self.file_position.clone(),
            })
        } else {
            None
        }
    }
}
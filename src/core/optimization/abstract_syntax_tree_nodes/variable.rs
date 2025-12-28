use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::optimization::optimization_trait::{ConstFoldable, Optimization, OptimizationContext};
use crate::core::parser::static_type_context::StaticTypeContext;

impl Optimization for Variable<'=', ';'> {
    fn o1(&mut self, static_type_context: &mut StaticTypeContext, optimization: OptimizationContext) -> OptimizationContext {
        let mut optimization = optimization;
        
        optimization = self.assignable.o1(static_type_context, optimization.clone());

        if self.is_const() {
            if let Some(assignable_const) = self.assignable.const_fold(static_type_context, &optimization) {
                optimization.constant_variables.insert(self.l_value.identifier(), assignable_const);
            }
        }

        optimization
    }
}

impl ConstFoldable for Variable<'=', ';'> {
    fn is_const(&self) -> bool {
        self.assignable.is_const()
    }

    fn const_fold(&self, static_type_context: &StaticTypeContext, optimization_context: &OptimizationContext) -> Option<Self> {
        Some(Variable {
            l_value: self.l_value.clone(),
            mutability: self.mutability,
            ty: self.ty.clone(),
            define: self.define,
            assignable: self.assignable.const_fold(static_type_context, optimization_context)?,
            file_position: self.file_position.clone(),
        })
    }
}
use crate::core::model::types::float::FloatAST;
use crate::core::optimization::optimization_trait::{AssignmentConstFoldable, OptimizationContext};
use crate::core::parser::static_type_context::StaticTypeContext;


impl AssignmentConstFoldable for FloatAST {
    fn const_fold(&self, _static_type_context: &StaticTypeContext, _optimization_context: &OptimizationContext) -> Option<crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable> {
        Some(crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable::Float(self.clone()))
    }
}


impl FloatAST {
    pub fn add(&self, right: &FloatAST, _static_type_context: &crate::core::parser::static_type_context::StaticTypeContext) -> Option<FloatAST> {
        if self.ty != right.ty {
            return None;
        }

        Some(FloatAST {
            value: self.value + right.value,
            ty: self.ty.clone(),
        })
    }
    
    pub fn sub(&self, right: &FloatAST, _static_type_context: &crate::core::parser::static_type_context::StaticTypeContext) -> Option<FloatAST> {
        if self.ty != right.ty {
            return None;
        }

        Some(FloatAST {
            value: self.value - right.value,
            ty: self.ty.clone(),
        })
    }
    
    pub fn mul(&self, right: &FloatAST, _static_type_context: &crate::core::parser::static_type_context::StaticTypeContext) -> Option<FloatAST> {
        if self.ty != right.ty {
            return None;
        }

        Some(FloatAST {
            value: self.value * right.value,
            ty: self.ty.clone(),
        })
    }
    
    pub fn div(&self, right: &FloatAST, _static_type_context: &crate::core::parser::static_type_context::StaticTypeContext) -> Option<FloatAST> {
        if self.ty != right.ty {
            return None;
        }

        Some(FloatAST {
            value: self.value / right.value,
            ty: self.ty.clone(),
        })
    }
}
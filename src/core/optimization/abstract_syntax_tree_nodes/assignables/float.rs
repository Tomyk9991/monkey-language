use crate::core::model::types::float::FloatAST;

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
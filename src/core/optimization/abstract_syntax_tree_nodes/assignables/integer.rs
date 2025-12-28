use crate::core::model::types::integer::{IntegerAST, IntegerType};
use crate::core::optimization::optimization_trait::{ConstFoldable, OptimizationContext};
use crate::core::parser::static_type_context::StaticTypeContext;

impl ConstFoldable for IntegerAST {
    fn is_const(&self) -> bool {
        true
    }

    fn const_fold(&self, _static_type_context: &StaticTypeContext, _optimization_context: &OptimizationContext) -> Option<Self> {
        Some(self.clone())
    }
}

impl IntegerAST {
    fn apply_bin_op<SignedOp, UnsignedOp>(&self, right: &IntegerAST, signed_op: SignedOp, unsigned_op: UnsignedOp) -> Option<IntegerAST>
    where SignedOp: Fn(i128, i128) -> i128, UnsignedOp: Fn(u128, u128) -> u128 {
        if self.ty != right.ty {
            return None;
        }

        match self.ty {
            IntegerType::I8 => {
                let a = self.value.parse::<i8>().ok()? as i128;
                let b = right.value.parse::<i8>().ok()? as i128;
                let res = signed_op(a, b) as i8;
                Some(IntegerAST { value: res.to_string(), ty: IntegerType::I8 })
            }
            IntegerType::U8 => {
                let a = self.value.parse::<u8>().ok()? as u128;
                let b = right.value.parse::<u8>().ok()? as u128;
                let res = unsigned_op(a, b) as u8;
                Some(IntegerAST { value: res.to_string(), ty: IntegerType::U8 })
            }
            IntegerType::I16 => {
                let a = self.value.parse::<i16>().ok()? as i128;
                let b = right.value.parse::<i16>().ok()? as i128;
                let res = signed_op(a, b) as i16;
                Some(IntegerAST { value: res.to_string(), ty: IntegerType::I16 })
            }
            IntegerType::U16 => {
                let a = self.value.parse::<u16>().ok()? as u128;
                let b = right.value.parse::<u16>().ok()? as u128;
                let res = unsigned_op(a, b) as u16;
                Some(IntegerAST { value: res.to_string(), ty: IntegerType::U16 })
            }
            IntegerType::I32 => {
                let a = self.value.parse::<i32>().ok()? as i128;
                let b = right.value.parse::<i32>().ok()? as i128;
                let res = signed_op(a, b) as i32;
                Some(IntegerAST { value: res.to_string(), ty: IntegerType::I32 })
            }
            IntegerType::U32 => {
                let a = self.value.parse::<u32>().ok()? as u128;
                let b = right.value.parse::<u32>().ok()? as u128;
                let res = unsigned_op(a, b) as u32;
                Some(IntegerAST { value: res.to_string(), ty: IntegerType::U32 })
            }
            IntegerType::I64 => {
                let a = self.value.parse::<i64>().ok()? as i128;
                let b = right.value.parse::<i64>().ok()? as i128;
                let res = signed_op(a, b) as i64;
                Some(IntegerAST { value: res.to_string(), ty: IntegerType::I64 })
            }
            IntegerType::U64 => {
                let a = self.value.parse::<u64>().ok()? as u128;
                let b = right.value.parse::<u64>().ok()? as u128;
                let res = unsigned_op(a, b) as u64;
                Some(IntegerAST { value: res.to_string(), ty: IntegerType::U64 })
            }
        }
    }

    pub fn add(&self, right: &IntegerAST, _static_type_context: &crate::core::parser::static_type_context::StaticTypeContext) -> Option<IntegerAST> {
        self.apply_bin_op(right, |a, b| a + b, |a, b| a + b)
    }

    pub fn sub(&self, right: &IntegerAST, _static_type_context: &crate::core::parser::static_type_context::StaticTypeContext) -> Option<IntegerAST> {
        self.apply_bin_op(right, |a, b| a - b, |a, b| a - b)
    }

    pub fn mul(&self, right: &IntegerAST, _static_type_context: &crate::core::parser::static_type_context::StaticTypeContext) -> Option<IntegerAST> {
        self.apply_bin_op(right, |a, b| a * b, |a, b| a * b)
    }

    pub fn div(&self, right: &IntegerAST, _static_type_context: &crate::core::parser::static_type_context::StaticTypeContext) -> Option<IntegerAST> {
        self.apply_bin_op(right, |a, b| a / b, |a, b| a / b)
    }
}
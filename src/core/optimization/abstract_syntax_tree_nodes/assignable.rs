use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::optimization::optimization_trait::{AssignmentConstFoldable, ConstFoldable, Optimization, OptimizationContext};
use crate::core::parser::static_type_context::StaticTypeContext;

impl Optimization for Assignable {
    fn o1(&mut self, static_type_context: &mut StaticTypeContext, optimization: OptimizationContext) -> OptimizationContext {
        if let Some(folded) = self.const_fold(static_type_context, &optimization) {
            *self = folded;
        }
        
        optimization
    }
}

impl ConstFoldable for Assignable {
    fn is_const(&self) -> bool {
        match self {
            Assignable::Integer(_) | Assignable::Float(_) | Assignable::String(_) | Assignable::Boolean(_) => true,
            Assignable::MethodCall(method_call) => method_call.is_const(),
            Assignable::Expression(expression) => expression.is_const(),
            _ => false
        }
    }

    fn const_fold(&self, static_type_context: &StaticTypeContext, optimization_context: &OptimizationContext) -> Option<Self> {
        match self {
            Assignable::Integer(integer) => Some(Assignable::Integer(integer.const_fold(static_type_context, optimization_context)?)),
            Assignable::MethodCall(method_call) => Some(method_call.const_fold(static_type_context, optimization_context)?),
            Assignable::Float(float) => Some(float.const_fold(static_type_context, optimization_context)?),
            Assignable::String(string) => Some(string.const_fold(static_type_context, optimization_context)?),
            Assignable::Boolean(boolean) => Some(boolean.const_fold(static_type_context, optimization_context)?),
            Assignable::Expression(expression) => {
                if let (Some(value), None, None, None) = (&expression.value, &expression.lhs, &expression.rhs, &expression.prefix_arithmetic) {
                    return Some(*value.clone());
                }

                Some(Assignable::Expression(expression.const_fold(static_type_context, optimization_context)?))
            },
            _ => None,
        }
    }
}

impl Assignable {

    pub fn add(&self, right: Assignable, static_type_context: &StaticTypeContext) -> Option<Assignable> {
        match (self, right) {
            (Assignable::String(left), Assignable::String(right)) => Some(Assignable::String(left.add(&right, static_type_context)?)),
            (Assignable::Float(left), Assignable::Float(right)) =>Some(Assignable::Float(left.add(&right, static_type_context)?)),
            (Assignable::Integer(left), Assignable::Integer(right)) => Some(Assignable::Integer(left.add(&right, static_type_context)?)),
            _ => None,
        }
    }

    pub fn sub(&self, right: Assignable, static_type_context: &StaticTypeContext) -> Option<Assignable> {
        match (self, right) {
            (Assignable::Float(left), Assignable::Float(right)) =>Some(Assignable::Float(left.sub(&right, static_type_context)?)),
            (Assignable::Integer(left), Assignable::Integer(right)) => Some(Assignable::Integer(left.sub(&right, static_type_context)?)),
            _ => None,
        }
    }

    pub fn mul(&self, right: Assignable, static_type_context: &StaticTypeContext) -> Option<Assignable> {
        match (self, right) {
            (Assignable::Float(left), Assignable::Float(right)) =>Some(Assignable::Float(left.mul(&right, static_type_context)?)),
            (Assignable::Integer(left), Assignable::Integer(right)) => Some(Assignable::Integer(left.mul(&right, static_type_context)?)),
            _ => None,
        }
    }

    pub fn div(&self, right: Assignable, static_type_context: &StaticTypeContext) -> Option<Assignable> {
        match (self, right) {
            (Assignable::Float(left), Assignable::Float(right)) =>Some(Assignable::Float(left.div(&right, static_type_context)?)),
            (Assignable::Integer(left), Assignable::Integer(right)) => Some(Assignable::Integer(left.div(&right, static_type_context)?)),
            _ => None,
        }
    }
}
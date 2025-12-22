use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::optimization::optimization_trait::{Optimization, OptimizationContext};
use crate::core::parser::static_type_context::StaticTypeContext;

impl Optimization for Assignable {
    fn o1(&mut self, static_type_context: &mut StaticTypeContext, optimization: OptimizationContext) -> OptimizationContext {
        match self {
            Assignable::Expression(expression) => {
                expression.o1(static_type_context, optimization)
            },
            _ => optimization,
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
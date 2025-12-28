use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::parser::static_type_context::StaticTypeContext;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct OptimizationContext {
    // pub program: ASTParser,
    pub constant_variables: HashMap<String, Assignable>,
    pub const_method_definitions: HashMap<String, Assignable>,
}

pub trait Optimization {
    /// Apply O1 optimization to the AST node. Returns an updated OptimizationContext.
    fn o1(&mut self, static_type_context: &mut StaticTypeContext, optimization: OptimizationContext) -> OptimizationContext;
}

pub trait ConstFoldable {
    /// Check if the AST node is constant.
    fn is_const(&self) -> bool;

    /// Perform constant folding on the AST node. Returns an optimized AST node if folding was possible.
    fn const_fold(&self, static_type_context: &StaticTypeContext, optimization_context: &OptimizationContext) -> Option<Self> where Self: Sized;
}

pub trait AssignmentConstFoldable {
    fn is_const(&self) -> bool {
        true
    }
    /// Perform constant folding on the Assignable. Returns an optimized Assignable (maybe even another) if folding was possible.
    fn const_fold(&self, static_type_context: &StaticTypeContext, optimization_context: &OptimizationContext) -> Option<Assignable> where Self: Sized;
}
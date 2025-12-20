use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use crate::core::model::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use crate::core::model::abstract_syntax_tree_nodes::assignables::object::Object;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::abstract_syntax_tree_nodes::parameter::Parameter;
use crate::core::model::types::array::Array;
use crate::core::model::types::boolean::Boolean;
use crate::core::model::types::float::FloatAST;
use crate::core::model::types::integer::IntegerAST;
use crate::core::model::types::static_string::StaticString;
use std::fmt::{Display, Formatter};

/// AST node for assignable abstract_syntax_tree_nodes. Numbers, strings, method calls, other variables, objects, and arithmetic / boolean equations.
#[derive(Debug, PartialEq, Clone)]
pub enum Assignable {
    String(StaticString),
    Integer(IntegerAST),
    Float(FloatAST),
    Parameter(Parameter),
    Boolean(Boolean),
    MethodCall(MethodCall),
    Identifier(Identifier),
    Object(Object),
    Array(Array),
    Expression(Expression),
}

impl Display for Assignable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Assignable::String(node) => format!("{}", node),
            Assignable::Integer(node) => format!("{}", node),
            Assignable::Float(node) => format!("{}", node),
            Assignable::Boolean(node) => format!("{}", node),
            Assignable::MethodCall(node) => format!("{}", node),
            Assignable::Identifier(node) => format!("{}", node),
            Assignable::Object(node) => format!("{}", node),
            Assignable::Expression(node) => format!("{}", node),
            Assignable::Parameter(node) => format!("{}", node),
            Assignable::Array(node) => format!("{}", node),
        })
    }
}

impl Default for Assignable {
    fn default() -> Self {
        Assignable::Integer(IntegerAST::default())
    }
}
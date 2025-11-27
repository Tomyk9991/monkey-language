use std::fmt::{Display, Formatter};
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
    ArithmeticEquation(Expression),
}

#[derive(Debug)]
pub enum AssignableError {
    PatternNotMatched { target_value: String }
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
            Assignable::ArithmeticEquation(node) => format!("{}", node),
            Assignable::Parameter(node) => format!("{}", node),
            Assignable::Array(node) => format!("{}", node),
        })
    }
}

impl std::error::Error for AssignableError {}

impl Display for AssignableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            AssignableError::PatternNotMatched { target_value }
            => format!("Pattern not matched for: `{target_value}`\n\tAssignments are: string: \"String\", integer: 21, -125, double: -51.1512, 152.1521")
        })
    }
}

impl Default for Assignable {
    fn default() -> Self {
        Assignable::Integer(IntegerAST::default())
    }
}
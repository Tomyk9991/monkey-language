use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::types::float::FloatType;
use crate::core::model::types::integer::IntegerType;
use crate::core::model::types::mutability::Mutability;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
pub enum Type {
    Integer(IntegerType, Mutability),
    Float(FloatType, Mutability),
    Bool(Mutability),
    #[default]
    Void,
    Array(Box<Type>, usize, Mutability),
    Custom(Identifier, Mutability),
    /// Special type to represent type returns from statements like if and loops. Types here are used as a feedback channel for analytical information, not exclusively as a mathematical type.
    Statement
}
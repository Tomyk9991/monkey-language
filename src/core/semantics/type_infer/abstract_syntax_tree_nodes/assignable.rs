use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::types::mutability::Mutability;
use crate::core::model::types::ty::Type;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type;
use crate::core::parser::types::r#type::InferTypeError;
use crate::core::semantics::type_infer::infer_type::InferType;

impl InferType for Assignable {
    fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<Type, Box<InferTypeError>> {
        match self {
            Assignable::String(_) => Ok(r#type::common::string()),
            Assignable::Integer(a) => Ok(Type::Integer(a.ty.clone(), Mutability::Immutable)),
            Assignable::Array(array) => Ok(array.infer_type(type_context)?),
            Assignable::Float(a) => Ok(Type::Float(a.ty.clone(), Mutability::Immutable)),
            Assignable::Boolean(_) => Ok(Type::Bool(Mutability::Immutable)),
            Assignable::Object(object) => Ok(Type::Custom(Identifier { name: object.ty.to_string() }, Mutability::Immutable)),
            Assignable::Expression(expression) => {Ok(expression.infer_type(type_context)?)}
            Assignable::MethodCall(method_call) => { Ok(method_call.infer_type(type_context)?) }
            Assignable::Identifier(var) => Ok(var.infer_type(type_context)?),
            Assignable::Parameter(r) => Ok(r.ty.clone()),
        }
    }
}

impl Assignable {
    pub fn get_type(&self, type_context: &StaticTypeContext) -> Option<Type> {
        match self {
            Assignable::String(_) => Some(r#type::common::string()),
            Assignable::Integer(a) => Some(Type::Integer(a.ty.clone(), Mutability::Immutable)),
            Assignable::Float(node) => Some(Type::Float(node.ty.clone(), Mutability::Immutable)),
            Assignable::Boolean(_) => Some(Type::Bool(Mutability::Immutable)),
            Assignable::Object(node) => Some(Type::Custom(Identifier { name: node.ty.to_string() }, Mutability::Immutable)),
            Assignable::Array(node) => node.values[0].get_type(type_context),
            Assignable::Expression(node) => node.get_type(type_context),
            Assignable::MethodCall(node) => node.get_type(type_context),
            Assignable::Identifier(identifier) => identifier.get_type(type_context),
            Assignable::Parameter(param) => Some(param.ty.clone()),
        }
    }
}
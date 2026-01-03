use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::semantics::static_type_check::static_type_checker::StaticTypeCheckError;
use crate::core::semantics::static_type_check::StaticTypeCheck;

impl StaticTypeCheck for Assignable {
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        match self {
            Assignable::Object(object) => object.static_type_check(type_context),
            Assignable::Array(array) => array.static_type_check(type_context),
            Assignable::String(_) | Assignable::Integer(_) | 
            Assignable::Float(_) | Assignable::Parameter(_) | 
            Assignable::Boolean(_) | Assignable::MethodCall(_) | 
            Assignable::Identifier(_) | Assignable::Expression(_) => Ok(())
        }
    }
}
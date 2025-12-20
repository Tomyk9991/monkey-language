use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::types::ty::Type;
use crate::core::parser::static_type_context::StaticTypeContext;

impl Identifier {
    pub fn get_type(&self, static_type_context: &StaticTypeContext) -> Option<Type> {
        if let Some(variable) = static_type_context.context.iter().find(|var| {
            match &var.l_value {
                LValue::Identifier(a) => a.name.as_str() == self.name.as_str(),
                _ => false,
            }
        }) {
            if variable.l_value.identifier() == self.name.as_str() {
                variable.ty.clone()
            } else {
                None
            }
        } else {
            None
        }
    }
}
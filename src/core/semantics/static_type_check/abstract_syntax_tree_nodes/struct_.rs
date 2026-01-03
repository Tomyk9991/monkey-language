use crate::core::model::abstract_syntax_tree_nodes::struct_::Struct;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::semantics::static_type_check::static_type_checker::StaticTypeCheckError;
use crate::core::semantics::static_type_check::StaticTypeCheck;

impl StaticTypeCheck for Struct {
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        type_context.custom_defined_types.insert(self.ty.clone(), self.clone());
        Ok(())
    }
}
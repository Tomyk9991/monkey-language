use crate::core::model::abstract_syntax_tree_nodes::import::Import;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::semantics::static_type_check::static_type_check::StaticTypeCheck;
use crate::core::semantics::static_type_check::static_type_checker::StaticTypeCheckError;

impl StaticTypeCheck for Import {
    fn static_type_check(&self, _type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        Ok(())
    }
}
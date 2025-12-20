use uuid::Uuid;
use crate::core::model::abstract_syntax_tree_nodes::identifier::{Identifier};


impl Identifier {
    pub fn uuid() -> Identifier {
        Identifier {
            name: Uuid::new_v4().to_string(),
        }
    }
}
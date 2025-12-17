use std::fmt::{Debug, Display, Formatter};
use crate::core::lexer::token_with_span::FilePosition;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::types::ty::Type;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::{InferTypeError, MethodCallSignatureMismatchCause};
use crate::core::semantics::static_type_check::static_type_check::StaticTypeCheck;

#[derive(Debug)]
pub enum StaticTypeCheckError {
    UnresolvedReference { name: LValue, file_position: FilePosition },
    NoTypePresent { name: LValue, file_position: FilePosition },
    VoidType { assignable: Assignable, file_position: FilePosition },
    ImmutabilityViolated { name: LValue, file_position: FilePosition },
    InferredError(InferTypeError),
}

impl std::error::Error for StaticTypeCheckError {}

impl Display for StaticTypeCheckError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            StaticTypeCheckError::UnresolvedReference { name, file_position } => format!("Line: {}\tUnresolved reference: `{name}`", file_position),
            StaticTypeCheckError::InferredError(err) => err.to_string(),
            StaticTypeCheckError::NoTypePresent { name, file_position } => format!("Line: {}\tType not inferred: `{name}`", file_position),
            StaticTypeCheckError::ImmutabilityViolated { name, file_position } => format!("Line: {}\tThis symbol isn't declared mutable: `{name}`", file_position),
            StaticTypeCheckError::VoidType { assignable, file_position } => format!("Line: {}\tCannot assign void to a variable: `{assignable}`", file_position),
        })
    }
}

impl From<InferTypeError> for StaticTypeCheckError {
    fn from(value: InferTypeError) -> Self {
        StaticTypeCheckError::InferredError(value)
    }
}


/// Recursively perform static type checking for all nodes in the scope
pub fn static_type_check(scope: &Vec<AbstractSyntaxTreeNode>) -> Result<StaticTypeContext, StaticTypeCheckError> {
    // check if a variable, which is not a defined variable has an invalid re-assignment
    // let a = 1.0;
    // a = 5;
    let mut type_context: StaticTypeContext = StaticTypeContext::new(scope);
    type_context.colliding_symbols()?;
    static_type_check_rec(&scope, &mut type_context)?;
    
    Ok(type_context)
}

pub fn static_type_check_rec(scope: &Vec<AbstractSyntaxTreeNode>, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
    if let Some(ty) = &type_context.expected_return_type {
        if scope.is_empty() && ty.return_type != Type::Void {
            return Err(StaticTypeCheckError::InferredError(InferTypeError::MethodReturnSignatureMismatch {
                expected: ty.return_type.clone(),
                method_name: ty.method_name.to_string(),
                file_position: ty.method_header_line.clone(),
                cause: MethodCallSignatureMismatchCause::ReturnMismatch,
            }));
        }
    }

    for node in scope {
        node.static_type_check(type_context)?;
    }


    Ok(())
}
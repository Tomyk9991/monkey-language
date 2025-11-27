use std::fmt::{Debug, Display, Formatter};

use crate::core::io::code_line::CodeLine;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::scope::Scope;
use crate::core::model::types::ty::Type;
use crate::core::scanner::static_type_context::{StaticTypeContext};
use crate::core::scanner::types::r#type::{InferTypeError, MethodCallSignatureMismatchCause};
use crate::core::semantics::type_checker::StaticTypeCheck;

#[derive(Debug)]
pub enum StaticTypeCheckError {
    UnresolvedReference { name: LValue, code_line: CodeLine },
    NoTypePresent { name: LValue, code_line: CodeLine },
    VoidType { assignable: Assignable, code_line: CodeLine },
    ImmutabilityViolated { name: LValue, code_line: CodeLine },
    InferredError(InferTypeError),
}

impl std::error::Error for StaticTypeCheckError {}

impl Display for StaticTypeCheckError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            StaticTypeCheckError::UnresolvedReference { name, code_line } => format!("Line: {:?}\tUnresolved reference: `{name}`", code_line.actual_line_number),
            StaticTypeCheckError::InferredError(err) => err.to_string(),
            StaticTypeCheckError::NoTypePresent { name, code_line } => format!("Line: {:?}\tType not inferred: `{name}`", code_line.actual_line_number),
            StaticTypeCheckError::ImmutabilityViolated { name, code_line } => format!("Line: {:?}\tThis symbol isn't declared mutable: `{name}`", code_line.actual_line_number),
            StaticTypeCheckError::VoidType { assignable, code_line } => format!("Line: {:?}\tCannot assign void to a variable: `{assignable}`", code_line.actual_line_number),
        })
    }
}

impl From<InferTypeError> for StaticTypeCheckError {
    fn from(value: InferTypeError) -> Self {
        StaticTypeCheckError::InferredError(value)
    }
}

pub fn static_type_check(scope: &Scope) -> Result<StaticTypeContext, StaticTypeCheckError> {
    // check if a variable, which is not a defined variable has an invalid re-assignment
    // let a = 1.0;
    // a = 5;
    let mut type_context: StaticTypeContext = StaticTypeContext::new(&scope.ast_nodes);
    type_context.colliding_symbols()?;
    static_type_check_rec(&scope.ast_nodes, &mut type_context)?;

    Ok(type_context)
}

pub fn static_type_check_rec(scope: &Vec<AbstractSyntaxTreeNode>, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
    if let Some(ty) = &type_context.expected_return_type {
        if scope.is_empty() && ty.return_type != Type::Void {
            return Err(StaticTypeCheckError::InferredError(InferTypeError::MethodReturnSignatureMismatch {
                expected: ty.return_type.clone(),
                method_name: ty.method_name.to_string(),
                method_head_line: ty.method_header_line.clone(),
                cause: MethodCallSignatureMismatchCause::ReturnMissing,
            }));
        }
    }

    for node in scope {
        node.static_type_check(type_context)?;
    }


    Ok(())
}
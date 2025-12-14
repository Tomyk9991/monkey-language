use std::collections::HashMap;
use std::ops::{Deref, DerefMut, Range};
use crate::core::lexer::token_with_span::FilePosition;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::abstract_syntax_tree_nodes::method_definition::MethodDefinition;
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::types::ty::Type;
use crate::core::parser::types::r#type::{InferTypeError};

#[derive(Debug, Clone)]
pub struct CurrentMethodInfo {
    pub return_type: Type,
    pub method_header_line: Range<usize>,
    pub method_name: String,
}

/// Contains all static type information about the provided scope
/// At the moment variables and method definitions are included
#[derive(Debug, Default, Clone)]
pub struct StaticTypeContext {
    pub context: Vec<Variable<'=', ';'>>,
    pub expected_return_type: Option<CurrentMethodInfo>,
    pub current_file_position: FilePosition,
    pub methods: Vec<MethodDefinition>
}

impl StaticTypeContext {
    // adds all information from the other context to this context
    pub fn merge(&mut self, other: StaticTypeContext) {
        for variable in other.context {
            self.context.push(variable);
        }
    }

    /// checks, if the provided methods have any name collisions
    pub fn colliding_symbols(&self) -> Result<(), InferTypeError> {
        let default = FilePosition::default();
        for method in &self.methods {
            let context = StaticTypeContext::new(&method.stack);
            let mut hash_map: HashMap<String, (usize, &FilePosition)> = HashMap::new();

            for argument in &method.arguments {
                if let Some((counter, _)) = hash_map.get_mut(argument.identifier.identifier().as_str()) {
                    *counter += 1;
                } else {
                    hash_map.insert(argument.identifier.identifier(), (1, &method.file_position));
                }
            }

            for variable in &context.context {
                if !variable.define { continue; }
                let value = match &variable.l_value {
                    LValue::Identifier(a) => a.name.as_str(),
                    _ => continue,
                };
                if let Some((counter, _)) = hash_map.get_mut(value) {
                    *counter += 1;
                } else {
                    // hash_map.insert(value, (1, &variable.code_line));
                    hash_map.insert(value.to_string(), (1, &default));
                }
            }

            for (key, (value, code_line)) in &hash_map {
                if *value > 1 {
                    return Err(InferTypeError::NameCollision(key.to_string(), (*code_line).clone()));
                }
            }
        }

        Ok(())
    }
}

impl Deref for StaticTypeContext {
    type Target = Vec<Variable<'=', ';'>>;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl DerefMut for StaticTypeContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.context
    }
}

impl StaticTypeContext {
    /// Constructs a context containing all type information the ast can infer. This is especially useful to infer further types, that could not be inferred before like function calls and variable assignments
    pub fn new(scope: &Vec<AbstractSyntaxTreeNode>) -> StaticTypeContext {
        let mut context: Vec<Variable<'=', ';'>> = Vec::new();
        let mut methods = Vec::new();

        for node in scope {
            match node {
                AbstractSyntaxTreeNode::Variable(variable) => {
                    if variable.ty.is_some() {
                        context.push(variable.clone());
                    }
                }
                AbstractSyntaxTreeNode::MethodDefinition(method_definition) => {
                    methods.push(method_definition.clone());
                },
                AbstractSyntaxTreeNode::For(for_loop) => {
                    if for_loop.initialization.ty.is_some() {
                        context.push(for_loop.initialization.clone());
                    }
                },
                AbstractSyntaxTreeNode::While(_) | AbstractSyntaxTreeNode::MethodCall(_) | AbstractSyntaxTreeNode::If(_) | AbstractSyntaxTreeNode::Import(_) | AbstractSyntaxTreeNode::Return(_) => {}
            }
        }


        Self {
            context,
            expected_return_type: None,
            methods,
        }
    }
}
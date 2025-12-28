use crate::core::lexer::token_with_span::FilePosition;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::optimization::optimization_trait::{Optimization, OptimizationContext};
use crate::core::parser::ast_parser::ASTParser;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::utils::find_recursive_in::{FindASTNode};

impl ASTParser {
    pub fn o1(&mut self, static_type_context: &mut StaticTypeContext, optimization: OptimizationContext) -> ASTParser {
        let mut current_optimization_context: OptimizationContext = optimization;

        let mut old_program = self.program.clone();

        for ast_node in &mut self.program {
            current_optimization_context = ast_node.o1(static_type_context, current_optimization_context);
        }

        while old_program != self.program {
            for ast_node in &mut self.program {
                current_optimization_context = ast_node.o1(static_type_context, current_optimization_context);
            }

            old_program = self.program.clone();
        }

        let _ = self.remove_uncalled_methods();
        let ast_parse = self.remove_unused_variables(static_type_context);

        ast_parse.clone()
    }

    fn remove_unused_variables(&mut self, _static_type_context: &mut StaticTypeContext) -> &mut ASTParser {
        // get stack, from main program or if not main function exists, from the top level program
        let scope: &Vec<AbstractSyntaxTreeNode> = if self.has_main_method {
            self.program.iter().find_map(|ast_node| {
                if let AbstractSyntaxTreeNode::MethodDefinition(method_definition) = ast_node {
                    if method_definition.identifier.identifier() == "main" && method_definition.arguments.is_empty() && !method_definition.is_extern {
                        return Some(&method_definition.stack);
                    }
                    None
                } else {
                    None
                }
            }).unwrap_or(&self.program)
        } else {
            &self.program
        };

        let static_type_context = &mut StaticTypeContext::new(scope);
        let finder = FindASTNode::<Variable<'=', ';'>>::from(Box::new(find_variable_use_in_assignable));
        let used_variables = finder.find_ast_node_in_stack(scope, static_type_context).iter().map(|a| a.l_value.identifier().clone()).collect::<Vec<_>>();


        let mut unused_variables = vec![];


        for token in &self.program {
            if let AbstractSyntaxTreeNode::Variable(variable) = token {
                if !used_variables.contains(&variable.l_value.identifier()) {
                    unused_variables.push(variable.clone());
                }
            }
        }

        let mut indices = unused_variables.iter().filter_map(|used_variable| {
            self.program.iter().position(|token| matches!(token, AbstractSyntaxTreeNode::Variable(variable) if variable == used_variable))
        }).collect::<Vec<_>>();

        indices.sort();

        indices.iter().rev().for_each(|index| { self.program.remove(*index); });
        self
    }

    fn remove_uncalled_methods(&mut self) -> &mut ASTParser {
        // get stack, from main program or if not main function exists, from the top level program
        let scope: &Vec<AbstractSyntaxTreeNode> = if self.has_main_method {
            self.program.iter().find_map(|ast_node| {
                if let AbstractSyntaxTreeNode::MethodDefinition(method_definition) = ast_node {
                    if method_definition.identifier.identifier() == "main" && method_definition.arguments.is_empty() && !method_definition.is_extern {
                        return Some(&method_definition.stack);
                    }
                    None
                } else {
                    None
                }
            }).unwrap_or(&self.program)
        } else {
            &self.program
        };

        let static_type_context = &mut StaticTypeContext::new(scope);
        let finder = FindASTNode::<MethodCall>::from(Box::new(find_method_call_in_assignable));
        let called_methods = finder.find_ast_node_in_stack(scope, static_type_context).iter().map(|m| m.method_label_name(static_type_context)).collect::<Vec<_>>();

        let mut uncalled_methods = vec![];


        for token in &self.program {
            if let AbstractSyntaxTreeNode::MethodDefinition(method_definition) = token {
                if method_definition.identifier.identifier() == "main" { continue; }

                let method_label_name = method_definition.method_label_name();

                if !called_methods.contains(&method_label_name) {
                    uncalled_methods.push(method_label_name);
                }
            }
        }

        let mut indices = uncalled_methods.iter().filter_map(|called_method| {
            self.program.iter().position(|token| matches!(token, AbstractSyntaxTreeNode::MethodDefinition(method_def) if method_def.method_label_name() == *called_method))
        }).collect::<Vec<_>>();
        indices.sort();

        indices.iter().rev().for_each(|index| { self.program.remove(*index); });
        self
    }
}

fn find_variable_use_in_assignable(v: &FindASTNode<Variable<'=', ';'>>, assignable: &Assignable, static_type_context: &mut StaticTypeContext) -> Option<Vec<Variable<'=', ';'>>> {
    match assignable {
        Assignable::MethodCall(method_call) => {
            let mut elements = vec![];
            let func = &v.assignable_match;

            for arg in &method_call.arguments {
                if let Some(mut more) = func(v, arg, static_type_context) {
                    elements.append(&mut more);
                }
            }

            if elements.is_empty() {
                None
            } else {
                Some(elements)
            }
        }
        Assignable::Expression(a) => {
            let func = &v.expression_match;
            func(v, a, static_type_context)
        }
        Assignable::Identifier(identifier) => {
            Some(vec![Variable {
                l_value: LValue::Identifier(identifier.clone()),
                mutability: true,
                ty: None,
                assignable: Assignable::Identifier(identifier.clone()),
                file_position: FilePosition::default(),
                define: false,
            }])
        }
        Assignable::String(_) | Assignable::Integer(_) |
        Assignable::Float(_) | Assignable::Parameter(_) |
        Assignable::Boolean(_) |
        Assignable::Object(_) => None,
        Assignable::Array(array) => {
            let mut elements = vec![];
            let func = &v.assignable_match;
            for value in &array.values {
                if let Some(mut more) = func(v, value, static_type_context) {
                    elements.append(&mut more);
                }
            }

            if elements.is_empty() {
                None
            } else {
                Some(elements)
            }
        }
    }
}



fn find_method_call_in_assignable(v: &FindASTNode<MethodCall>, assignable: &Assignable, static_type_context: &mut StaticTypeContext) -> Option<Vec<MethodCall>> {
    match assignable {
        Assignable::MethodCall(method_call) => {
            Some(vec![method_call.clone()])
        }
        Assignable::Expression(a) => {
            let func = &v.expression_match;
            func(v, a, static_type_context)
        }
        Assignable::String(_) | Assignable::Integer(_) |
        Assignable::Float(_) | Assignable::Parameter(_) |
        Assignable::Boolean(_) | Assignable::Identifier(_) |
        Assignable::Object(_) => None,
        Assignable::Array(array) => {
            let mut elements = vec![];
            let func = &v.assignable_match;
            for value in &array.values {
                if let Some(mut more) = func(v, value, static_type_context) {
                    elements.append(&mut more);
                }
            }

            if elements.is_empty() {
                None
            } else {
                Some(elements)
            }
        }
    }
}

use std::collections::HashSet;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use crate::core::model::abstract_syntax_tree_nodes::method_definition::MethodDefinition;
use crate::core::optimization::optimization_trait::{Optimization, OptimizationContext};
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::semantics::type_infer::infer_type::InferType;

impl Optimization for MethodDefinition {
    fn o1(&mut self, static_type_context: &mut StaticTypeContext, optimization: OptimizationContext) -> OptimizationContext {
        let mut current_optimization_context: OptimizationContext = optimization;

        for ast_node in &mut self.stack {
            current_optimization_context = ast_node.o1(static_type_context, current_optimization_context);
        }

        let mut ast_parse = current_optimization_context.program;
        // get stack, from main program or if not main function exists, from the top level program
        let scope: &Vec<AbstractSyntaxTreeNode> = if ast_parse.has_main_method {
            ast_parse.program.iter().find_map(|ast_node| {
                if let AbstractSyntaxTreeNode::MethodDefinition(method_definition) = ast_node {
                    if method_definition.identifier.identifier() == "main" && method_definition.arguments.is_empty() && !method_definition.is_extern {
                        return Some(&method_definition.stack);
                    }
                    None
                } else {
                    None
                }
            }).unwrap_or(&ast_parse.program)
        } else {
            &ast_parse.program
        };

        let static_type_context = &mut StaticTypeContext::new(scope);
        let called_methods = Self::method_calls_in_stack(scope, static_type_context);

        let mut uncalled_methods = vec![];


        for token in &ast_parse.program {
            if let AbstractSyntaxTreeNode::MethodDefinition(method_definition) = token {
                if method_definition.identifier.identifier() == "main" { continue; }

                let method_label_name = method_definition.method_label_name();

                if !called_methods.contains(&method_label_name) {
                    uncalled_methods.push(method_label_name);
                }
            }
        }

        let mut indices = uncalled_methods.iter().filter_map(|called_method| {
            ast_parse.program.iter().position(|token| matches!(token, AbstractSyntaxTreeNode::MethodDefinition(method_def) if method_def.method_label_name() == *called_method))
        }).collect::<Vec<_>>();
        indices.sort();

        indices.iter().rev().for_each(|index| { ast_parse.program.remove(*index); });

        OptimizationContext {
            program: ast_parse,
        }
    }
}

impl MethodDefinition {
    fn method_call_in_assignable(assignable: &Assignable, static_type_context: &mut StaticTypeContext) -> Option<Vec<String>> {
        match assignable {
            Assignable::MethodCall(method_call) => {
                let name = method_call.method_label_name(static_type_context);
                Some(vec![name])
            }
            Assignable::Expression(a) => {
                Self::method_calls_in_expression(a, static_type_context)
            }
            Assignable::String(_) | Assignable::Integer(_) |
            Assignable::Float(_) | Assignable::Parameter(_) |
            Assignable::Boolean(_) | Assignable::Identifier(_) |
            Assignable::Object(_) => None,
            Assignable::Array(array) => {
                let mut elements = vec![];
                for value in &array.values {
                    if let Some(mut more) = Self::method_call_in_assignable(value, static_type_context) {
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

    fn method_calls_in_expression(expression: &Expression, static_type_context: &mut StaticTypeContext) -> Option<Vec<String>> {
        if let Some(a) = &expression.value {
            return Self::method_call_in_assignable(a, static_type_context);
        }

        let mut final_result = vec![];
        if let Some(lhs) = &expression.lhs {
            if let Some(lhs_result) = Self::method_calls_in_expression(lhs, static_type_context) {
                lhs_result.iter().for_each(|a| final_result.push(a.clone()));
            }
        }

        if let Some(rhs) = &expression.rhs {
            if let Some(rhs_result) = Self::method_calls_in_expression(rhs, static_type_context) {
                rhs_result.iter().for_each(|a| final_result.push(a.clone()));
            }
        }

        if final_result.is_empty() {
            None
        } else {
            Some(final_result)
        }
    }

    fn method_calls_in_stack(stack: &Vec<AbstractSyntaxTreeNode>, static_type_context: &mut StaticTypeContext) -> Vec<String> {
        let mut called_methods = HashSet::new();

        for node in stack {
            // infer type to populate static type context
            let mut cloned_node = node.clone();
            let _ = cloned_node.infer_type(static_type_context);

            match node {
                AbstractSyntaxTreeNode::Variable(variable) => {
                    if let Some(calls) = Self::method_call_in_assignable(&variable.assignable, static_type_context) {
                        calls.iter().for_each(|a| { called_methods.insert(a.clone()); });
                    }
                }
                AbstractSyntaxTreeNode::MethodCall(method_call) => {
                    for args in &method_call.arguments {
                        if let Some(calls) = Self::method_call_in_assignable(args, static_type_context) {
                            calls.iter().for_each(|a| { called_methods.insert(a.clone()); });
                        }
                    }

                    called_methods.insert(method_call.method_label_name(static_type_context).to_string());
                }
                AbstractSyntaxTreeNode::If(if_definition) => {
                    Self::method_calls_in_stack(&if_definition.if_stack, static_type_context).iter().for_each(|a| { called_methods.insert(a.clone()); });
                    if let Some(else_stack) = &if_definition.else_stack {
                        Self::method_calls_in_stack(else_stack, static_type_context).iter().for_each(|a| { called_methods.insert(a.clone()); })
                    }
                }
                AbstractSyntaxTreeNode::For(for_loop) => {
                    Self::method_calls_in_stack(&for_loop.stack, static_type_context).iter().for_each(|a| { called_methods.insert(a.clone()); });
                }
                AbstractSyntaxTreeNode::While(while_loop) => {
                    Self::method_calls_in_stack(&while_loop.stack, static_type_context).iter().for_each(|a| { called_methods.insert(a.clone()); });
                }
                AbstractSyntaxTreeNode::MethodDefinition(_) | AbstractSyntaxTreeNode::Import(_) | AbstractSyntaxTreeNode::Return(_) => {}
            }
        }

        called_methods.iter().cloned().collect::<Vec<_>>()
    }
}
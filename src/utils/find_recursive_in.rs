use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use crate::core::model::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::semantics::type_infer::infer_type::InferType;

pub trait Findable<T>: Sized + Eq + Hash + Clone {
}

impl Eq for MethodCall {}

impl Hash for MethodCall {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.method_label_name(&StaticTypeContext::new(&vec![])).hash(state);
    }
}

impl Findable<MethodCall> for MethodCall {

}

impl Eq for Variable<'=', ';'> {}


impl Hash for Variable<'=', ';'> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.l_value.identifier().hash(state)
    }
}

impl Findable<Variable<'=', ';'>> for Variable<'=', ';'> {

}


type AssignableMatchFn<T> = dyn Fn(&FindASTNode<T>, &Assignable, &mut StaticTypeContext) -> Option<Vec<T>>;
type ExpressionMatchFn<T> = dyn Fn(&FindASTNode<T>, &Expression, &mut StaticTypeContext) -> Option<Vec<T>>;
pub struct FindASTNode<T: Findable<T>> {
    pub assignable_match: Box<AssignableMatchFn<T>>,
    pub expression_match: Box<ExpressionMatchFn<T>>,
}

impl<T: Findable<T> + 'static> FindASTNode<T> {
    pub fn from(assignable_match: Box<AssignableMatchFn<T>>) -> Self {
        FindASTNode {
            assignable_match,
            expression_match: Box::new(find_ast_node_in_expression),
        }
    }
}
fn find_ast_node_in_expression<T: Findable<T>>(value: &FindASTNode<T>, expression: &Expression, static_type_context: &mut StaticTypeContext) -> Option<Vec<T>> {
    if let Some(a) = &expression.value {
        let func = &value.assignable_match;
        return func(value, a, static_type_context);
    }

    let mut final_result: Vec<T> = vec![];
    if let Some(lhs) = &expression.lhs {
        let func = &value.expression_match;
        if let Some(lhs_result) = func(value, lhs, static_type_context) {
            lhs_result.iter().for_each(|a| final_result.push(a.clone()));
        }
    }

    if let Some(rhs) = &expression.rhs {
        let func = &value.expression_match;
        if let Some(rhs_result) = func(value, rhs, static_type_context) {
            rhs_result.iter().for_each(|a| final_result.push(a.clone()));
        }
    }

    if final_result.is_empty() {
        None
    } else {
        Some(final_result)
    }
}

impl <T: Findable<T>> FindASTNode<T> {
    pub fn find_ast_node_in_stack(&self, stack: &Vec<AbstractSyntaxTreeNode>, static_type_context: &mut StaticTypeContext) -> Vec<T> {
        let mut called_ast_node: HashSet<T> = HashSet::new();

        for node in stack {
            let mut cloned_node = node.clone();
            // infer type to populate static type context
            let _ = cloned_node.infer_type(static_type_context);

            match node {
                AbstractSyntaxTreeNode::Variable(variable) => {
                    let func = &self.assignable_match;
                    if let Some(calls) = func(self, &variable.assignable, static_type_context) {
                        calls.iter().for_each(|a| { called_ast_node.insert(a.clone()); });
                    }
                }
                AbstractSyntaxTreeNode::MethodCall(method_call) => {
                    let func = &self.assignable_match;
                    for args in &method_call.arguments {
                        if let Some(calls) = func(self, args, static_type_context) {
                            calls.iter().for_each(|a| { called_ast_node.insert(a.clone()); });
                        }
                    }

                    if let Some(calls) = func(self, &Assignable::MethodCall(method_call.clone()), static_type_context) {
                        calls.iter().for_each(|a| { called_ast_node.insert(a.clone()); });
                    }
                }
                AbstractSyntaxTreeNode::If(if_definition) => {
                    self.find_ast_node_in_stack(&if_definition.if_stack, static_type_context).iter().for_each(|a| { called_ast_node.insert(a.clone()); });
                    if let Some(else_stack) = &if_definition.else_stack {
                        self.find_ast_node_in_stack(else_stack, static_type_context).iter().for_each(|a| { called_ast_node.insert(a.clone()); })
                    }
                }
                AbstractSyntaxTreeNode::For(for_loop) => {
                    self.find_ast_node_in_stack(&for_loop.stack, static_type_context).iter().for_each(|a| { called_ast_node.insert(a.clone()); });
                }
                AbstractSyntaxTreeNode::While(while_loop) => {
                    self.find_ast_node_in_stack(&while_loop.stack, static_type_context).iter().for_each(|a| { called_ast_node.insert(a.clone()); });
                }
                AbstractSyntaxTreeNode::Return(node) => {
                    if let Some(ret_assignable) = &node.assignable {
                        let func = &self.assignable_match;
                        if let Some(calls) = func(self, ret_assignable, static_type_context) {
                            calls.iter().for_each(|a| { called_ast_node.insert(a.clone()); });
                        }
                    }
                }
                AbstractSyntaxTreeNode::MethodDefinition(_) | AbstractSyntaxTreeNode::Import(_) | AbstractSyntaxTreeNode::StructDefinition(_) => {}
            }
        }

        called_ast_node.iter().cloned().collect::<Vec<_>>()
    }
}


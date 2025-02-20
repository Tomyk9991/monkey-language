use std::collections::{HashSet};
use std::fmt::{Debug, Display, Formatter};
use crate::core::lexer::collect_tokens_until_scope_close::CollectTokensUntilScopeClose;
use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseResult};
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::scanner::errors::EmptyIteratorErr;
use crate::core::scanner::static_type_context::StaticTypeContext;
use crate::core::scanner::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::scanner::parser::{ASTParser};
use crate::core::scanner::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use crate::core::scanner::abstract_syntax_tree_nodes::r#for::For;
use crate::core::scanner::abstract_syntax_tree_nodes::r#if::If;
use crate::core::scanner::abstract_syntax_tree_nodes::method_definition::MethodDefinition;
use crate::core::scanner::abstract_syntax_tree_nodes::scope_ending::ScopeEnding;
use crate::core::scanner::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::scanner::{Lines, TryParse};
use crate::core::scanner::abstract_syntax_tree_nodes::import::Import;
use crate::core::scanner::abstract_syntax_tree_nodes::r#while::While;
use crate::core::scanner::abstract_syntax_tree_nodes::r#return::Return;
use crate::core::scanner::scope_iterator::ScopeIterator;
use crate::core::scanner::types::r#type::InferTypeError;
use crate::pattern;

/// AST nodes inside scope
#[derive(Clone, Default)]
pub struct Scope {
    pub ast_nodes: Vec<AbstractSyntaxTreeNode>
}

impl Parse for Scope {
    fn parse(tokens: &[TokenWithSpan]) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default {
        if let Some(MatchResult::Collect(scope_tokens)) = pattern!(tokens, CurlyBraceOpen, @parse CollectTokensUntilScopeClose, CurlyBraceClose) {
            let mut index = 0;
            let mut ast_nodes = vec![];
            let mut total_consumed = 0;

            'outer: while index < scope_tokens.len() {
                let mut scope_iterator = Scope::iter();
                let mut consumed = 0;

                for parsing_function in scope_iterator {
                    consumed += if let Ok(ast) = parsing_function(&scope_tokens[index..]) {
                        ast_nodes.push(ast.result.clone());
                        ast.consumed
                    } else {
                        0
                    };

                    index += consumed;
                    total_consumed += consumed;
                    if index >= scope_tokens.len() {
                        break 'outer;
                    }
                }

                if consumed == 0 {
                    return Err(Error::UnexpectedToken(scope_tokens[index].clone()));
                }
            }

            return Ok(ParseResult {
                result: Scope {
                    ast_nodes,
                },
                consumed: total_consumed + 2
            })
        }

        Err(Error::UnexpectedToken(tokens[0].clone()))
    }
}

impl Scope {
    ///Optimizing for level 1
    pub fn o1(&mut self, static_type_context: &StaticTypeContext) {
        self.optimize_methods(static_type_context);
        // self.remove_double_strings();
    }
}

impl Scope {
    pub fn iter() -> ScopeIterator {
        ScopeIterator::new()
    }

    pub fn infer_type(stack: &mut Vec<AbstractSyntaxTreeNode>, type_context: &mut StaticTypeContext) -> Result<(), InferTypeError> {
        let variables_len = type_context.len();

        let scoped_checker = StaticTypeContext::new(stack);
        type_context.merge(scoped_checker);

        ASTParser::infer_types(stack, type_context)?;

        let amount_pop = type_context.len() - variables_len;

        for _ in 0..amount_pop {
            let _ = type_context.pop();
        }

        Ok(())
    }

    fn method_call_in_assignable(assignable: &Assignable, static_type_context: &StaticTypeContext) -> Option<Vec<String>> {
        match assignable {
            Assignable::MethodCall(method_call) => {
                let name = method_call.method_label_name(static_type_context);
                Some(vec![name])
            }
            Assignable::ArithmeticEquation(a) => {
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

    fn method_calls_in_expression(expression: &Expression, static_type_context: &StaticTypeContext) -> Option<Vec<String>> {
        if let Some(a) = &expression.value {
            return Self::method_call_in_assignable(a.as_ref(), static_type_context);
        }

        let mut final_result = vec![];
        if let Some(lhs) = &expression.lhs {
            if let Some(lhs_result) = Self::method_calls_in_expression(lhs.as_ref(), static_type_context) {
                lhs_result.iter().for_each(|a| final_result.push(a.clone()));
            }
        }

        if let Some(rhs) = &expression.rhs {
            if let Some(rhs_result) = Self::method_calls_in_expression(rhs.as_ref(), static_type_context) {
                rhs_result.iter().for_each(|a| final_result.push(a.clone()));
            }
        }

        if final_result.is_empty() {
            None
        } else {
            Some(final_result)
        }
    }

    fn method_calls_in_stack(stack: &Vec<AbstractSyntaxTreeNode>, static_type_context: &StaticTypeContext) -> Vec<String> {
        let mut called_methods = HashSet::new();

        for node in stack {
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
                AbstractSyntaxTreeNode::MethodDefinition(_) | AbstractSyntaxTreeNode::Import(_) | AbstractSyntaxTreeNode::Return(_) | AbstractSyntaxTreeNode::ScopeEnding(_) => {}
            }
        }

        called_methods.iter().cloned().collect::<Vec<_>>()
    }

    /// Optimize methods out, which are not traversed down from the main method
    pub fn optimize_methods(&mut self, static_type_context: &StaticTypeContext) {
        if let Some(AbstractSyntaxTreeNode::MethodDefinition(main_method)) = self.ast_nodes.iter().find(|a| matches!(a, AbstractSyntaxTreeNode::MethodDefinition(main) if main.identifier.name == "main")) {
            let called_methods = Self::method_calls_in_stack(&main_method.stack, static_type_context);
            let mut uncalled_methods = vec![];

            for node in &self.ast_nodes {
                if let AbstractSyntaxTreeNode::MethodDefinition(method_definition) = node {
                    if method_definition.identifier.name == "main" { continue; }

                    if !called_methods.contains(&method_definition.method_label_name()) {
                        uncalled_methods.push(method_definition.method_label_name());
                    }
                }
            }

            let mut indices = uncalled_methods.iter().filter_map(|called_method| {
                self.ast_nodes.iter().position(|node| matches!(node, AbstractSyntaxTreeNode::MethodDefinition(method_def) if method_def.identifier.name == *called_method))
            }).collect::<Vec<_>>();
            indices.sort();

            indices.iter().rev().for_each(|index| { self.ast_nodes.remove(*index); });
        }
    }
}

pub enum ScopeError {
    ParsingError { message: String },
    InferredError(InferTypeError),
    EmptyIterator(EmptyIteratorErr)
}

impl PatternNotMatchedError for ScopeError {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, ScopeError::ParsingError { .. })
    }
}

impl Debug for ScopeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for ScopeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ScopeError::ParsingError { message } => message.to_string(),
            ScopeError::EmptyIterator(e) => e.to_string(),
            ScopeError::InferredError(e) => e.to_string()
        })
    }
}


impl std::error::Error for ScopeError {}

macro_rules! ast_node_expand {
    ($code_lines_iterator: ident, $(($ast_node_implementation:ty, $ast_node_type:ident, $iterates_over_same_scope:ident)),*) => {
        $(
            match <$ast_node_implementation as TryParse>::try_parse($code_lines_iterator) {
                Ok(t) => {
                    if $iterates_over_same_scope {
                        $code_lines_iterator.next();
                    }
                    return Ok(AbstractSyntaxTreeNode::$ast_node_type(t))
                },
                Err(err) => {
                    if !err.is_pattern_not_matched_error() {
                        return Err(ScopeError::ParsingError {
                            message: format!("{}", err)
                        })
                    }
                }
            }
        )*
    }
}


impl Debug for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ast_nodes_str = self.ast_nodes
            .iter()
            .fold(String::new(), |mut acc, node| {
                acc.push_str(&format!("\t{:?}\n", node));
                acc
            });
        
        write!(f, "Scope: [\n{}]", ast_nodes_str)
    }
}

impl Display for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scope: [\n{}]", self.ast_nodes
            .iter()
            .map(|node| {
                if let AbstractSyntaxTreeNode::MethodDefinition(md) = node {
                    let postfix = if !md.is_extern {
                        let mut target = String::new();
                        for inner_node in &md.stack { target += &format!("\n\t\t{inner_node}"); }
                        target
                    } else {
                        String::new()
                    };
                    format!("\t{}{postfix}\n", md)
                } else {
                    format!("\t{}\n", node)
                }
            })
            .collect::<String>())
    }
}

impl From<InferTypeError> for ScopeError {
    fn from(value: InferTypeError) -> Self {
        ScopeError::InferredError(value)
    }
}

pub trait PatternNotMatchedError {
    fn is_pattern_not_matched_error(&self) -> bool;
}


impl TryParse for Scope {
    type Output = AbstractSyntaxTreeNode;
    type Err = ScopeError;

    /// Tries to parse the code lines into a scope using a peekable iterator and a greedy algorithm
    /// # Returns
    /// * Ok(ASTNode) if the code lines iterator can be parsed into a scope
    /// * Err(ScopeError) if the code lines iterator cannot be parsed into a scope
    fn try_parse(code_lines_iterator: &mut Lines<'_>) -> anyhow::Result<Self::Output, ScopeError> {
        // let mut pattern_distances: Vec<(usize, Box<dyn Error>)> = vec![];
        let code_line = *code_lines_iterator.peek().ok_or(ScopeError::EmptyIterator(EmptyIteratorErr))?;

        ast_node_expand!(code_lines_iterator,
            (Import,               Import,              true),
            (Variable<'=', ';'>,   Variable,            true),
            (MethodCall,           MethodCall,          true),
            (ScopeEnding,          ScopeEnding,         true),
            (Return,               Return,              true),
            (If,                   If,                  false),
            (MethodDefinition,     MethodDefinition,    false),
            (For,                  For,                 false),
            (While,                While,               false)
        );

        let c = *code_lines_iterator.peek().ok_or(ScopeError::EmptyIterator(EmptyIteratorErr))?;
        Err(ScopeError::ParsingError {
            message: format!("Unexpected node: {:?}: {}", c.actual_line_number, code_line.line)
        })
    }
}
use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use crate::core::model::abstract_syntax_tree_nodes::for_::For;
use crate::core::model::abstract_syntax_tree_nodes::if_::If;
use crate::core::model::abstract_syntax_tree_nodes::import::Import;
use crate::core::model::abstract_syntax_tree_nodes::method_definition::MethodDefinition;
use crate::core::model::abstract_syntax_tree_nodes::ret::Return;
use crate::core::model::abstract_syntax_tree_nodes::scope_ending::ScopeEnding;
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::abstract_syntax_tree_nodes::while_::While;

/// An iterator over every possible AST node that can be parsed.
#[derive(Default)]
pub struct ScopeIterator {
    started: bool,
    index: AbstractSyntaxTreeNode,
}

pub struct ScopeIterationItem {
    pub parser: Box<dyn Fn(&[TokenWithSpan]) -> Result<ParseResult<AbstractSyntaxTreeNode>, Error>>,
    pub name: &'static str,
}

pub trait ScopeIteratorItem: Into<AbstractSyntaxTreeNode> { }

impl ScopeIterator {
    pub fn new() -> Self {
        Self {
            started: false,
            index: AbstractSyntaxTreeNode::default(),
        }
    }
}


impl Iterator for ScopeIterator {
    type Item = ScopeIterationItem;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            self.index = AbstractSyntaxTreeNode::If(If::default());
            let value = If::parse;
            return Some(ScopeIterationItem {
                parser: Box::new(move |tokens| value(tokens, ParseOptions::default())?.into()),
                name: "If",
            });
        }

        let next_token = match self.index {
            AbstractSyntaxTreeNode::If(_) => AbstractSyntaxTreeNode::Variable(Variable::default()),
            AbstractSyntaxTreeNode::Variable(_) => AbstractSyntaxTreeNode::MethodCall(MethodCall::default()),
            AbstractSyntaxTreeNode::MethodCall(_) => AbstractSyntaxTreeNode::MethodDefinition(MethodDefinition::default()),
            AbstractSyntaxTreeNode::MethodDefinition(_) => AbstractSyntaxTreeNode::Import(Import::default()),
            AbstractSyntaxTreeNode::Import(_) => AbstractSyntaxTreeNode::Return(Return::default()),
            AbstractSyntaxTreeNode::Return(_) => AbstractSyntaxTreeNode::ScopeEnding(ScopeEnding::default()),
            AbstractSyntaxTreeNode::ScopeEnding(_) => AbstractSyntaxTreeNode::For(For::default()),
            AbstractSyntaxTreeNode::For(_) => AbstractSyntaxTreeNode::While(While::default()),
            AbstractSyntaxTreeNode::While(_) => AbstractSyntaxTreeNode::If(If::default()),
        };

        self.index = next_token.clone();
        if matches!(next_token, AbstractSyntaxTreeNode::If(_)) {
            return None;
        }

        Some(match next_token {
            AbstractSyntaxTreeNode::If(_) => ScopeIterationItem {
                parser: Box::new(move |tokens| If::parse(tokens, ParseOptions::default())?.into()),
                name: "If",
            },
            AbstractSyntaxTreeNode::Variable(_) => ScopeIterationItem {
                parser: Box::new(move |tokens| Variable::<'=', ';'>::parse(tokens, ParseOptions::default())?.into()),
                name: "Variable",
            },
            AbstractSyntaxTreeNode::For(_) => ScopeIterationItem {
                parser: Box::new(move |tokens| For::parse(tokens, ParseOptions::default())?.into()),
                name: "For",
            },
            AbstractSyntaxTreeNode::Import(_) => ScopeIterationItem {
                parser: Box::new(move |tokens| Import::parse(tokens, ParseOptions::default())?.into()),
                name: "Import",
            },
            _ => {
                // create a new box with an error
                ScopeIterationItem {
                    parser: Box::new(move |_| { Err(Error::UnexpectedEOF) }),
                    name: "Not implemented",
                }
            }

            // AbstractSyntaxTreeNode::MethodCall(_) => Box::new(move |tokens| MethodCall::parse(tokens)?.into()),
            // AbstractSyntaxTreeNode::MethodDefinition(_) => Box::new(move |tokens| MethodDefinition::parse(tokens)?.into()),
            // AbstractSyntaxTreeNode::Import(_) => Box::new(move |tokens| Import::parse(tokens)?.into()),
            // AbstractSyntaxTreeNode::Return(_) => Box::new(move |tokens| Return::parse(tokens)?.into()),
            // AbstractSyntaxTreeNode::ScopeEnding(_) => Box::new(move |tokens| ScopeClosing::parse(tokens)?.into()),
            // AbstractSyntaxTreeNode::For(_) => Box::new(move |tokens| For::parse(tokens)?.into()),
            // AbstractSyntaxTreeNode::While(_) => Box::new(move |tokens| While::parse(tokens)?.into()),
        })
    }
}
use crate::core::lexer::error::Error;
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::abstract_syntax_tree_nodes::for_::For;
use crate::core::model::abstract_syntax_tree_nodes::if_::If;
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;

#[derive(Debug, Clone, Default)]
pub struct ParseResult<T: Default + Clone> {
    // parsed result
    pub result: T,
    // amount of tokens consumed
    pub consumed: usize,
}

pub struct ParseOptions {
    pub ignore_expression: bool,
}

impl Default for ParseOptions {
    fn default() -> Self {
        ParseOptions {
            ignore_expression: false,
        }
    }
}


pub trait Parse: Default + Clone {
    fn parse(tokens: &[TokenWithSpan], options: ParseOptions) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default;
}


impl From<ParseResult<Variable<'=', ';'>>> for Result<ParseResult<AbstractSyntaxTreeNode>, Error> {
    fn from(value: ParseResult<Variable<'=', ';'>>) -> Self {
        Ok(ParseResult {
            result: AbstractSyntaxTreeNode::Variable(value.result),
            consumed: value.consumed,
        })
    }
}


impl From<ParseResult<For>> for Result<ParseResult<AbstractSyntaxTreeNode>, Error> {
    fn from(value: ParseResult<For>) -> Self {
        Ok(ParseResult {
            result: AbstractSyntaxTreeNode::For(value.result),
            consumed: value.consumed,
        })
    }
}

impl From<ParseResult<If>> for Result<ParseResult<AbstractSyntaxTreeNode>, Error> {
    fn from(value: ParseResult<If>) -> Self {
        Ok(ParseResult {
            result: AbstractSyntaxTreeNode::If(value.result),
            consumed: value.consumed,
        })
    }
}
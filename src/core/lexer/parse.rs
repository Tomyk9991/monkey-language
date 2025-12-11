use crate::core::lexer::error::Error;
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::for_::For;
use crate::core::model::abstract_syntax_tree_nodes::if_::If;
use crate::core::model::abstract_syntax_tree_nodes::import::Import;
use crate::core::model::abstract_syntax_tree_nodes::method_definition::MethodDefinition;
use crate::core::model::abstract_syntax_tree_nodes::ret::Return;
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::abstract_syntax_tree_nodes::while_::While;

#[derive(Debug, Clone, Default)]
pub struct ParseResult<T: Default + Clone> {
    // parsed result
    pub result: T,
    // amount of tokens consumed
    pub consumed: usize,
}

pub struct ParseOptions {
    pub ignore_expression: bool,
    pub can_be_mutable: bool,
}

#[derive(Default)]
pub struct ParseOptionsBuilder {
    ignore_expression: bool,
    can_be_mutable: bool,
}

impl ParseOptionsBuilder {
    pub fn with_can_be_mutable(&self, can_be_mutable: bool) -> ParseOptionsBuilder {
        ParseOptionsBuilder {
            ignore_expression: self.ignore_expression,
            can_be_mutable,
        }
    }

    pub fn with_ignore_expression(&self, ignore_expression: bool) -> ParseOptionsBuilder {
        ParseOptionsBuilder {
            ignore_expression,
            can_be_mutable: self.can_be_mutable,
        }
    }

    pub fn build(&self) -> ParseOptions {
        ParseOptions {
            ignore_expression: self.ignore_expression,
            can_be_mutable: self.can_be_mutable,
        }
    }
}

impl ParseOptions {
    pub fn builder() -> ParseOptionsBuilder {
        ParseOptionsBuilder::default()
    }
}

impl Default for ParseOptions {
    fn default() -> Self {
        ParseOptions {
            ignore_expression: false,
            can_be_mutable: true,
        }
    }
}


pub trait Parse: Default + Clone + Sized {
    /// Parses the provided tokens into the implementing type
    ///
    /// # Arguments
    ///
    /// * `tokens`: A slice of TokenWithSpan to parse from
    /// * `options`: Parsing options to customize the parsing behavior
    ///
    /// returns: Result<ParseResult<Self>, Error>
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::core::lexer::token_with_span::TokenWithSpan;
    /// use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
    /// use crate::core::model::abstract_syntax_tree_nodes::if_::If;
    /// use crate::core::lexer::token::Token;
    /// use crate::core::lexer::error::Error;
    /// let tokens = vec![
    ///     TokenWithSpan::new(Token::If, 0, 2),
    ///     TokenWithSpan::new(Token::ParenthesisOpen, 3, 4),
    ///     TokenWithSpan::new(Token::Literal("x".to_string()), 5, 6),
    ///     TokenWithSpan::new(Token::Operator(">".to_string()), 7, 8),
    ///     TokenWithSpan::new(Token::Numbers("10".to_string()), 9, 11),
    ///     TokenWithSpan::new(Token::ParenthesisClose, 12, 13),
    /// ];
    /// let parse_result: Result<ParseResult<If>, Error> = If::parse(&tokens, ParseOptions::default());
    /// ```
    fn parse(tokens: &[TokenWithSpan], options: ParseOptions) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default;
}

impl From<ParseResult<MethodDefinition>> for Result<ParseResult<AbstractSyntaxTreeNode>, Error> {
    fn from(value: ParseResult<MethodDefinition>) -> Self {
        Ok(ParseResult {
            result: AbstractSyntaxTreeNode::MethodDefinition(value.result),
            consumed: value.consumed,
        })
    }
}

impl From<ParseResult<While>> for Result<ParseResult<AbstractSyntaxTreeNode>, Error> {
    fn from(value: ParseResult<While>) -> Self {
        Ok(ParseResult {
            result: AbstractSyntaxTreeNode::While(value.result),
            consumed: value.consumed,
        })
    }
}

impl From<ParseResult<Return>> for Result<ParseResult<AbstractSyntaxTreeNode>, Error> {
    fn from(value: ParseResult<Return>) -> Self {
        Ok(ParseResult {
            result: AbstractSyntaxTreeNode::Return(value.result),
            consumed: value.consumed,
        })
    }
}

impl From<ParseResult<Import>> for Result<ParseResult<AbstractSyntaxTreeNode>, Error> {
    fn from(value: ParseResult<Import>) -> Self {
        Ok(ParseResult {
            result: AbstractSyntaxTreeNode::Import(value.result),
            consumed: value.consumed,
        })
    }
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
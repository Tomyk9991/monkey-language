use crate::core::lexer::error::Error;
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use crate::core::model::abstract_syntax_tree_nodes::for_::For;
use crate::core::model::abstract_syntax_tree_nodes::if_::If;
use crate::core::model::abstract_syntax_tree_nodes::import::Import;
use crate::core::model::abstract_syntax_tree_nodes::method_definition::MethodDefinition;
use crate::core::model::abstract_syntax_tree_nodes::ret::Return;
use crate::core::model::abstract_syntax_tree_nodes::struct_::Struct;
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
    pub ends_with_semicolon: bool,
}

#[derive(Default)]
pub struct ParseOptionsBuilder {
    ignore_expression: bool,
    can_be_mutable: bool,
    ends_with_semicolon: bool,
}

impl ParseOptionsBuilder {
    pub fn with_can_be_mutable(&self, can_be_mutable: bool) -> ParseOptionsBuilder {
        ParseOptionsBuilder {
            ignore_expression: self.ignore_expression,
            ends_with_semicolon: self.ends_with_semicolon,
            can_be_mutable,
        }
    }

    pub fn with_ignore_expression(&self, ignore_expression: bool) -> ParseOptionsBuilder {
        ParseOptionsBuilder {
            ignore_expression,
            ends_with_semicolon: self.ends_with_semicolon,
            can_be_mutable: self.can_be_mutable,
        }
    }

    pub fn with_ends_with_semicolon(&self, ends_with_semicolon: bool) -> ParseOptionsBuilder {
        ParseOptionsBuilder {
            ignore_expression: self.ignore_expression,
            ends_with_semicolon,
            can_be_mutable: self.can_be_mutable,
        }
    }

    pub fn build(&self) -> ParseOptions {
        ParseOptions {
            ignore_expression: self.ignore_expression,
            can_be_mutable: self.can_be_mutable,
            ends_with_semicolon: self.ends_with_semicolon,
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
            ends_with_semicolon: false,
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
    /// use monkey_language::core::lexer::token_with_span::TokenWithSpan;
    /// use monkey_language::core::lexer::parse::{Parse, ParseOptions, ParseResult};
    /// use monkey_language::core::model::abstract_syntax_tree_nodes::if_::If;
    /// use monkey_language::core::lexer::token::Token;
    /// use monkey_language::core::lexer::error::Error;
    /// let tokens = vec![
    ///     TokenWithSpan::new(Token::If, 1, 3),
    ///     TokenWithSpan::new(Token::ParenthesisOpen, 4, 5),
    ///     TokenWithSpan::new(Token::Literal("x".to_string()), 6, 7),
    ///     TokenWithSpan::new(Token::GreaterThan, 8, 9),
    ///     TokenWithSpan::new(Token::Numbers("10".to_string()), 10, 12),
    ///     TokenWithSpan::new(Token::ParenthesisClose, 13, 14),
    ///     TokenWithSpan::new(Token::CurlyBraceOpen, 15, 16),
    ///     TokenWithSpan::new(Token::CurlyBraceClose, 17, 18),
    /// ];
    /// let parse_result: Result<ParseResult<If>, Error> = If::parse(&tokens, ParseOptions::default());
    /// assert!(parse_result.is_ok());
    /// ```
    fn parse(tokens: &[TokenWithSpan], options: ParseOptions) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default;
}

impl From<ParseResult<MethodCall>> for Result<ParseResult<AbstractSyntaxTreeNode>, Error> {
    fn from(value: ParseResult<MethodCall>) -> Self {
        Ok(ParseResult {
            result: AbstractSyntaxTreeNode::MethodCall(value.result),
            consumed: value.consumed,
        })
    }
}

impl From<ParseResult<Struct>> for Result<ParseResult<AbstractSyntaxTreeNode>, Error> {
    fn from(value: ParseResult<Struct>) -> Self {
        Ok(ParseResult {
            result: AbstractSyntaxTreeNode::StructDefinition(value.result),
            consumed: value.consumed,
        })
    }
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
use std::fmt::{Display, Formatter};
use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::scope::Scope;

#[derive(Clone, Default, Debug)]
pub struct ASTParser {
    pub program: Vec<AbstractSyntaxTreeNode>,
    pub has_main_method: bool,
}

impl Display for ASTParser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let buffer = String::new();

        let indent_level: usize = 0;
        for astn in &self.program {
            writeln!(f, "{:width$}", astn, width = indent_level)?;
        }

        write!(f, "{}", buffer)
    }
}

impl ASTParser {
    pub fn parse(tokens: &[TokenWithSpan]) -> Result<ParseResult<Self>, Error> where Self: Sized + Clone {
        let mut tokens = tokens.to_vec();

        tokens.insert(0, TokenWithSpan {token: Token::CurlyBraceOpen, span: FilePosition::default() });
        tokens.push(TokenWithSpan {token: Token::CurlyBraceClose, span: FilePosition::default() });

        let program = Scope::parse(&tokens, ParseOptions::default())?;

        assert_eq!(program.consumed - 2, tokens.len() - 2);

        let mut has_main_method = false;
        for ast_nodes in &program.result.ast_nodes {
            if let AbstractSyntaxTreeNode::MethodDefinition(method_definition) = ast_nodes {
                if method_definition.identifier.identifier() == "main" && method_definition.arguments.is_empty() && !method_definition.is_extern {
                    has_main_method = true;
                    break;
                }
            }
        }

        Ok(ParseResult {
            result: ASTParser {
                program: program.result.ast_nodes,
                has_main_method
            },
            consumed: program.consumed - 2, // reduce the open and close scope. those virtual tokens are not part of the program
        })
    }
}
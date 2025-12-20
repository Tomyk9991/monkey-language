use crate::core::lexer::collect_tokens_until_scope_close::CollectTokensFromUntil;
use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::scope::Scope;
use crate::core::parser::scope_iterator::ScopeIterator;
use crate::pattern;
use std::fmt::{Debug, Display, Formatter};

impl Parse for Scope {
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default {
        if let Some(MatchResult::Collect(scope_tokens)) = pattern!(tokens, CurlyBraceOpen, @parse CollectTokensFromUntil<'{', '}'>, CurlyBraceClose) {
            let mut index = 0;
            let mut ast_nodes = vec![];
            let mut total_consumed = 0;

            'outer: while index < scope_tokens.len() {
                let scope_iterator = Scope::iter();
                let mut consumed = 0;

                for parsing_iteration_item in scope_iterator {
                    let parsing_function = parsing_iteration_item.parser;

                    consumed += match parsing_function(&scope_tokens[index..]) {
                        Ok(ast) => {
                            ast_nodes.push(ast.result.clone());
                            ast.consumed
                        }
                        // this type of error counts as unrecoverable, so we return it
                        Err(err) if matches!(err, Error::WithContext { .. }) => {
                            return Err(err);
                        },
                        Err(_) => {
                            0
                        }
                    };


                    index += consumed;
                    total_consumed += consumed;


                    if consumed > 0 {
                        break;
                    }

                    if index >= scope_tokens.len() {
                        break 'outer;
                    }
                }

                if consumed == 0 {
                    return Err(Error::UnexpectedToken(scope_tokens[index].clone()))
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
    pub fn iter() -> ScopeIterator {
        ScopeIterator::new()
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
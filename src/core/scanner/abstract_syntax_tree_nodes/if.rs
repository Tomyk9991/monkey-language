use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::asm_result::{ASMResult, ASMResultError, ASMResultVariance};
use crate::core::code_generator::generator::Stack;
use crate::core::lexer::error::{Error, ErrorMatch};
use crate::core::lexer::parse::{Parse, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_match::{MatchResult};
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::model::abstract_syntax_tree_nodes::if_::{If, IfError};
use crate::core::model::scope::Scope;
use crate::core::scanner::errors::EmptyIteratorErr;
use crate::core::scanner::scope::{PatternNotMatchedError, ScopeError};
use crate::core::scanner::static_type_context::StaticTypeContext;
use crate::core::scanner::{Lines, TryParse};
use crate::core::scanner::types::r#type::{InferTypeError};
use crate::core::semantics::type_checker::{InferType, StaticTypeCheck};
use crate::core::semantics::type_checker::static_type_checker::{static_type_check_rec, StaticTypeCheckError};
use crate::pattern;


impl TryFrom<Result<ParseResult<Self>, Error>> for If {
    type Error = Error;

    fn try_from(value: Result<ParseResult<Self>, Error>) -> Result<Self, Self::Error> {
        match value {
            Ok(value) => Ok(value.result),
            Err(err) => Err(err)
        }
    }
}


impl Parse for If {
    fn parse(tokens: &[TokenWithSpan]) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default {
        let mut parse_result = ParseResult::<If>::default();
        let mut parsing_fulfilled = false;
        let mut assign_token_count = 0;

        if let Some((MatchResult::Parse(assign))) = pattern!(tokens, If, ParenthesisOpen, @parse Assignable, ParenthesisClose) {
            assign_token_count = assign.consumed;
            let scope = Scope::parse(&tokens[assign.consumed + 3..])?;

            parse_result.result = If {
                condition: assign.result,
                if_stack: scope.result.ast_nodes,
                else_stack: None,
                file_position: FilePosition::from_min_max(&tokens[0], &tokens[assign.consumed + scope.consumed + 2])
            };

            parse_result.consumed = assign.consumed + scope.consumed + 3;
            parsing_fulfilled = true;
        } else {
            return Err(Error::first_unexpected_token(tokens, &vec![Token::If.into(), Token::ParenthesisOpen.into(), ErrorMatch::Collect(assign_token_count), Token::ParenthesisClose.into()]));
        }

        if let [TokenWithSpan { token: Token::Else, .. }, ..] = &tokens[parse_result.consumed..]  {
            assign_token_count += 1;
            let else_scope = Scope::parse(&tokens[parse_result.consumed + 1..])?;

            parse_result.result.else_stack = Some(else_scope.result.ast_nodes);
            parse_result.result.file_position = FilePosition::from_min_max(&tokens[0], &tokens[parse_result.consumed + else_scope.consumed]);
            parse_result.consumed += else_scope.consumed + 1;
        }

        match parsing_fulfilled {
            true => Ok(parse_result),
            false => Err(Error::first_unexpected_token(tokens, &vec![Token::If.into(), Token::ParenthesisOpen.into(), ErrorMatch::Collect(assign_token_count), Token::ParenthesisClose.into()]))
        }
    }
}

impl If {
    pub fn ends_with_return_in_each_branch(&self) -> bool {
        if self.else_stack.is_none() {
            return false;
        }

        if let [.., last_if] = &self.if_stack[..] {
            if let AbstractSyntaxTreeNode::If(inner_if) = last_if {
                return inner_if.ends_with_return_in_each_branch();
            }

            if let Some(else_stack) = &self.else_stack {
                if let [.., last_else] = &else_stack[..] {
                    return matches!(last_if, AbstractSyntaxTreeNode::Return(_)) && matches!(last_else, AbstractSyntaxTreeNode::Return(_));
                }
            }
        }

        false
    }
}

impl InferType for If {
    fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<(), InferTypeError> {
        Scope::infer_type(&mut self.if_stack, type_context)?;

        if let Some(else_stack) = &mut self.else_stack {
            Scope::infer_type(else_stack, type_context)?;
        }

        Ok(())
    }
}


impl PatternNotMatchedError for IfError {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, IfError::PatternNotMatched {..})
    }
}

impl From<AssignableError> for IfError {
    fn from(value: AssignableError) -> Self {
        IfError::AssignableErr(value)
    }
}

impl StaticTypeCheck for If {
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        // let variables_len = type_context.context.len();
        // let condition_type = self.condition.infer_type_with_context(type_context, &self.file_position)?;
        //
        // if !matches!(condition_type, Type::Bool(_)) {
        //     return Err(StaticTypeCheckError::InferredError(InferTypeError::MismatchedTypes {
        //         expected: Type::Bool(Mutability::Immutable),
        //         actual: condition_type,
        //         code_line: self.file_position.clone(),
        //     }));
        // }
        //
        // static_type_check_rec(&self.if_stack, type_context)?;
        //
        // let amount_pop = type_context.context.len() - variables_len;
        //
        // for _ in 0..amount_pop {
        //     let _ = type_context.context.pop();
        // }
        //
        // if let Some(else_stack) = &self.else_stack {
        //     let variables_len = type_context.context.len();
        //
        //     static_type_check_rec(else_stack, type_context)?;
        //
        //     let amount_pop = type_context.context.len() - variables_len;
        //
        //     for _ in 0..amount_pop {
        //         let _ = type_context.context.pop();
        //     }
        // }

        Ok(())
    }
}

impl TryParse for If {
    type Output = If;
    type Err = IfError;

    fn try_parse(code_lines_iterator: &mut Lines<'_>) -> anyhow::Result<Self::Output, Self::Err> {
        // let if_header = *code_lines_iterator
        //     .peek()
        //     .ok_or(IfErr::EmptyIterator(EmptyIteratorErr))?;
        //
        // let split_alloc = if_header.split(vec![' ']);
        // let split_ref = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();
        //
        // let mut if_stack = vec![];
        // let mut else_stack: Option<Vec<AbstractSyntaxTreeNode>> = None;
        //
        // let mut requested_else_block = false;
        //
        // if let ["if", "(", condition @ .., ")", "{"] = &split_ref[..] {
        //     let condition = condition.join(" ");
        //     let condition = Assignable::from_str(&condition)?;
        //
        //     // consume the header
        //     let _ = code_lines_iterator.next();
        //
        //     // collect the body
        //     'outer: while code_lines_iterator.peek().is_some() {
        //         if let Some(next_line) = code_lines_iterator.peek() {
        //             let split_alloc = next_line.split(vec![' ']);
        //             let split_ref = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();
        //
        //             if let ["else", "{"] = &split_ref[..] {
        //                 // consume the "else {"
        //                 let _ = code_lines_iterator.next();
        //
        //                 if else_stack.is_none() {
        //                     else_stack = Some(vec![]);
        //                 }
        //
        //                 while code_lines_iterator.peek().is_some() {
        //                     let node = Scope::try_parse(code_lines_iterator)
        //                         .map_err(IfErr::ScopeErrorErr)?;
        //
        //                     if let AbstractSyntaxTreeNode::ScopeEnding(_) = node {
        //                         break 'outer;
        //                     }
        //
        //
        //                     if let Some(else_stack) = &mut else_stack {
        //                         else_stack.push(node);
        //                     }
        //                 }
        //             } else if requested_else_block {
        //                 break 'outer;
        //             }
        //         }
        //
        //         let node = Scope::try_parse(code_lines_iterator)
        //             .map_err(IfErr::ScopeErrorErr)?;
        //
        //         if let AbstractSyntaxTreeNode::ScopeEnding(_) = node {
        //             // after breaking, because you've read "}". check if else block starts. if so, dont break.
        //             requested_else_block = true;
        //             continue;
        //         }
        //
        //         if_stack.push(node);
        //     }
        //
        //     return Ok(If {
        //         condition,
        //         if_stack,
        //         else_stack,
        //         file_position: if_header.clone(),
        //     });
        // }


        Err(IfError::PatternNotMatched {
            target_value: "if".to_string()
        })
        // Err(IfErr::PatternNotMatched {
        //     target_value: if_header.line.to_string()
        // })
    }
}
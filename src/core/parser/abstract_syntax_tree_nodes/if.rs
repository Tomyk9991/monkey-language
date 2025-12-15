use std::str::FromStr;
use crate::core::lexer::error::{Error, ErrorMatch};
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::model::abstract_syntax_tree_nodes::if_::{If, IfError};
use crate::core::model::scope::Scope;
use crate::core::parser::scope::PatternNotMatchedError;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::r#type::InferTypeError;
use crate::core::semantics::static_type_check::static_type_check::StaticTypeCheck;
use crate::core::semantics::static_type_check::static_type_checker::StaticTypeCheckError;
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
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default {
        let mut parse_result = ParseResult::<If>::default();
        let mut parsing_fulfilled = false;
        let mut assign_token_count = 0;

        if let Some((MatchResult::Parse(assign))) = pattern!(tokens, If, ParenthesisOpen, @parse Assignable, ParenthesisClose) {
            assign_token_count = assign.consumed;
            let scope = Scope::parse(&tokens[assign.consumed + 3..], ParseOptions::default())
                .map_err(|e| Error::Callstack(Box::new(e)).with_context(&tokens[0]))?;

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
            let else_scope = Scope::parse(&tokens[parse_result.consumed + 1..], ParseOptions::default())
                .map_err(|e| Error::Callstack(Box::new(e)).with_context(&tokens[parse_result.consumed]))?;

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
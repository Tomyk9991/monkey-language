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
use crate::core::scanner::errors::EmptyIteratorErr;
use crate::core::scanner::scope::{PatternNotMatchedError, Scope, ScopeError};
use crate::core::scanner::static_type_context::StaticTypeContext;
use crate::core::scanner::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::scanner::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableErr};
use crate::core::scanner::{Lines, TryParse};
use crate::core::scanner::types::r#type::{InferTypeError, Mutability, Type};
use crate::core::semantics::type_checker::{InferType, StaticTypeCheck};
use crate::core::semantics::type_checker::static_type_checker::{static_type_check_rec, StaticTypeCheckError};
use crate::pattern;

/// AST node for if definition.
/// # Pattern
/// - `if (condition) {Body}`
/// - `if (condition) {Body} else {Body}`
#[derive(Debug, PartialEq, Clone, Default)]
pub struct If {
    pub condition: Assignable,
    pub if_stack: Vec<AbstractSyntaxTreeNode>,
    pub else_stack: Option<Vec<AbstractSyntaxTreeNode>>,
    pub file_position: FilePosition,
}

impl TryFrom<Result<ParseResult<Self>, Error>> for If {
    type Error = Error;

    fn try_from(value: Result<ParseResult<Self>, Error>) -> Result<Self, Self::Error> {
        match value {
            Ok(value) => Ok(value.result),
            Err(err) => Err(err)
        }
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

impl Parse for If {
    fn parse(tokens: &[TokenWithSpan]) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default {
        let mut assign_token_count = 0;
        if let Some((MatchResult::Parse(assign))) = pattern!(tokens, If, ParenthesisOpen, @parse Assignable, ParenthesisClose) {
            assign_token_count = assign.consumed;
            let scope = Scope::parse(&tokens[assign.consumed + 3..])?;

            return Ok(ParseResult{
                result: If {
                    condition: assign.result,
                    if_stack: scope.result.ast_nodes,
                    else_stack: None,
                    file_position: FilePosition::from_min_max(&tokens[0], &tokens[assign.consumed + scope.consumed + 2])
                },
                consumed: assign.consumed + scope.consumed + 3,
            })
        }


        Err(Error::first_unexpected_token(tokens, &vec![Token::If.into(), Token::ParenthesisOpen.into(), ErrorMatch::Collect(assign_token_count), Token::ParenthesisClose.into()]))
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

#[derive(Debug)]
pub enum IfErr {
    PatternNotMatched { target_value: String },
    AssignableErr(AssignableErr),
    ScopeErrorErr(ScopeError),
    EmptyIterator(EmptyIteratorErr),
}

impl PatternNotMatchedError for IfErr {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, IfErr::PatternNotMatched {..})
    }
}

impl From<AssignableErr> for IfErr {
    fn from(value: AssignableErr) -> Self {
        IfErr::AssignableErr(value)
    }
}

impl Display for If {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if self.else_stack.is_some() {
                format!("if ({}) {{Body}} else {{Body}}", self.condition)
            } else {
                format!("if ({}) {{Body}}", self.condition)
            }
        )
    }
}

impl std::error::Error for IfErr {}

impl Display for IfErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            IfErr::PatternNotMatched { target_value }
            => format!("Pattern not matched for: `{target_value}`\n\t if(condition) {{ }}"),
            IfErr::AssignableErr(a) => a.to_string(),
            IfErr::ScopeErrorErr(a) => a.to_string(),
            IfErr::EmptyIterator(e) => e.to_string(),
        })
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
    type Err = IfErr;

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


        Err(IfErr::PatternNotMatched {
            target_value: "if".to_string()
        })
        // Err(IfErr::PatternNotMatched {
        //     target_value: if_header.line.to_string()
        // })
    }
}


impl ToASM for If {
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        let mut target = String::new();

        target.push_str(&format!("    ; if condition ({})\n", self.condition));

        let continue_label = stack.create_label();
        let else_label = stack.create_label();

        let jump_label: &str = if self.else_stack.is_some() {
            &else_label
        } else {
            &continue_label
        };

        let result = format!("    cmp {}, 0\n", match &self.condition.to_asm(stack, meta, options.clone())? {
            ASMResult::Inline(t) => t.to_owned(),
            ASMResult::MultilineResulted(t, r) => {
                target += t;
                r.to_string()
            }
            ASMResult::Multiline(_) => return Err(ASMResultError::UnexpectedVariance {
                expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                actual: ASMResultVariance::Multiline,
                ast_node: "if node".to_string(),
            }.into())
        });
        target += &result;


        target.push_str(&format!("    jne {}\n", jump_label));


        target.push_str(&ASMBuilder::ident_comment_line("if branch"));
        target.push_str(&stack.generate_scope(&self.if_stack, meta, options)?);
        target.push_str(&format!("    jmp {}\n", continue_label));


        if let Some(else_stack) = &self.else_stack {
            target.push_str(&format!("{}:\n", else_label));
            target.push_str(&ASMBuilder::ident_comment_line("else branch"));
            target.push_str(&stack.generate_scope::<InterimResultOption>(else_stack, meta, None)?);
        }

        target.push_str(&format!("{}:\n", continue_label));
        target.push_str(&format!("    ; Continue after \"{}\"\n", self));
        Ok(ASMResult::Multiline(target))
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        true
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        0
    }

    fn data_section(&self, stack: &mut Stack, meta: &mut MetaInfo) -> bool {
        let mut has_before_label_asm = false;
        let count_before = stack.label_count;


        if self.condition.data_section(stack, meta) {
            has_before_label_asm = true;
            stack.label_count -= 1;
        }

        for node in &self.if_stack {
            if node.data_section(stack, meta) {
                has_before_label_asm = true;
                stack.label_count -= 1;
            }
        }

        if let Some(else_stack) = &self.else_stack {
            for node in else_stack {
                if node.data_section(stack, meta) {
                    has_before_label_asm = true;
                    stack.label_count -= 1;
                }
            }
        }

        stack.label_count = count_before;
        has_before_label_asm
    }
}

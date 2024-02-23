use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;
use std::str::FromStr;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_result::{ASMOptions, ASMResult, ASMResultError, ASMResultVariance, InterimResultOption};
use crate::core::code_generator::generator::Stack;

use crate::core::io::code_line::CodeLine;
use crate::core::lexer::errors::EmptyIteratorErr;
use crate::core::lexer::scope::{PatternNotMatchedError, Scope, ScopeError};
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::token::Token;
use crate::core::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::core::lexer::TryParse;
use crate::core::lexer::types::type_token::InferTypeError;
use crate::core::type_checker::InferType;

/// Token for if definition.
/// # Pattern
/// - `if (condition) {Body}`
/// - `if (condition) {Body} else {Body}`
#[derive(Debug, PartialEq, Clone)]
pub struct IfDefinition {
    pub condition: AssignableToken,
    pub if_stack: Vec<Token>,
    pub else_stack: Option<Vec<Token>>,
    pub code_line: CodeLine
}

impl IfDefinition {
    pub fn ends_with_return_in_each_branch(&self) -> bool {
        if self.else_stack.is_none() {
            return false;
        }

        if let [.., last_if] = &self.if_stack[..] {
            if let Token::IfDefinition(inner_if) = last_if {
                return inner_if.ends_with_return_in_each_branch();
            }

            if let Some(else_stack) = &self.else_stack {
                if let [.., last_else] = &else_stack[..] {
                    return matches!(last_if, Token::Return(_)) && matches!(last_else, Token::Return(_));
                }
            }
        }

        false
    }
}

impl InferType for IfDefinition {
    fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<(), InferTypeError> {
        Scope::infer_type(&mut self.if_stack, type_context)?;

        if let Some(else_stack) = &mut self.else_stack {
            Scope::infer_type(else_stack, type_context)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum IfDefinitionErr {
    PatternNotMatched { target_value: String },
    AssignableTokenErr(AssignableTokenErr),
    ScopeErrorErr(ScopeError),
    EmptyIterator(EmptyIteratorErr),
}

impl PatternNotMatchedError for IfDefinitionErr {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, IfDefinitionErr::PatternNotMatched {..})
    }
}

impl From<AssignableTokenErr> for IfDefinitionErr {
    fn from(value: AssignableTokenErr) -> Self {
        IfDefinitionErr::AssignableTokenErr(value)
    }
}

impl Display for IfDefinition {
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

impl Error for IfDefinitionErr {}

impl Display for IfDefinitionErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            IfDefinitionErr::PatternNotMatched { target_value }
            => format!("Pattern not matched for: `{target_value}`\n\t if(condition) {{ }}"),
            IfDefinitionErr::AssignableTokenErr(a) => a.to_string(),
            IfDefinitionErr::ScopeErrorErr(a) => a.to_string(),
            IfDefinitionErr::EmptyIterator(e) => e.to_string(),
        })
    }
}

impl TryParse for IfDefinition {
    type Output = IfDefinition;
    type Err = IfDefinitionErr;

    fn try_parse(code_lines_iterator: &mut Peekable<Iter<CodeLine>>) -> anyhow::Result<Self::Output, Self::Err> {
        let if_header = *code_lines_iterator
            .peek()
            .ok_or(IfDefinitionErr::EmptyIterator(EmptyIteratorErr))?;

        let split_alloc = if_header.split(vec![' ']);
        let split_ref = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();

        let mut if_stack = vec![];
        let mut else_stack: Option<Vec<Token>> = None;

        let mut requested_else_block = false;

        if let ["if", "(", condition@ .. , ")", "{"] = &split_ref[..] {
            let condition = condition.join(" ");
            let condition = AssignableToken::from_str(&condition)?;

            // consume the header
            let _ = code_lines_iterator.next();

            // collect the body
            'outer: while code_lines_iterator.peek().is_some() {
                if let Some(next_line) = code_lines_iterator.peek() {
                    let split_alloc = next_line.split(vec![' ']);
                    let split_ref = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();

                    if let ["else", "{"] = &split_ref[..] {
                        // consume the "else {"
                        let _ = code_lines_iterator.next();

                        if else_stack.is_none() {
                            else_stack = Some(vec![]);
                        }

                        while code_lines_iterator.peek().is_some() {
                            let token = Scope::try_parse(code_lines_iterator)
                                .map_err(IfDefinitionErr::ScopeErrorErr)?;

                            if let Token::ScopeClosing(_) = token {
                                break 'outer;
                            }


                            if let Some(else_stack) = &mut else_stack {
                                else_stack.push(token);
                            }
                        }
                    } else if requested_else_block {
                        break 'outer;
                    }
                }

                let token = Scope::try_parse(code_lines_iterator)
                    .map_err(IfDefinitionErr::ScopeErrorErr)?;

                if let Token::ScopeClosing(_) = token {
                    // after breaking, because you've read "}". check if else block starts. if so, dont break.
                    requested_else_block = true;
                    continue;
                }

                if_stack.push(token);
            }

            return Ok(IfDefinition {
                condition,
                if_stack,
                else_stack,
                code_line: if_header.clone(),
            });
        }


        Err(IfDefinitionErr::PatternNotMatched {
            target_value: if_header.line.to_string()
        })
    }
}


impl ToASM for IfDefinition {
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

        target.push_str(&format!("    ; if {} > 0 => run if stack: \n", self.condition));
        let result = format!("    cmp {}, 0\n", match &self.condition.to_asm(stack, meta, options.clone())? {
            ASMResult::Inline(t) => t.to_owned(),
            ASMResult::MultilineResulted(t, r) => {
                target += t;
                r.to_string()
            }
            ASMResult::Multiline(_) => return Err(ASMResultError::UnexpectedVariance {
                expected: vec![ASMResultVariance::Inline, ASMResultVariance::MultilineResulted],
                actual: ASMResultVariance::Multiline,
                token: "if token".to_string(),
            }.into())
        });
        target += &result;


        target.push_str(&format!("    je {}\n", jump_label));


        target.push_str("    ; if branch\n");
        target.push_str(&stack.generate_scope(&self.if_stack, meta, options)?);
        target.push_str(&format!("    jmp {}\n", continue_label));


        if let Some(else_stack) = &self.else_stack {
            target.push_str(&format!("{}:\n", else_label));
            target.push_str(&format!("    ; else branch \"{}\"\n", self));
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

    fn before_label(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Option<Result<String, ASMGenerateError>> {
        None
    }
}

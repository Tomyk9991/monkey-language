use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::core::code_generator::asm_result::{ASMResult, ASMResultError, ASMResultVariance};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::errors::EmptyIteratorErr;
use crate::core::lexer::scope::{PatternNotMatchedError, Scope, ScopeError};
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::token::Token;
use crate::core::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::core::lexer::tokens::assignable_tokens::method_call_token::DyckError;
use crate::core::lexer::{Lines, TryParse};
use crate::core::lexer::types::type_token::{InferTypeError, Mutability, TypeToken};
use crate::core::type_checker::{InferType, StaticTypeCheck};
use crate::core::type_checker::static_type_checker::{static_type_check_rec, StaticTypeCheckError};

#[derive(Debug, PartialEq, Clone)]
pub struct WhileToken {
    pub condition: AssignableToken,
    pub stack: Vec<Token>,
    pub code_line: CodeLine
}

#[derive(Debug)]
pub enum WhileTokenErr {
    PatternNotMatched { target_value: String },
    AssignableTokenErr(AssignableTokenErr),
    ScopeErrorErr(ScopeError),
    DyckLanguageErr { target_value: String, ordering: Ordering },
    EmptyIterator(EmptyIteratorErr)
}

impl PatternNotMatchedError for WhileTokenErr {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, WhileTokenErr::PatternNotMatched { .. })
    }
}

impl InferType for WhileToken {
    fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<(), InferTypeError> {
        Scope::infer_type(&mut self.stack, type_context)?;

        Ok(())
    }
}

impl Display for WhileToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "while ({}) {{Body}}", self.condition)
    }
}

impl From<DyckError> for WhileTokenErr {
    fn from(value: DyckError) -> Self {
        WhileTokenErr::DyckLanguageErr { target_value: value.target_value, ordering: value.ordering }
    }
}

impl From<AssignableTokenErr> for WhileTokenErr {
    fn from(value: AssignableTokenErr) -> Self {
        WhileTokenErr::AssignableTokenErr(value)
    }
}

impl Error for WhileTokenErr { }

impl Display for WhileTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            WhileTokenErr::PatternNotMatched { target_value } =>
                format!("Pattern not matched for: `{target_value}`\n\t while (condition) {{}}"),
            WhileTokenErr::AssignableTokenErr(a) => a.to_string(),
            WhileTokenErr::EmptyIterator(e) => e.to_string(),
            WhileTokenErr::ScopeErrorErr(a) => a.to_string(),
            WhileTokenErr::DyckLanguageErr { target_value, ordering } => {
                let error: String = match ordering {
                    Ordering::Less => String::from("Expected `)`"),
                    Ordering::Equal => String::from("Expected expression between `,`"),
                    Ordering::Greater => String::from("Expected `(`")
                };
                format!("\"{target_value}\": {error}")
            }
        })
    }
}

impl StaticTypeCheck for WhileToken {
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        let variables_len = type_context.context.len();
        let condition_type = self.condition.infer_type_with_context(type_context, &self.code_line)?;

        if !matches!(condition_type, TypeToken::Bool(_)) {
            return Err(StaticTypeCheckError::InferredError(InferTypeError::MismatchedTypes {
                expected: TypeToken::Bool(Mutability::Immutable),
                actual: condition_type,
                code_line: self.code_line.clone(),
            }));
        }

        static_type_check_rec(&self.stack, type_context)?;

        let amount_pop = type_context.context.len() - variables_len;

        for _ in 0..amount_pop {
            let _ = type_context.context.pop();
        }
        
        Ok(())
    }
}

impl TryParse for WhileToken {
    type Output = WhileToken;
    type Err = WhileTokenErr;

    fn try_parse(code_lines_iterator: &mut Lines<'_>) -> anyhow::Result<Self::Output, Self::Err> {
        let while_header = *code_lines_iterator
            .peek()
            .ok_or(WhileTokenErr::EmptyIterator(EmptyIteratorErr))?;

        let split_alloc = while_header.split(vec![' ']);
        let split_ref = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();
        let mut stack = vec![];

        if let ["while", "(", condition @ .., ")", "{"] = &split_ref[..] {
            let condition = condition.join(" ");
            let condition = AssignableToken::from_str(&condition)?;

            // consume the header
            let _ = code_lines_iterator.next();

            while code_lines_iterator.peek().is_some() {
                let token = Scope::try_parse(code_lines_iterator).map_err(WhileTokenErr::ScopeErrorErr)?;

                if let Token::ScopeClosing(_) = token {
                    break;
                }

                stack.push(token);
            }

            return Ok(WhileToken {
                condition,
                stack,
                code_line: while_header.clone(),
            })
        }

        Err(WhileTokenErr::PatternNotMatched {
            target_value: while_header.line.to_string(),
        })
    }
}

impl ToASM for WhileToken {
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        let label1 = stack.create_label();
        let label2 = stack.create_label();
        let mut target = String::new();

        target += &ASMBuilder::ident(&ASMBuilder::comment_line(&format!("while ({})", self.condition)));

        target += &ASMBuilder::ident_line(&format!("jmp {label1}"));
        target += &ASMBuilder::line(&format!("{label2}:"));
        target += &stack.generate_scope(&self.stack, meta, options.clone())?;

        target += &ASMBuilder::line(&format!("{label1}:"));
        let general_purpose_register = self.condition.to_asm(stack, meta, options.clone())?
            .apply_with(&mut target)
            .allow(ASMResultVariance::MultilineResulted)
            .token("while")
            .finish()?;

        if let Some(general_purpose_register) = general_purpose_register {
            target += &ASMBuilder::ident_line(&format!("cmp {general_purpose_register}, 0"));
            target += &ASMBuilder::ident_line(&format!("jne {label2}"));
        } else {
            return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                expected: vec![ASMResultVariance::MultilineResulted],
                actual: ASMResultVariance::from(&self.condition.to_asm(stack, meta, options)?),
                token: "while".to_string(),
            }));
        }

        Ok(ASMResult::Multiline(target))
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        0
    }

    fn data_section(&self, stack: &mut Stack, meta: &mut MetaInfo) -> bool {
        let mut has_before_label_asm = false;
        let count_before = stack.label_count;

        for token in &self.stack {
            if token.data_section(stack, meta) {
                has_before_label_asm = true;
                stack.label_count -= 1;
            }
        }

        if self.condition.data_section(stack, meta) {
            has_before_label_asm = true;
            stack.label_count -= 1;
        }

        stack.label_count = count_before;


        has_before_label_asm
    }
}
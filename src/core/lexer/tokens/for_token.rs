use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;
use std::str::FromStr;
use crate::core::code_generator::asm_result::{ASMOptions, ASMResult};
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::errors::EmptyIteratorErr;
use crate::core::lexer::scope::{PatternNotMatchedError, Scope, ScopeError};
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::token::Token;
use crate::core::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::core::lexer::tokens::assignable_tokens::method_call_token::{dyck_language, DyckError};
use crate::core::lexer::tokens::variable_token::{ParseVariableTokenErr, VariableToken};
use crate::core::lexer::TryParse;
use crate::core::lexer::types::type_token::InferTypeError;
use crate::core::type_checker::InferType;

#[derive(Debug, PartialEq, Clone)]
pub struct ForToken {
    pub initialization: VariableToken<'=', ';'>,
    pub condition: AssignableToken,
    pub update: VariableToken<'=', ';'>,
    pub stack: Vec<Token>,
    pub code_line: CodeLine
}

#[derive(Debug)]
pub enum ForTokenErr {
    PatternNotMatched { target_value: String },
    AssignableTokenErr(AssignableTokenErr),
    ParseVariableTokenErr(ParseVariableTokenErr),
    ScopeErrorErr(ScopeError),
    DyckLanguageErr { target_value: String, ordering: Ordering },
    EmptyIterator(EmptyIteratorErr),
}

impl PatternNotMatchedError for ForTokenErr {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, ForTokenErr::PatternNotMatched { .. })
    }
}

impl InferType for ForToken {
    fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<(), InferTypeError> {
        Scope::infer_type(&mut self.stack, type_context)?;

        Ok(())
    }
}

impl Display for ForToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("for ({}; {}; {}) {{Body}}", self.initialization, self.condition, self.update))
    }
}

impl From<DyckError> for ForTokenErr {
    fn from(s: DyckError) -> Self {
        ForTokenErr::DyckLanguageErr { target_value: s.target_value, ordering: s.ordering }
    }
}

impl From<ParseVariableTokenErr> for ForTokenErr {
    fn from(value: ParseVariableTokenErr) -> Self {
        ForTokenErr::ParseVariableTokenErr(value)
    }
}

impl From<AssignableTokenErr> for ForTokenErr {
    fn from(value: AssignableTokenErr) -> Self {
        ForTokenErr::AssignableTokenErr(value)
    }
}

impl Error for ForTokenErr { }

impl Display for ForTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ForTokenErr::PatternNotMatched { target_value } =>
                format!("Pattern mot matched for: `{target_value}`\n\t for (initializiation; condition; update) {{}}"),
            ForTokenErr::AssignableTokenErr(a) => a.to_string(),
            ForTokenErr::ParseVariableTokenErr(a) => a.to_string(),
            ForTokenErr::ScopeErrorErr(a) => a.to_string(),
            ForTokenErr::EmptyIterator(e) => e.to_string(),
            ForTokenErr::DyckLanguageErr { target_value, ordering } =>
            {
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


impl TryParse for ForToken {
    type Output = ForToken;
    type Err = ForTokenErr;

    fn try_parse(code_lines_iterator: &mut Peekable<Iter<CodeLine>>) -> anyhow::Result<Self::Output, Self::Err> {
        let for_header = *code_lines_iterator
            .peek()
            .ok_or(ForTokenErr::EmptyIterator(EmptyIteratorErr))?;

        let split_alloc = for_header.split(vec![' ']);
        let split_ref = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();
        let test = dyck_language(&split_ref.join(" ").to_string(), [vec![], vec![';'], vec![]])?;

        let mut split_ref: Vec<&str> = vec![];
        let mut split = test[0].splitn(3, ' ').collect::<Vec<_>>();

        split.iter().for_each(|a| split_ref.push(a));
        split_ref.push(";");

        split_ref.push(&test[1]);
        split_ref.push(";");

        let mut split = test[2].rsplitn(3, ' ').collect::<Vec<_>>();
        split.reverse();
        split.iter().for_each(|a| split_ref.push(a));

        let mut tokens = vec![];
        if let ["for", "(", initialization, ";", condition, ";", update, ")", "{"] = &split_ref[..] {
            let initialization = VariableToken::<'=', ';'>::try_parse(&CodeLine::imaginary(&format!("{} ;", initialization)))?;
            let condition = AssignableToken::from_str(condition)?;
            let update = VariableToken::<'=', ';'>::try_parse(&CodeLine::imaginary(&format!("{} ;", update)))?;

            // consume the header
            let _ = code_lines_iterator.next();
            while code_lines_iterator.peek().is_some() {
                let token = Scope::try_parse(code_lines_iterator).map_err(ForTokenErr::ScopeErrorErr)?;

                if let Token::ScopeClosing(_) = token {
                    break;
                }

                tokens.push(token);
            }

            return Ok(ForToken {
                initialization,
                condition,
                update,
                stack: tokens,
                code_line: for_header.clone(),
            });
        }

        Err(ForTokenErr::PatternNotMatched {
            target_value: for_header.line.to_string(),
        })
    }
}

impl ToASM for ForToken {
    fn to_asm<T: ASMOptions + 'static>(&self, _stack: &mut Stack, _meta: &mut MetaInfo, _options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        todo!()
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, meta: &mut MetaInfo) -> usize {
        self.initialization.byte_size(meta)
    }

    fn before_label(&self, stack: &mut Stack, meta: &mut MetaInfo) -> Option<Result<String, ASMGenerateError>> {
        let mut target = String::new();
        let mut has_before_label_asm = false;
        let count_before = stack.label_count;

        if let Some(before_label) = self.initialization.before_label(stack, meta) {
            match before_label {
                Ok(before_label) => {
                    target += &ASMBuilder::line(&(before_label));
                    has_before_label_asm = true;
                }
                Err(err) => return Some(Err(err))
            }

            stack.label_count -= 1;
        }

        if let Some(before_label) = self.condition.before_label(stack, meta) {
            match before_label {
                Ok(before_label) => {
                    target += &ASMBuilder::line(&(before_label));
                    has_before_label_asm = true;
                }
                Err(err) => return Some(Err(err))
            }

            stack.label_count -= 1;
        }

        if let Some(before_label) = self.update.before_label(stack, meta) {
            match before_label {
                Ok(before_label) => {
                    target += &ASMBuilder::line(&(before_label));
                    has_before_label_asm = true;
                }
                Err(err) => return Some(Err(err))
            }

            stack.label_count -= 1;
        }

        stack.label_count = count_before;


        if has_before_label_asm {
            Some(Ok(target))
        } else {
            None
        }
    }
}
use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::{ASMResult, ASMResultError, ASMResultVariance};
use crate::core::code_generator::generator::Stack;
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::errors::EmptyIteratorErr;
use crate::core::lexer::scope::{PatternNotMatchedError, Scope, ScopeError};
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::token::Token;
use crate::core::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::core::lexer::tokens::assignable_tokens::method_call_token::{dyck_language, DyckError};
use crate::core::lexer::tokens::variable_token::{ParseVariableTokenErr, VariableToken};
use crate::core::lexer::{Lines, TryParse};
use crate::core::lexer::types::type_token::{InferTypeError, Mutability, TypeToken};
use crate::core::type_checker::{InferType, StaticTypeCheck};
use crate::core::type_checker::static_type_checker::{static_type_check, static_type_check_rec, StaticTypeCheckError};

#[derive(Debug, PartialEq, Clone)]
pub struct ForToken {
    pub initialization: VariableToken<'=', ';'>,
    pub condition: AssignableToken,
    pub update: VariableToken<'=', ';'>,
    pub stack: Vec<Token>,
    pub code_line: CodeLine,
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
        let mut scope = String::new();
        self.stack.iter().for_each(|a| scope += &format!("\t{}\n", a));
        write!(f, "for ({}; {}; {}) \n{scope}", self.initialization, self.condition, self.update)
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

impl Error for ForTokenErr {}

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

impl StaticTypeCheck for ForToken {
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        // add for header variables
        type_context.context.push(self.initialization.clone());

        let variables_len = type_context.context.len();
        let condition_type = self.condition.infer_type_with_context(type_context, &self.code_line)?;

        if !matches!(condition_type, TypeToken::Bool(_)) {
            return Err(StaticTypeCheckError::InferredError(InferTypeError::MismatchedTypes {
                expected: TypeToken::Bool(Mutability::Immutable),
                actual: condition_type,
                code_line: self.code_line.clone(),
            }));
        }

        static_type_check(&Scope {
            tokens: vec![
                Token::Variable(self.initialization.clone()),
                Token::Variable(self.update.clone()),
            ],
        })?;

        if self.update.define {
            return Err(StaticTypeCheckError::InferredError(InferTypeError::DefineNotAllowed(self.update.clone(), self.code_line.clone())));
        }

        static_type_check_rec(&self.stack, type_context)?;

        let amount_pop = type_context.context.len() - variables_len;

        for _ in 0..amount_pop {
            let _ = type_context.context.pop();
        }
        
        Ok(())
    }
}

impl TryParse for ForToken {
    type Output = ForToken;
    type Err = ForTokenErr;

    fn try_parse(code_lines_iterator: &mut Lines<'_>) -> anyhow::Result<Self::Output, Self::Err> {
        let for_header = *code_lines_iterator
            .peek()
            .ok_or(ForTokenErr::EmptyIterator(EmptyIteratorErr))?;

        let split_alloc = for_header.split(vec![' ']);
        let split_ref = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();
        let split_values = dyck_language(&split_ref.join(" ").to_string(), [vec![], vec![';'], vec![]])?;

        if split_values.len() != 3 {
            return Err(ForTokenErr::PatternNotMatched {
                target_value: for_header.line.clone(),
            })
        }

        let mut split_ref: Vec<&str> = vec![];
        let split = split_values[0].splitn(3, ' ').collect::<Vec<_>>();

        split.iter().for_each(|a| split_ref.push(a));
        split_ref.push(";");

        split_ref.push(&split_values[1]);
        split_ref.push(";");

        let mut split = split_values[2].rsplitn(3, ' ').collect::<Vec<_>>();
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
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        let label1 = stack.create_label();
        let label2 = stack.create_label();
        let mut target = String::new();

        target += &ASMBuilder::ident(&ASMBuilder::comment_line(&format!("for ({}; {}; {})", self.initialization, self.condition, self.update)));
        let _ = self.initialization.to_asm(stack, meta, options.clone())?
            .apply_with(&mut target)
            .allow(ASMResultVariance::Inline)
            .allow(ASMResultVariance::MultilineResulted)
            .allow(ASMResultVariance::Multiline)
            .token("for")
            .finish()?;

        target += &ASMBuilder::ident_line(&format!("jmp {label1}"));


        target += &ASMBuilder::line(&format!("{label2}:"));

        target += &stack.generate_scope(&self.stack, meta, options.clone())?;

        let _ = self.update.to_asm(stack, meta, options.clone())?
            .apply_with(&mut target)
            .allow(ASMResultVariance::Inline)
            .allow(ASMResultVariance::MultilineResulted)
            .allow(ASMResultVariance::Multiline)
            .token("for")
            .finish()?;

        target += &ASMBuilder::line(&format!("{label1}:"));
        let general_purpose_register = self.condition.to_asm(stack, meta, options.clone())?
            .apply_with(&mut target)
            .allow(ASMResultVariance::MultilineResulted)
            .token("for")
            .finish()?;

        if let Some(general_purpose_register) = general_purpose_register {
            target += &ASMBuilder::ident_line(&format!("cmp {general_purpose_register}, 0"));
            target += &ASMBuilder::ident_line(&format!("jne {label2}"));
        } else {
            return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                expected: vec![ASMResultVariance::MultilineResulted],
                actual: ASMResultVariance::from(&self.condition.to_asm(stack, meta, options)?),
                token: "for".to_string(),
            }));
        }


        Ok(ASMResult::Multiline(target))
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, meta: &mut MetaInfo) -> usize {
        self.initialization.byte_size(meta)
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

        if self.initialization.data_section(stack, meta) {
            has_before_label_asm = true;
            stack.label_count -= 1;
        }

        if self.condition.data_section(stack, meta) {
            has_before_label_asm = true;
            stack.label_count -= 1;
        }

        if self.update.data_section(stack, meta) {
            has_before_label_asm = true;
            stack.label_count -= 1;
        }


        stack.label_count = count_before;


        has_before_label_asm
    }
}
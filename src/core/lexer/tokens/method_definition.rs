use crate::core::io::code_line::CodeLine;
use crate::core::lexer::scope::{Scope, ScopeError};
use crate::core::lexer::token::Token;
use crate::core::lexer::tokens::assignable_token::AssignableTokenErr;
use crate::core::lexer::tokens::name_token::{NameToken, NameTokenErr};
use crate::core::lexer::TryParse;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;
use std::str::FromStr;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::constants::FUNCTION_KEYWORD;
use crate::core::lexer::errors::EmptyIteratorErr;
use crate::core::lexer::levenshtein_distance::PatternedLevenshteinDistance;
use crate::core::lexer::levenshtein_distance::{ArgumentsIgnoreSummarizeTransform, EmptyParenthesesExpand, PatternedLevenshteinString, QuoteSummarizeTransform};
use crate::core::lexer::type_token::{InferTypeError, TypeToken};

/// Token for method definition. Pattern is `fn function_name(argument1, ..., argumentN): returnType { }`
#[derive(Debug, PartialEq, Clone)]
pub struct MethodDefinition {
    pub name: NameToken,
    pub return_type: TypeToken,
    pub arguments: Vec<(NameToken, TypeToken)>,
    pub stack: Vec<Token>,
    pub is_extern: bool,
    pub code_line: CodeLine
}

#[derive(Debug)]
pub enum MethodDefinitionErr {
    PatternNotMatched { target_value: String },
    NameTokenErr(NameTokenErr),
    ReturnTokenErr(InferTypeError),
    AssignableTokenErr(AssignableTokenErr),
    ScopeErrorErr(ScopeError),
    EmptyIterator(EmptyIteratorErr),
}

impl From<AssignableTokenErr> for MethodDefinitionErr {
    fn from(value: AssignableTokenErr) -> Self {
        MethodDefinitionErr::AssignableTokenErr(value)
    }
}

impl From<NameTokenErr> for MethodDefinitionErr {
    fn from(value: NameTokenErr) -> Self {
        MethodDefinitionErr::NameTokenErr(value)
    }
}

impl From<InferTypeError> for MethodDefinitionErr {
    fn from(value: InferTypeError) -> Self {
        MethodDefinitionErr::ReturnTokenErr(value)
    }
}

impl Display for MethodDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}fn {}({}): {}{}",
            if self.is_extern { "extern " } else { "" },
            self.name,
            self.arguments
                .iter()
                .map(|(name, ty)| format!("{}: {}", name, ty))
                .collect::<Vec<String>>()
                .join(", "),
            self.return_type,
            if self.is_extern { ";" } else { " {{Body}}" }
        )
    }
}

impl Error for MethodDefinitionErr {}

impl Display for MethodDefinitionErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            MethodDefinitionErr::PatternNotMatched { target_value}
            => format!("Pattern not matched for: `{target_value}`\n\t fn function_name(argument1, ..., argumentN): returnType {{ }}"),
            MethodDefinitionErr::AssignableTokenErr(a) => a.to_string(),
            MethodDefinitionErr::NameTokenErr(a) => a.to_string(),
            MethodDefinitionErr::ReturnTokenErr(a) => a.to_string(),
            MethodDefinitionErr::EmptyIterator(e) => e.to_string(),
            MethodDefinitionErr::ScopeErrorErr(a) => a.to_string(),
        })
    }
}

impl TryParse for MethodDefinition {
    type Output = MethodDefinition;
    type Err = MethodDefinitionErr;

    fn try_parse(code_lines_iterator: &mut Peekable<Iter<CodeLine>>) -> anyhow::Result<Self, MethodDefinitionErr> {
        let method_header = *code_lines_iterator
            .peek()
            .ok_or(MethodDefinitionErr::EmptyIterator(EmptyIteratorErr))?;

        let split_alloc = method_header.split(vec![' ']);
        let split_ref = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();

        // todo: check if parameter names are duplicates

        let (is_extern, fn_name, _generic_type, arguments, return_type) = match &split_ref[..] {
            ["extern", "fn", name, "<", generic_type, ">", "(", arguments @ .., ")", ":", return_type, ";"] => (true, name, Some(generic_type) ,arguments, return_type),
            ["extern", "fn", name, "(", arguments @ .., ")", ":", return_type, ";"] => (true, name, None, arguments, return_type),
            ["fn", name, "(", arguments @ .., ")", ":", return_type, "{"] => (false, name, None, arguments, return_type),
            _ => return Err(MethodDefinitionErr::PatternNotMatched { target_value: method_header.line.to_string() })
        };

        let mut tokens = vec![];
        // consume the header
        let _ = code_lines_iterator.next();

        // consume the body
        if !is_extern {
            while code_lines_iterator.peek().is_some() {
                let token = Scope::try_parse(code_lines_iterator).map_err(MethodDefinitionErr::ScopeErrorErr)?;

                if let Token::ScopeClosing(_) = token {
                    break;
                }

                tokens.push(token);
            }
        }

        Ok(MethodDefinition {
            name: NameToken::from_str(fn_name, false)?,
            return_type: TypeToken::from_str(return_type)?,
            arguments: Self::type_arguments(method_header, arguments)?,
            stack: tokens,
            is_extern,
            code_line: method_header.clone(),
        })
    }
}

impl MethodDefinition {
    fn type_arguments(method_header: &CodeLine, arguments: &[&str]) -> Result<Vec<(NameToken, TypeToken)>, MethodDefinitionErr> {
        let arguments_string = arguments.join("");
        let arguments = arguments_string.split(',').filter(|a| !a.is_empty()).collect::<Vec<_>>();
        let mut type_arguments = vec![];

        for argument in arguments {
            if let [name, ty] = &argument.split(':').collect::<Vec<&str>>()[..] {
                type_arguments.push((NameToken::from_str(name, false)?, TypeToken::from_str(ty)?));
            } else {
                return Err(MethodDefinitionErr::PatternNotMatched { target_value: method_header.line.clone() })
            }
        }

        Ok(type_arguments)
    }
}

impl ToASM for MethodDefinition {
    fn to_asm(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Result<String, ASMGenerateError> {
        todo!()
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        todo!()
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        0
    }

    fn before_label(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Option<Result<String, ASMGenerateError>> {
        None
    }
}

impl PatternedLevenshteinDistance for MethodDefinition {
    fn distance_from_code_line(code_line: &CodeLine) -> usize {
        let method_header_pattern = PatternedLevenshteinString::default()
            .insert(FUNCTION_KEYWORD)
            .insert(PatternedLevenshteinString::ignore())
            .insert("(")
            .insert(PatternedLevenshteinString::ignore())
            .insert(")")
            .insert(":")
            .insert(PatternedLevenshteinString::ignore())
            .insert("{");

        <MethodDefinition as PatternedLevenshteinDistance>::distance(
            PatternedLevenshteinString::match_to(
                &code_line.line,
                &method_header_pattern,
                vec![
                    Box::new(QuoteSummarizeTransform),
                    Box::new(EmptyParenthesesExpand),
                    Box::new(ArgumentsIgnoreSummarizeTransform)
                ]
            ),
            method_header_pattern
        )
    }
}

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;
use std::str::FromStr;
use crate::core::lexer::tokens::assignable_token::AssignableToken;
use crate::core::lexer::tokens::variable_token::VariableToken;
use crate::core::code_generator::conventions::calling_convention_from;

use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::asm_result::{ASMResult, ASMResultVariance};
use crate::core::code_generator::conventions::CallingRegister;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::registers::ByteSize;
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::errors::EmptyIteratorErr;
use crate::core::lexer::scope::{PatternNotMatchedError, Scope, ScopeError};
use crate::core::lexer::static_type_context::{CurrentMethodInfo, StaticTypeContext};
use crate::core::lexer::token::Token;
use crate::core::lexer::tokens::assignable_token::AssignableTokenErr;
use crate::core::lexer::tokens::name_token::{NameToken, NameTokenErr};
use crate::core::lexer::TryParse;
use crate::core::lexer::types::type_token::{InferTypeError, MethodCallSignatureMismatchCause, TypeToken};
use crate::core::type_checker::static_type_checker::{static_type_check_rec, StaticTypeCheckError};
use crate::core::type_checker::StaticTypeCheck;
use crate::utils::math;

/// Token for method definition. Pattern is `fn function_name(argument1, ..., argumentN): returnType { }`
#[derive(Debug, PartialEq, Clone)]
pub struct MethodDefinition {
    pub name: NameToken,
    pub return_type: TypeToken,
    pub arguments: Vec<(NameToken, TypeToken)>,
    pub stack: Vec<Token>,
    pub is_extern: bool,
    pub code_line: CodeLine,
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

impl PatternNotMatchedError for MethodDefinitionErr {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, MethodDefinitionErr::PatternNotMatched {..})
    }
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
            MethodDefinitionErr::PatternNotMatched { target_value }
            => format!("Pattern not matched for: `{target_value}`\n\t fn function_name(argument1, ..., argumentN): returnType {{ }}"),
            MethodDefinitionErr::AssignableTokenErr(a) => a.to_string(),
            MethodDefinitionErr::NameTokenErr(a) => a.to_string(),
            MethodDefinitionErr::ReturnTokenErr(a) => a.to_string(),
            MethodDefinitionErr::EmptyIterator(e) => e.to_string(),
            MethodDefinitionErr::ScopeErrorErr(a) => a.to_string(),
        })
    }
}

impl StaticTypeCheck for MethodDefinition {
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        // add the parameters to the type information
        for (argument_name, argument_type) in &self.arguments {
            type_context.context.push(VariableToken {
                name_token: argument_name.clone(),
                mutability: false,
                ty: Some(argument_type.clone()),
                define: true,
                assignable: AssignableToken::default(),
                code_line: Default::default(),
            });
        }

        let variables_len = type_context.context.len();
        type_context.expected_return_type = Some(CurrentMethodInfo {
            return_type: self.return_type.clone(),
            method_header_line: self.code_line.actual_line_number.clone(),
            method_name: self.name.name.to_string(),
        });

        static_type_check_rec(&self.stack, type_context)?;

        if self.return_type != TypeToken::Void {
            if let [.., last] = &self.stack[..] {
                let mut method_return_signature_mismatch = false;
                let mut cause = MethodCallSignatureMismatchCause::ReturnMissing;

                if let Token::If(if_definition) = &last {
                    method_return_signature_mismatch = !if_definition.ends_with_return_in_each_branch();
                    if method_return_signature_mismatch {
                        cause = MethodCallSignatureMismatchCause::IfCondition;
                    }
                } else if !matches!(last, Token::Return(_)) {
                    method_return_signature_mismatch = true;
                }

                if method_return_signature_mismatch {
                    if let Some(expected_return_type) = &type_context.expected_return_type {
                        return Err(StaticTypeCheckError::InferredError(InferTypeError::MethodReturnSignatureMismatch {
                            expected: expected_return_type.return_type.clone(),
                            method_name: expected_return_type.method_name.to_string(),
                            method_head_line: expected_return_type.method_header_line.to_owned(),
                            cause,
                        }));
                    }
                }
            }
        }


        let amount_pop = (type_context.context.len() - variables_len) + self.arguments.len();

        for _ in 0..amount_pop {
            let _ = type_context.context.pop();
        }

        type_context.expected_return_type = None;
        Ok(())
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

        let (is_extern, fn_name, _generic_type, arguments, return_type) = match &split_ref[..] {
            ["extern", "fn", name, "<", generic_type, ">", "(", arguments @ .., ")", ":", return_type, ";"] => (true, name, Some(generic_type), arguments, return_type),
            ["extern", "fn", name, "(", arguments @ .., ")", ":", return_type, ";"] => (true, name, None, arguments, return_type),
            ["fn", name, "(", arguments @ .., ")", ":", return_type, "{"] => (false, name, None, arguments, return_type),

            ["extern", "fn", name, "<", generic_type, ">", "(", arguments @ .., ")", ":", ";"] => (true, name, Some(generic_type), arguments, &"void"),
            ["extern", "fn", name, "(", arguments @ .., ")", ";"] => (true, name, None, arguments, &"void"),
            ["fn", name, "(", arguments @ .., ")", "{"] => (false, name, None, arguments, &"void"),
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
                return Err(MethodDefinitionErr::PatternNotMatched { target_value: method_header.line.clone() });
            }
        }

        Ok(type_arguments)
    }

    pub fn method_label_name(&self) -> String {
        if self.name.name == "main" {
            return "main".to_string();
        }

        let parameters = if self.arguments.is_empty() {
            "void".to_string()
        } else {
            self.arguments.iter().map(|a| a.1.to_string()).collect::<Vec<String>>().join("_")
        }.replace('*', "ptr");


        let return_type = self.return_type.to_string().replace('*', "ptr");

        format!(".{}_{}~{}", self.name, parameters, return_type)
    }
}

impl ToASM for MethodDefinition {
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        let mut label_header: String = String::new();


        label_header += &ASMBuilder::line(&format!("{}:", self.method_label_name()));
        label_header += &ASMBuilder::ident_line("push rbp");
        label_header += &ASMBuilder::mov_ident_line("rbp", "rsp");


        label_header += &ASMBuilder::ident(&ASMBuilder::comment_line("Reserve stack space as MS convention. Shadow stacking"));
        let mut stack_allocation = 32; // per default microsoft convention requires 32 byte as a shadow stack
        let mut method_scope: String = String::new();

        let calling_convention = calling_convention_from(self, &meta.target_os);

        for (index, (argument_name, argument_type)) in self.arguments.iter().enumerate() {
            if let Some(stack_location) = stack.variables.iter().rfind(|v| v.name.name == argument_name.name) {
                let destination = stack_location.name.clone().to_asm(stack, meta, options.clone())?;
                let source = match &calling_convention[index][0] {
                    CallingRegister::Register(r) => {
                        if matches!(argument_type, TypeToken::Float(_)) {
                            r.to_string()
                        } else {
                            r.to_size_register(&ByteSize::try_from(argument_type.byte_size())?).to_string()
                        }
                    }
                    CallingRegister::Stack => "popppp".to_string()
                };

                method_scope.push_str(&ASMBuilder::mov_x_ident_line(destination, source, if let TypeToken::Float(f) = &argument_type {
                    Some(f.byte_size())
                } else {
                    None
                }));
            } else {
                return Err(ASMGenerateError::UnresolvedReference { name: argument_name.name.to_string(), code_line: self.code_line.clone()})
            }
        }

        meta.static_type_information.expected_return_type = Some(CurrentMethodInfo {
            return_type: self.return_type.clone(),
            method_header_line: self.code_line.actual_line_number.clone(),
            method_name: self.name.name.to_string(),
        });

        for token in &self.stack {
            meta.code_line = token.code_line();
            stack_allocation += token.byte_size(meta);


            let variables_len = meta.static_type_information.len();

            if let Some(scope_stacks) = token.scope() {
                for scope_stack in scope_stacks {
                    meta.static_type_information.merge(StaticTypeContext::new(scope_stack));
                }
            }

            let _ = token.to_asm::<InterimResultOption>(stack, meta, None)?
                .apply_with(&mut method_scope)
                .allow(ASMResultVariance::Inline)
                .allow(ASMResultVariance::MultilineResulted)
                .allow(ASMResultVariance::Multiline)
                .token("method definition")
                .finish()?;

            token.data_section(stack, meta);

            let amount_pop = meta.static_type_information.len() - variables_len;

            for _ in 0..amount_pop {
                let _ = meta.static_type_information.pop();
            }
        }

        meta.static_type_information.expected_return_type = None;

        let stack_allocation_asm = ASMBuilder::ident_line(&format!("sub rsp, {}", math::lowest_power_of_2_gt_n(stack_allocation)));
        let leave_statement = if self.return_type == TypeToken::Void { "    leave\n    ret\n".to_string() } else { String::new() };

        Ok(ASMResult::Multiline(format!("{}{}{}{}", label_header, stack_allocation_asm, method_scope, leave_statement)))
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        true
    }

    fn byte_size(&self, _meta: &mut MetaInfo) -> usize {
        0
    }
}

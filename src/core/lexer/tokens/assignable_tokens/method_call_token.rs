use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;
use std::str::FromStr;
use crate::core::code_generator::{ASMGenerateError, conventions, MetaInfo};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::conventions::CallingRegister;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::target_os::TargetOS;
use crate::core::code_generator::ToASM;
use crate::core::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::core::lexer::tokens::name_token::{NameToken, NameTokenErr};
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::errors::EmptyIteratorErr;
use crate::core::lexer::levenshtein_distance::{ArgumentsIgnoreSummarizeTransform, EmptyParenthesesExpand, PatternedLevenshteinDistance, PatternedLevenshteinString, QuoteSummarizeTransform};
use crate::core::lexer::tokenizer::StaticTypeContext;
use crate::core::lexer::TryParse;
use crate::core::lexer::type_token::{InferTypeError, MethodCallArgumentTypeMismatch};

#[derive(Debug, PartialEq, Clone)]
pub struct MethodCallToken {
    pub name: NameToken,
    pub arguments: Vec<AssignableToken>,
    pub code_line: CodeLine
}

impl Display for MethodCallToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.arguments
            .iter()
            .map(|ass| format!("{}", ass))
            .collect::<Vec<String>>()
            .join(", "))
    }
}

#[derive(Debug)]
pub enum MethodCallTokenErr {
    PatternNotMatched { target_value: String },
    NameTokenErr(NameTokenErr),
    DyckLanguageErr { target_value: String, ordering: Ordering },
    AssignableTokenErr(AssignableTokenErr),
    EmptyIterator(EmptyIteratorErr)
}

impl std::error::Error for MethodCallTokenErr {}

impl From<NameTokenErr> for MethodCallTokenErr {
    fn from(value: NameTokenErr) -> Self { MethodCallTokenErr::NameTokenErr(value) }
}

impl From<AssignableTokenErr> for MethodCallTokenErr {
    fn from(value: AssignableTokenErr) -> Self { MethodCallTokenErr::AssignableTokenErr(value) }
}

impl From<DyckError> for MethodCallTokenErr {
    fn from(s: DyckError) -> Self {
        MethodCallTokenErr::DyckLanguageErr { target_value: s.target_value, ordering: s.ordering }
    }
}

impl Display for MethodCallTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            MethodCallTokenErr::PatternNotMatched { target_value } => format!("\"{target_value}\" must match: methodName(assignable1, ..., assignableN)"),
            MethodCallTokenErr::AssignableTokenErr(a) => a.to_string(),
            MethodCallTokenErr::NameTokenErr(a) => a.to_string(),
            MethodCallTokenErr::DyckLanguageErr { target_value, ordering } =>
            {
                let error: String = match ordering {
                    Ordering::Less => String::from("Expected `)`"),
                    Ordering::Equal => String::from("Expected expression between `,`"),
                    Ordering::Greater => String::from("Expected `(`")
                };
                format!("\"{target_value}\": {error}")
            }
            MethodCallTokenErr::EmptyIterator(e) => e.to_string()
        };

        write!(f, "{}", message)
    }
}

impl FromStr for MethodCallToken {
    type Err = MethodCallTokenErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut code_line = CodeLine::imaginary(s);

        if !s.ends_with(';') {
            code_line.line += " ;";
        }

        MethodCallToken::try_parse(&code_line)
    }
}

impl TryParse for MethodCallToken {
    type Output = MethodCallToken;
    type Err = MethodCallTokenErr;

    fn try_parse(code_lines_iterator: &mut Peekable<Iter<CodeLine>>) -> anyhow::Result<Self::Output, Self::Err> {
        let code_line = *code_lines_iterator.peek().ok_or(MethodCallTokenErr::EmptyIterator(EmptyIteratorErr))?;
        MethodCallToken::try_parse(code_line)
    }
}

impl MethodCallToken {
    pub fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self, MethodCallTokenErr> {
        let split_alloc = code_line.split(vec![' ', ';']);
        let split = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();

        if let [name, "(", ")", ";"] = &split[..] {
            Ok(MethodCallToken {
                name: NameToken::from_str(name, false)?,
                arguments: vec![],
                code_line: code_line.clone(),
            })
        } else if let [name, "(", argument_segments @ .., ")", ";"] = &split[..] {
            let joined = &argument_segments.join(" ");
            let argument_strings = dyck_language(joined, [vec!['{', '('], vec![','], vec!['}', ')']])?;

            let arguments = argument_strings
                .iter()
                .map(|s| AssignableToken::from_str(s))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(MethodCallToken {
                name: NameToken::from_str(name, false)?,
                arguments,
                code_line: code_line.clone()
            })
        } else {
            Err(MethodCallTokenErr::PatternNotMatched { target_value: code_line.line.to_string() })
        }
    }

    pub fn type_check(&self, context: &StaticTypeContext, code_line: &CodeLine) -> Result<(), InferTypeError> {
        if let Some(method_def) = context.methods.iter().find(|m| m.name == self.name) {
            if method_def.arguments.len() != self.arguments.len() {
                return Err(InferTypeError::MethodCallArgumentAmountMismatch {
                    expected: method_def.arguments.len(),
                    actual: self.arguments.len(),
                    code_line: self.code_line.clone()
                })
            }

            let zipped = method_def.arguments
                .iter()
                .zip(&self.arguments);

            for (index, (argument_def, argument_call)) in zipped.enumerate() {
                let def_type = argument_def.1.clone();
                let call_type = argument_call.infer_type_with_context(context, code_line)?;

                if def_type != call_type {
                    return Err(InferTypeError::MethodCallArgumentTypeMismatch {
                        info: Box::new(MethodCallArgumentTypeMismatch {
                            expected: def_type,
                            actual: call_type,
                            nth_parameter: index + 1,
                            code_line: code_line.clone()
                        })
                    })
                }
            }


            return Ok(())
        }

        Err(InferTypeError::UnresolvedReference(self.name.name.clone(), code_line.clone()))
    }
}

impl ToASM for MethodCallToken {
    fn to_asm(&self, stack: &mut Stack, meta: &mut MetaInfo) -> Result<String, crate::core::code_generator::ASMGenerateError> {
        let calling_convention = conventions::calling_convention(meta, &self.arguments)?;

        let mut result = String::new();

        let zipped = calling_convention.iter().zip(&self.arguments);
        for (convention, argument) in zipped {
            let parsed_argument = match argument {
                AssignableToken::ArithmeticEquation(_) | AssignableToken::BooleanEquation(_) => {
                    result += &ASMBuilder::push(&argument.to_asm(stack, meta)?);
                    String::from("rax")
                }
                _ => {
                    argument.to_asm(stack, meta)?.to_string()
                }
            };

            match meta.target_os {
                TargetOS::Windows => {
                    match convention {
                        CallingRegister::Register(register) =>
                            result += &ASMBuilder::ident(&format!("mov {}, {}", register, parsed_argument.replace("DWORD", "QWORD"))),
                        CallingRegister::Stack =>
                            result += &ASMBuilder::ident(&format!("push {}", parsed_argument.replace("DWORD", "QWORD")))
                    }
                    result += &ASMBuilder::push(" ");
                    result += &ASMBuilder::comment_line(&format!("Parameter ({})", argument));
                }
                TargetOS::WindowsSubsystemLinux | TargetOS::Linux => {
                    unimplemented!("Not implemented for linux yet");
                }
            }
        }
        result += &ASMBuilder::ident(&ASMBuilder::comment_line(&self.to_string()));
        result += &ASMBuilder::ident_line(&format!("call {}", self.name));

        return Ok(result);
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        true
    }

    fn byte_size(&self, meta: &mut MetaInfo) -> usize {
        return if let Some(method_def) = meta.static_type_information.methods.iter().find(|m| m.name == self.name) {
            method_def.return_type.byte_size()
        } else {
            0
        }
    }

    fn before_label(&self, stack: &mut Stack, meta: &mut MetaInfo) -> Option<Result<String, ASMGenerateError>> {
        let mut asb = String::new();
        let mut has_before_label_asm = false;
        for argument in &self.arguments {
            if let Some(before_label) = argument.before_label(stack, meta) {
                match before_label {
                    Ok(before_label) => {
                        asb += &ASMBuilder::line(&(before_label));
                        has_before_label_asm = true;
                    },
                    Err(err) => return Some(Err(err))
                }
            }
        }

        return if has_before_label_asm { Some(Ok(asb)) } else { None }
    }
}

impl PatternedLevenshteinDistance for MethodCallToken {
    fn distance_from_code_line(code_line: &CodeLine) -> usize {
        let method_call_pattern = PatternedLevenshteinString::default()
            .insert(PatternedLevenshteinString::ignore())
            .insert("(")
            .insert(PatternedLevenshteinString::ignore())
            .insert(")")
            .insert(";");


        <MethodCallToken as PatternedLevenshteinDistance>::distance(
            PatternedLevenshteinString::match_to(
                &code_line.line,
                &method_call_pattern,
                vec![
                    Box::new(QuoteSummarizeTransform),
                    Box::new(EmptyParenthesesExpand),
                    Box::new(ArgumentsIgnoreSummarizeTransform)
                ],
            ),
            method_call_pattern,
        )
    }
}


#[derive(Debug)]
pub struct DyckError {
    pub target_value: String,
    pub ordering: Ordering
}

pub trait ArrayOrObject<T> {
    fn list(&self) -> Vec<T>;
}

impl ArrayOrObject<char> for char {
    fn list(&self) -> Vec<char> {
        vec![*self]
    }
}

impl ArrayOrObject<char> for Vec<char> {
    fn list(&self) -> Vec<char> {
        self.clone()
    }
}


/// # Formal definition
/// Let Σ = {( ) [a-z A-Z]}
///
/// {u ∈ Σ* | all prefixes of u contain no more )'s than ('s and the number of ('s in equals the number of )'s }
pub fn dyck_language<T: ArrayOrObject<char>>(parameter_string: &str, values: [T; 3]) -> Result<Vec<String>, DyckError> {
    let mut individual_parameters: Vec<String> = Vec::new();
    let mut counter = 0;
    let mut current_start_index = 0;

    for (index, c) in parameter_string.chars().enumerate() {
        if values[0].list().contains(&c) { // opening
                counter += 1;
        } else if values[2].list().contains(&c) { // closing
            counter -= 1;
        } else if values[1].list().contains(&c) && counter == 0 { // seperator
            let value = &parameter_string[current_start_index..index].trim();
        
            if value.is_empty() {
                return Err(DyckError {
                    target_value: parameter_string.to_string(),
                    ordering: Ordering::Equal
                });
            }
        
            individual_parameters.push(value.to_string());
            current_start_index = index + 1;
        }
    }

    return match counter {
        number if number > 0 => Err(DyckError {
            target_value: parameter_string.to_string(),
            ordering: Ordering::Less
        }),
        number if number < 0 => return Err(DyckError {
            target_value: parameter_string.to_string(),
            ordering: Ordering::Greater
        }),
        _ => {
            let s = parameter_string[current_start_index..parameter_string.len()].trim().to_string();
            if !s.is_empty() {
                individual_parameters.push(parameter_string[current_start_index..parameter_string.len()].trim().to_string());
            }

            Ok(individual_parameters)
        }
    }
}

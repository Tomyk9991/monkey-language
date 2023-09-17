use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::slice::Iter;
use std::str::FromStr;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::target_os::TargetOS;
use crate::core::code_generator::ToASM;
use crate::core::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::core::lexer::tokens::name_token::{NameToken, NameTokenErr};
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::errors::EmptyIteratorErr;
use crate::core::lexer::levenshtein_distance::{ArgumentsIgnoreSummarizeTransform, EmptyParenthesesExpand, PatternedLevenshteinDistance, PatternedLevenshteinString, QuoteSummarizeTransform};
use crate::core::lexer::token::Token;
use crate::core::lexer::TryParse;

#[derive(Debug, PartialEq, Clone)]
pub struct MethodCallToken {
    pub name: NameToken,
    pub arguments: Vec<AssignableToken>,
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
            })
        } else {
            Err(MethodCallTokenErr::PatternNotMatched { target_value: code_line.line.to_string() })
        }
    }
}

impl ToASM for MethodCallToken {
    fn to_asm(&self, stack: &mut Stack, target_os: &TargetOS) -> Result<String, crate::core::code_generator::Error> {
        // TODO finish properly. For Now just a method "exit" is supported to early return
        if self.name.name == "exit" {
            let mut result = String::new();

            let parsed_argument = &self.arguments[0].to_asm(stack, target_os)?.to_string();
            result.push_str(parsed_argument);
            result.push_str(&stack.pop_stack("rax"));
            result.push_str(&format!("    ; {}\n", self));

            result.push_str(&stack.push_stack("rax"));


            match target_os {
                TargetOS::WindowsSubsystemLinux | TargetOS::Linux => {
                    result.push_str("    mov rax, 60\n");
                    result.push_str(&stack.pop_stack("rdi"));
                    result.push_str("    syscall\n");
                }
                TargetOS::Windows => {
                    result.push_str("    mov rcx, rax\n");
                    result.push_str("    call ExitProcess\n");
                }
            }

            return Ok(result);
        }


        let method_call_token = Token::MethodCall(self.clone());
        Err(crate::core::code_generator::Error::NotImplemented { token: format!("{}", method_call_token) })
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

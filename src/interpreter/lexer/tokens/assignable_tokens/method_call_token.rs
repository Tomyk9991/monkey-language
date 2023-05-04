use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::interpreter::lexer::tokens::assignable_token::{AssignableToken, AssignableTokenErr};
use crate::interpreter::lexer::tokens::name_token::{NameToken, NameTokenErr};
use crate::interpreter::lexer::Visibility;
use crate::interpreter::constants::KEYWORDS;
use crate::interpreter::io::code_line::CodeLine;
use crate::interpreter::lexer::levenshtein_distance::{ArgumentsIgnoreSummarizeTransform, EmptyMethodCallExpand, PatternedLevenshteinDistance, PatternedLevenshteinString, QuoteSummarizeTransform};
use crate::interpreter::lexer::tokens::variable_token::VariableToken;

#[derive(Debug)]
pub struct MethodCallToken {
    name: NameToken,
    arguments: Vec<AssignableToken>,
}

#[derive(Debug)]
pub enum MethodCallTokenErr {
    PatternNotMatched { target_value: String },
    NameTokenErr(NameTokenErr),
    DyckLanguageErr { target_value: String, ordering: Ordering },
    AssignableTokenErr(AssignableTokenErr),
}

impl Error for MethodCallTokenErr {}

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
        };

        write!(f, "{}", message)
    }
}

impl FromStr for MethodCallToken {
    type Err = MethodCallTokenErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut code_line = CodeLine::imaginary(s);

        if !s.ends_with(";") {
            code_line.line += " ;";
        }

        MethodCallToken::try_parse(&code_line)
    }
}

impl MethodCallToken {
    pub fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self, MethodCallTokenErr> {
        let split_alloc = code_line.split(vec![' ', ';']);
        let split = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();

        return if let [name, "(", ")", ";"] = &split[..] {
            Ok(MethodCallToken {
                name: NameToken::from_str(name)?,
                arguments: vec![],
            })
        } else if let [name, "(", argument_segments @ .., ")", ";"] = &split[..] {
            let argument_strings = dyck_language(&argument_segments.join(" "), ['(', ',', ')'])?;
            let arguments = argument_strings
                .iter()
                .map(|s| AssignableToken::try_from(s))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(MethodCallToken {
                name: NameToken::from_str(name)?,
                arguments,
            })
        } else {
            Err(MethodCallTokenErr::PatternNotMatched { target_value: code_line.line.to_string() })
        };
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
                    Box::new(EmptyMethodCallExpand),
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
        return vec![self.clone()];
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
        if values[0].list().contains(&c) {
                counter += 1;
        } else if values[2].list().contains(&c) {
            counter -= 1;
        } else if values[1].list().contains(&c) && counter == 0 {
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
            individual_parameters.push(parameter_string[current_start_index..parameter_string.len()].trim().to_string());
            Ok(individual_parameters)
        }
    }
}

use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::interpreter::io::code_line::CodeLine;
use crate::interpreter::lexer::tokens::assignable_token::AssignableTokenErr;
use crate::interpreter::lexer::tokens::assignable_tokens::method_call_token::{dyck_language, DyckError};
use crate::interpreter::lexer::tokens::name_token::NameTokenErr;
use crate::interpreter::lexer::tokens::variable_token::{ParseVariableTokenErr, VariableToken};

#[derive(Debug, PartialEq)]
pub struct ObjectToken {
    pub variables: Vec<VariableToken<':', ','>>
}

impl Display for ObjectToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}}}", self.variables.iter().map(|var| format!("{}", var)).collect::<Vec<String>>().join(", "))
    }
}

#[derive(Debug)]
pub enum ObjectTokenErr {
    PatternNotMatched { target_value: String },
    NameTokenErr(NameTokenErr),
    DyckLanguageErr { target_value: String, ordering : Ordering },
    AssignableTokenErr(AssignableTokenErr),
    ParseVariableTokenErr(ParseVariableTokenErr)
}

impl Error for ObjectTokenErr { }

impl From<NameTokenErr> for ObjectTokenErr {
    fn from(err: NameTokenErr) -> Self { ObjectTokenErr::NameTokenErr(err) }
}

impl From<AssignableTokenErr> for ObjectTokenErr {
    fn from(value: AssignableTokenErr) -> Self { ObjectTokenErr::AssignableTokenErr(value) }
}

impl From<ParseVariableTokenErr> for ObjectTokenErr {
    fn from(s: ParseVariableTokenErr) -> Self {
        ObjectTokenErr::ParseVariableTokenErr(s)
    }
}

impl From<DyckError> for ObjectTokenErr {
    fn from(s: DyckError) -> Self {
        ObjectTokenErr::DyckLanguageErr { target_value: s.target_value, ordering: s.ordering }
    }
}

impl Display for ObjectTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ObjectTokenErr::PatternNotMatched { target_value } => format!("\"{target_value}\" must match: methodName(assignable1, ..., assignableN)"),
            ObjectTokenErr::AssignableTokenErr(a) => a.to_string(),
            ObjectTokenErr::NameTokenErr(a) => a.to_string(),
            ObjectTokenErr::DyckLanguageErr { target_value, ordering } =>
                {
                    let error: String = match ordering {
                        Ordering::Less => String::from("Expected `)`"),
                        Ordering::Equal => String::from("Expected expression between `,`"),
                        Ordering::Greater => String::from("Expected `(`")
                    };
                    format!("\"{target_value}\": {error}")
                }
            ObjectTokenErr::ParseVariableTokenErr(err) => err.to_string()
        };

        write!(f, "{}", message)
    }
}

impl FromStr for ObjectToken {
    type Err = ObjectTokenErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut code_line = CodeLine::imaginary(s);
        
        if !s.ends_with(';') {
            code_line.line += " ;";
        }
        
        ObjectToken::try_parse(&code_line)
    }
}

impl ObjectToken {
    pub fn try_parse(code_line: &CodeLine) -> anyhow::Result<Self, ObjectTokenErr> {
        let split_alloc = code_line.split(vec![' ', ';']);
        let split = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();
        
        return if let ["{", arguments_segments @ .., "}", ";"] = &split[..] {
            let mut argument_strings = dyck_language(&arguments_segments.join(" "),[vec!['{', '('], vec![','], vec!['}', ')']])?;
            argument_strings.iter_mut().for_each(|s|
                if !s.ends_with(',') {
                    s.push_str(" ,")
                }
            );

            let arguments = argument_strings
                .iter()
                .map(|s| VariableToken::try_parse(&CodeLine::imaginary(s)))
                .collect::<Result<Vec<_>, _>>()?;
            
            Ok(ObjectToken {
                variables: arguments,
            })
        } else {
            Err(ObjectTokenErr::PatternNotMatched { target_value: code_line.line.to_string() })
        };
    }
}
use std::env;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub struct ProgramVariable<const PARAMETER_INCLUDED: bool> {
    pub key: String,
    value: Option<String>
}

#[derive(Debug)]
pub enum ProgramVariableErr {
    NotFound(String)
}

impl Display for ProgramVariableErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ProgramVariableErr::NotFound(value) => {
                format!("Couldn't find the argument: {}", value)
            }
        })
    }
}

impl Error for ProgramVariableErr { }

impl TryFrom<Vec<&'static str>> for ProgramVariable<false> {
    type Error = ProgramVariableErr;

    fn try_from(target: Vec<&'static str>) -> Result<Self, Self::Error> {
        let args = env::args().collect::<Vec<_>>();

        for arg in &args {
            if target.contains(&arg.to_lowercase().as_str()) {
                return Ok(ProgramVariable::<false> {
                    key: arg.to_string(),
                    value: None,
                })
            }
        }

        Err(ProgramVariableErr::NotFound(target.join(", ")))
    }
}

impl ProgramVariable<true> {
    pub fn get_value(&self) -> String {
        return self.value.clone().unwrap_or(String::from(""));
    }
}

impl TryFrom<Vec<&'static str>> for ProgramVariable<true> {
    type Error = ProgramVariableErr;

    fn try_from(target: Vec<&'static str>) -> Result<Self, Self::Error> {
        let args = env::args().collect::<Vec<_>>();

        for key_value_pair in args.windows(2).skip(1) {
            if let [key, value] = key_value_pair {
                if target.contains(&key.to_lowercase().as_str()) {
                    return Ok(ProgramVariable {
                        key: key.to_string(),
                        value: Some(value.to_string())
                    });
                }
            }
        }

        Err(ProgramVariableErr::NotFound(target.join(", ")))
    }
}
use std::env;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub struct ProgramVariable {
    pub key: String,
    pub value: String
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

impl TryFrom<Vec<&'static str>> for ProgramVariable {
    type Error = ProgramVariableErr;

    fn try_from(target: Vec<&'static str>) -> Result<Self, Self::Error> {
        let args = env::args().collect::<Vec<_>>();

        for key_value_pair in args.windows(2).skip(1) {
            if let [key, value] = &key_value_pair[..] {
                if target.contains(&key.to_lowercase().as_str()) {
                    return Ok(ProgramVariable {
                        key: key.to_string(),
                        value: value.to_string()
                    });
                }
            }
        }

        return Err(ProgramVariableErr::NotFound(target.join(", ")));
    }
}
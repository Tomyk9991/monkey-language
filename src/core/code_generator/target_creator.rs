use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::core::code_generator::target_os::TargetOS;


#[derive(Debug)]
pub enum Error {
    NotInDirectory
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

pub struct TargetCreator {
    pub path_to_target_directory: String,
}

impl TargetCreator {
    pub fn compile(&self, target_os: TargetOS) -> i32 {
        target_os.compile(self)
    }
    
    pub fn execute(&self, target_os: &TargetOS) -> i32 {
        target_os.execute(self)
    }
}

pub trait CompileAndExecute {
    fn compile(&self, target_creator: &TargetCreator) -> i32;
    fn execute(&self, target_creator: &TargetCreator) -> i32;
}

impl TargetCreator {
    pub fn write_to(&self, file_name: &str, content: &str) -> std::io::Result<()> {
        if !Path::new(&self.path_to_target_directory).is_dir() {
            std::fs::create_dir_all(&self.path_to_target_directory)?;
        }

        let mut file = File::create(format!("{}/{}", self.path_to_target_directory, file_name))?;
        file.write_all(content.as_bytes())
    }
}

impl TryFrom<(&str, &TargetOS)> for TargetCreator {
    type Error = Error;

    /// Creates a `TargetCreator` instance from the main file in the same directory
    fn try_from((value, target_os): (&str, &TargetOS)) -> Result<Self, Self::Error> {
        let path = Path::new(value);

        if let Some(parent_path) = path.parent() {
            return Ok(TargetCreator {
                path_to_target_directory: parent_path.display().to_string() + "/target/" + &format!("{:?}", target_os)
            });
        }

        Err(Error::NotInDirectory)
    }
}
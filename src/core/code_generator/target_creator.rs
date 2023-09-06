use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{Command, ExitStatus};

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
    pub fn compile_and_execute(&self, wsl: bool) -> i32 {
        println!("Compiling...");

        let mut status: Option<ExitStatus> = None;


        let output = Command::new("cmd")
            .output()
            .expect("Failed to execute command");

        if !output.stderr.is_empty() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Error: \n{}", stderr);

            return 1;
        }


        let output = if wsl {
            Command::new("wsl")
                .args(vec!["nasm", "-felf64", "main.asm", "&&", "ld", "-o", "main", "main.o", ";", "./main"])
                .output()
                .expect("Failed to execute command")
        } else {
            Command::new("nasm")
                .args(vec!["-felf64", "main.asm", "&&", "ld", "-o", "main", "main.o", ";", "./main"])
                .output()
                .expect("Failed to execute command")
        };

        if !output.stdout.is_empty() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{}", stdout);
        }

        status = Some(output.status);


        if !output.stderr.is_empty() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Error: \n{}", stderr);
        }

        if let Some(status) = status {
            if let Some(code) = status.code() {
                return code;
            }
        }

        return 1;
    }
}

impl TargetCreator {
    pub fn write_to(&self, file_name: &str, content: &str) -> std::io::Result<()> {
        if !Path::new(&self.path_to_target_directory).is_dir() {
            std::fs::create_dir_all(&self.path_to_target_directory)?;
        }

        let mut file = File::create(format!("{}/{}", self.path_to_target_directory, file_name))?;
        return file.write_all(content.as_bytes());
    }

    pub fn path_to(&self, file_name: &str) -> String {
        return format!("{}/{}", self.path_to_target_directory, file_name);
    }
}

impl TryFrom<&str> for TargetCreator {
    type Error = Error;

    /// Creates a `TargetCreator` instance from the main file in the same directory
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let path = Path::new(value);

        if let Some(parent_path) = path.parent() {
            return Ok(TargetCreator {
                path_to_target_directory: parent_path.display().to_string() + "/target"
            });
        }

        return Err(Error::NotInDirectory);
    }
}
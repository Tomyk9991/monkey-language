use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use colored::Colorize;

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

        if let Ok(output) = Command::new("cmd").output() {
            if !output.stderr.is_empty() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Error: \n{}", stderr);

                return 1;
            }
        }


        println!("{} `{}`...", "Running".green(), self.path_to_target_directory);
        if let Ok(output) = if wsl {
            Command::new("wsl").args(vec!["nasm", "-felf64", "main.asm", "&&", "ld", "-o", "main", "main.o", ";", "./main"])
                .output()
        } else {
            Command::new("nasm").args(vec!["-felf64", "main.asm", "&&", "ld", "-o", "main", "main.o", ";", "./main"])
                .output()
        } {
            if !output.stdout.is_empty() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("{}", stdout);
            }

            let status = Some(output.status);


            if !output.stderr.is_empty() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Error: \n{}", stderr);
            }

            if let Some(status) = status {
                if let Some(code) = status.code() {
                    return code;
                }
            }
        }


        1
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

        Err(Error::NotInDirectory)
    }
}
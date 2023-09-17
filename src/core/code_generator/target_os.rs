use std::fmt::{Display, Formatter};
use std::process::Command;
use std::str::FromStr;
use colored::Colorize;
use crate::core::code_generator::target_creator::{CompileAndExecute, TargetCreator};

#[derive(Clone, Debug, PartialEq)]
pub enum TargetOS {
    Windows,
    Linux,
    WindowsSubsystemLinux
}

impl Display for TargetOS {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TargetOS::Windows => "Windows",
            TargetOS::Linux => "Linux",
            TargetOS::WindowsSubsystemLinux => "Windows Subsystem for Linux"
        })
    }
}

impl FromStr for TargetOS {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "windows" => Ok(TargetOS::Windows),
            "wsl" | "windowssubsystemlinux" => Ok(TargetOS::WindowsSubsystemLinux),
            "linux" => Ok(TargetOS::Linux),
            default => Err(format!("Not supported Os: {}", default))
        }
    }
}

impl TargetOS {
    fn run_on_windows() -> Option<i32> {
        if let Ok(output) = Command::new("cmd").output() {
            if !output.stderr.is_empty() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Error: \n{}", stderr);

                return Some(1);
            }
        }

        None
    }

    fn run_generic_commands(program: &str, args: Vec<&str>) -> i32 {
        if let Ok(output) = Command::new(program).args(args)
            .output() {
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

            return 1;
        }

        -1
    }
}

impl CompileAndExecute for TargetOS {
    fn compile_and_execute(&self, target_creator: &TargetCreator) -> i32 {
        println!("Compiling...");

        if self == &TargetOS::Windows || self == &TargetOS::WindowsSubsystemLinux {
            if let Some(return_value) = TargetOS::run_on_windows() {
                return return_value;
            }
        }

        println!("{} `{}`...", "Running".green(), target_creator.path_to_target_directory);

        match self {
            TargetOS::Windows => {
                // nasm -f win64 main.asm ; gcc main.obj -o main ; .\main.exe ; echo $LASTEXITCODE
                TargetOS::run_generic_commands("nasm", vec!["-f", "win64", "main.asm"]);
                TargetOS::run_generic_commands("gcc", vec!["main.obj", "-o", "main"]);
                TargetOS::run_generic_commands("./main.exe", vec![])
            }
            TargetOS::Linux => {
                TargetOS::run_generic_commands("nasm", vec!["-felf64", "main.asm", "&&", "ld", "-o", "main", "main.o", ";", "./main"])
            }
            TargetOS::WindowsSubsystemLinux => {
                TargetOS::run_generic_commands("wsl", vec!["nasm", "-felf64", "main.asm", "&&", "ld", "-o", "main", "main.o", ";", "./main"])
            }
        }
    }
}
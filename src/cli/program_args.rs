use std::str::FromStr;
use clap::Parser;
use crate::core::code_generator::target_os::TargetOS;

#[derive(Parser, Debug)]
#[command(author, version, about = crate::cli::main_screen::print_help_screen(), long_about = None)]
pub struct ProgramArgs {
    /// Main source file
    #[arg(short, long, default_value_t = String::from("."))]
    pub input: String,
    /// Target OS (Supported Linux, Windows, WSL)
    #[arg(short, long)]
    pub target_os: TargetOS,
    #[arg(long, default_value_t = false)]
    /// build the project without running it
    pub build: bool,
    #[arg(short, long)]
    /// Print the scope with the given option (Supported: production, debug)
    pub print_scope: Option<PrintOption>
}

#[derive(Clone, Debug)]
pub enum PrintOption {
    Production,
    Debug
}

impl FromStr for PrintOption {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "production" => Ok(PrintOption::Production),
            "debug" => Ok(PrintOption::Debug),
            default => Err(format!("Not supported print option: {}", default))
        }
    }
}
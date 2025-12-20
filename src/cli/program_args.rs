use std::fmt::{Display, Formatter};
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
    pub print_scope: Option<PrintOption>,
    #[arg(short = 'o', long, default_value_t = OptimizationLevel::O1)]
    /// Describes the level of provided optimization
    pub optimization_level: OptimizationLevel,
}


#[derive(Debug, PartialEq, Clone)]
pub enum OptimizationLevel {
    O0,
    O1,
    O2,
    O3,
}

impl Display for OptimizationLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            OptimizationLevel::O0 => "o0",
            OptimizationLevel::O1 => "o1",
            OptimizationLevel::O2 => "o2",
            OptimizationLevel::O3 => "o3",
        })
    }
}

impl FromStr for OptimizationLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "o0" | "0" => Ok(OptimizationLevel::O0),
            "o1" | "1" => Ok(OptimizationLevel::O1),
            "o2" | "2" => Ok(OptimizationLevel::O2),
            "o3" | "3" => Ok(OptimizationLevel::O3),
            _ => Err("Optimization level not supported by the compiler".to_string())
        }
    }
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
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
    pub target_os: TargetOS
}
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = crate::cli::main_screen::print_help_screen(), long_about = None)]
pub struct ProgramArgs {
    /// Main source file
    #[arg(short, long, default_value_t = String::from("."))]
    pub input: String,
    /// Run the compilation steps through WSL
    #[arg(short, long, default_value_t = false)]
    pub wsl: bool
}
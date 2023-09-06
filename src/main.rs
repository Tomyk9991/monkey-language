use std::process::{Command, ExitStatus};
use clap::Parser;
use crate::cli::program_args::ProgramArgs;
use crate::core::code_generator::generator::Generator;
use crate::core::code_generator::target_creator::TargetCreator;
use crate::core::io::monkey_file::MonkeyFile;
use crate::core::lexer::tokenizer::Lexer;

mod core;
mod cli;
mod utils;

fn main() -> anyhow::Result<()> {
    let args = ProgramArgs::parse();

    let main_file = args.input.clone();
    let file: MonkeyFile = MonkeyFile::read(main_file)?;

    let top_level_scope = Lexer::from(file).tokenize()?;

    println!("=>{:<12} Done lexing", " ");
    println!("{}", top_level_scope);

    let source_code = r#"a = 5;"#;
    let basic_scope = Lexer::from(MonkeyFile::read_from_str(source_code))
        .tokenize()?;

    // https://github.com/orosmatthew/hydrogen-cpp/blob/master/src/generation.hpp#L107
    // https://github.com/orosmatthew/hydrogen-cpp/blob/master/src/parser.hpp

    let mut code_generator = Generator::from(basic_scope);

    let target_creator = TargetCreator::try_from(args.input.as_str())?;
    // target_creator.write_to("main.asm", &code_generator.generate()?)?;

    let s = std::env::current_dir()?;

    std::env::set_current_dir(target_creator.path_to_target_directory.as_str())?;
    let status = target_creator.compile_and_execute(args.wsl);
    std::env::set_current_dir(s)?;

    println!("Process finished with exit code {}", status);
    Ok(())
}
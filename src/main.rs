use clap::Parser;
use colored::Colorize;
use crate::cli::program_args::ProgramArgs;
use crate::core::code_generator::generator::{ASMGenerator};
use crate::core::code_generator::target_creator::TargetCreator;
use crate::core::io::monkey_file::MonkeyFile;
use crate::core::lexer::tokenizer::Lexer;
use crate::core::type_checker::static_type_checker::static_type_check;

mod core;
mod cli;
mod utils;

fn main() -> anyhow::Result<()> {
    let args = ProgramArgs::parse();

    let entry_point_file = args.input.clone();
    let money_file: MonkeyFile = MonkeyFile::read(entry_point_file)?;

// 1) Build AST
    let top_level_scope = Lexer::from(money_file).tokenize()?;
    // println!("{:?}", top_level_scope);

// 2) Static Type Checking
    static_type_check(&top_level_scope)?;


// 3) Building
    let mut code_generator = ASMGenerator::from((top_level_scope, args.target_os.clone()));

    let target_creator = TargetCreator::try_from((args.input.as_str(), &args.target_os))?;
    let asm_result = code_generator.generate()?;

    target_creator.write_to("main.asm", &asm_result)?;

    let s = std::env::current_dir()?;

    std::env::set_current_dir(target_creator.path_to_target_directory.as_str())?;
    {
        let build_status = target_creator.compile(args.target_os.clone());
        println!("Completing build. Status: {build_status} {}", match build_status {
            0 => "Successful".green(),
            _ => "Failed".red(),
        });

// 4) Running
        if !args.build {
            let status = target_creator.execute(args.target_os);
            println!("Process finished with exit code {}", status);
        }
    }
    std::env::set_current_dir(s)?;

    Ok(())
}
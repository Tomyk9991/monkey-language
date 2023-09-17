use clap::Parser;
use crate::cli::program_args::ProgramArgs;
use crate::core::code_generator::generator::{SourceCodeGenerator};
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

    println!("{}", top_level_scope);

    let source_code = r#"a = 10;
    if (a) {
        a = 1;

        if (a) {
            a = 20;
        } else {
            a = 13;
        }
    } else {
        a = 0;
        if (a) {
            a = 30;
        } else {
            a = 244;
        }
    }

    exit(a);
    "#;
    let basic_scope = Lexer::from(MonkeyFile::read_from_str(source_code))
        .tokenize()?;

    let mut code_generator = SourceCodeGenerator::from((basic_scope, args.target_os.clone()));

    let target_creator = TargetCreator::try_from(args.input.as_str())?;
    let asm_result = code_generator.generate()?;

    target_creator.write_to("main.asm", &asm_result)?;

    let s = std::env::current_dir()?;

    std::env::set_current_dir(target_creator.path_to_target_directory.as_str())?;
    let status = target_creator.compile_and_execute(args.target_os);
    std::env::set_current_dir(s)?;

    println!("Process finished with exit code {}", status);
    Ok(())
}
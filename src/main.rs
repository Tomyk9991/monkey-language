use clap::Parser;
use crate::cli::program_args::ProgramArgs;
use crate::core::io::monkey_file::MonkeyFile;
use crate::core::lexer::tokenizer::Lexer;

mod core;
mod cli;
mod utils;


fn main() -> anyhow::Result<()> {
    let args = ProgramArgs::parse();

    let main_file = args.input;
    let file: MonkeyFile = MonkeyFile::read(main_file)?;

    let top_level_scope = Lexer::from(file).tokenize()?;

    println!("=>{:<12} Done lexing", " ");
    println!("{:?}", top_level_scope);


    // let interpreter: Interpreter = Interpreter::new();
    //
    // for instruction in instructions {
    //     instruction.execute();
    // }
    

    Ok(())
}

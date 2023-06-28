use crate::cli::program_variable::ProgramVariable;
use crate::core::io::monkey_file::MonkeyFile;
use crate::core::lexer::tokenizer::Lexer;

mod core;
mod cli;
mod utils;


fn main() -> anyhow::Result<()> {
    let main_file: ProgramVariable = ProgramVariable::try_from(vec!["i", "-i"])?;
    let file: MonkeyFile = MonkeyFile::read(main_file.value)?;

    let top_level_scope = Lexer::from(file).tokenize()?;

    println!("=>{:<12} {}", " ", "Done lexing");
    println!("{:?}", top_level_scope);

    
    // let interpreter: Interpreter = Interpreter::new();
    //
    // for instruction in instructions {
    //     instruction.execute();
    // }
    

    Ok(())
}

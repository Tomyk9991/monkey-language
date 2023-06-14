use crate::cli::program_variable::ProgramVariable;
use crate::interpreter::io::monkey_file::MonkeyFile;
use crate::interpreter::lexer::tokenizer::Lexer;
use crate::interpreter::lexer::tokens::assignable_tokens::equation_parser::{EquationToken};

mod interpreter;
mod cli;
mod utils;


fn main() -> anyhow::Result<()> {
    let main_file: ProgramVariable = ProgramVariable::try_from(vec!["i", "-i"])?;
    let file: MonkeyFile = MonkeyFile::read(main_file.value)?;

    println!("{:#?}", file.lines);

    let mut lexer = Lexer::from(file);
    let top_level_scope = lexer.tokenize()?;

    println!("Done lexing");
    println!("{:?}", top_level_scope);

    
    // let interpreter: Interpreter = Interpreter::new();
    //
    // for instruction in instructions {
    //     instruction.execute();
    // }
    

    Ok(())
}

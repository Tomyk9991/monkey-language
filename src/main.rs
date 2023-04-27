use crate::cli::program_variable::ProgramVariable;
use crate::interpreter::io::monkey_file::MonkeyFile;
use crate::interpreter::lexer::lexer::Lexer;

mod interpreter;
mod cli;
mod utils;

fn hallo(name: &str) -> String {
    let a: &'static str = "Hallo";
    String::from("hallo") + name
}

fn main() -> anyhow::Result<()> {
    let main_file: ProgramVariable = ProgramVariable::try_from(vec!["i", "-i"])?;
    let file: MonkeyFile = MonkeyFile::read(main_file.value)?;

    let mut lexer = Lexer::from(&file);

    let top_level_scope = lexer.tokenize()?;

    println!("{:?}", top_level_scope);

    // let interpreter: Interpreter = Interpreter::new();
    //
    // for instruction in instructions {
    //     instruction.execute();
    // }

    Ok(())
}

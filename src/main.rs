use crate::cli::program_variable::{ProgramVariable};
use crate::core::io::monkey_file::MonkeyFile;
use crate::core::lexer::tokenizer::Lexer;

mod core;
mod cli;
mod utils;


fn main() -> anyhow::Result<()> {
    let flags = vec![
        (ProgramVariable::<false>::try_from(vec!["-h", "--help"]), || { println!("{}", cli::main_screen::help_screen()); })
    ];

    for (flag, action) in flags {
        if let Ok(_) = flag {
            action();
        }
    }

    let main_file = ProgramVariable::<true>::try_from(vec!["-i", "--input"])?;
    let file: MonkeyFile = MonkeyFile::read(main_file.get_value())?;

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

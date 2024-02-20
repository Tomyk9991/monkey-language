use clap::Parser;
use colored::Colorize;
use crate::cli::program_args::{PrintOption, ProgramArgs};
use crate::core::code_generator::generator::{ASMGenerator};
use crate::core::code_generator::target_creator::TargetCreator;
use crate::core::code_generator::target_os::TargetOS;
use crate::core::io::monkey_file::MonkeyFile;
use crate::core::lexer::tokenizer::Lexer;
use crate::core::type_checker::static_type_checker::static_type_check;

mod core;
mod cli;
mod utils;


fn run_compiler() -> anyhow::Result<()> {
    let only_write = false;
    let args = ProgramArgs::parse();
    
    let entry_point_file = args.input.clone();
    let monkey_file: MonkeyFile = MonkeyFile::read(entry_point_file)?;

// 1) Build AST
    let top_level_scope = Lexer::from(monkey_file).tokenize()?;

    if let Some(print_scope) = args.print_scope {
        match print_scope {
            PrintOption::Production => println!("{}", top_level_scope),
            PrintOption::Debug => println!("{:?}", top_level_scope)
        };
    }

// 2) Static Type Checking
    static_type_check(&top_level_scope)?;


// 3) Building
    let mut code_generator = ASMGenerator::from((top_level_scope, args.target_os.clone(), true));

    let target_creator = TargetCreator::try_from((args.input.as_str(), &args.target_os))?;
    let asm_result = code_generator.generate()?;

    target_creator.write_to("main.asm", &asm_result)?;

    if only_write {
        return Ok(())
    }

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
            let status = target_creator.execute(&args.target_os);
            println!("Process finished with exit code {}", status);

            if args.target_os == TargetOS::Windows {
                #[cfg(target_os = "windows")]
                {
                    let status_code = windows_core::HRESULT(status);

                    // https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/596a1078-e883-4972-9bbc-49e60bebca55
                    if status_code.is_err() {
                        let error = windows_core::Error::from(status_code);
                        let message = error.message();
                        if message.is_empty() {
                            if let Some(hard_coded_message) = more_windows_errors(status) {
                                println!("{}", hard_coded_message);
                            }
                        }
                        println!("Error: {error}");
                    }
                }
            }
        }
    }
    std::env::set_current_dir(s)?;

    Ok(())
}

fn main() {
    if let Err(error) = run_compiler() {
        eprintln!("{} {error}", "Error:".red());
    }
}

#[cfg(target_os = "windows")]
/// looks up more numbers, maybe there is a hardcoded message
fn more_windows_errors(status: i32) -> Option<String> {
    match status {
        -1073741676 => Some("Integer division by zero".to_string()),
        -1073741675 => Some("Integer overflow".to_string()),
        -1073741571 => Some("Stack overflow".to_string()),
        -1073741819 => Some("Pointing to invalid memory".to_string()),
        _ => None
    }
}
use crate::cli::program_args::{OptimizationLevel, PrintOption, ProgramArgs};
use crate::core::io::monkey_file::MonkeyFile;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::parser::parser::ASTParser;
use crate::core::semantics::static_type_check::static_type_checker::static_type_check;
use crate::core::semantics::type_infer::type_inferer::infer_type;
use clap::Parser;
use colored::Colorize;
use crate::core::code_generator::generator::ASMGenerator;
use crate::core::code_generator::target_creator::TargetCreator;
use crate::core::code_generator::target_os::TargetOS;
use crate::core::optimization::optimization::Optimization;
use crate::core::optimization::optimization::OptimizationContext;

mod cli;
mod core;
mod utils;

fn run_compiler() -> anyhow::Result<()> {
    let only_write = false;

    let args = ProgramArgs::parse();
    let entry_point_file = args.input.clone();

    let monkey_file: MonkeyFile = MonkeyFile::read(entry_point_file)?;

    // 1) Build AST
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;

    let program: &mut Vec<AbstractSyntaxTreeNode> = &mut top_level_scope.result.program;

    // 2) Static Type Checking
    infer_type(program)?;
    let mut static_type_context = static_type_check(&top_level_scope.result.program)?;

    // 3) o1 Optimization
    let top_level_scope = if args.optimization_level == OptimizationLevel::O1 {
        let optimization_context = top_level_scope.result.o1(&mut static_type_context, OptimizationContext::from(top_level_scope.result.clone()));
        optimization_context.program
    } else {
        top_level_scope.result
    };

    if let Some(print_scope) = &args.print_scope {
        match print_scope {
            PrintOption::Production => println!("{}", top_level_scope),
            PrintOption::Debug => println!("{:#?}", top_level_scope),
        };
    }
    // 3) Building
    let got_main = top_level_scope.has_main_method;
    let mut code_generator = ASMGenerator::from((top_level_scope.program, args.target_os.clone(), got_main));
    let target_creator = TargetCreator::try_from((args.input.as_str(), &args.target_os))?;
    let asm_result = code_generator.generate()?;

    target_creator.write_to("main.asm", &asm_result)?;

    if only_write {
        return Ok(())
    }

    with_path(target_creator.path_to_target_directory.as_str(), || {
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
                        if let Some(hard_coded_message) = more_windows_errors(status) {
                            println!("{}", hard_coded_message);
                        } else {
                            let message = error.message();
                            println!("Error: {message}");
                        }
                    }
                }
            }
        }
        Ok(())
    })?;

    Ok(())
}

fn main() {
    if let Err(error) = run_compiler() {
        eprintln!("{}\n\t{error}", "Error:".red());
    }
}

fn with_path<F>(path: &str, f: F) -> anyhow::Result<()>
where
    F: FnOnce() -> anyhow::Result<()>,
{
    let s = std::env::current_dir()?;

    std::env::set_current_dir(path)?;
    f()?;
    std::env::set_current_dir(s)?;

    Ok(())
}

#[cfg(target_os = "windows")]
/// looks up more numbers, maybe there is a hardcoded message
fn more_windows_errors(status: i32) -> Option<String> {
    match status {
        -1073741676 => Some("Integer division by zero".to_string()),
        -1073741675 => Some("Integer overflow".to_string()),
        -1073741571 => Some("Stack overflow".to_string()),
        -1073741819 => Some("Access violation. Pointing to invalid memory".to_string()),
        _ => None,
    }
}

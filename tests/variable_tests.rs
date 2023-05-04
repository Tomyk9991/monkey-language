use monkey_language::interpreter::io::monkey_file::MonkeyFile;
use monkey_language::interpreter::lexer::tokenizer::Lexer;
use monkey_language::interpreter::lexer::scope::ScopeError;


#[test]
fn variable_test() -> anyhow::Result<()> {
    let variables = r#"
    fisch = "Fische sind wirklich wirklich toll";
    hallo = "Thomas"; tschuess = 5;
    mallo = "";
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(variables);
    let mut lexer = Lexer::from(&monkey_file);
    let top_level_scope = lexer.tokenize();

    if let Err(err) = top_level_scope {
        println!("{}", err);
        if let None = err.downcast_ref::<ScopeError>() {
            assert!(false);
        }
    } else {
        let top_level_scope = top_level_scope.unwrap();
        println!("{:?}", top_level_scope);
    }

    Ok(())
}
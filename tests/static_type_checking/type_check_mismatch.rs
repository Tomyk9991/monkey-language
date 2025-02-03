use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::scope::ScopeError;
use monkey_language::core::lexer::parser::Lexer;
use monkey_language::core::lexer::types::r#type::InferTypeError;

#[test]
fn wrong_index_type() -> anyhow::Result<()> {
    let code = r#"
    let a: [i32, 5] = [1, 2, 3, 4, 5];
    let b = a["0"];
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.parse();

    assert!(top_level_scope.is_err());
    assert!(matches!(top_level_scope, Err(ScopeError::InferredError(InferTypeError::IllegalIndexOperation(_, _)))));
    Ok(())
}
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::parser::ast_parser::ASTParser;
use monkey_language::core::parser::types::r#type::InferTypeError;
use monkey_language::core::semantics::type_infer::type_inferer::infer_type;

#[test]
fn wrong_index_type() -> anyhow::Result<()> {
    let code = r#"
    let a: [i32, 5] = [1, 2, 3, 4, 5];
    let b = a["0"];
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    let infer_result = infer_type(&mut top_level_scope.result.program);

    assert!(infer_result.is_err());
    if let Err(e) = infer_result {
        let s = *e;
        assert!(matches!(s, InferTypeError::IllegalIndexOperation(_, _)));
    }
    Ok(())
}
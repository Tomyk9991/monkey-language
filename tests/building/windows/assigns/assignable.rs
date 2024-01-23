use monkey_language::core::code_generator::generator::{Stack};
use monkey_language::core::code_generator::MetaInfo;
use monkey_language::core::code_generator::target_os::TargetOS;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::type_checker::static_type_checker::static_type_check;
use monkey_language::core::code_generator::ToASM;
use monkey_language::core::lexer::token::Token;
use monkey_language::core::lexer::tokens::assignable_token::AssignableToken;

#[test]
fn string_assign() -> anyhow::Result<()> {
    let code = r#"
    let a: *string = "Hallo";
    "#;

    let asm_result = asm_from_assign_code(&code)?;

    let expected = r#"
.label0:
    db "Hallo", 0
    ; let a: *string = "Hallo"
    mov QWORD [rbp - 8], .label0
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

fn asm_from_assign_code(code: &str) -> anyhow::Result<String> {
    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    static_type_check(&top_level_scope)?;

    let mut asm_result = String::new();

    if let [token] = &top_level_scope.tokens[..] {
        let mut stack = Stack::default();
        let mut meta = MetaInfo {
            code_line: Default::default(),
            target_os: TargetOS::Windows,
            static_type_information: Default::default(),
        };

        if let Token::Variable(variable_token) = token {
            let asm = token.to_asm(&mut stack, &mut meta)?;

            if let AssignableToken::String(string) = &variable_token.assignable {
                let s = string.before_label(&mut stack, &mut meta);
                if let Some(s) = s {
                    asm_result += &s?;
                }
            }

            asm_result += &asm;
        }
    }


    return Ok(asm_result);
}
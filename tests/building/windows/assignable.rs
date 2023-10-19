use monkey_language::core::code_generator::generator::{ASMGenerator, Stack};
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

    let expected = r#"
.label0:
    db "Hallo", 0
    ; let a: *string = "Hallo"
    mov QWORD [rbp - 8], .label0
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn i32_assign() -> anyhow::Result<()> {
    let code = r#"
    let a: i32 = 512;
    "#;

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

        let asm = token.to_asm(&mut stack, &mut meta)?;
        asm_result += &asm;
    }

    let expected = r#"
; let a: i32 = 512
    mov DWORD [rbp - 4], 512
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn full_program_assignable() -> anyhow::Result<()> {
    let code = r#"
    let a: *string = "Testing string";
    let b: i32 = 512;
    let c = b;
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    static_type_check(&top_level_scope)?;
    let mut code_generator = ASMGenerator::from((top_level_scope, TargetOS::Windows));
    let asm_result = String::from(code_generator.generate()?.trim());

    println!("{}", asm_result);

    let expected = r#"; This assembly is targeted for the Windows Operating System
segment .text
global main


.label0:
    db "Testing string", 0
main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 48
    ; let a: *string = "Testing string"
    mov QWORD [rbp - 8], .label0
    ; let b: i32 = 512
    mov DWORD [rbp - 12], 512
    ; let c: i32 = b
    mov eax, DWORD [rbp - 12]
    mov DWORD [rbp - 16], eax
    leave
    ret"#;

    assert_eq!(expected, asm_result);

    Ok(())
}
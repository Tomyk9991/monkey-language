use monkey_language::core::code_generator::generator::ASMGenerator;
use monkey_language::core::code_generator::target_os::TargetOS;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::scanner::parser::ASTParser;
use monkey_language::core::semantics::type_checker::static_type_checker::static_type_check;

#[test]
fn bool_to_i32() -> anyhow::Result<()> {
    let code = r#"
    let a: bool = true;
    let b: bool = false;

    let c: i32 = (i32) a;
    let d: i32 = (i32) b;
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
    let mut lexer = ASTParser::from(monkey_file);
    let top_level_scope = lexer.parse()?;

    static_type_check(&top_level_scope)?;

    let mut code_generator = ASMGenerator::from((top_level_scope, TargetOS::Windows));
    let asm_result = code_generator.generate()?;


    println!("{}", asm_result);

    let expected = r#"
    ; This assembly is targeted for the Windows Operating System
segment .text
global main


main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; let a: bool = true
    mov BYTE [rbp - 1], 1
    ; let b: bool = false
    mov BYTE [rbp - 2], 0
    ; let c: i32 = (i32)a
    ; Cast: (bool) -> (i32)
    ; Cast: (u8) -> (i32)
    movzx eax, BYTE [rbp - 1]
    mov DWORD [rbp - 6], eax
    ; let d: i32 = (i32)b
    ; Cast: (bool) -> (i32)
    ; Cast: (u8) -> (i32)
    movzx eax, BYTE [rbp - 2]
    mov DWORD [rbp - 10], eax
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}
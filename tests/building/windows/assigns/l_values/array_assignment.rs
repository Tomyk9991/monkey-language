use monkey_language::core::code_generator::generator::ASMGenerator;
use monkey_language::core::code_generator::target_os::TargetOS;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::parser::Lexer;
use monkey_language::core::type_checker::static_type_checker::static_type_check;

#[test]
fn array_left_assign() -> anyhow::Result<()> {
    let code = r#"
    let mut a: [i32, 5] = [1, 2, 3, 4, 5];
    let b = a[0];
    a[0] = 10;
    let c = a[0];
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.parse()?;

    static_type_check(&top_level_scope)?;

    let mut code_generator = ASMGenerator::from((top_level_scope, TargetOS::Windows));
    let asm_result = code_generator.generate()?;

    let expected = r#"
    ; This assembly is targeted for the Windows Operating System
segment .text
global main


main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; let a: [i32; 5] = [1, 2, 3, 4, 5]
    ; [1, 2, 3, 4, 5]
    mov DWORD [rbp - 20], 1
    mov DWORD [rbp - 16], 2
    mov DWORD [rbp - 12], 3
    mov DWORD [rbp - 8], 4
    mov DWORD [rbp - 4], 5
    ; let b: i32 = a[0]
    mov eax, DWORD [rbp - (4 + 4 * 4)]
    mov DWORD [rbp - 24], eax
    ; a[0]: i32 = 10
    mov DWORD [rbp - (4 + 4 * 4)], 10
    ; let c: i32 = a[0]
    mov eax, DWORD [rbp - (4 + 4 * 4)]
    mov DWORD [rbp - 28], eax
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}
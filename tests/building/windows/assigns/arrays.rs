use monkey_language::core::code_generator::generator::ASMGenerator;
use monkey_language::core::code_generator::target_os::TargetOS;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::type_checker::static_type_checker::static_type_check;

#[test]
fn array_test_simple() -> anyhow::Result<()> {
    let code = r#"
    let a: [i32, 5] = [1, 2, 3, 4, 5];
    let b = a[0];
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    static_type_check(&top_level_scope)?;

    let mut code_generator = ASMGenerator::from((top_level_scope, TargetOS::Windows));
    let asm_result = code_generator.generate()?;

    println!("{}", asm_result.trim());

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
    mov DWORD [rbp - 4], 1
    mov DWORD [rbp - 8], 2
    mov DWORD [rbp - 12], 3
    mov DWORD [rbp - 16], 4
    mov DWORD [rbp - 20], 5
    ; let b: i32 = a[0]
    mov eax, DWORD [rbp - (4 + 0 * 4)]
    mov DWORD [rbp - 24], eax
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn array_test_variable() -> anyhow::Result<()> {
    let code = r#"
    let a: [i32, 5] = [1, 2, 3, 4, 5];
    let k = 1;
    let b = a[k];
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    static_type_check(&top_level_scope)?;

    let mut code_generator = ASMGenerator::from((top_level_scope, TargetOS::Windows));
    let asm_result = code_generator.generate()?;

    println!("{}", asm_result.trim());

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
    mov DWORD [rbp - 4], 1
    mov DWORD [rbp - 8], 2
    mov DWORD [rbp - 12], 3
    mov DWORD [rbp - 16], 4
    mov DWORD [rbp - 20], 5
    ; let k: i32 = 1
    mov DWORD [rbp - 24], 1
    ; let b: i32 = a[k]
    mov eax, DWORD [rbp - 24]
    cdqe
    imul rax, 4
    sub rax, 0
    neg rax
    mov eax, DWORD [rbp - 4 + rax]
    mov DWORD [rbp - 28], eax
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn array_test_complex() -> anyhow::Result<()> {
    let code = r#"
    let a: [i32, 5] = [1, 2, 3, 4, 5];
    let k = 1;
    let b = a[12 / 3 - 3];
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    static_type_check(&top_level_scope)?;

    let mut code_generator = ASMGenerator::from((top_level_scope, TargetOS::Windows));
    let asm_result = code_generator.generate()?;

    println!("{}", asm_result.trim());

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
    mov DWORD [rbp - 4], 1
    mov DWORD [rbp - 8], 2
    mov DWORD [rbp - 12], 3
    mov DWORD [rbp - 16], 4
    mov DWORD [rbp - 20], 5
    ; let k: i32 = 1
    mov DWORD [rbp - 24], 1
    ; let b: i32 = a[((12 / 3) - 3)]
    ; ((12 / 3) - 3)
    ; (12 / 3)
    mov eax, 12
    mov r14d, edx
    mov r13d, eax
    mov r12d, ecx
    mov ecx, eax
    mov eax, 3
    mov edx, 0
    idiv ecx
    mov edx, r14d
    mov ecx, r12d
    sub eax, 3
    cdqe
    imul rax, 4
    sub rax, 0
    neg rax
    mov eax, DWORD [rbp - 4 + rax]
    mov DWORD [rbp - 28], eax
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}
use monkey_language::core::code_generator::generator::ASMGenerator;
use monkey_language::core::code_generator::target_os::TargetOS;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::parser::Lexer;
use monkey_language::core::type_checker::static_type_checker::static_type_check;

#[test]
fn single_for() -> anyhow::Result<()> {
    let code = r#"
    let mut a = 0;
    for (let mut i = 0; i < 5; i = i + 1) {
        a = a + i;
    }
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
    ; let a: i32 = 0
    mov DWORD [rbp - 4], 0
    ; for (let i: i32 = 0; (i < 5); i = (i + 1))
    ; let i: i32 = 0
    mov DWORD [rbp - 8], 0
    jmp .label0
.label1:
    ; a = (a + i)
    ; (a + i)
    mov eax, DWORD [rbp - 4]
    add eax, DWORD [rbp - 8]
    mov DWORD [rbp - 4], eax
    ; i = (i + 1)
    ; (i + 1)
    mov eax, DWORD [rbp - 8]
    add eax, 1
    mov DWORD [rbp - 8], eax
.label0:
    ; (i < 5)
    mov eax, DWORD [rbp - 8]
    cmp eax, 5
    setl al
    cmp al, 0
    jne .label1
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn multiple_for() -> anyhow::Result<()> {
    let code = r#"
fn inc(a: i32): i32 {
    return a + 1;
}

let mut a: i32 = 0;

for (let mut i: i32 = 0; i < 5; i = i + 1) {
    for (let mut j: i32 = 0; j < 5; j = j + 1) {
        a = inc(a);
    }
}
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


.inc_i32~i32:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    mov DWORD [rbp - 4], ecx
    ; return (a + 1)
    ; (a + 1)
    mov eax, DWORD [rbp - 4]
    add eax, 1
    leave
    ret
main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; let a: i32 = 0
    mov DWORD [rbp - 4], 0
    ; for (let i: i32 = 0; (i < 5); i = (i + 1))
    ; let i: i32 = 0
    mov DWORD [rbp - 8], 0
    jmp .label0
.label1:
    ; for (let j: i32 = 0; (j < 5); j = (j + 1))
    ; let j: i32 = 0
    mov DWORD [rbp - 12], 0
    jmp .label2
.label3:
    ; a = inc(a)
    mov ecx, DWORD [rbp - 4]
    ; inc(a)
    call .inc_i32~i32
    mov DWORD [rbp - 4], eax
    ; j = (j + 1)
    ; (j + 1)
    mov eax, DWORD [rbp - 12]
    add eax, 1
    mov DWORD [rbp - 12], eax
.label2:
    ; (j < 5)
    mov eax, DWORD [rbp - 12]
    cmp eax, 5
    setl al
    cmp al, 0
    jne .label3
    ; i = (i + 1)
    ; (i + 1)
    mov eax, DWORD [rbp - 8]
    add eax, 1
    mov DWORD [rbp - 8], eax
.label0:
    ; (i < 5)
    mov eax, DWORD [rbp - 8]
    cmp eax, 5
    setl al
    cmp al, 0
    jne .label1
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}
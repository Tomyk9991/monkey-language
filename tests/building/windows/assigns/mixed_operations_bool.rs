use monkey_language::core::code_generator::generator::ASMGenerator;
use monkey_language::core::code_generator::target_os::TargetOS;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::type_checker::static_type_checker::static_type_check;

#[test]
fn mixed_operations_mul() -> anyhow::Result<()> {
    let code = r#"
let a: bool = true | true & true;
let s = (i32)a;
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

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
    ; let a: bool = (true | (true & true))
    ; (true | (true & true))
    ; (true & true)
    mov al, 1
    and al, 1
    mov dl, al
    mov al, 1
    or al, dl
    mov BYTE [rbp - 1], al
    ; let s: i32 = (i32)a
    ; Cast: (bool) -> (i32)
    ; Cast: (u8) -> (i32)
    movzx edx, BYTE [rbp - 1]
    mov DWORD [rbp - 5], edx
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn mixed_operations_sub() -> anyhow::Result<()> {
    let code = r#"
let a: bool = true & true | false;
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

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
    ; let a: bool = ((true & true) | false)
    ; ((true & true) | false)
    ; (true & true)
    mov al, 1
    and al, 1
    or al, 0
    mov BYTE [rbp - 1], al
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn mixed_operations() -> anyhow::Result<()> {
    let code = r#"
extern fn printf(format: *string, value: i32): void;
let a: bool = ((true | true) & false | (true & true)) & ((false | false) || true | (true & false)) & ((true | false) | false && (false | true));
let s = (i32)a;
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    static_type_check(&top_level_scope)?;

    let mut code_generator = ASMGenerator::from((top_level_scope, TargetOS::Windows));
    let asm_result = code_generator.generate()?;


    println!("{}", asm_result);

    let expected = r#"
    ; This assembly is targeted for the Windows Operating System
segment .text
global main

extern printf

main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; let a: bool = (((((true | true) & false) | (true & true)) & ((false | false) || (true | (true & false)))) & (((true | false) | false) && (false | true)))
    ; (((((true | true) & false) | (true & true)) & ((false | false) || (true | (true & false)))) & (((true | false) | false) && (false | true)))
    ; ((((true | true) & false) | (true & true)) & ((false | false) || (true | (true & false))))
    ; (((true | true) & false) | (true & true))
    ; ((true | true) & false)
    ; (true | true)
    mov al, 1
    or al, 1
    and al, 0
    mov dil, al
    push rdi
    xor rdi, rdi
    ; (true & true)
    mov al, 1
    and al, 1
    push rax
    xor rax, rax
    pop rdi
    pop rax
    or al, dil
    mov dil, al
    push rdi
    xor rdi, rdi
    ; ((false | false) || (true | (true & false)))
    ; (false | false)
    mov al, 0
    or al, 0
    mov dil, al
    push rdi
    xor rdi, rdi
    ; (true | (true & false))
    ; (true & false)
    mov al, 1
    and al, 0
    mov dl, al
    mov al, 1
    or al, dl
    push rax
    xor rax, rax
    pop rdi
    pop rax
    mov r14b, dl
    mov r13b, al
    mov r12b, cl
    mov cl, al
    mov al, dil
    mov dl, 0
    cmp cl, 0
    jne .label0
    mov al, dil
    cmp al, 0
    je .label1
.label0:
    mov eax, 1
    jmp .label2
.label1:
    mov eax, 0
.label2:
    mov dl, r14b
    mov cl, r12b
    push rax
    xor rax, rax
    pop rdi
    pop rax
    and al, dil
    mov dil, al
    push rdi
    xor rdi, rdi
    ; (((true | false) | false) && (false | true))
    ; ((true | false) | false)
    ; (true | false)
    mov al, 1
    or al, 0
    or al, 0
    mov dil, al
    push rdi
    xor rdi, rdi
    ; (false | true)
    mov al, 0
    or al, 1
    push rax
    xor rax, rax
    pop rdi
    pop rax
    mov r14b, dl
    mov r13b, al
    mov r12b, cl
    mov cl, al
    mov al, dil
    mov dl, 0
    cmp cl, 0
    je .label3
    mov al, dil
    cmp al, 0
    je .label3
    mov eax, 1
    jmp .label4
.label3:
    mov eax, 0
.label4:
    mov dl, r14b
    mov cl, r12b
    push rax
    xor rax, rax
    pop rdi
    pop rax
    and al, dil
    mov BYTE [rbp - 1], al
    ; let s: i32 = (i32)a
    ; Cast: (bool) -> (i32)
    ; Cast: (u8) -> (i32)
    movzx edi, BYTE [rbp - 1]
    mov DWORD [rbp - 5], edi
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}
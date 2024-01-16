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

#[test]
fn expression_assign() -> anyhow::Result<()> {
    let code = r#"
    let a: i32 = 5 + 3;
    "#;

    let asm_result = asm_from_assign_code(&code)?;


    let expected = r#"
; let a: i32 = (5 Add 3)
    ; (5 Add 3)
    mov eax, 5
    add eax, 3
    mov DWORD [rbp - 4], eax
    "#;

    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: i32 = (5 + 2) + 8;
    "#;

    let asm_result = asm_from_assign_code(&code)?;


    let expected = r#"
    ; let a: i32 = ((5 Add 2) Add 8)
    ; ((5 Add 2) Add 8)
    ; (5 Add 2)
    mov eax, 5
    add eax, 2
    add eax, 8
    mov DWORD [rbp - 4], eax
    "#;

    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: i32 = 5 + (2 + 8);
    "#;

    let asm_result = asm_from_assign_code(&code)?;

    let expected = r#"
    ; let a: i32 = (5 Add (2 Add 8))
    ; (5 Add (2 Add 8))
    ; (2 Add 8)
    mov eax, 2
    add eax, 8
    mov edx, eax
    mov eax, 5
    add eax, edx
    mov DWORD [rbp - 4], eax
    "#;

    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: i32 = (5 + 3) + (2 + 8);
    "#;

    let asm_result = asm_from_assign_code(&code)?;


    let expected = r#"
    ; let a: i32 = ((5 Add 3) Add (2 Add 8))
    ; ((5 Add 3) Add (2 Add 8))
    ; (5 Add 3)
    mov eax, 5
    add eax, 3
    mov ecx, eax
    ; (2 Add 8)
    mov eax, 2
    add eax, 8
    mov edi, eax
    add ecx, edi
    mov eax, ecx
    mov DWORD [rbp - 4], eax
    "#;


    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: i32 = 6;
    "#;

    let asm_result = asm_from_assign_code(&code)?;


    let expected = r#"
    ; let a: i32 = 6
    mov DWORD [rbp - 4], 6
    "#;

    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: i32 = (6);
    "#;

    let asm_result = asm_from_assign_code(&code)?;


    let expected = r#"
    ; let a: i32 = 6
    mov eax, 6
    mov DWORD [rbp - 4], eax
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn pointer_assign_test() -> anyhow::Result<()> {
    let code = r#"
    let a: i32 = 5;
    let b: *i32 = &a;
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    static_type_check(&top_level_scope)?;

    let mut code_generator = ASMGenerator::from((top_level_scope, TargetOS::Windows));
    let asm_result = code_generator.generate()?;

    println!("\n{asm_result}");

    let expected = r#"
; This assembly is targeted for the Windows Operating System
segment .text
global main


main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 44
    ; let a: i32 = 5
    mov DWORD [rbp - 4], 5
    ; let b: *i32 = &a
    lea rax, [rbp - 4]
    mov QWORD [rbp - 12], rax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}


#[test]
fn pointer_assign_multiple_test() -> anyhow::Result<()> {
    let code = r#"
extern fn printf(format: *string, value: i32): void;
extern fn ExitProcess(exitCode: i32): void;

let a: i32 = 5;
let b: *i32 = &a;
let c: **i32 = &b;
let d: *i32 = *c;

let ref: **i32 = c;
let f: i32 = *d;
let g: i32 = **c;

let format: *string = "Das ist ein Test %d";
printf(format, *b);

ExitProcess(*b);
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
extern ExitProcess

.label0:
    db "Das ist ein Test %d", 0
main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 84
    ; let a: i32 = 5
    mov DWORD [rbp - 4], 5
    ; let b: *i32 = &a
    lea rax, [rbp - 4]
    mov QWORD [rbp - 12], rax
    ; let c: **i32 = &b
    lea rax, [rbp - 12]
    mov QWORD [rbp - 20], rax
    ; let d: *i32 = *c
    mov rax, QWORD [rbp - 20]
    mov rax, QWORD [rax]
    mov QWORD [rbp - 28], rax
    ; let ref: **i32 = c
    mov rax, QWORD [rbp - 20]
    mov QWORD [rbp - 36], rax
    ; let f: i32 = *d
    mov rax, QWORD [rbp - 28]
    mov rax, QWORD [rax]
    mov DWORD [rbp - 40], eax
    ; let g: i32 = **c
    mov rax, QWORD [rbp - 20]
    mov rax, QWORD [rax]
    mov rax, QWORD [rax]
    mov DWORD [rbp - 44], eax
    ; let format: *string = "Das ist ein Test %d"
    mov QWORD [rbp - 52], .label0
    mov rcx, QWORD [rbp - 52] ; Parameter (format)
    mov rax, QWORD [rbp - 12]
    mov rax, QWORD [rax]
    mov rdx, rax ; Parameter (*b)
    ; printf(format, *b)
    call printf
    mov rax, QWORD [rbp - 12]
    mov rax, QWORD [rax]
    mov rcx, rax ; Parameter (*b)
    ; ExitProcess(*b)
    call ExitProcess
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn pointer_deref_operation_lhs() -> anyhow::Result<()> {
    let code = r#"
let a: i32 = 5;
let b: *i32 = &a;
let addition = *b + 1;
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

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
    sub rsp, 48
    ; let a: i32 = 5
    mov DWORD [rbp - 4], 5
    ; let b: *i32 = &a
    lea rax, [rbp - 4]
    mov QWORD [rbp - 12], rax
    ; let addition: i32 = (*b Add 1)
    ; (*b Add 1)
    mov rax, QWORD [rbp - 12]
    mov rax, QWORD [rax]
    add eax, 1
    mov DWORD [rbp - 16], eax
    leave
    ret
    "#;

    println!("{}", asm_result);

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn pointer_deref_operation_rhs() -> anyhow::Result<()> {
    let code = r#"
let a: i32 = 5;
let b: *i32 = &a;
let addition = 1 + *b;
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
    sub rsp, 48
    ; let a: i32 = 5
    mov DWORD [rbp - 4], 5
    ; let b: *i32 = &a
    lea rax, [rbp - 4]
    mov QWORD [rbp - 12], rax
    ; let addition: i32 = (1 Add *b)
    ; (1 Add *b)
    mov eax, 1
    mov rdx, QWORD [rbp - 12]
    mov rdx, QWORD [rdx]
    add eax, edx
    mov DWORD [rbp - 16], eax
    leave
    ret

    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}


#[test]
fn pointer_deref_operation_lhs_rhs() -> anyhow::Result<()> {
    let code = r#"
let a: i32 = 5;
let b: *i32 = &a;
let addition = *b + *b;
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
    sub rsp, 48
    ; let a: i32 = 5
    mov DWORD [rbp - 4], 5
    ; let b: *i32 = &a
    lea rax, [rbp - 4]
    mov QWORD [rbp - 12], rax
    ; let addition: i32 = (*b Add *b)
    ; (*b Add *b)
    mov rax, QWORD [rbp - 12]
    mov rax, QWORD [rax]
    mov rdx, QWORD [rbp - 12]
    mov rdx, QWORD [rdx]
    add eax, edx
    mov DWORD [rbp - 16], eax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn pointer_deref_operation_lhs_expression() -> anyhow::Result<()> {
    let code = r#"
let a: i32 = 5;
let b: *i32 = &a;
let addition = *b + (0 + 1);
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

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
    sub rsp, 48
    ; let a: i32 = 5
    mov DWORD [rbp - 4], 5
    ; let b: *i32 = &a
    lea rax, [rbp - 4]
    mov QWORD [rbp - 12], rax
    ; let addition: i32 = (*b Add (0 Add 1))
    ; (*b Add (0 Add 1))
    ; (0 Add 1)
    mov eax, 0
    add eax, 1
    mov edx, eax
    mov rax, QWORD [rbp - 12]
    mov rax, QWORD [rax]
    add eax, edx
    mov DWORD [rbp - 16], eax
    leave
    ret
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn pointer_deref_operation_expression_rhs() -> anyhow::Result<()> {
    let code = r#"
let a: i32 = 5;
let b: *i32 = &a;
let addition = (0 + 1) + *b;
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
    sub rsp, 48
    ; let a: i32 = 5
    mov DWORD [rbp - 4], 5
    ; let b: *i32 = &a
    lea rax, [rbp - 4]
    mov QWORD [rbp - 12], rax
    ; let addition: i32 = ((0 Add 1) Add *b)
    ; ((0 Add 1) Add *b)
    ; (0 Add 1)
    mov eax, 0
    add eax, 1
    mov rdx, QWORD [rbp - 12]
    mov rdx, QWORD [rdx]
    add eax, edx
    mov DWORD [rbp - 16], eax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn pointer_deref_operation_expression_expression() -> anyhow::Result<()> {
    let code = r#"
let a: i32 = 5;
let b: *i32 = &a;
let addition = (*b + *b) + (*b + *b);
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
    sub rsp, 48
    ; let a: i32 = 5
    mov DWORD [rbp - 4], 5
    ; let b: *i32 = &a
    lea rax, [rbp - 4]
    mov QWORD [rbp - 12], rax
    ; let addition: i32 = ((*b Add *b) Add (*b Add *b))
    ; ((*b Add *b) Add (*b Add *b))
    ; (*b Add *b)
    mov rax, QWORD [rbp - 12]
    mov rax, QWORD [rax]
    mov rdx, QWORD [rbp - 12]
    mov rdx, QWORD [rdx]
    add eax, edx
    mov ecx, eax
    ; (*b Add *b)
    mov rax, QWORD [rbp - 12]
    mov rax, QWORD [rax]
    mov rdx, QWORD [rbp - 12]
    mov rdx, QWORD [rdx]
    add eax, edx
    mov edi, eax
    add ecx, edi
    mov eax, ecx
    mov DWORD [rbp - 16], eax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn pointer_deref_operation_complex_expression_expression() -> anyhow::Result<()> {
    let code = r#"
let a: i32 = 5;
let b: *i32 = &a;

let c: i32 = 13;
let d: *i32 = &c;

let addition = (((*d + *b) + (*b + *d)) + (*b + *b)) + ((*b + (*b + *b)) + (*b + (*d + *b)));
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
    sub rsp, 60
    ; let a: i32 = 5
    mov DWORD [rbp - 4], 5
    ; let b: *i32 = &a
    lea rax, [rbp - 4]
    mov QWORD [rbp - 12], rax
    ; let c: i32 = 13
    mov DWORD [rbp - 16], 13
    ; let d: *i32 = &c
    lea rax, [rbp - 16]
    mov QWORD [rbp - 24], rax
    ; let addition: i32 = ((((*d Add *b) Add (*b Add *d)) Add (*b Add *b)) Add ((*b Add (*b Add *b)) Add (*b Add (*d Add *b))))
    ; ((((*d Add *b) Add (*b Add *d)) Add (*b Add *b)) Add ((*b Add (*b Add *b)) Add (*b Add (*d Add *b))))
    ; (((*d Add *b) Add (*b Add *d)) Add (*b Add *b))
    ; ((*d Add *b) Add (*b Add *d))
    ; (*d Add *b)
    mov rax, QWORD [rbp - 24]
    mov rax, QWORD [rax]
    mov rdx, QWORD [rbp - 12]
    mov rdx, QWORD [rdx]
    add eax, edx
    mov ecx, eax
    ; (*b Add *d)
    mov rax, QWORD [rbp - 12]
    mov rax, QWORD [rax]
    mov rdx, QWORD [rbp - 24]
    mov rdx, QWORD [rdx]
    add eax, edx
    mov edi, eax
    add ecx, edi
    mov eax, ecx
    push rax
    xor rax, rax
    ; (*b Add *b)
    mov rax, QWORD [rbp - 12]
    mov rax, QWORD [rax]
    mov rdx, QWORD [rbp - 12]
    mov rdx, QWORD [rdx]
    add eax, edx
    push rax
    xor rax, rax
    pop rdi
    pop rax
    add eax, edi
    push rax
    xor rax, rax
    ; ((*b Add (*b Add *b)) Add (*b Add (*d Add *b)))
    ; (*b Add (*b Add *b))
    ; (*b Add *b)
    mov rax, QWORD [rbp - 12]
    mov rax, QWORD [rax]
    mov rdx, QWORD [rbp - 12]
    mov rdx, QWORD [rdx]
    add eax, edx
    mov edx, eax
    mov rax, QWORD [rbp - 12]
    mov rax, QWORD [rax]
    add eax, edx
    mov edi, eax
    push rdi
    xor rdi, rdi
    ; (*b Add (*d Add *b))
    ; (*d Add *b)
    mov rax, QWORD [rbp - 24]
    mov rax, QWORD [rax]
    mov rdx, QWORD [rbp - 12]
    mov rdx, QWORD [rdx]
    add eax, edx
    mov edx, eax
    mov rax, QWORD [rbp - 12]
    mov rax, QWORD [rax]
    add eax, edx
    push rax
    xor rax, rax
    pop rdi
    pop rax
    add eax, edi
    push rax
    xor rax, rax
    pop rdi
    pop rax
    add eax, edi
    mov DWORD [rbp - 28], eax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}


#[test]
fn single_expression_test() -> anyhow::Result<()> {
    let code = r#"
    let a: i32 = 5;
    let b: i32 = a;
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;


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
    sub rsp, 40
    ; let a: i32 = 5
    mov DWORD [rbp - 4], 5
    ; let b: i32 = a
    mov eax, DWORD [rbp - 4]
    mov DWORD [rbp - 8], eax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: i32 = 5;
    let b: i32 = (a);
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    static_type_check(&top_level_scope)?;

    let mut code_generator = ASMGenerator::from((top_level_scope, TargetOS::Windows));

    let asm_result = code_generator.generate()?;

    assert_eq!(expected.trim(), asm_result.trim());

    Ok(())
}

#[test]
fn i32_assign() -> anyhow::Result<()> {
    let code = r#"
    let a: i32 = 512;
    "#;

    let asm_result = asm_from_assign_code(&code)?;

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

#[test]
fn assignable_different_integer_types() -> anyhow::Result<()> {
    let code = r#"
    let a: i64 = 512;
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


main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 40
    ; let a: i64 = 512
    mov QWORD [rbp - 8], 512
    leave
    ret"#;

    assert_eq!(expected, asm_result);

    Ok(())
}

#[test]
fn basic_add_different_type() -> anyhow::Result<()> {
    let code = r#"
    let a: i64 = 512;
    let b: i64 = 5;
    let c = a + b;
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


main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 56
    ; let a: i64 = 512
    mov QWORD [rbp - 8], 512
    ; let b: i64 = 5
    mov QWORD [rbp - 16], 5
    ; let c: i64 = (a Add b)
    ; (a Add b)
    mov rax, QWORD [rbp - 8]
    add rax, QWORD [rbp - 16]
    mov QWORD [rbp - 24], rax
    leave
    ret"#;

    assert_eq!(expected, asm_result);

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
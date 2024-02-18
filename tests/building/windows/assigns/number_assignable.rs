use monkey_language::core::code_generator::generator::ASMGenerator;
use monkey_language::core::code_generator::target_os::TargetOS;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::type_checker::static_type_checker::static_type_check;

#[test]
fn expression_assign() -> anyhow::Result<()> {
    let code = r#"
    let a: i32 = 5 + 3;
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    static_type_check(&top_level_scope)?;

    let mut code_generator = ASMGenerator::from((top_level_scope, TargetOS::Windows));
    let asm_result = code_generator.generate()?;


    let expected = r#"; This assembly is targeted for the Windows Operating System
segment .text
global main


main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 36
    ; let a: i32 = (5 + 3)
    ; (5 + 3)
    mov eax, 5
    add eax, 3
    mov DWORD [rbp - 4], eax
    leave
    ret"#;

    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: i32 = (5 + 2) + 8;
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
    sub rsp, 36
    ; let a: i32 = ((5 + 2) + 8)
    ; ((5 + 2) + 8)
    ; (5 + 2)
    mov eax, 5
    add eax, 2
    add eax, 8
    mov DWORD [rbp - 4], eax
    leave
    ret
"#;

    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: i32 = 5 + (2 + 8);
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
    sub rsp, 36
    ; let a: i32 = (5 + (2 + 8))
    ; (5 + (2 + 8))
    ; (2 + 8)
    mov eax, 2
    add eax, 8
    mov edx, eax
    mov eax, 5
    add eax, edx
    mov DWORD [rbp - 4], eax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: i32 = (5 + 3) + (2 + 8);
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    static_type_check(&top_level_scope)?;

    let mut code_generator = ASMGenerator::from((top_level_scope, TargetOS::Windows));
    let asm_result = code_generator.generate()?;


    let expected = r#"; This assembly is targeted for the Windows Operating System
segment .text
global main


main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 36
    ; let a: i32 = ((5 + 3) + (2 + 8))
    ; ((5 + 3) + (2 + 8))
    ; (5 + 3)
    mov eax, 5
    add eax, 3
    mov ecx, eax
    ; (2 + 8)
    mov eax, 2
    add eax, 8
    mov edi, eax
    add ecx, edi
    mov DWORD [rbp - 4], ecx
    leave
    ret
    "#;


    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: i32 = 6;
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
    sub rsp, 36
    ; let a: i32 = 6
    mov DWORD [rbp - 4], 6
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: i32 = (6);
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
    sub rsp, 36
    ; let a: i32 = 6
    mov eax, 6
    mov DWORD [rbp - 4], eax
    leave
    ret
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
let a: i32 = 5;
let b: *i32 = &a;
let c: **i32 = &b;
let d: *i32 = *c;

let ref: **i32 = c;
let f: i32 = *d;
let g: i32 = **c;
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
    sub rsp, 76
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
    ; let addition: i32 = (*b + 1)
    ; (*b + 1)
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
    ; let addition: i32 = (1 + *b)
    ; (1 + *b)
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
    ; let addition: i32 = (*b + *b)
    ; (*b + *b)
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
    ; let addition: i32 = (*b + (0 + 1))
    ; (*b + (0 + 1))
    ; (0 + 1)
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
    ; let addition: i32 = ((0 + 1) + *b)
    ; ((0 + 1) + *b)
    ; (0 + 1)
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
    ; let addition: i32 = ((*b + *b) + (*b + *b))
    ; ((*b + *b) + (*b + *b))
    ; (*b + *b)
    mov rax, QWORD [rbp - 12]
    mov rax, QWORD [rax]
    mov rdx, QWORD [rbp - 12]
    mov rdx, QWORD [rdx]
    add eax, edx
    mov ecx, eax
    ; (*b + *b)
    mov rax, QWORD [rbp - 12]
    mov rax, QWORD [rax]
    mov rdx, QWORD [rbp - 12]
    mov rdx, QWORD [rdx]
    add eax, edx
    mov edi, eax
    add ecx, edi
    mov DWORD [rbp - 16], ecx
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
    ; let addition: i32 = ((((*d + *b) + (*b + *d)) + (*b + *b)) + ((*b + (*b + *b)) + (*b + (*d + *b))))
    ; ((((*d + *b) + (*b + *d)) + (*b + *b)) + ((*b + (*b + *b)) + (*b + (*d + *b))))
    ; (((*d + *b) + (*b + *d)) + (*b + *b))
    ; ((*d + *b) + (*b + *d))
    ; (*d + *b)
    mov rax, QWORD [rbp - 24]
    mov rax, QWORD [rax]
    mov rdx, QWORD [rbp - 12]
    mov rdx, QWORD [rdx]
    add eax, edx
    mov ecx, eax
    ; (*b + *d)
    mov rax, QWORD [rbp - 12]
    mov rax, QWORD [rax]
    mov rdx, QWORD [rbp - 24]
    mov rdx, QWORD [rdx]
    add eax, edx
    mov edi, eax
    add ecx, edi
    mov edi, ecx
    push rdi
    xor rdi, rdi
    ; (*b + *b)
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
    mov edi, eax
    push rdi
    xor rdi, rdi
    ; ((*b + (*b + *b)) + (*b + (*d + *b)))
    ; (*b + (*b + *b))
    ; (*b + *b)
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
    ; (*b + (*d + *b))
    ; (*d + *b)
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
    sub rsp, 36
    ; let a: i32 = 512
    mov DWORD [rbp - 4], 512
    leave
    ret"#;

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

    let expected = r#"
    ; This assembly is targeted for the Windows Operating System
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
    ; let c: i64 = (a + b)
    ; (a + b)
    mov rax, QWORD [rbp - 8]
    add rax, QWORD [rbp - 16]
    mov QWORD [rbp - 24], rax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());

    Ok(())
}

#[test]
fn pointer_deref_operation_complex_expression_expression_i64() -> anyhow::Result<()> {
    let code = r#"
let a: i64 = 5;
let b: *i64 = &a;

let c: i64 = 13;
let d: *i64 = &c;

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
    sub rsp, 72
    ; let a: i64 = 5
    mov QWORD [rbp - 8], 5
    ; let b: *i64 = &a
    lea rax, [rbp - 8]
    mov QWORD [rbp - 16], rax
    ; let c: i64 = 13
    mov QWORD [rbp - 24], 13
    ; let d: *i64 = &c
    lea rax, [rbp - 24]
    mov QWORD [rbp - 32], rax
    ; let addition: i64 = ((((*d + *b) + (*b + *d)) + (*b + *b)) + ((*b + (*b + *b)) + (*b + (*d + *b))))
    ; ((((*d + *b) + (*b + *d)) + (*b + *b)) + ((*b + (*b + *b)) + (*b + (*d + *b))))
    ; (((*d + *b) + (*b + *d)) + (*b + *b))
    ; ((*d + *b) + (*b + *d))
    ; (*d + *b)
    mov rax, QWORD [rbp - 32]
    mov rax, QWORD [rax]
    mov rdx, QWORD [rbp - 16]
    mov rdx, QWORD [rdx]
    add rax, rdx
    mov rcx, rax
    ; (*b + *d)
    mov rax, QWORD [rbp - 16]
    mov rax, QWORD [rax]
    mov rdx, QWORD [rbp - 32]
    mov rdx, QWORD [rdx]
    add rax, rdx
    mov rdi, rax
    add rcx, rdi
    mov rdi, rcx
    push rdi
    xor rdi, rdi
    ; (*b + *b)
    mov rax, QWORD [rbp - 16]
    mov rax, QWORD [rax]
    mov rdx, QWORD [rbp - 16]
    mov rdx, QWORD [rdx]
    add rax, rdx
    push rax
    xor rax, rax
    pop rdi
    pop rax
    add rax, rdi
    mov rdi, rax
    push rdi
    xor rdi, rdi
    ; ((*b + (*b + *b)) + (*b + (*d + *b)))
    ; (*b + (*b + *b))
    ; (*b + *b)
    mov rax, QWORD [rbp - 16]
    mov rax, QWORD [rax]
    mov rdx, QWORD [rbp - 16]
    mov rdx, QWORD [rdx]
    add rax, rdx
    mov rdx, rax
    mov rax, QWORD [rbp - 16]
    mov rax, QWORD [rax]
    add rax, rdx
    mov rdi, rax
    push rdi
    xor rdi, rdi
    ; (*b + (*d + *b))
    ; (*d + *b)
    mov rax, QWORD [rbp - 32]
    mov rax, QWORD [rax]
    mov rdx, QWORD [rbp - 16]
    mov rdx, QWORD [rdx]
    add rax, rdx
    mov rdx, rax
    mov rax, QWORD [rbp - 16]
    mov rax, QWORD [rax]
    add rax, rdx
    push rax
    xor rax, rax
    pop rdi
    pop rax
    add rax, rdi
    push rax
    xor rax, rax
    pop rdi
    pop rax
    add rax, rdi
    mov QWORD [rbp - 40], rax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

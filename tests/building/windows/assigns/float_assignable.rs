use monkey_language::core::code_generator::generator::{ASMGenerator, Stack};
use monkey_language::core::code_generator::MetaInfo;
use monkey_language::core::code_generator::target_os::TargetOS;
use monkey_language::core::code_generator::ToASM;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::token::Token;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::lexer::tokens::assignable_token::AssignableToken;
use monkey_language::core::type_checker::static_type_checker::static_type_check;

#[test]
fn expression_assign() -> anyhow::Result<()> {
    let code = r#"
    let a: f64 = 5.0;
    let b: f64 = 10.1;
    let c: f64 = a + b;
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
    sub rsp, 56
    ; let a: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    ; let b: f64 = 10.1
    mov rax, __?float64?__(10.1)
    mov QWORD [rbp - 16], rax
    ; let c: f64 = (a Add b)
    ; (a Add b)
    movq xmm0, QWORD [rbp - 8]
    addsd xmm0, QWORD [rbp - 16]
    movq rax, xmm0
    mov QWORD [rbp - 24], rax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: f32 = 5.0 + 3.0;
    "#;

    let asm_result = asm_from_assign_code(&code)?;


    let expected = r#"
    ; let a: f32 = (5 Add 3)
    ; (5 Add 3)
    mov eax, __?float32?__(5.0)
    movd xmm0, eax
    mov eax, __?float32?__(3.0)
    movd xmm3, eax
    addss xmm0, xmm3
    movd eax, xmm0
    mov DWORD [rbp - 4], eax
    "#;

    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: f64 = (5.0_f64 + 2.0_f64) + 8.0_f64;
    "#;

    let asm_result = asm_from_assign_code(&code)?;


    let expected = r#"
    ; let a: f64 = ((5 Add 2) Add 8)
    ; ((5 Add 2) Add 8)
    ; (5 Add 2)
    mov rax, __?float64?__(5.0)
    movq xmm0, rax
    mov rax, __?float64?__(2.0)
    movq xmm3, rax
    addsd xmm0, xmm3
    mov rax, __?float64?__(8.0)
    movq xmm3, rax
    addsd xmm0, xmm3
    movq rax, xmm0
    mov QWORD [rbp - 8], rax
    "#;


    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: f64 = 5.0_f64 + (2.0_f64 + 8.0_f64);
    "#;

    let asm_result = asm_from_assign_code(&code)?;

    let expected = r#"
    ; let a: f64 = (5 Add (2 Add 8))
    ; (5 Add (2 Add 8))
    ; (2 Add 8)
    mov rax, __?float64?__(2.0)
    movq xmm0, rax
    mov rax, __?float64?__(8.0)
    movq xmm3, rax
    addsd xmm0, xmm3
    movq xmm3, xmm0
    mov rax, __?float64?__(5.0)
    movq xmm4, rax
    movq xmm0, xmm4
    addsd xmm0, xmm3
    movq rax, xmm0
    mov QWORD [rbp - 8], rax
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: f64 = (5.0_f64 + 3.0_f64) + (2.0_f64 + 8.0_f64);
    "#;

    let asm_result = asm_from_assign_code(&code)?;


    let expected = r#"
    ; let a: f64 = ((5 Add 3) Add (2 Add 8))
    ; ((5 Add 3) Add (2 Add 8))
    ; (5 Add 3)
    mov rax, __?float64?__(5.0)
    movq xmm0, rax
    mov rax, __?float64?__(3.0)
    movq xmm3, rax
    addsd xmm0, xmm3
    movq xmm1, xmm0
    ; (2 Add 8)
    mov rax, __?float64?__(2.0)
    movq xmm0, rax
    mov rax, __?float64?__(8.0)
    movq xmm3, rax
    addsd xmm0, xmm3
    movq xmm2, xmm0
    addsd xmm1, xmm2
    movq xmm0, xmm1
    movq rax, xmm0
    mov QWORD [rbp - 8], rax
    "#;


    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: f64 = 6.0_f64;
    "#;

    let asm_result = asm_from_assign_code(&code)?;


    let expected = r#"
    ; let a: f64 = 6
    mov rax, __?float64?__(6.0)
    mov QWORD [rbp - 8], rax
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: f64 = (6.0_f64);
    "#;

    let asm_result = asm_from_assign_code(&code)?;


    let expected = r#"
    ; let a: f64 = 6
    movq rax, xmm0
    mov rax, __?float64?__(6.0)
    mov QWORD [rbp - 8], rax
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn pointer_assign_test() -> anyhow::Result<()> {
    let code = r#"
    let a: f64 = 5.0_f64;
    let b: *f64 = &a;
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
    ; let a: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    ; let b: *f64 = &a
    lea rax, [rbp - 8]
    mov QWORD [rbp - 16], rax
    leave
    ret
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}


#[test]
fn pointer_assign_multiple_test() -> anyhow::Result<()> {
    let code = r#"
extern fn printf(format: *string, value: f64): void;
extern fn ExitProcess(exitCode: i32): void;

let a: f64 = 5.0_f64;
let b: *f64 = &a;
let c: **f64 = &b;
let d: *f64 = *c;

let ref: **f64 = c;
let f: f64 = *d;
let g: f64 = **c;

let format: *string = "Das ist ein Test %f";
printf(format, *b);

ExitProcess(0);
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
    db "Das ist ein Test %f", 0
main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 96
    ; let a: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    ; let b: *f64 = &a
    lea rax, [rbp - 8]
    mov QWORD [rbp - 16], rax
    ; let c: **f64 = &b
    lea rax, [rbp - 16]
    mov QWORD [rbp - 24], rax
    ; let d: *f64 = *c
    mov rax, QWORD [rbp - 24]
    mov rax, QWORD [rax]
    mov QWORD [rbp - 32], rax
    ; let ref: **f64 = c
    mov rax, QWORD [rbp - 24]
    mov QWORD [rbp - 40], rax
    ; let f: f64 = *d
    movq rax, xmm0
    mov rax, QWORD [rbp - 32]
    mov rax, QWORD [rax]
    mov QWORD [rbp - 48], rax
    ; let g: f64 = **c
    movq rax, xmm0
    mov rax, QWORD [rbp - 24]
    mov rax, QWORD [rax]
    mov rax, QWORD [rax]
    mov QWORD [rbp - 56], rax
    ; let format: *string = "Das ist ein Test %f"
    mov QWORD [rbp - 64], .label0
    mov rcx, QWORD [rbp - 64] ; Parameter (format)
    mov rax, QWORD [rbp - 16]
    mov rax, QWORD [rax]
    movq xmm1, rax ; Parameter (*b)
    mov rdx, rax ; Parameter (*b)
    ; printf(format, *b)
    call printf
    mov rcx, 0 ; Parameter (0)
    ; ExitProcess(0)
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
let a: f64 = 5.0_f64;
let b: *f64 = &a;
let addition = *b + 1.0_f64;
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
    sub rsp, 56
    ; let a: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    ; let b: *f64 = &a
    lea rax, [rbp - 8]
    mov QWORD [rbp - 16], rax
    ; let addition: f64 = (*b Add 1)
    ; (*b Add 1)
    mov rax, QWORD [rbp - 16]
    mov rax, QWORD [rax]
    movq xmm0, rax
    mov rax, __?float64?__(1.0)
    movq xmm3, rax
    addsd xmm0, xmm3
    movq rax, xmm0
    mov QWORD [rbp - 24], rax
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
let a: f64 = 5.0_f64;
let b: *f64 = &a;
let addition = 1.0_f64 + *b;
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
    sub rsp, 56
    ; let a: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    ; let b: *f64 = &a
    lea rax, [rbp - 8]
    mov QWORD [rbp - 16], rax
    ; let addition: f64 = (1 Add *b)
    ; (1 Add *b)
    mov rax, __?float64?__(1.0)
    movq xmm0, rax
    mov rdx, QWORD [rbp - 16]
    mov rdx, QWORD [rdx]
    movq xmm3, rdx
    addsd xmm0, xmm3
    movq rax, xmm0
    mov QWORD [rbp - 24], rax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}


#[test]
fn pointer_deref_operation_lhs_rhs() -> anyhow::Result<()> {
    let code = r#"
let a: f64 = 5.0_f64;
let b: *f64 = &a;
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
    sub rsp, 56
    ; let a: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    ; let b: *f64 = &a
    lea rax, [rbp - 8]
    mov QWORD [rbp - 16], rax
    ; let addition: f64 = (*b Add *b)
    ; (*b Add *b)
    mov rax, QWORD [rbp - 16]
    mov rax, QWORD [rax]
    movq xmm0, rax
    mov rdx, QWORD [rbp - 16]
    mov rdx, QWORD [rdx]
    movq xmm3, rdx
    addsd xmm0, xmm3
    movq rax, xmm0
    mov QWORD [rbp - 24], rax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn pointer_deref_operation_lhs_expression() -> anyhow::Result<()> {
    let code = r#"
let a: f64 = 5.0_f64;
let b: *f64 = &a;
let addition = *b + (0.0_f64 + 1.0_f64);
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
    sub rsp, 56
    ; let a: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    ; let b: *f64 = &a
    lea rax, [rbp - 8]
    mov QWORD [rbp - 16], rax
    ; let addition: f64 = (*b Add (0 Add 1))
    ; (*b Add (0 Add 1))
    ; (0 Add 1)
    mov rax, __?float64?__(0.0)
    movq xmm0, rax
    mov rax, __?float64?__(1.0)
    movq xmm3, rax
    addsd xmm0, xmm3
    movq xmm3, xmm0
    mov rax, QWORD [rbp - 16]
    mov rax, QWORD [rax]
    movq xmm0, rax
    addsd xmm0, xmm3
    movq rax, xmm0
    mov QWORD [rbp - 24], rax
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
let a: f64 = 5.0_f64;
let b: *f64 = &a;
let addition = (0.0_f64 + 1.0_f64) + *b;
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
    sub rsp, 56
    ; let a: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    ; let b: *f64 = &a
    lea rax, [rbp - 8]
    mov QWORD [rbp - 16], rax
    ; let addition: f64 = ((0 Add 1) Add *b)
    ; ((0 Add 1) Add *b)
    ; (0 Add 1)
    mov rax, __?float64?__(0.0)
    movq xmm0, rax
    mov rax, __?float64?__(1.0)
    movq xmm3, rax
    addsd xmm0, xmm3
    mov rdx, QWORD [rbp - 16]
    mov rdx, QWORD [rdx]
    movq xmm3, rdx
    addsd xmm0, xmm3
    movq rax, xmm0
    mov QWORD [rbp - 24], rax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn pointer_deref_operation_expression_expression() -> anyhow::Result<()> {
    let code = r#"
let a: f64 = 5.0_f64;
let b: *f64 = &a;
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
    sub rsp, 56
    ; let a: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    ; let b: *f64 = &a
    lea rax, [rbp - 8]
    mov QWORD [rbp - 16], rax
    ; let addition: f64 = ((*b Add *b) Add (*b Add *b))
    ; ((*b Add *b) Add (*b Add *b))
    ; (*b Add *b)
    mov rax, QWORD [rbp - 16]
    mov rax, QWORD [rax]
    movq xmm0, rax
    mov rdx, QWORD [rbp - 16]
    mov rdx, QWORD [rdx]
    movq xmm3, rdx
    addsd xmm0, xmm3
    movq xmm1, xmm0
    ; (*b Add *b)
    mov rax, QWORD [rbp - 16]
    mov rax, QWORD [rax]
    movq xmm0, rax
    mov rdx, QWORD [rbp - 16]
    mov rdx, QWORD [rdx]
    movq xmm3, rdx
    addsd xmm0, xmm3
    movq xmm2, xmm0
    addsd xmm1, xmm2
    movq xmm0, xmm1
    movq rax, xmm0
    mov QWORD [rbp - 24], rax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn pointer_deref_operation_complex_expression_expression() -> anyhow::Result<()> {
    let code = r#"
let a: f64 = 5.0_f64;
let b: *f64 = &a;

let c: f64 = 13.0_f64;
let d: *f64 = &c;

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
    ; let a: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    ; let b: *f64 = &a
    lea rax, [rbp - 8]
    mov QWORD [rbp - 16], rax
    ; let c: f64 = 13
    mov rax, __?float64?__(13.0)
    mov QWORD [rbp - 24], rax
    ; let d: *f64 = &c
    lea rax, [rbp - 24]
    mov QWORD [rbp - 32], rax
    ; let addition: f64 = ((((*d Add *b) Add (*b Add *d)) Add (*b Add *b)) Add ((*b Add (*b Add *b)) Add (*b Add (*d Add *b))))
    ; ((((*d Add *b) Add (*b Add *d)) Add (*b Add *b)) Add ((*b Add (*b Add *b)) Add (*b Add (*d Add *b))))
    ; (((*d Add *b) Add (*b Add *d)) Add (*b Add *b))
    ; ((*d Add *b) Add (*b Add *d))
    ; (*d Add *b)
    mov rax, QWORD [rbp - 32]
    mov rax, QWORD [rax]
    movq xmm0, rax
    mov rdx, QWORD [rbp - 16]
    mov rdx, QWORD [rdx]
    movq xmm3, rdx
    addsd xmm0, xmm3
    movq xmm1, xmm0
    ; (*b Add *d)
    mov rax, QWORD [rbp - 16]
    mov rax, QWORD [rax]
    movq xmm0, rax
    mov rdx, QWORD [rbp - 32]
    mov rdx, QWORD [rdx]
    movq xmm3, rdx
    addsd xmm0, xmm3
    movq xmm2, xmm0
    addsd xmm1, xmm2
    movq xmm0, xmm1
    movq rax, xmm0
    push rax
    xor rax, rax
    ; (*b Add *b)
    mov rax, QWORD [rbp - 16]
    mov rax, QWORD [rax]
    movq xmm0, rax
    mov rdx, QWORD [rbp - 16]
    mov rdx, QWORD [rdx]
    movq xmm3, rdx
    addsd xmm0, xmm3
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    movq xmm2, rdi
    pop rax
    movq xmm0, rax
    addsd xmm0, xmm2
    movq rax, xmm0
    push rax
    xor rax, rax
    ; ((*b Add (*b Add *b)) Add (*b Add (*d Add *b)))
    ; (*b Add (*b Add *b))
    ; (*b Add *b)
    mov rax, QWORD [rbp - 16]
    mov rax, QWORD [rax]
    movq xmm0, rax
    mov rdx, QWORD [rbp - 16]
    mov rdx, QWORD [rdx]
    movq xmm3, rdx
    addsd xmm0, xmm3
    movq xmm3, xmm0
    mov rax, QWORD [rbp - 16]
    mov rax, QWORD [rax]
    movq xmm0, rax
    addsd xmm0, xmm3
    movq xmm2, xmm0
    movq rdi, xmm2
    push rdi
    xor rdi, rdi
    ; (*b Add (*d Add *b))
    ; (*d Add *b)
    mov rax, QWORD [rbp - 32]
    mov rax, QWORD [rax]
    movq xmm0, rax
    mov rdx, QWORD [rbp - 16]
    mov rdx, QWORD [rdx]
    movq xmm3, rdx
    addsd xmm0, xmm3
    movq xmm3, xmm0
    mov rax, QWORD [rbp - 16]
    mov rax, QWORD [rax]
    movq xmm0, rax
    addsd xmm0, xmm3
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    movq xmm2, rdi
    pop rax
    movq xmm0, rax
    addsd xmm0, xmm2
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    movq xmm2, rdi
    pop rax
    movq xmm0, rax
    addsd xmm0, xmm2
    movq rax, xmm0
    mov QWORD [rbp - 40], rax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}


#[test]
fn single_expression_test() -> anyhow::Result<()> {
    let code = r#"
    let a: f64 = 5.0_f64;
    let b: f64 = a;
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
    ; let a: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    ; let b: f64 = a
    movq rax, xmm0
    mov rax, QWORD [rbp - 8]
    mov QWORD [rbp - 16], rax
    leave
    ret
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: f64 = 5.0_f64;
    let b: f64 = (a);
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    static_type_check(&top_level_scope)?;

    let mut code_generator = ASMGenerator::from((top_level_scope, TargetOS::Windows));

    let asm_result = code_generator.generate()?;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());

    Ok(())
}

#[test]
fn f32_assign() -> anyhow::Result<()> {
    let code = r#"
    let a: f32 = 512.0;
    "#;

    let asm_result = asm_from_assign_code(&code)?;

    let expected = r#"
    ; let a: f32 = 512
    mov eax, __?float32?__(512.0)
    mov DWORD [rbp - 4], eax
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

// #[test]
// fn full_program_assignable() -> anyhow::Result<()> {
//     let code = r#"
//     let a: *string = "Testing string";
//     let b: i32 = 512;
//     let c = b;
//     "#;
//
//     let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
//     let mut lexer = Lexer::from(monkey_file);
//     let top_level_scope = lexer.tokenize()?;
//
//     static_type_check(&top_level_scope)?;
//     let mut code_generator = ASMGenerator::from((top_level_scope, TargetOS::Windows));
//     let asm_result = String::from(code_generator.generate()?.trim());
//
//     println!("{}", asm_result);
//
//     let expected = r#"; This assembly is targeted for the Windows Operating System
// segment .text
// global main
//
//
// .label0:
//     db "Testing string", 0
// main:
//     push rbp
//     mov rbp, rsp
//     ; Reserve stack space as MS convention. Shadow stacking
//     sub rsp, 48
//     ; let a: *string = "Testing string"
//     mov QWORD [rbp - 8], .label0
//     ; let b: i32 = 512
//     mov DWORD [rbp - 12], 512
//     ; let c: i32 = b
//     mov eax, DWORD [rbp - 12]
//     mov DWORD [rbp - 16], eax
//     leave
//     ret"#;
//
//     assert_eq!(expected, asm_result);
//
//     Ok(())
// }
//
// #[test]
// fn assignable_different_integer_types() -> anyhow::Result<()> {
//     let code = r#"
//     let a: i64 = 512;
//     "#;
//
//     let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
//     let mut lexer = Lexer::from(monkey_file);
//     let top_level_scope = lexer.tokenize()?;
//
//     static_type_check(&top_level_scope)?;
//     let mut code_generator = ASMGenerator::from((top_level_scope, TargetOS::Windows));
//     let asm_result = String::from(code_generator.generate()?.trim());
//
//     println!("{}", asm_result);
//
//     let expected = r#"; This assembly is targeted for the Windows Operating System
// segment .text
// global main
//
//
// main:
//     push rbp
//     mov rbp, rsp
//     ; Reserve stack space as MS convention. Shadow stacking
//     sub rsp, 40
//     ; let a: i64 = 512
//     mov QWORD [rbp - 8], 512
//     leave
//     ret"#;
//
//     assert_eq!(expected, asm_result);
//
//     Ok(())
// }
//
// #[test]
// fn basic_add_different_type() -> anyhow::Result<()> {
//     let code = r#"
//     let a: i64 = 512;
//     let b: i64 = 5;
//     let c = a + b;
//     "#;
//
//     let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
//     let mut lexer = Lexer::from(monkey_file);
//     let top_level_scope = lexer.tokenize()?;
//
//     static_type_check(&top_level_scope)?;
//     let mut code_generator = ASMGenerator::from((top_level_scope, TargetOS::Windows));
//     let asm_result = String::from(code_generator.generate()?.trim());
//
//     println!("{}", asm_result);
//
//     let expected = r#"; This assembly is targeted for the Windows Operating System
// segment .text
// global main
//
//
// main:
//     push rbp
//     mov rbp, rsp
//     ; Reserve stack space as MS convention. Shadow stacking
//     sub rsp, 56
//     ; let a: i64 = 512
//     mov QWORD [rbp - 8], 512
//     ; let b: i64 = 5
//     mov QWORD [rbp - 16], 5
//     ; let c: i64 = (a Add b)
//     ; (a Add b)
//     mov rax, QWORD [rbp - 8]
//     add rax, QWORD [rbp - 16]
//     mov QWORD [rbp - 24], rax
//     leave
//     ret"#;
//
//     assert_eq!(expected, asm_result);
//
//     Ok(())
// }
//
// #[test]
// fn pointer_deref_operation_complex_expression_expression_i64() -> anyhow::Result<()> {
//     let code = r#"
// let a: i64 = 5;
// let b: *i64 = &a;
//
// let c: i64 = 13;
// let d: *i64 = &c;
//
// let addition = (((*d + *b) + (*b + *d)) + (*b + *b)) + ((*b + (*b + *b)) + (*b + (*d + *b)));
//     "#;
//
//     let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
//     let mut lexer = Lexer::from(monkey_file);
//     let top_level_scope = lexer.tokenize()?;
//
//     static_type_check(&top_level_scope)?;
//
//     let mut code_generator = ASMGenerator::from((top_level_scope, TargetOS::Windows));
//     let asm_result = code_generator.generate()?;
//
//
//     println!("{}", asm_result);
//
//     let expected = r#"
// ; This assembly is targeted for the Windows Operating System
// segment .text
// global main
//
//
// main:
//     push rbp
//     mov rbp, rsp
//     ; Reserve stack space as MS convention. Shadow stacking
//     sub rsp, 72
//     ; let a: i64 = 5
//     mov QWORD [rbp - 8], 5
//     ; let b: *i64 = &a
//     lea rax, [rbp - 8]
//     mov QWORD [rbp - 16], rax
//     ; let c: i64 = 13
//     mov QWORD [rbp - 24], 13
//     ; let d: *i64 = &c
//     lea rax, [rbp - 24]
//     mov QWORD [rbp - 32], rax
//     ; let addition: i64 = ((((*d Add *b) Add (*b Add *d)) Add (*b Add *b)) Add ((*b Add (*b Add *b)) Add (*b Add (*d Add *b))))
//     ; ((((*d Add *b) Add (*b Add *d)) Add (*b Add *b)) Add ((*b Add (*b Add *b)) Add (*b Add (*d Add *b))))
//     ; (((*d Add *b) Add (*b Add *d)) Add (*b Add *b))
//     ; ((*d Add *b) Add (*b Add *d))
//     ; (*d Add *b)
//     mov rax, QWORD [rbp - 32]
//     mov rax, QWORD [rax]
//     mov rdx, QWORD [rbp - 16]
//     mov rdx, QWORD [rdx]
//     add rax, rdx
//     mov rcx, rax
//     ; (*b Add *d)
//     mov rax, QWORD [rbp - 16]
//     mov rax, QWORD [rax]
//     mov rdx, QWORD [rbp - 32]
//     mov rdx, QWORD [rdx]
//     add rax, rdx
//     mov rdi, rax
//     add rcx, rdi
//     mov rax, rcx
//     push rax
//     xor rax, rax
//     ; (*b Add *b)
//     mov rax, QWORD [rbp - 16]
//     mov rax, QWORD [rax]
//     mov rdx, QWORD [rbp - 16]
//     mov rdx, QWORD [rdx]
//     add rax, rdx
//     push rax
//     xor rax, rax
//     pop rdi
//     pop rax
//     add rax, rdi
//     push rax
//     xor rax, rax
//     ; ((*b Add (*b Add *b)) Add (*b Add (*d Add *b)))
//     ; (*b Add (*b Add *b))
//     ; (*b Add *b)
//     mov rax, QWORD [rbp - 16]
//     mov rax, QWORD [rax]
//     mov rdx, QWORD [rbp - 16]
//     mov rdx, QWORD [rdx]
//     add rax, rdx
//     mov rdx, rax
//     mov rax, QWORD [rbp - 16]
//     mov rax, QWORD [rax]
//     add rax, rdx
//     mov rdi, rax
//     push rdi
//     xor rdi, rdi
//     ; (*b Add (*d Add *b))
//     ; (*d Add *b)
//     mov rax, QWORD [rbp - 32]
//     mov rax, QWORD [rax]
//     mov rdx, QWORD [rbp - 16]
//     mov rdx, QWORD [rdx]
//     add rax, rdx
//     mov rdx, rax
//     mov rax, QWORD [rbp - 16]
//     mov rax, QWORD [rax]
//     add rax, rdx
//     push rax
//     xor rax, rax
//     pop rdi
//     pop rax
//     add rax, rdi
//     push rax
//     xor rax, rax
//     pop rdi
//     pop rax
//     add rax, rdi
//     mov QWORD [rbp - 40], rax
//     leave
//     ret
//     "#;
//
//     assert_eq!(expected.trim(), asm_result.trim());
//     Ok(())
// }
//
//
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
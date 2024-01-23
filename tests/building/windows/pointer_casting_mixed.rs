use monkey_language::core::code_generator::generator::ASMGenerator;
use monkey_language::core::code_generator::target_os::TargetOS;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::type_checker::static_type_checker::static_type_check;

#[test]
fn float_cast_deref_simple() -> anyhow::Result<()> {
    let code = r#"
let a: f64 = 5.0_f64;
let b: *f64 = &a;

let c: f32 = (f32)*b;
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
    sub rsp, 52
    ; let a: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    ; let b: *f64 = &a
    lea rax, [rbp - 8]
    mov QWORD [rbp - 16], rax
    ; let c: f32 = (f32)*b
    movd eax, xmm0
    mov rax, QWORD [rbp - 16]
    mov rax, QWORD [rax]
    movq xmm7, rax
    cvtsd2ss xmm7, xmm7
    movd eax, xmm7
    mov DWORD [rbp - 20], eax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn float_cast_deref_middle() -> anyhow::Result<()> {
    let code = r#"
let a: f64 = 5.0_f64;
let b: *f64 = &a;
let c: **f64 = &b;

let d = (f32)**c;
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
    ; let a: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    ; let b: *f64 = &a
    lea rax, [rbp - 8]
    mov QWORD [rbp - 16], rax
    ; let c: **f64 = &b
    lea rax, [rbp - 16]
    mov QWORD [rbp - 24], rax
    ; let d: f32 = (f32)**c
    movd eax, xmm0
    mov rax, QWORD [rbp - 24]
    mov rax, QWORD [rax]
    mov rax, QWORD [rax]
    movq xmm7, rax
    cvtsd2ss xmm7, xmm7
    movd eax, xmm7
    mov DWORD [rbp - 28], eax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn float_cast_deref_complex() -> anyhow::Result<()> {
    let code = r#"
let a: f64 = 5.0_f64;
let b: *f64 = &a;

let c: f32 = ((f32)*b + (f32)*b) * (f32)*b;
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
    sub rsp, 52
    ; let a: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    ; let b: *f64 = &a
    lea rax, [rbp - 8]
    mov QWORD [rbp - 16], rax
    ; let c: f32 = (((f32)*b Add (f32)*b) Mul (f32)*b)
    ; (((f32)*b Add (f32)*b) Mul (f32)*b)
    ; ((f32)*b Add (f32)*b)
    mov rax, QWORD [rbp - 16]
    mov rax, QWORD [rax]
    movq xmm0, rax
    movq xmm7, rax
    cvtsd2ss xmm7, xmm7
    movd eax, xmm7
    movd xmm0, eax
    mov rdx, QWORD [rbp - 16]
    mov rdx, QWORD [rdx]
    movq xmm3, rdx
    movq xmm7, rdx
    cvtsd2ss xmm7, xmm7
    movd edx, xmm7
    movd xmm3, edx
    addss xmm0, xmm3
    mov rdx, QWORD [rbp - 16]
    mov rdx, QWORD [rdx]
    movq xmm3, rdx
    movq xmm7, rdx
    cvtsd2ss xmm7, xmm7
    movd edx, xmm7
    movd xmm3, edx
    imulss xmm0, xmm3
    movd eax, xmm0
    mov DWORD [rbp - 20], eax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}
use monkey_language::core::code_generator::generator::ASMGenerator;
use monkey_language::core::code_generator::target_os::TargetOS;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::type_checker::static_type_checker::static_type_check;

#[test]
fn mixed_operations_mul() -> anyhow::Result<()> {
    let code = r#"
extern fn printf(format: *string, value: f64): void;
let a: f32 = 5.0 + 1.0 * 100.0;
printf("%f", (f64)a);
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

.label0:
    db "%f", 0

main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 36
    ; let a: f32 = (5 + (1 * 100))
    ; (5 + (1 * 100))
    ; (1 * 100)
    mov eax, __?float32?__(1.0)
    movd xmm0, eax
    mov eax, __?float32?__(100.0)
    movd xmm3, eax
    mulss xmm0, xmm3
    movq xmm3, xmm0
    mov eax, __?float32?__(5.0)
    movd xmm4, eax
    movq xmm0, xmm4
    addss xmm0, xmm3
    movd eax, xmm0
    mov DWORD [rbp - 4], eax
    mov rcx, .label0 ; Parameter ("%f")
    mov eax, DWORD [rbp - 4]
    movd xmm7, eax
    cvtss2sd xmm7, xmm7
    movq rax, xmm7
    movq xmm1, rax ; Parameter ((f64)a)
    mov rdx, rax ; Parameter ((f64)a)
    ; printf("%f", (f64)a)
    call printf
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn mixed_operations_sub() -> anyhow::Result<()> {
    let code = r#"
extern fn printf(format: *string, value: f64): void;
let a: f32 = 5.0 * 1.0 - 100.0;
printf("%f", (f64)a);
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

.label0:
    db "%f", 0

main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 36
    ; let a: f32 = ((5 * 1) - 100)
    ; ((5 * 1) - 100)
    ; (5 * 1)
    mov eax, __?float32?__(5.0)
    movd xmm0, eax
    mov eax, __?float32?__(1.0)
    movd xmm3, eax
    mulss xmm0, xmm3
    mov eax, __?float32?__(100.0)
    movd xmm3, eax
    subss xmm0, xmm3
    movd eax, xmm0
    mov DWORD [rbp - 4], eax
    mov rcx, .label0 ; Parameter ("%f")
    mov eax, DWORD [rbp - 4]
    movd xmm7, eax
    cvtss2sd xmm7, xmm7
    movq rax, xmm7
    movq xmm1, rax ; Parameter ((f64)a)
    mov rdx, rax ; Parameter ((f64)a)
    ; printf("%f", (f64)a)
    call printf
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn mixed_operations() -> anyhow::Result<()> {
    let code = r#"
extern fn printf(format: *string, value: f64): void;
let a: f32 = ((3.5 + 1.2) * 4.8 - (9.6 / 2.4)) * ((7.2 + 3.6) / 2.1 - (8.4 * 3.7)) + ((6.3 - 2.1) * 3.8 / (7.9 + 4.2));
printf("%f", (f64)a);
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

.label0:
    db "%f", 0

main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 36
    ; let a: f32 = (((((3.5 + 1.2) * 4.8) - (9.6 / 2.4)) * (((7.2 + 3.6) / 2.1) - (8.4 * 3.7))) + (((6.3 - 2.1) * 3.8) / (7.9 + 4.2)))
    ; (((((3.5 + 1.2) * 4.8) - (9.6 / 2.4)) * (((7.2 + 3.6) / 2.1) - (8.4 * 3.7))) + (((6.3 - 2.1) * 3.8) / (7.9 + 4.2)))
    ; ((((3.5 + 1.2) * 4.8) - (9.6 / 2.4)) * (((7.2 + 3.6) / 2.1) - (8.4 * 3.7)))
    ; (((3.5 + 1.2) * 4.8) - (9.6 / 2.4))
    ; ((3.5 + 1.2) * 4.8)
    ; (3.5 + 1.2)
    mov eax, __?float32?__(3.5)
    movd xmm0, eax
    mov eax, __?float32?__(1.2)
    movd xmm3, eax
    addss xmm0, xmm3
    mov eax, __?float32?__(4.8)
    movd xmm3, eax
    mulss xmm0, xmm3
    movq xmm2, xmm0
    movq rdi, xmm2
    push rdi
    xor rdi, rdi
    ; (9.6 / 2.4)
    mov eax, __?float32?__(9.6)
    movd xmm0, eax
    mov eax, __?float32?__(2.4)
    movd xmm3, eax
    divss xmm0, xmm3
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    movd xmm2, edi
    pop rax
    movd xmm0, eax
    subss xmm0, xmm2
    movq rax, xmm0
    push rax
    xor rax, rax
    ; (((7.2 + 3.6) / 2.1) - (8.4 * 3.7))
    ; ((7.2 + 3.6) / 2.1)
    ; (7.2 + 3.6)
    mov eax, __?float32?__(7.2)
    movd xmm0, eax
    mov eax, __?float32?__(3.6)
    movd xmm3, eax
    addss xmm0, xmm3
    mov eax, __?float32?__(2.1)
    movd xmm3, eax
    divss xmm0, xmm3
    movq xmm2, xmm0
    movq rdi, xmm2
    push rdi
    xor rdi, rdi
    ; (8.4 * 3.7)
    mov eax, __?float32?__(8.4)
    movd xmm0, eax
    mov eax, __?float32?__(3.7)
    movd xmm3, eax
    mulss xmm0, xmm3
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    movd xmm2, edi
    pop rax
    movd xmm0, eax
    subss xmm0, xmm2
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    movd xmm2, edi
    pop rax
    movd xmm0, eax
    mulss xmm0, xmm2
    movq rax, xmm0
    push rax
    xor rax, rax
    ; (((6.3 - 2.1) * 3.8) / (7.9 + 4.2))
    ; ((6.3 - 2.1) * 3.8)
    ; (6.3 - 2.1)
    mov eax, __?float32?__(6.3)
    movd xmm0, eax
    mov eax, __?float32?__(2.1)
    movd xmm3, eax
    subss xmm0, xmm3
    mov eax, __?float32?__(3.8)
    movd xmm3, eax
    mulss xmm0, xmm3
    movq xmm2, xmm0
    movq rdi, xmm2
    push rdi
    xor rdi, rdi
    ; (7.9 + 4.2)
    mov eax, __?float32?__(7.9)
    movd xmm0, eax
    mov eax, __?float32?__(4.2)
    movd xmm3, eax
    addss xmm0, xmm3
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    movd xmm2, edi
    pop rax
    movd xmm0, eax
    divss xmm0, xmm2
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    movd xmm2, edi
    pop rax
    movd xmm0, eax
    addss xmm0, xmm2
    movd eax, xmm0
    mov DWORD [rbp - 4], eax
    mov rcx, .label0 ; Parameter ("%f")
    mov eax, DWORD [rbp - 4]
    movd xmm7, eax
    cvtss2sd xmm7, xmm7
    movq rax, xmm7
    movq xmm1, rax ; Parameter ((f64)a)
    mov rdx, rax ; Parameter ((f64)a)
    ; printf("%f", (f64)a)
    call printf
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn mixed_operations_f64() -> anyhow::Result<()> {
    let code = r#"
extern fn printf(format: *string, value: f64): void;
let a: f64 = ((3.5_f64 + 1.2_f64) * 4.8_f64 - (9.6_f64 / 2.4_f64)) * ((7.2_f64 + 3.6_f64) / 2.1_f64 - (8.4_f64 * 3.7_f64)) + ((6.3_f64 - 2.1_f64) * 3.8_f64 / (7.9_f64 + 4.2_f64));
printf("%f", a);
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

.label0:
    db "%f", 0

main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 40
    ; let a: f64 = (((((3.5 + 1.2) * 4.8) - (9.6 / 2.4)) * (((7.2 + 3.6) / 2.1) - (8.4 * 3.7))) + (((6.3 - 2.1) * 3.8) / (7.9 + 4.2)))
    ; (((((3.5 + 1.2) * 4.8) - (9.6 / 2.4)) * (((7.2 + 3.6) / 2.1) - (8.4 * 3.7))) + (((6.3 - 2.1) * 3.8) / (7.9 + 4.2)))
    ; ((((3.5 + 1.2) * 4.8) - (9.6 / 2.4)) * (((7.2 + 3.6) / 2.1) - (8.4 * 3.7)))
    ; (((3.5 + 1.2) * 4.8) - (9.6 / 2.4))
    ; ((3.5 + 1.2) * 4.8)
    ; (3.5 + 1.2)
    mov rax, __?float64?__(3.5)
    movq xmm0, rax
    mov rax, __?float64?__(1.2)
    movq xmm3, rax
    addsd xmm0, xmm3
    mov rax, __?float64?__(4.8)
    movq xmm3, rax
    mulsd xmm0, xmm3
    movq xmm2, xmm0
    movq rdi, xmm2
    push rdi
    xor rdi, rdi
    ; (9.6 / 2.4)
    mov rax, __?float64?__(9.6)
    movq xmm0, rax
    mov rax, __?float64?__(2.4)
    movq xmm3, rax
    divsd xmm0, xmm3
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    movq xmm2, rdi
    pop rax
    movq xmm0, rax
    subsd xmm0, xmm2
    movq rax, xmm0
    push rax
    xor rax, rax
    ; (((7.2 + 3.6) / 2.1) - (8.4 * 3.7))
    ; ((7.2 + 3.6) / 2.1)
    ; (7.2 + 3.6)
    mov rax, __?float64?__(7.2)
    movq xmm0, rax
    mov rax, __?float64?__(3.6)
    movq xmm3, rax
    addsd xmm0, xmm3
    mov rax, __?float64?__(2.1)
    movq xmm3, rax
    divsd xmm0, xmm3
    movq xmm2, xmm0
    movq rdi, xmm2
    push rdi
    xor rdi, rdi
    ; (8.4 * 3.7)
    mov rax, __?float64?__(8.4)
    movq xmm0, rax
    mov rax, __?float64?__(3.7)
    movq xmm3, rax
    mulsd xmm0, xmm3
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    movq xmm2, rdi
    pop rax
    movq xmm0, rax
    subsd xmm0, xmm2
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    movq xmm2, rdi
    pop rax
    movq xmm0, rax
    mulsd xmm0, xmm2
    movq rax, xmm0
    push rax
    xor rax, rax
    ; (((6.3 - 2.1) * 3.8) / (7.9 + 4.2))
    ; ((6.3 - 2.1) * 3.8)
    ; (6.3 - 2.1)
    mov rax, __?float64?__(6.3)
    movq xmm0, rax
    mov rax, __?float64?__(2.1)
    movq xmm3, rax
    subsd xmm0, xmm3
    mov rax, __?float64?__(3.8)
    movq xmm3, rax
    mulsd xmm0, xmm3
    movq xmm2, xmm0
    movq rdi, xmm2
    push rdi
    xor rdi, rdi
    ; (7.9 + 4.2)
    mov rax, __?float64?__(7.9)
    movq xmm0, rax
    mov rax, __?float64?__(4.2)
    movq xmm3, rax
    addsd xmm0, xmm3
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    movq xmm2, rdi
    pop rax
    movq xmm0, rax
    divsd xmm0, xmm2
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    movq xmm2, rdi
    pop rax
    movq xmm0, rax
    addsd xmm0, xmm2
    movq rax, xmm0
    mov QWORD [rbp - 8], rax
    mov rcx, .label0 ; Parameter ("%f")
    movq xmm1, QWORD [rbp - 8] ; Parameter (a)
    mov rdx, QWORD [rbp - 8] ; Parameter (a)
    ; printf("%f", a)
    call printf
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn mixed_operations_div_0() -> anyhow::Result<()> {
    let code = r#"
extern fn printf(format: *string, value: f64): void;
let a: f32 = 5.0 * 1.0 / 0.0;
printf("%f", (f64)a);
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

.label0:
    db "%f", 0

main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 36
    ; let a: f32 = ((5 * 1) / 0)
    ; ((5 * 1) / 0)
    ; (5 * 1)
    mov eax, __?float32?__(5.0)
    movd xmm0, eax
    mov eax, __?float32?__(1.0)
    movd xmm3, eax
    mulss xmm0, xmm3
    mov eax, __?float32?__(0.0)
    movd xmm3, eax
    divss xmm0, xmm3
    movd eax, xmm0
    mov DWORD [rbp - 4], eax
    mov rcx, .label0 ; Parameter ("%f")
    mov eax, DWORD [rbp - 4]
    movd xmm7, eax
    cvtss2sd xmm7, xmm7
    movq rax, xmm7
    movq xmm1, rax ; Parameter ((f64)a)
    mov rdx, rax ; Parameter ((f64)a)
    ; printf("%f", (f64)a)
    call printf
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}
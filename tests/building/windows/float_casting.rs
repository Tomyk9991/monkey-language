use monkey_language::core::code_generator::generator::ASMGenerator;
use monkey_language::core::code_generator::target_os::TargetOS;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::type_checker::static_type_checker::static_type_check;

#[test]
fn float_cast_simple() -> anyhow::Result<()> {
    let code = r#"
let a: f64 = 5.0_f64;
let b: f32 = (f32) a;
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
    sub rsp, 44
    ; let a: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    ; let b: f32 = (f32)a
    movd eax, xmm0
    mov rax, QWORD [rbp - 8]
    movq xmm7, rax
    cvtsd2ss xmm7, xmm7
    movd eax, xmm7
    mov DWORD [rbp - 12], eax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn float_cast_inline() -> anyhow::Result<()> {
    let code = r#"
extern fn printf(format: *string, value: void): void;
let a: f32 = 5.0;
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
    ; let a: f32 = 5
    mov eax, __?float32?__(5.0)
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
fn float_double_cast() -> anyhow::Result<()> {
    let code = r#"
extern fn printf(format: *string, value: void): void;
let a: f64 = 5.0_f64;
printf("%f", (f64)(f32)a);
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
    ; let a: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    mov rcx, .label0 ; Parameter ("%f")
    mov rax, QWORD [rbp - 8]
    movq xmm7, rax
    cvtsd2ss xmm7, xmm7
    movd eax, xmm7
    movd eax, xmm7
    movd xmm7, eax
    cvtss2sd xmm7, xmm7
    movq rax, xmm7
    movq xmm1, rax ; Parameter ((f64)(f32)a)
    mov rdx, rax ; Parameter ((f64)(f32)a)
    ; printf("%f", (f64)(f32)a)
    call printf
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn float_cast_expression() -> anyhow::Result<()> {
    let code = r#"
extern fn printf(format: *string, value: void): void;
let a: f64 = 5.0_f64;
let b: f32 = (f32)(a + 1.0_f64);
printf("%f", (f64) b);
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
    sub rsp, 44
    ; let a: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    ; let b: f32 = (f32)(a Add 1)
    movd eax, xmm0
    ; (a Add 1)
    movq xmm0, QWORD [rbp - 8]
    mov rax, __?float64?__(1.0)
    movq xmm3, rax
    addsd xmm0, xmm3
    movq rax, xmm0
    movq xmm7, rax
    cvtsd2ss xmm7, xmm7
    movd eax, xmm7
    mov DWORD [rbp - 12], eax
    mov rcx, .label0 ; Parameter ("%f")
    mov eax, DWORD [rbp - 12]
    movd xmm7, eax
    cvtss2sd xmm7, xmm7
    movq rax, xmm7
    movq xmm1, rax ; Parameter ((f64)b)
    mov rdx, rax ; Parameter ((f64)b)
    ; printf("%f", (f64)b)
    call printf
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn float_cast_lhs() -> anyhow::Result<()> {
    let code = r#"
extern fn printf(format: *string, value: void): void;
let a: f64 = 5.0_f64;
let b: f32 = ((f32)a + 1.0_f32);
printf("%f", (f64) b);
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
    sub rsp, 44
    ; let a: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    ; let b: f32 = ((f32)a Add 1)
    ; ((f32)a Add 1)
    mov rax, QWORD [rbp - 8]
    movq xmm7, rax
    cvtsd2ss xmm7, xmm7
    movd eax, xmm7
    movd xmm0, eax
    mov eax, __?float32?__(1.0)
    movd xmm3, eax
    addss xmm0, xmm3
    movd eax, xmm0
    mov DWORD [rbp - 12], eax
    mov rcx, .label0 ; Parameter ("%f")
    mov eax, DWORD [rbp - 12]
    movd xmm7, eax
    cvtss2sd xmm7, xmm7
    movq rax, xmm7
    movq xmm1, rax ; Parameter ((f64)b)
    mov rdx, rax ; Parameter ((f64)b)
    ; printf("%f", (f64)b)
    call printf
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn float_cast_lhs_rhs() -> anyhow::Result<()> {
    let code = r#"
extern fn printf(format: *string, value: void): void;
let a: f64 = 5.0_f64;
let b: f64 = 11.3_f64;

let c: f32 = ((f32) a + (f32)b);
printf("%f", (f64) c);
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
    sub rsp, 52
    ; let a: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    ; let b: f64 = 11.3
    mov rax, __?float64?__(11.3)
    mov QWORD [rbp - 16], rax
    ; let c: f32 = ((f32)a Add (f32)b)
    ; ((f32)a Add (f32)b)
    mov rax, QWORD [rbp - 8]
    movq xmm7, rax
    cvtsd2ss xmm7, xmm7
    movd eax, xmm7
    movd xmm0, eax
    mov rdx, QWORD [rbp - 16]
    movq xmm7, rdx
    cvtsd2ss xmm7, xmm7
    movd edx, xmm7
    movd xmm3, edx
    addss xmm0, xmm3
    movd eax, xmm0
    mov DWORD [rbp - 20], eax
    mov rcx, .label0 ; Parameter ("%f")
    mov eax, DWORD [rbp - 20]
    movd xmm7, eax
    cvtss2sd xmm7, xmm7
    movq rax, xmm7
    movq xmm1, rax ; Parameter ((f64)c)
    mov rdx, rax ; Parameter ((f64)c)
    ; printf("%f", (f64)c)
    call printf
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn float_cast_rhs() -> anyhow::Result<()> {
    let code = r#"
extern fn printf(format: *string, value: void): void;
let a: f64 = 5.0_f64;
let b: f32 = (1.0_f32 + (f32)a);
printf("%f", (f64) b);
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
    sub rsp, 44
    ; let a: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    ; let b: f32 = (1 Add (f32)a)
    ; (1 Add (f32)a)
    mov eax, __?float32?__(1.0)
    movd xmm0, eax
    mov rdx, QWORD [rbp - 8]
    movq xmm7, rdx
    cvtsd2ss xmm7, xmm7
    movd edx, xmm7
    movd xmm3, edx
    addss xmm0, xmm3
    movd eax, xmm0
    mov DWORD [rbp - 12], eax
    mov rcx, .label0 ; Parameter ("%f")
    mov eax, DWORD [rbp - 12]
    movd xmm7, eax
    cvtss2sd xmm7, xmm7
    movq rax, xmm7
    movq xmm1, rax ; Parameter ((f64)b)
    mov rdx, rax ; Parameter ((f64)b)
    ; printf("%f", (f64)b)
    call printf
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn float_cast_lhs_expression() -> anyhow::Result<()> {
    let code = r#"
extern fn printf(format: *string, value: void): void;
let a: f64 = 5.0_f64;
let b: f32 = (f32)a + (1.0_f32 + 5.1_f32);
printf("%f", (f64) b);
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
    sub rsp, 44
    ; let a: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    ; let b: f32 = ((f32)a Add (1 Add 5.1))
    ; ((f32)a Add (1 Add 5.1))
    ; (1 Add 5.1)
    mov eax, __?float32?__(1.0)
    movd xmm0, eax
    mov eax, __?float32?__(5.1)
    movd xmm3, eax
    addss xmm0, xmm3
    movq xmm3, xmm0
    mov rax, QWORD [rbp - 8]
    movq xmm7, rax
    cvtsd2ss xmm7, xmm7
    movd eax, xmm7
    movd xmm0, eax
    addss xmm0, xmm3
    movd eax, xmm0
    mov DWORD [rbp - 12], eax
    mov rcx, .label0 ; Parameter ("%f")
    mov eax, DWORD [rbp - 12]
    movd xmm7, eax
    cvtss2sd xmm7, xmm7
    movq rax, xmm7
    movq xmm1, rax ; Parameter ((f64)b)
    mov rdx, rax ; Parameter ((f64)b)
    ; printf("%f", (f64)b)
    call printf
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn float_cast_expression_rhs() -> anyhow::Result<()> {
    let code = r#"
extern fn printf(format: *string, value: void): void;
let a: f64 = 5.0_f64;
let b: f32 = (1.0_f32 + 5.1_f32) + (f32)a;
printf("%f", (f64) b);
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
    sub rsp, 44
    ; let a: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    ; let b: f32 = ((1 Add 5.1) Add (f32)a)
    ; ((1 Add 5.1) Add (f32)a)
    ; (1 Add 5.1)
    mov eax, __?float32?__(1.0)
    movd xmm0, eax
    mov eax, __?float32?__(5.1)
    movd xmm3, eax
    addss xmm0, xmm3
    mov rdx, QWORD [rbp - 8]
    movq xmm7, rdx
    cvtsd2ss xmm7, xmm7
    movd edx, xmm7
    movd xmm3, edx
    addss xmm0, xmm3
    movd eax, xmm0
    mov DWORD [rbp - 12], eax
    mov rcx, .label0 ; Parameter ("%f")
    mov eax, DWORD [rbp - 12]
    movd xmm7, eax
    cvtss2sd xmm7, xmm7
    movq rax, xmm7
    movq xmm1, rax ; Parameter ((f64)b)
    mov rdx, rax ; Parameter ((f64)b)
    ; printf("%f", (f64)b)
    call printf
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn float_cast_expression_expression() -> anyhow::Result<()> {
    let code = r#"
extern fn printf(format: *string, value: void): void;
let a: f64 = 5.0_f64;
let b: f32 = ((f32)a + (f32)a) + ((f32)a + (f32)a);
printf("%f", (f64) b);
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
    sub rsp, 44
    ; let a: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    ; let b: f32 = (((f32)a Add (f32)a) Add ((f32)a Add (f32)a))
    ; (((f32)a Add (f32)a) Add ((f32)a Add (f32)a))
    ; ((f32)a Add (f32)a)
    mov rax, QWORD [rbp - 8]
    movq xmm7, rax
    cvtsd2ss xmm7, xmm7
    movd eax, xmm7
    movd xmm0, eax
    mov rdx, QWORD [rbp - 8]
    movq xmm7, rdx
    cvtsd2ss xmm7, xmm7
    movd edx, xmm7
    movd xmm3, edx
    addss xmm0, xmm3
    movq xmm1, xmm0
    ; ((f32)a Add (f32)a)
    mov rax, QWORD [rbp - 8]
    movq xmm7, rax
    cvtsd2ss xmm7, xmm7
    movd eax, xmm7
    movd xmm0, eax
    mov rdx, QWORD [rbp - 8]
    movq xmm7, rdx
    cvtsd2ss xmm7, xmm7
    movd edx, xmm7
    movd xmm3, edx
    addss xmm0, xmm3
    movq xmm2, xmm0
    addss xmm1, xmm2
    movq xmm0, xmm1
    movd eax, xmm0
    mov DWORD [rbp - 12], eax
    mov rcx, .label0 ; Parameter ("%f")
    mov eax, DWORD [rbp - 12]
    movd xmm7, eax
    cvtss2sd xmm7, xmm7
    movq rax, xmm7
    movq xmm1, rax ; Parameter ((f64)b)
    mov rdx, rax ; Parameter ((f64)b)
    ; printf("%f", (f64)b)
    call printf
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn float_cast_complex_expression() -> anyhow::Result<()> {
    let code = r#"
extern fn printf(format: *string, value: void): void;
let b: f64 = 5.0_f64;
let d: f64 = 13.0_f64;
let addition: f32 = ((((f32)d + (f32)b) + ((f32)b + (f32)d)) + ((f32)b + (f32)b)) + (((f32)b + ((f32)b + (f32)b)) + ((f32)b + ((f32)d + (f32)b)));
printf("%f", (f64) addition);
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
    sub rsp, 52
    ; let b: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    ; let d: f64 = 13
    mov rax, __?float64?__(13.0)
    mov QWORD [rbp - 16], rax
    ; let addition: f32 = (((((f32)d Add (f32)b) Add ((f32)b Add (f32)d)) Add ((f32)b Add (f32)b)) Add (((f32)b Add ((f32)b Add (f32)b)) Add ((f32)b Add ((f32)d Add (f32)b))))
    ; (((((f32)d Add (f32)b) Add ((f32)b Add (f32)d)) Add ((f32)b Add (f32)b)) Add (((f32)b Add ((f32)b Add (f32)b)) Add ((f32)b Add ((f32)d Add (f32)b))))
    ; ((((f32)d Add (f32)b) Add ((f32)b Add (f32)d)) Add ((f32)b Add (f32)b))
    ; (((f32)d Add (f32)b) Add ((f32)b Add (f32)d))
    ; ((f32)d Add (f32)b)
    mov rax, QWORD [rbp - 16]
    movq xmm7, rax
    cvtsd2ss xmm7, xmm7
    movd eax, xmm7
    movd xmm0, eax
    mov rdx, QWORD [rbp - 8]
    movq xmm7, rdx
    cvtsd2ss xmm7, xmm7
    movd edx, xmm7
    movd xmm3, edx
    addss xmm0, xmm3
    movq xmm1, xmm0
    ; ((f32)b Add (f32)d)
    mov rax, QWORD [rbp - 8]
    movq xmm7, rax
    cvtsd2ss xmm7, xmm7
    movd eax, xmm7
    movd xmm0, eax
    mov rdx, QWORD [rbp - 16]
    movq xmm7, rdx
    cvtsd2ss xmm7, xmm7
    movd edx, xmm7
    movd xmm3, edx
    addss xmm0, xmm3
    movq xmm2, xmm0
    addss xmm1, xmm2
    movq xmm0, xmm1
    movq rax, xmm0
    push rax
    xor rax, rax
    ; ((f32)b Add (f32)b)
    mov rax, QWORD [rbp - 8]
    movq xmm7, rax
    cvtsd2ss xmm7, xmm7
    movd eax, xmm7
    movd xmm0, eax
    mov rdx, QWORD [rbp - 8]
    movq xmm7, rdx
    cvtsd2ss xmm7, xmm7
    movd edx, xmm7
    movd xmm3, edx
    addss xmm0, xmm3
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    movd xmm2, edi
    pop rax
    movd xmm0, eax
    addss xmm0, xmm2
    movq rax, xmm0
    push rax
    xor rax, rax
    ; (((f32)b Add ((f32)b Add (f32)b)) Add ((f32)b Add ((f32)d Add (f32)b)))
    ; ((f32)b Add ((f32)b Add (f32)b))
    ; ((f32)b Add (f32)b)
    mov rax, QWORD [rbp - 8]
    movq xmm7, rax
    cvtsd2ss xmm7, xmm7
    movd eax, xmm7
    movd xmm0, eax
    mov rdx, QWORD [rbp - 8]
    movq xmm7, rdx
    cvtsd2ss xmm7, xmm7
    movd edx, xmm7
    movd xmm3, edx
    addss xmm0, xmm3
    movq xmm3, xmm0
    mov rax, QWORD [rbp - 8]
    movq xmm7, rax
    cvtsd2ss xmm7, xmm7
    movd eax, xmm7
    movd xmm0, eax
    addss xmm0, xmm3
    movq xmm2, xmm0
    movq rdi, xmm2
    push rdi
    xor rdi, rdi
    ; ((f32)b Add ((f32)d Add (f32)b))
    ; ((f32)d Add (f32)b)
    mov rax, QWORD [rbp - 16]
    movq xmm7, rax
    cvtsd2ss xmm7, xmm7
    movd eax, xmm7
    movd xmm0, eax
    mov rdx, QWORD [rbp - 8]
    movq xmm7, rdx
    cvtsd2ss xmm7, xmm7
    movd edx, xmm7
    movd xmm3, edx
    addss xmm0, xmm3
    movq xmm3, xmm0
    mov rax, QWORD [rbp - 8]
    movq xmm7, rax
    cvtsd2ss xmm7, xmm7
    movd eax, xmm7
    movd xmm0, eax
    addss xmm0, xmm3
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    movd xmm2, edi
    pop rax
    movd xmm0, eax
    addss xmm0, xmm2
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    movd xmm2, edi
    pop rax
    movd xmm0, eax
    addss xmm0, xmm2
    movd eax, xmm0
    mov DWORD [rbp - 20], eax
    mov rcx, .label0 ; Parameter ("%f")
    mov eax, DWORD [rbp - 20]
    movd xmm7, eax
    cvtss2sd xmm7, xmm7
    movq rax, xmm7
    movq xmm1, rax ; Parameter ((f64)addition)
    mov rdx, rax ; Parameter ((f64)addition)
    ; printf("%f", (f64)addition)
    call printf
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}
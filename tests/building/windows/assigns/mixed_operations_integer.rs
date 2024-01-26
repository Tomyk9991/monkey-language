use monkey_language::core::code_generator::generator::ASMGenerator;
use monkey_language::core::code_generator::target_os::TargetOS;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::type_checker::static_type_checker::static_type_check;

#[test]
fn mixed_operations_mul() -> anyhow::Result<()> {
    let code = r#"extern fn printf(format: *string, value: void): void;
let a: i32 = 5 + 1 * 100;
printf("%d", a);
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
    db "%d", 0

main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 36
    ; let a: i32 = (5 Add (1 Mul 100))
    ; (5 Add (1 Mul 100))
    ; (1 Mul 100)
    mov eax, 1
    imul eax, 100
    mov edx, eax
    mov eax, 5
    add eax, edx
    mov DWORD [rbp - 4], eax
    mov rcx, .label0 ; Parameter ("%d")
    mov rdx, QWORD [rbp - 4] ; Parameter (a)
    ; printf("%d", a)
    call printf
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn mixed_operations_div() -> anyhow::Result<()> {
    let code = r#"
extern fn printf(format: *string, value: void): void;
let a: i32 = 5 * 1 / 100;
printf("%d", a);
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
    db "%d", 0

main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 36
    ; let a: i32 = ((5 Mul 1) Div 100)
    ; ((5 Mul 1) Div 100)
    ; (5 Mul 1)
    mov eax, 5
    imul eax, 1
    mov r14d, edx
    mov r13d, eax
    mov r12d, ecx
    mov ecx, 100
    mov edx, 0
    idiv ecx
    mov edx, r14d
    mov ecx, r12d
    mov DWORD [rbp - 4], eax
    mov rcx, .label0 ; Parameter ("%d")
    mov rdx, QWORD [rbp - 4] ; Parameter (a)
    ; printf("%d", a)
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
extern fn printf(format: *string, value: void): void;
let a: i32 = 5 * 1 - 100;
printf("%d", a);
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
    db "%d", 0

main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 36
    ; let a: i32 = ((5 Mul 1) Sub 100)
    ; ((5 Mul 1) Sub 100)
    ; (5 Mul 1)
    mov eax, 5
    imul eax, 1
    sub eax, 100
    mov DWORD [rbp - 4], eax
    mov rcx, .label0 ; Parameter ("%d")
    mov rdx, QWORD [rbp - 4] ; Parameter (a)
    ; printf("%d", a)
    call printf
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn weird_0_test() -> anyhow::Result<()> {
    let code = r#"
extern fn printf(format: *string, value: void): void;
let d = ((6 - 2) * 3 / (7 + 4));
printf("%d", d);
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
    db "%d", 0

main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 36
    ; let d: i32 = (((6 Sub 2) Mul 3) Div (7 Add 4))
    ; (((6 Sub 2) Mul 3) Div (7 Add 4))
    ; ((6 Sub 2) Mul 3)
    ; (6 Sub 2)
    mov eax, 6
    sub eax, 2
    imul eax, 3
    mov edi, eax
    push rdi
    xor rdi, rdi
    ; (7 Add 4)
    mov eax, 7
    add eax, 4
    push rax
    xor rax, rax
    pop rdi
    pop rax
    mov r14d, edx
    mov r13d, eax
    mov r12d, ecx
    mov ecx, edi
    mov edx, 0
    idiv ecx
    mov edx, r14d
    mov ecx, r12d
    mov DWORD [rbp - 4], eax
    mov rcx, .label0 ; Parameter ("%d")
    mov rdx, QWORD [rbp - 4] ; Parameter (d)
    ; printf("%d", d)
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
extern fn printf(format: *string, value: void): void;
let a: i32 = ((3 + 1) * 4 - (9 / 2)) * ((7 + 3) / 2 - (8 * 3)) + ((6 - 2) * 3 / (7 + 4));
printf("%d", a);
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
    db "%d", 0

main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 36
    ; let a: i32 = (((((3 Add 1) Mul 4) Sub (9 Div 2)) Mul (((7 Add 3) Div 2) Sub (8 Mul 3))) Add (((6 Sub 2) Mul 3) Div (7 Add 4)))
    ; (((((3 Add 1) Mul 4) Sub (9 Div 2)) Mul (((7 Add 3) Div 2) Sub (8 Mul 3))) Add (((6 Sub 2) Mul 3) Div (7 Add 4)))
    ; ((((3 Add 1) Mul 4) Sub (9 Div 2)) Mul (((7 Add 3) Div 2) Sub (8 Mul 3)))
    ; (((3 Add 1) Mul 4) Sub (9 Div 2))
    ; ((3 Add 1) Mul 4)
    ; (3 Add 1)
    mov eax, 3
    add eax, 1
    imul eax, 4
    mov edi, eax
    push rdi
    xor rdi, rdi
    ; (9 Div 2)
    mov eax, 9
    mov r14d, edx
    mov r13d, eax
    mov r12d, ecx
    mov ecx, 2
    mov edx, 0
    idiv ecx
    mov edx, r14d
    mov ecx, r12d
    push rax
    xor rax, rax
    pop rdi
    pop rax
    sub eax, edi
    push rax
    xor rax, rax
    ; (((7 Add 3) Div 2) Sub (8 Mul 3))
    ; ((7 Add 3) Div 2)
    ; (7 Add 3)
    mov eax, 7
    add eax, 3
    mov r14d, edx
    mov r13d, eax
    mov r12d, ecx
    mov ecx, 2
    mov edx, 0
    idiv ecx
    mov edx, r14d
    mov ecx, r12d
    mov edi, eax
    push rdi
    xor rdi, rdi
    ; (8 Mul 3)
    mov eax, 8
    imul eax, 3
    push rax
    xor rax, rax
    pop rdi
    pop rax
    sub eax, edi
    push rax
    xor rax, rax
    pop rdi
    pop rax
    imul eax, edi
    push rax
    xor rax, rax
    ; (((6 Sub 2) Mul 3) Div (7 Add 4))
    ; ((6 Sub 2) Mul 3)
    ; (6 Sub 2)
    mov eax, 6
    sub eax, 2
    imul eax, 3
    mov edi, eax
    push rdi
    xor rdi, rdi
    ; (7 Add 4)
    mov eax, 7
    add eax, 4
    push rax
    xor rax, rax
    pop rdi
    pop rax
    mov r14d, edx
    mov r13d, eax
    mov r12d, ecx
    mov ecx, edi
    mov edx, 0
    idiv ecx
    mov edx, r14d
    mov ecx, r12d
    push rax
    xor rax, rax
    pop rdi
    pop rax
    add eax, edi
    mov DWORD [rbp - 4], eax
    mov rcx, .label0 ; Parameter ("%d")
    mov rdx, QWORD [rbp - 4] ; Parameter (a)
    ; printf("%d", a)
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
extern fn printf(format: *string, value: void): void;
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
    ; let a: f64 = (((((3.5 Add 1.2) Mul 4.8) Sub (9.6 Div 2.4)) Mul (((7.2 Add 3.6) Div 2.1) Sub (8.4 Mul 3.7))) Add (((6.3 Sub 2.1) Mul 3.8) Div (7.9 Add 4.2)))
    ; (((((3.5 Add 1.2) Mul 4.8) Sub (9.6 Div 2.4)) Mul (((7.2 Add 3.6) Div 2.1) Sub (8.4 Mul 3.7))) Add (((6.3 Sub 2.1) Mul 3.8) Div (7.9 Add 4.2)))
    ; ((((3.5 Add 1.2) Mul 4.8) Sub (9.6 Div 2.4)) Mul (((7.2 Add 3.6) Div 2.1) Sub (8.4 Mul 3.7)))
    ; (((3.5 Add 1.2) Mul 4.8) Sub (9.6 Div 2.4))
    ; ((3.5 Add 1.2) Mul 4.8)
    ; (3.5 Add 1.2)
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
    ; (9.6 Div 2.4)
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
    ; (((7.2 Add 3.6) Div 2.1) Sub (8.4 Mul 3.7))
    ; ((7.2 Add 3.6) Div 2.1)
    ; (7.2 Add 3.6)
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
    ; (8.4 Mul 3.7)
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
    ; (((6.3 Sub 2.1) Mul 3.8) Div (7.9 Add 4.2))
    ; ((6.3 Sub 2.1) Mul 3.8)
    ; (6.3 Sub 2.1)
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
    ; (7.9 Add 4.2)
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
extern fn printf(format: *string, value: void): void;
let a: i32 = 5 * 1 / 0;
printf("%d", a);
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
    db "%d", 0

main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 36
    ; let a: i32 = ((5 Mul 1) Div 0)
    ; ((5 Mul 1) Div 0)
    ; (5 Mul 1)
    mov eax, 5
    imul eax, 1
    mov r14d, edx
    mov r13d, eax
    mov r12d, ecx
    mov ecx, 0
    mov edx, 0
    idiv ecx
    mov edx, r14d
    mov ecx, r12d
    mov DWORD [rbp - 4], eax
    mov rcx, .label0 ; Parameter ("%d")
    mov rdx, QWORD [rbp - 4] ; Parameter (a)
    ; printf("%d", a)
    call printf
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}
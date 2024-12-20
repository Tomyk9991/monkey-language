use monkey_language::core::code_generator::generator::ASMGenerator;
use monkey_language::core::code_generator::target_os::TargetOS;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::type_checker::static_type_checker::static_type_check;

#[test]
fn mixed_operations_mul() -> anyhow::Result<()> {
    let code = r#"
let a: i32 = 5 + 1 * 100;
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
    ; let a: i32 = (5 + (1 * 100))
    ; (5 + (1 * 100))
    ; (1 * 100)
    mov eax, 1
    imul eax, 100
    mov edx, eax
    mov eax, 5
    add eax, edx
    mov DWORD [rbp - 4], eax
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn mixed_operations_div() -> anyhow::Result<()> {
    let code = r#"
let a: i32 = 5 * 1 / 100;
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
    ; let a: i32 = ((5 * 1) / 100)
    ; ((5 * 1) / 100)
    ; (5 * 1)
    mov eax, 5
    imul eax, 1
    mov r14d, edx
    mov r13d, eax
    mov r12d, ecx
    mov ecx, eax
    mov eax, 100
    mov edx, 0
    idiv ecx
    mov edx, r14d
    mov ecx, r12d
    mov DWORD [rbp - 4], eax
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
let a: i32 = 5 * 1 - 100;
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
    ; let a: i32 = ((5 * 1) - 100)
    ; ((5 * 1) - 100)
    ; (5 * 1)
    mov eax, 5
    imul eax, 1
    sub eax, 100
    mov DWORD [rbp - 4], eax
    ; return 0
    mov eax, 0
    leave
    ret"#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn weird_0_test() -> anyhow::Result<()> {
    let code = r#"
let d = ((6 - 2) * 3 / (7 + 4));
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
    ; let d: i32 = (((6 - 2) * 3) / (7 + 4))
    ; (((6 - 2) * 3) / (7 + 4))
    ; ((6 - 2) * 3)
    ; (6 - 2)
    mov eax, 6
    sub eax, 2
    imul eax, 3
    mov edi, eax
    push rdi
    xor rdi, rdi
    ; (7 + 4)
    mov eax, 7
    add eax, 4
    push rax
    xor rax, rax
    pop rdi
    pop rax
    mov r14d, edx
    mov r13d, eax
    mov r12d, ecx
    mov ecx, eax
    mov eax, edi
    mov edx, 0
    idiv ecx
    mov edx, r14d
    mov ecx, r12d
    mov DWORD [rbp - 4], eax
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
let a: i32 = ((3 + 1) * 4 - (9 / 2)) * ((7 + 3) / 2 - (8 * 3)) + ((6 - 2) * 3 / (7 + 4));
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
    ; let a: i32 = (((((3 + 1) * 4) - (9 / 2)) * (((7 + 3) / 2) - (8 * 3))) + (((6 - 2) * 3) / (7 + 4)))
    ; (((((3 + 1) * 4) - (9 / 2)) * (((7 + 3) / 2) - (8 * 3))) + (((6 - 2) * 3) / (7 + 4)))
    ; ((((3 + 1) * 4) - (9 / 2)) * (((7 + 3) / 2) - (8 * 3)))
    ; (((3 + 1) * 4) - (9 / 2))
    ; ((3 + 1) * 4)
    ; (3 + 1)
    mov eax, 3
    add eax, 1
    imul eax, 4
    mov edi, eax
    push rdi
    xor rdi, rdi
    ; (9 / 2)
    mov eax, 9
    mov r14d, edx
    mov r13d, eax
    mov r12d, ecx
    mov ecx, eax
    mov eax, 2
    mov edx, 0
    idiv ecx
    mov edx, r14d
    mov ecx, r12d
    push rax
    xor rax, rax
    pop rdi
    pop rax
    sub eax, edi
    mov edi, eax
    push rdi
    xor rdi, rdi
    ; (((7 + 3) / 2) - (8 * 3))
    ; ((7 + 3) / 2)
    ; (7 + 3)
    mov eax, 7
    add eax, 3
    mov r14d, edx
    mov r13d, eax
    mov r12d, ecx
    mov ecx, eax
    mov eax, 2
    mov edx, 0
    idiv ecx
    mov edx, r14d
    mov ecx, r12d
    mov edi, eax
    push rdi
    xor rdi, rdi
    ; (8 * 3)
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
    mov edi, eax
    push rdi
    xor rdi, rdi
    ; (((6 - 2) * 3) / (7 + 4))
    ; ((6 - 2) * 3)
    ; (6 - 2)
    mov eax, 6
    sub eax, 2
    imul eax, 3
    mov edi, eax
    push rdi
    xor rdi, rdi
    ; (7 + 4)
    mov eax, 7
    add eax, 4
    push rax
    xor rax, rax
    pop rdi
    pop rax
    mov r14d, edx
    mov r13d, eax
    mov r12d, ecx
    mov ecx, eax
    mov eax, edi
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
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn mixed_operations_f64() -> anyhow::Result<()> {
    let code = r#"
let a: f64 = ((3.5_f64 + 1.2_f64) * 4.8_f64 - (9.6_f64 / 2.4_f64)) * ((7.2_f64 + 3.6_f64) / 2.1_f64 - (8.4_f64 * 3.7_f64)) + ((6.3_f64 - 2.1_f64) * 3.8_f64 / (7.9_f64 + 4.2_f64));
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
    ; let a: f64 = (((((3.5 + 1.2) * 4.8) - (9.6 / 2.4)) * (((7.2 + 3.6) / 2.1) - (8.4 * 3.7))) + (((6.3 - 2.1) * 3.8) / (7.9 + 4.2)))
    ; (((((3.5 + 1.2) * 4.8) - (9.6 / 2.4)) * (((7.2 + 3.6) / 2.1) - (8.4 * 3.7))) + (((6.3 - 2.1) * 3.8) / (7.9 + 4.2)))
    ; ((((3.5 + 1.2) * 4.8) - (9.6 / 2.4)) * (((7.2 + 3.6) / 2.1) - (8.4 * 3.7)))
    ; (((3.5 + 1.2) * 4.8) - (9.6 / 2.4))
    ; ((3.5 + 1.2) * 4.8)
    ; (3.5 + 1.2)
    mov rax, __?float64?__(3.5)
    movq xmm0, rax
    mov rdx, __?float64?__(1.2)
    movq xmm3, rdx
    addsd xmm0, xmm3
    mov rdx, __?float64?__(4.8)
    movq xmm3, rdx
    mulsd xmm0, xmm3
    movq xmm2, xmm0
    movq rdi, xmm2
    push rdi
    xor rdi, rdi
    ; (9.6 / 2.4)
    mov rax, __?float64?__(9.6)
    movq xmm0, rax
    mov rdx, __?float64?__(2.4)
    movq xmm3, rdx
    divsd xmm0, xmm3
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    pop rax
    movq xmm0, rax
    movq xmm2, rdi
    subsd xmm0, xmm2
    movq xmm2, xmm0
    movq rdi, xmm2
    push rdi
    xor rdi, rdi
    ; (((7.2 + 3.6) / 2.1) - (8.4 * 3.7))
    ; ((7.2 + 3.6) / 2.1)
    ; (7.2 + 3.6)
    mov rax, __?float64?__(7.2)
    movq xmm0, rax
    mov rdx, __?float64?__(3.6)
    movq xmm3, rdx
    addsd xmm0, xmm3
    mov rdx, __?float64?__(2.1)
    movq xmm3, rdx
    divsd xmm0, xmm3
    movq xmm2, xmm0
    movq rdi, xmm2
    push rdi
    xor rdi, rdi
    ; (8.4 * 3.7)
    mov rax, __?float64?__(8.4)
    movq xmm0, rax
    mov rdx, __?float64?__(3.7)
    movq xmm3, rdx
    mulsd xmm0, xmm3
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    pop rax
    movq xmm0, rax
    movq xmm2, rdi
    subsd xmm0, xmm2
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    pop rax
    movq xmm0, rax
    movq xmm2, rdi
    mulsd xmm0, xmm2
    movq xmm2, xmm0
    movq rdi, xmm2
    push rdi
    xor rdi, rdi
    ; (((6.3 - 2.1) * 3.8) / (7.9 + 4.2))
    ; ((6.3 - 2.1) * 3.8)
    ; (6.3 - 2.1)
    mov rax, __?float64?__(6.3)
    movq xmm0, rax
    mov rdx, __?float64?__(2.1)
    movq xmm3, rdx
    subsd xmm0, xmm3
    mov rdx, __?float64?__(3.8)
    movq xmm3, rdx
    mulsd xmm0, xmm3
    movq xmm2, xmm0
    movq rdi, xmm2
    push rdi
    xor rdi, rdi
    ; (7.9 + 4.2)
    mov rax, __?float64?__(7.9)
    movq xmm0, rax
    mov rdx, __?float64?__(4.2)
    movq xmm3, rdx
    addsd xmm0, xmm3
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    pop rax
    movq xmm0, rax
    movq xmm2, rdi
    divsd xmm0, xmm2
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    pop rax
    movq xmm0, rax
    movq xmm2, rdi
    addsd xmm0, xmm2
    movq rax, xmm0
    mov QWORD [rbp - 8], rax
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn mixed_operations_div_0() -> anyhow::Result<()> {
    let code = r#"
let a: i32 = 5 * 1 / 0;
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
    ; let a: i32 = ((5 * 1) / 0)
    ; ((5 * 1) / 0)
    ; (5 * 1)
    mov eax, 5
    imul eax, 1
    mov r14d, edx
    mov r13d, eax
    mov r12d, ecx
    mov ecx, eax
    mov eax, 0
    mov edx, 0
    idiv ecx
    mov edx, r14d
    mov ecx, r12d
    mov DWORD [rbp - 4], eax
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn mixed_operations_bool() -> anyhow::Result<()> {
    let code = r#"
let a: i32 = ((i32)(5 << 4 < 7) * 8 - 9 % 3) + 3;
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
    ; let a: i32 = ((((i32)((5 << 4) < 7) * 8) - (9 % 3)) + 3)
    ; ((((i32)((5 << 4) < 7) * 8) - (9 % 3)) + 3)
    ; (((i32)((5 << 4) < 7) * 8) - (9 % 3))
    ; ((i32)((5 << 4) < 7) * 8)
    ; ((5 << 4) < 7)
    ; (5 << 4)
    mov eax, 5
    mov r14d, edx
    mov r13d, eax
    mov r12d, ecx
    mov ecx, eax
    mov eax, 4
    mov edx, 0
    shl eax, cl
    mov edx, r14d
    mov ecx, r12d
    cmp eax, 7
    setl al
    ; Cast: (bool) -> (i32)
    ; Cast: (u8) -> (i32)
    movzx eax, al
    imul eax, 8
    mov ecx, eax
    ; (9 % 3)
    mov eax, 9
    mov r14d, edx
    mov r13d, eax
    mov r12d, ecx
    mov ecx, eax
    mov eax, 3
    mov edx, 0
    idiv ecx
    mov eax, edx
    mov edx, r14d
    mov ecx, r12d
    mov edi, eax
    sub ecx, edi
    add ecx, 3
    mov DWORD [rbp - 4], ecx
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}
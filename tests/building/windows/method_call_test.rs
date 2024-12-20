use monkey_language::core::code_generator::generator::ASMGenerator;
use monkey_language::core::code_generator::target_os::TargetOS;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::type_checker::static_type_checker::static_type_check;

#[test]
fn method_build_and_call() -> anyhow::Result<()> {
    let code = r#"
    fn add(a: i32, b: i32): i32 {
        return a + b;
    }

    add(5, 3);
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


.add_i32_i32~i32:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    mov DWORD [rbp - 4], ecx
    mov DWORD [rbp - 8], edx
    ; return (a + b)
    ; (a + b)
    mov eax, DWORD [rbp - 4]
    add eax, DWORD [rbp - 8]
    leave
    ret
main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    mov ecx, 5
    mov edx, 3
    ; add(5, 3)
    call .add_i32_i32~i32
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
fn method_call_expression_assign() -> anyhow::Result<()> {
    let code = r#"
    fn add(a: i32, b: i32): i32 {
        return a + b;
    }

    add(5 + 3 * 8 - 9, 100 * 8 - 9 * 4);
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


.add_i32_i32~i32:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    mov DWORD [rbp - 4], ecx
    mov DWORD [rbp - 8], edx
    ; return (a + b)
    ; (a + b)
    mov eax, DWORD [rbp - 4]
    add eax, DWORD [rbp - 8]
    leave
    ret
main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; ((100 * 8) - (9 * 4))
    ; (100 * 8)
    mov eax, 100
    imul eax, 8
    mov ecx, eax
    ; (9 * 4)
    mov eax, 9
    imul eax, 4
    mov edi, eax
    sub ecx, edi
    push rcx
    ; ((5 + (3 * 8)) - 9)
    ; (5 + (3 * 8))
    ; (3 * 8)
    mov eax, 3
    imul eax, 8
    mov edx, eax
    mov eax, 5
    add eax, edx
    sub eax, 9
    push rax
    pop rcx
    pop rdx
    ; add(((5 + (3 * 8)) - 9), ((100 * 8) - (9 * 4)))
    call .add_i32_i32~i32
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
fn method_call_expression_float() -> anyhow::Result<()> {
    let code = r#"
    fn add(a: f32, b: f32): f32 {
        return a + b;
    }

    add(5.0 + 3.0 * 8.0 - 9.0, 100.0 * 8.0 - 9.0 * 4.0);
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


.add_f32_f32~f32:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    movd DWORD [rbp - 4], xmm0
    movd DWORD [rbp - 8], xmm1
    ; return (a + b)
    ; (a + b)
    mov eax, DWORD [rbp - 4]
    movd xmm0, eax
    mov edx, DWORD [rbp - 8]
    movd xmm3, edx
    addss xmm0, xmm3
    movd eax, xmm0
    leave
    ret
main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; ((100 * 8) - (9 * 4))
    ; (100 * 8)
    mov eax, __?float32?__(100.0)
    movd xmm0, eax
    mov edx, __?float32?__(8.0)
    movd xmm3, edx
    mulss xmm0, xmm3
    movq xmm1, xmm0
    ; (9 * 4)
    mov eax, __?float32?__(9.0)
    movd xmm0, eax
    mov edx, __?float32?__(4.0)
    movd xmm3, edx
    mulss xmm0, xmm3
    movq xmm2, xmm0
    subss xmm1, xmm2
    movq rcx, xmm1
    push rcx
    ; ((5 + (3 * 8)) - 9)
    ; (5 + (3 * 8))
    ; (3 * 8)
    mov eax, __?float32?__(3.0)
    movd xmm0, eax
    mov edx, __?float32?__(8.0)
    movd xmm3, edx
    mulss xmm0, xmm3
    movq xmm3, xmm0
    mov eax, __?float32?__(5.0)
    movd xmm0, eax
    addss xmm0, xmm3
    mov edx, __?float32?__(9.0)
    movd xmm3, edx
    subss xmm0, xmm3
    movq rax, xmm0
    push rax
    pop rax
    movq xmm0, rax
    pop rcx
    movq xmm1, rcx
    ; add(((5 + (3 * 8)) - 9), ((100 * 8) - (9 * 4)))
    call .add_f32_f32~f32
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
fn method_overload_call_expression() -> anyhow::Result<()> {
    let code = r#"
    fn add(a: f32, b: f32): f32 {
        return a + b;
    }

    fn add(a: i32, b: i32): i32 {
        return a + b;
    }

    add(5 + 3 * 8 - 9, 100 * 8 - 9 * 4);
    add(5.0 + 3.0 * 8.0 - 9.0, 100.0 * 8.0 - 9.0 * 4.0);
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


.add_f32_f32~f32:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    movd DWORD [rbp - 4], xmm0
    movd DWORD [rbp - 8], xmm1
    ; return (a + b)
    ; (a + b)
    mov eax, DWORD [rbp - 4]
    movd xmm0, eax
    mov edx, DWORD [rbp - 8]
    movd xmm3, edx
    addss xmm0, xmm3
    movd eax, xmm0
    leave
    ret
.add_i32_i32~i32:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    mov DWORD [rbp - 4], ecx
    mov DWORD [rbp - 8], edx
    ; return (a + b)
    ; (a + b)
    mov eax, DWORD [rbp - 4]
    add eax, DWORD [rbp - 8]
    leave
    ret
main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; ((100 * 8) - (9 * 4))
    ; (100 * 8)
    mov eax, 100
    imul eax, 8
    mov ecx, eax
    ; (9 * 4)
    mov eax, 9
    imul eax, 4
    mov edi, eax
    sub ecx, edi
    push rcx
    ; ((5 + (3 * 8)) - 9)
    ; (5 + (3 * 8))
    ; (3 * 8)
    mov eax, 3
    imul eax, 8
    mov edx, eax
    mov eax, 5
    add eax, edx
    sub eax, 9
    push rax
    pop rcx
    pop rdx
    ; add(((5 + (3 * 8)) - 9), ((100 * 8) - (9 * 4)))
    call .add_i32_i32~i32
    ; ((100 * 8) - (9 * 4))
    ; (100 * 8)
    mov eax, __?float32?__(100.0)
    movd xmm0, eax
    mov edx, __?float32?__(8.0)
    movd xmm3, edx
    mulss xmm0, xmm3
    movq xmm1, xmm0
    ; (9 * 4)
    mov eax, __?float32?__(9.0)
    movd xmm0, eax
    mov edx, __?float32?__(4.0)
    movd xmm3, edx
    mulss xmm0, xmm3
    movq xmm2, xmm0
    subss xmm1, xmm2
    movq rcx, xmm1
    push rcx
    ; ((5 + (3 * 8)) - 9)
    ; (5 + (3 * 8))
    ; (3 * 8)
    mov eax, __?float32?__(3.0)
    movd xmm0, eax
    mov edx, __?float32?__(8.0)
    movd xmm3, edx
    mulss xmm0, xmm3
    movq xmm3, xmm0
    mov eax, __?float32?__(5.0)
    movd xmm0, eax
    addss xmm0, xmm3
    mov edx, __?float32?__(9.0)
    movd xmm3, edx
    subss xmm0, xmm3
    movq rax, xmm0
    push rax
    pop rax
    movq xmm0, rax
    pop rcx
    movq xmm1, rcx
    ; add(((5 + (3 * 8)) - 9), ((100 * 8) - (9 * 4)))
    call .add_f32_f32~f32
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
fn multiple_function_calls() -> anyhow::Result<()> {
    let code = r#"
    fn inc(a: i32): i32 {
        return a + 1;
    }

    inc(inc(1));
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
    mov ecx, 1
    ; inc(1)
    call .inc_i32~i32
    push rax
    pop rcx
    ; inc(inc(1))
    call .inc_i32~i32
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}
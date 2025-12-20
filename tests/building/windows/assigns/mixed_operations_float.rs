use monkey_language::core::code_generator::generator::ASMGenerator;
use monkey_language::core::code_generator::target_os::TargetOS;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::parser::parser::ASTParser;
use monkey_language::core::semantics::static_type_check::static_type_checker::static_type_check;
use monkey_language::core::semantics::type_infer::type_inferer::infer_type;

#[test]
fn mixed_operations_mul() -> anyhow::Result<()> {
    let code = r#"
let a: f32 = 5.0 + 1.0 * 100.0;
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;
    let _ = static_type_check(&mut top_level_scope.result.program)?;

    let mut code_generator = ASMGenerator::from((top_level_scope.result.program, TargetOS::Windows));
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
    ; let a: f32 = (5 + (1 * 100))
    ; (5 + (1 * 100))
    ; (1 * 100)
    mov eax, __?float32?__(1.0)
    movd xmm0, eax
    mov edx, __?float32?__(100.0)
    movd xmm3, edx
    mulss xmm0, xmm3
    movq xmm3, xmm0
    mov eax, __?float32?__(5.0)
    movd xmm0, eax
    addss xmm0, xmm3
    movd eax, xmm0
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
let a: f32 = 5.0 * 1.0 - 100.0;
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;
    let _ = static_type_check(&mut top_level_scope.result.program)?;

    let mut code_generator = ASMGenerator::from((top_level_scope.result.program, TargetOS::Windows));
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
    ; let a: f32 = ((5 * 1) - 100)
    ; ((5 * 1) - 100)
    ; (5 * 1)
    mov eax, __?float32?__(5.0)
    movd xmm0, eax
    mov edx, __?float32?__(1.0)
    movd xmm3, edx
    mulss xmm0, xmm3
    mov edx, __?float32?__(100.0)
    movd xmm3, edx
    subss xmm0, xmm3
    movd eax, xmm0
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
let a: f32 = ((3.5 + 1.2) * 4.8 - (9.6 / 2.4)) * ((7.2 + 3.6) / 2.1 - (8.4 * 3.7)) + ((6.3 - 2.1) * 3.8 / (7.9 + 4.2));
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;
    let _ = static_type_check(&mut top_level_scope.result.program)?;

    let mut code_generator = ASMGenerator::from((top_level_scope.result.program, TargetOS::Windows));
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
    ; let a: f32 = (((((3.5 + 1.2) * 4.8) - (9.6 / 2.4)) * (((7.2 + 3.6) / 2.1) - (8.4 * 3.7))) + (((6.3 - 2.1) * 3.8) / (7.9 + 4.2)))
    ; (((((3.5 + 1.2) * 4.8) - (9.6 / 2.4)) * (((7.2 + 3.6) / 2.1) - (8.4 * 3.7))) + (((6.3 - 2.1) * 3.8) / (7.9 + 4.2)))
    ; ((((3.5 + 1.2) * 4.8) - (9.6 / 2.4)) * (((7.2 + 3.6) / 2.1) - (8.4 * 3.7)))
    ; (((3.5 + 1.2) * 4.8) - (9.6 / 2.4))
    ; ((3.5 + 1.2) * 4.8)
    ; (3.5 + 1.2)
    mov eax, __?float32?__(3.5)
    movd xmm0, eax
    mov edx, __?float32?__(1.2)
    movd xmm3, edx
    addss xmm0, xmm3
    mov edx, __?float32?__(4.8)
    movd xmm3, edx
    mulss xmm0, xmm3
    movq xmm2, xmm0
    movq rdi, xmm2
    push rdi
    xor rdi, rdi
    ; (9.6 / 2.4)
    mov eax, __?float32?__(9.6)
    movd xmm0, eax
    mov edx, __?float32?__(2.4)
    movd xmm3, edx
    divss xmm0, xmm3
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    pop rax
    movd xmm0, eax
    movd xmm2, edi
    subss xmm0, xmm2
    movq xmm2, xmm0
    movq rdi, xmm2
    push rdi
    xor rdi, rdi
    ; (((7.2 + 3.6) / 2.1) - (8.4 * 3.7))
    ; ((7.2 + 3.6) / 2.1)
    ; (7.2 + 3.6)
    mov eax, __?float32?__(7.2)
    movd xmm0, eax
    mov edx, __?float32?__(3.6)
    movd xmm3, edx
    addss xmm0, xmm3
    mov edx, __?float32?__(2.1)
    movd xmm3, edx
    divss xmm0, xmm3
    movq xmm2, xmm0
    movq rdi, xmm2
    push rdi
    xor rdi, rdi
    ; (8.4 * 3.7)
    mov eax, __?float32?__(8.4)
    movd xmm0, eax
    mov edx, __?float32?__(3.7)
    movd xmm3, edx
    mulss xmm0, xmm3
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    pop rax
    movd xmm0, eax
    movd xmm2, edi
    subss xmm0, xmm2
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    pop rax
    movd xmm0, eax
    movd xmm2, edi
    mulss xmm0, xmm2
    movq xmm2, xmm0
    movq rdi, xmm2
    push rdi
    xor rdi, rdi
    ; (((6.3 - 2.1) * 3.8) / (7.9 + 4.2))
    ; ((6.3 - 2.1) * 3.8)
    ; (6.3 - 2.1)
    mov eax, __?float32?__(6.3)
    movd xmm0, eax
    mov edx, __?float32?__(2.1)
    movd xmm3, edx
    subss xmm0, xmm3
    mov edx, __?float32?__(3.8)
    movd xmm3, edx
    mulss xmm0, xmm3
    movq xmm2, xmm0
    movq rdi, xmm2
    push rdi
    xor rdi, rdi
    ; (7.9 + 4.2)
    mov eax, __?float32?__(7.9)
    movd xmm0, eax
    mov edx, __?float32?__(4.2)
    movd xmm3, edx
    addss xmm0, xmm3
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    pop rax
    movd xmm0, eax
    movd xmm2, edi
    divss xmm0, xmm2
    movq rax, xmm0
    push rax
    xor rax, rax
    pop rdi
    pop rax
    movd xmm0, eax
    movd xmm2, edi
    addss xmm0, xmm2
    movd eax, xmm0
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

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;
    let _ = static_type_check(&mut top_level_scope.result.program)?;

    let mut code_generator = ASMGenerator::from((top_level_scope.result.program, TargetOS::Windows));
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
let a: f32 = 5.0 * 1.0 / 0.0;
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;
    let _ = static_type_check(&mut top_level_scope.result.program)?;

    let mut code_generator = ASMGenerator::from((top_level_scope.result.program, TargetOS::Windows));
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
    ; let a: f32 = ((5 * 1) / 0)
    ; ((5 * 1) / 0)
    ; (5 * 1)
    mov eax, __?float32?__(5.0)
    movd xmm0, eax
    mov edx, __?float32?__(1.0)
    movd xmm3, edx
    mulss xmm0, xmm3
    mov edx, __?float32?__(0.0)
    movd xmm3, edx
    divss xmm0, xmm3
    movd eax, xmm0
    mov DWORD [rbp - 4], eax
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}
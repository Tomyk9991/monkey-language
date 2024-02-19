use monkey_language::core::code_generator::generator::{ASMGenerator};
use monkey_language::core::code_generator::target_os::TargetOS;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::type_checker::static_type_checker::static_type_check;

#[test]
fn expression_assign() -> anyhow::Result<()> {
    let code = r#"
    fn float_return(): f32 { return 5.0; }
    fn float_f64_return(): f64 { return 5.0_f64; }
    fn integer_return(): i32 { return 23; }

    let a: f32 = float_return();
    let b: f64 = float_f64_return();
    let c: i32 = integer_return() + (i32) b;
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


.float_return_void~f32:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; return 5
    mov eax, __?float32?__(5.0)
    leave
    ret
.float_f64_return_void~f64:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; return 5
    mov rax, __?float64?__(5.0)
    leave
    ret
.integer_return_void~i32:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; return 23
    mov eax, 23
    leave
    ret
main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; let a: f32 = float_return()
    ; float_return()
    call .float_return_void~f32
    mov DWORD [rbp - 4], eax
    ; let b: f64 = float_f64_return()
    ; float_f64_return()
    call .float_f64_return_void~f64
    mov QWORD [rbp - 12], rax
    ; let c: i32 = (integer_return() + (i32)b)
    ; (integer_return() + (i32)b)
    ; PushQ
    push rcx
    push rdi
    push rdx
    ; integer_return()
    call .integer_return_void~i32
    ; PopQ
    pop rdx
    pop rdi
    pop rcx
    ; Cast: (f64) -> (i32)
    mov rdx, QWORD [rbp - 12]
    movq xmm7, rdx
    cvtsd2si rdx, xmm7
    ; Cast: (i64) -> (i32)
    add eax, edx
    mov DWORD [rbp - 16], eax
    leave
    ret
    "#;


    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    fn int_5(): i32 { return 5; }
    fn int_3(): i32 { return 3; }
    fn float_5(): f32 { return 5.0; }
    fn float_3(): f32 { return 3.0; }
    fn float_f64_5(): f64 { return 5.0_f64; }
    fn float_f64_3(): f64 { return 3.0_f64; }

    let c: i32 = int_5() + int_3();
    let a: f32 = float_5() + float_3();
    let b: f64 = float_f64_5() + float_f64_3();
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


.int_5_void~i32:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; return 5
    mov eax, 5
    leave
    ret
.int_3_void~i32:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; return 3
    mov eax, 3
    leave
    ret
.float_5_void~f32:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; return 5
    mov eax, __?float32?__(5.0)
    leave
    ret
.float_3_void~f32:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; return 3
    mov eax, __?float32?__(3.0)
    leave
    ret
.float_f64_5_void~f64:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; return 5
    mov rax, __?float64?__(5.0)
    leave
    ret
.float_f64_3_void~f64:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; return 3
    mov rax, __?float64?__(3.0)
    leave
    ret
main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; let c: i32 = (int_5() + int_3())
    ; (int_5() + int_3())
    ; PushQ
    push rcx
    push rdi
    push rdx
    ; int_5()
    call .int_5_void~i32
    ; PopQ
    pop rdx
    pop rdi
    pop rcx
    ; PushQ
    push rax
    push rcx
    push rdi
    ; int_3()
    call .int_3_void~i32
    mov edx, eax
    ; PopQ
    pop rdi
    pop rcx
    pop rax
    add eax, edx
    mov DWORD [rbp - 4], eax
    ; let a: f32 = (float_5() + float_3())
    ; (float_5() + float_3())
    ; PushQ
    push rcx
    push rdi
    push rdx
    ; float_5()
    call .float_5_void~f32
    ; PopQ
    pop rdx
    pop rdi
    pop rcx
    movd xmm0, eax
    ; PushQ
    push rax
    push rcx
    push rdi
    ; float_3()
    call .float_3_void~f32
    mov edx, eax
    ; PopQ
    pop rdi
    pop rcx
    pop rax
    movd xmm3, edx
    addss xmm0, xmm3
    movd eax, xmm0
    mov DWORD [rbp - 8], eax
    ; let b: f64 = (float_f64_5() + float_f64_3())
    ; (float_f64_5() + float_f64_3())
    ; PushQ
    push rcx
    push rdi
    push rdx
    ; float_f64_5()
    call .float_f64_5_void~f64
    ; PopQ
    pop rdx
    pop rdi
    pop rcx
    movq xmm0, rax
    ; PushQ
    push rax
    push rcx
    push rdi
    ; float_f64_3()
    call .float_f64_3_void~f64
    mov rdx, rax
    ; PopQ
    pop rdi
    pop rcx
    pop rax
    movq xmm3, rdx
    addsd xmm0, xmm3
    movq rax, xmm0
    mov QWORD [rbp - 16], rax
    leave
    ret
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn pointer_assign_test() -> anyhow::Result<()> {
    let code = r#"
    fn f(): f64 { return 5.0_f64; }
    let a: f64 = f();
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


.f_void~f64:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; return 5
    mov rax, __?float64?__(5.0)
    leave
    ret
main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; let a: f64 = f()
    ; f()
    call .f_void~f64
    mov QWORD [rbp - 8], rax
    ; let b: *f64 = &a
    lea rax, [rbp - 8]
    mov QWORD [rbp - 16], rax
    leave
    ret"#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}


#[test]
fn pointer_assign_multiple_test() -> anyhow::Result<()> {
    let code = r#"
fn r(): f64 { return 5.0_f64; }

let a: f64 = r();
let b: *f64 = &a;
let c: **f64 = &b;
let d: *f64 = *c;

let ref: **f64 = c;
let f: f64 = *d;
let g: f64 = **c;
let h: *f64 = &r();
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


.r_void~f64:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; return 5
    mov rax, __?float64?__(5.0)
    leave
    ret
main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 128
    ; let a: f64 = r()
    ; r()
    call .r_void~f64
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
    mov rax, QWORD [rbp - 32]
    mov rax, QWORD [rax]
    mov QWORD [rbp - 48], rax
    ; let g: f64 = **c
    mov rax, QWORD [rbp - 24]
    mov rax, QWORD [rax]
    mov rax, QWORD [rax]
    mov QWORD [rbp - 56], rax
    ; let h: *f64 = &r()
    ; PushQ
    push rcx
    push rdi
    push rdx
    ; r()
    call .r_void~f64
    ; PopQ
    pop rdx
    pop rdi
    pop rcx
    mov QWORD [rbp - 64], rax
    lea rax, [rbp - 64]
    mov QWORD [rbp - 72], rax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn pointer_deref_operation_lhs() -> anyhow::Result<()> {
    let code = r#"
fn f1(): f64 { return 5.0_f64; }
fn f2(): f64 { return 1.0_f64; }
let a: f64 = f1();
let b: *f64 = &a;
let addition = *b + f2();
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


.f1_void~f64:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; return 5
    mov rax, __?float64?__(5.0)
    leave
    ret
.f2_void~f64:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; return 1
    mov rax, __?float64?__(1.0)
    leave
    ret
main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; let a: f64 = f1()
    ; f1()
    call .f1_void~f64
    mov QWORD [rbp - 8], rax
    ; let b: *f64 = &a
    lea rax, [rbp - 8]
    mov QWORD [rbp - 16], rax
    ; let addition: f64 = (*b + f2())
    ; (*b + f2())
    mov rax, QWORD [rbp - 16]
    mov rax, QWORD [rax]
    movq xmm0, rax
    ; PushQ
    push rax
    push rcx
    push rdi
    ; f2()
    call .f2_void~f64
    mov rdx, rax
    ; PopQ
    pop rdi
    pop rcx
    pop rax
    movq xmm3, rdx
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
fn f1(): f64 { return 5.0_f64; }
fn f2(): f64 { return 1.0_f64; }
let a: f64 = f1();
let b: *f64 = &a;
let addition = f2() + *b;
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


.f1_void~f64:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; return 5
    mov rax, __?float64?__(5.0)
    leave
    ret
.f2_void~f64:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; return 1
    mov rax, __?float64?__(1.0)
    leave
    ret
main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; let a: f64 = f1()
    ; f1()
    call .f1_void~f64
    mov QWORD [rbp - 8], rax
    ; let b: *f64 = &a
    lea rax, [rbp - 8]
    mov QWORD [rbp - 16], rax
    ; let addition: f64 = (f2() + *b)
    ; (f2() + *b)
    ; PushQ
    push rcx
    push rdi
    push rdx
    ; f2()
    call .f2_void~f64
    ; PopQ
    pop rdx
    pop rdi
    pop rcx
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
fn operation_lhs_rhs() -> anyhow::Result<()> {
    let code = r#"
fn f1(): f64 { return 5.0_f64; }
let addition = f1() + f1();
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


.f1_void~f64:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; return 5
    mov rax, __?float64?__(5.0)
    leave
    ret
main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; let addition: f64 = (f1() + f1())
    ; (f1() + f1())
    ; PushQ
    push rcx
    push rdi
    push rdx
    ; f1()
    call .f1_void~f64
    ; PopQ
    pop rdx
    pop rdi
    pop rcx
    movq xmm0, rax
    ; PushQ
    push rax
    push rcx
    push rdi
    ; f1()
    call .f1_void~f64
    mov rdx, rax
    ; PopQ
    pop rdi
    pop rcx
    pop rax
    movq xmm3, rdx
    addsd xmm0, xmm3
    movq rax, xmm0
    mov QWORD [rbp - 8], rax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn operation_lhs_expression() -> anyhow::Result<()> {
    let code = r#"
fn f1(): f64 { return 5.0_f64; }
let a: f64 = f1();
let addition = f1() + (a + a);
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


.f1_void~f64:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; return 5
    mov rax, __?float64?__(5.0)
    leave
    ret
main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; let a: f64 = f1()
    ; f1()
    call .f1_void~f64
    mov QWORD [rbp - 8], rax
    ; let addition: f64 = (f1() + (a + a))
    ; (f1() + (a + a))
    ; (a + a)
    mov rax, QWORD [rbp - 8]
    movq xmm0, rax
    mov rdx, QWORD [rbp - 8]
    movq xmm3, rdx
    addsd xmm0, xmm3
    movq xmm3, xmm0
    ; PushQ
    push rcx
    push rdi
    push rdx
    ; f1()
    call .f1_void~f64
    ; PopQ
    pop rdx
    pop rdi
    pop rcx
    addsd xmm0, xmm3
    movq rax, xmm0
    mov QWORD [rbp - 16], rax
    leave
    ret
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn operation_expression_rhs() -> anyhow::Result<()> {
    let code = r#"
fn f1(): f64 { return 5.0_f64; }
let a: f64 = f1();
let addition = (a + a) + f1();
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


.f1_void~f64:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; return 5
    mov rax, __?float64?__(5.0)
    leave
    ret
main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; let a: f64 = f1()
    ; f1()
    call .f1_void~f64
    mov QWORD [rbp - 8], rax
    ; let addition: f64 = ((a + a) + f1())
    ; ((a + a) + f1())
    ; (a + a)
    mov rax, QWORD [rbp - 8]
    movq xmm0, rax
    mov rdx, QWORD [rbp - 8]
    movq xmm3, rdx
    addsd xmm0, xmm3
    ; PushQ
    push rax
    push rcx
    push rdi
    ; f1()
    call .f1_void~f64
    mov rdx, rax
    ; PopQ
    pop rdi
    pop rcx
    pop rax
    movq xmm3, rdx
    addsd xmm0, xmm3
    movq rax, xmm0
    mov QWORD [rbp - 16], rax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn operation_expression_expression() -> anyhow::Result<()> {
    let code = r#"
fn f1(): f64 { return 5.0_f64; }
let a1 = (f1() + f1()) + f1();
let a2 = f1() + (f1() + f1());
let a3 = (f1() + f1()) + (f1() + f1());
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


.f1_void~f64:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; return 5
    mov rax, __?float64?__(5.0)
    leave
    ret
main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; let addition: f64 = ((f1() + f1()) + (f1() + f1()))
    ; ((f1() + f1()) + (f1() + f1()))
    ; (f1() + f1())
    ; PushQ
    push rcx
    push rdi
    push rdx
    ; f1()
    call .f1_void~f64
    ; PopQ
    pop rdx
    pop rdi
    pop rcx
    movq xmm0, rax
    ; PushQ
    push rax
    push rcx
    push rdi
    ; f1()
    call .f1_void~f64
    mov rdx, rax
    ; PopQ
    pop rdi
    pop rcx
    pop rax
    movq xmm3, rdx
    addsd xmm0, xmm3
    movq xmm1, xmm0
    ; (f1() + f1())
    ; PushQ
    push rcx
    push rdi
    push rdx
    ; f1()
    call .f1_void~f64
    ; PopQ
    pop rdx
    pop rdi
    pop rcx
    movq xmm0, rax
    ; PushQ
    push rax
    push rcx
    push rdi
    ; f1()
    call .f1_void~f64
    mov rdx, rax
    ; PopQ
    pop rdi
    pop rcx
    pop rax
    movq xmm3, rdx
    addsd xmm0, xmm3
    movq xmm2, xmm0
    addsd xmm1, xmm2
    movq rax, xmm1
    mov QWORD [rbp - 8], rax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn operation_complex_expression_expression() -> anyhow::Result<()> {
    let code = r#"
fn f1(): f64 { return 13.0_f64; }
fn f2(): f64 { return 5.0_f64; }

let addition = (((f1() + f2()) + (f2() + f1())) + (f2() + f2())) + ((f2() + (f2() + f2())) + (f2() + (f1() + f2())));
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


.f1_void~f64:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; return 13
    mov rax, __?float64?__(13.0)
    leave
    ret
.f2_void~f64:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; return 5
    mov rax, __?float64?__(5.0)
    leave
    ret
main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; let addition: f64 = ((((f1() + f2()) + (f2() + f1())) + (f2() + f2())) + ((f2() + (f2() + f2())) + (f2() + (f1() + f2()))))
    ; ((((f1() + f2()) + (f2() + f1())) + (f2() + f2())) + ((f2() + (f2() + f2())) + (f2() + (f1() + f2()))))
    ; (((f1() + f2()) + (f2() + f1())) + (f2() + f2()))
    ; ((f1() + f2()) + (f2() + f1()))
    ; (f1() + f2())
    ; PushQ
    push rcx
    push rdi
    push rdx
    ; f1()
    call .f1_void~f64
    movq xmm0, rax
    ; PopQ
    pop rdx
    pop rdi
    pop rcx
    ; PushQ
    push rax
    push rcx
    push rdi
    push rdx
    ; f2()
    call .f2_void~f64
    movq xmm3, rax
    ; PopQ
    pop rdx
    pop rdi
    pop rcx
    pop rax
    addsd xmm0, xmm3
    movq xmm1, xmm0
    ; (f2() + f1())
    ; PushQ
    push rcx
    push rdi
    push rdx
    ; f2()
    call .f2_void~f64
    movq xmm0, rax
    ; PopQ
    pop rdx
    pop rdi
    pop rcx
    ; PushQ
    push rax
    push rcx
    push rdi
    push rdx
    ; f1()
    call .f1_void~f64
    movq xmm3, rax
    ; PopQ
    pop rdx
    pop rdi
    pop rcx
    pop rax
    addsd xmm0, xmm3
    movq xmm2, xmm0
    addsd xmm1, xmm2
    movq xmm0, xmm1
    movq rax, xmm0
    push rax
    xor rax, rax
    ; (f2() + f2())
    ; PushQ
    push rcx
    push rdi
    push rdx
    ; f2()
    call .f2_void~f64
    movq xmm0, rax
    ; PopQ
    pop rdx
    pop rdi
    pop rcx
    ; PushQ
    push rax
    push rcx
    push rdi
    push rdx
    ; f2()
    call .f2_void~f64
    movq xmm3, rax
    ; PopQ
    pop rdx
    pop rdi
    pop rcx
    pop rax
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
    ; ((f2() + (f2() + f2())) + (f2() + (f1() + f2())))
    ; (f2() + (f2() + f2()))
    ; (f2() + f2())
    ; PushQ
    push rcx
    push rdi
    push rdx
    ; f2()
    call .f2_void~f64
    movq xmm0, rax
    ; PopQ
    pop rdx
    pop rdi
    pop rcx
    ; PushQ
    push rax
    push rcx
    push rdi
    push rdx
    ; f2()
    call .f2_void~f64
    movq xmm3, rax
    ; PopQ
    pop rdx
    pop rdi
    pop rcx
    pop rax
    addsd xmm0, xmm3
    movq xmm3, xmm0
    ; PushQ
    push rcx
    push rdi
    push rdx
    ; f2()
    call .f2_void~f64
    movq xmm0, rax
    ; PopQ
    pop rdx
    pop rdi
    pop rcx
    movq xmm0, rax
    addsd xmm0, xmm3
    movq xmm2, xmm0
    movq rdi, xmm2
    push rdi
    xor rdi, rdi
    ; (f2() + (f1() + f2()))
    ; (f1() + f2())
    ; PushQ
    push rcx
    push rdi
    push rdx
    ; f1()
    call .f1_void~f64
    movq xmm0, rax
    ; PopQ
    pop rdx
    pop rdi
    pop rcx
    ; PushQ
    push rax
    push rcx
    push rdi
    push rdx
    ; f2()
    call .f2_void~f64
    movq xmm3, rax
    ; PopQ
    pop rdx
    pop rdi
    pop rcx
    pop rax
    addsd xmm0, xmm3
    movq xmm3, xmm0
    ; PushQ
    push rcx
    push rdi
    push rdx
    ; f2()
    call .f2_void~f64
    movq xmm0, rax
    ; PopQ
    pop rdx
    pop rdi
    pop rcx
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
    mov QWORD [rbp - 8], rax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}


#[test]
fn single_expression_test() -> anyhow::Result<()> {
    let code = r#"
fn f1(): f64 { return 5.0_f64; }
    let a: f64 = f1();
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


.f1_void~f64:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; return 5
    mov rax, __?float64?__(5.0)
    leave
    ret
main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; let a: f64 = f1()
    ; f1()
    call .f1_void~f64
    mov QWORD [rbp - 8], rax
    ; let b: f64 = a
    mov rax, QWORD [rbp - 8]
    mov QWORD [rbp - 16], rax
    leave
    ret
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
fn f1(): f64 { return 5.0_f64; }
    let a: f64 = f1();
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
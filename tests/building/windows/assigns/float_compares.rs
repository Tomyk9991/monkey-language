use monkey_language::core::code_generator::generator::ASMGenerator;
use monkey_language::core::code_generator::target_os::TargetOS;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::type_checker::static_type_checker::static_type_check;

#[test]
fn simple_compare_f32() -> anyhow::Result<()> {
    let code = r#"
    let a: f32 = 5.0;
    let b: f32 = 3.0;

    let c: bool = a > b;
    let d: bool = 5.0 > 3.0;

    let e: bool = a < b;
    let f: bool = 5.0 < 3.0;

    let g: bool = a <= b;
    let h: bool = 5.0 <= 3.0;

    let i: bool = a >= b;
    let j: bool = 5.0 >= 3.0;

    let k: bool = a == b;
    let l: bool = 5.0 == 3.0;

    let m: bool = a != b;
    let n: bool = 5.0 != 3.0;
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
    sub rsp, 64
    ; let a: f32 = 5
    mov eax, __?float32?__(5.0)
    mov DWORD [rbp - 4], eax
    ; let b: f32 = 3
    mov eax, __?float32?__(3.0)
    mov DWORD [rbp - 8], eax
    ; let c: bool = (a > b)
    ; (a > b)
    mov eax, DWORD [rbp - 4]
    movd xmm0, eax
    mov edx, DWORD [rbp - 8]
    movd xmm3, edx
    ucomiss xmm0, xmm3
    seta al
    mov BYTE [rbp - 9], al
    ; let d: bool = (5 > 3)
    ; (5 > 3)
    mov eax, __?float32?__(5.0)
    movd xmm0, eax
    mov edx, __?float32?__(3.0)
    movd xmm3, edx
    ucomiss xmm0, xmm3
    seta al
    mov BYTE [rbp - 10], al
    ; let e: bool = (a < b)
    ; (a < b)
    mov eax, DWORD [rbp - 4]
    movd xmm0, eax
    mov edx, DWORD [rbp - 8]
    movd xmm3, edx
    ucomiss xmm0, xmm3
    setb al
    mov BYTE [rbp - 11], al
    ; let f: bool = (5 < 3)
    ; (5 < 3)
    mov eax, __?float32?__(5.0)
    movd xmm0, eax
    mov edx, __?float32?__(3.0)
    movd xmm3, edx
    ucomiss xmm0, xmm3
    setb al
    mov BYTE [rbp - 12], al
    ; let g: bool = (a <= b)
    ; (a <= b)
    mov eax, DWORD [rbp - 4]
    movd xmm0, eax
    mov edx, DWORD [rbp - 8]
    movd xmm3, edx
    ucomiss xmm0, xmm3
    setbe al
    mov BYTE [rbp - 13], al
    ; let h: bool = (5 <= 3)
    ; (5 <= 3)
    mov eax, __?float32?__(5.0)
    movd xmm0, eax
    mov edx, __?float32?__(3.0)
    movd xmm3, edx
    ucomiss xmm0, xmm3
    setbe al
    mov BYTE [rbp - 14], al
    ; let i: bool = (a >= b)
    ; (a >= b)
    mov eax, DWORD [rbp - 4]
    movd xmm0, eax
    mov edx, DWORD [rbp - 8]
    movd xmm3, edx
    ucomiss xmm0, xmm3
    setae al
    mov BYTE [rbp - 15], al
    ; let j: bool = (5 >= 3)
    ; (5 >= 3)
    mov eax, __?float32?__(5.0)
    movd xmm0, eax
    mov edx, __?float32?__(3.0)
    movd xmm3, edx
    ucomiss xmm0, xmm3
    setae al
    mov BYTE [rbp - 16], al
    ; let k: bool = (a == b)
    ; (a == b)
    mov eax, DWORD [rbp - 4]
    movd xmm0, eax
    mov edx, DWORD [rbp - 8]
    movd xmm3, edx
    ucomiss xmm0, xmm3
    sete al
    mov BYTE [rbp - 17], al
    ; let l: bool = (5 == 3)
    ; (5 == 3)
    mov eax, __?float32?__(5.0)
    movd xmm0, eax
    mov edx, __?float32?__(3.0)
    movd xmm3, edx
    ucomiss xmm0, xmm3
    sete al
    mov BYTE [rbp - 18], al
    ; let m: bool = (a != b)
    ; (a != b)
    mov eax, DWORD [rbp - 4]
    movd xmm0, eax
    mov edx, DWORD [rbp - 8]
    movd xmm3, edx
    ucomiss xmm0, xmm3
    setne al
    mov BYTE [rbp - 19], al
    ; let n: bool = (5 != 3)
    ; (5 != 3)
    mov eax, __?float32?__(5.0)
    movd xmm0, eax
    mov edx, __?float32?__(3.0)
    movd xmm3, edx
    ucomiss xmm0, xmm3
    setne al
    mov BYTE [rbp - 20], al
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
fn simple_compare_f64() -> anyhow::Result<()> {
    let code = r#"
    let a: f64 = 5.0;
    let b: f64 = 3.0;

    let c: bool = a > b;
    let d: bool = 5.0_f64 > 3.0_f64;

    let e: bool = a < b;
    let f: bool = 5.0_f64 < 3.0_f64;

    let g: bool = a <= b;
    let h: bool = 5.0_f64 <= 3.0_f64;

    let i: bool = a >= b;
    let j: bool = 5.0_f64 >= 3.0_f64;

    let k: bool = a == b;
    let l: bool = 5.0_f64 == 3.0_f64;

    let m: bool = a != b;
    let n: bool = 5.0_f64 != 3.0_f64;
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
    sub rsp, 64
    ; let a: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    ; let b: f64 = 3
    mov rax, __?float64?__(3.0)
    mov QWORD [rbp - 16], rax
    ; let c: bool = (a > b)
    ; (a > b)
    mov rax, QWORD [rbp - 8]
    movq xmm0, rax
    mov rdx, QWORD [rbp - 16]
    movq xmm3, rdx
    ucomisd xmm0, xmm3
    seta al
    mov BYTE [rbp - 17], al
    ; let d: bool = (5 > 3)
    ; (5 > 3)
    mov rax, __?float64?__(5.0)
    movq xmm0, rax
    mov rdx, __?float64?__(3.0)
    movq xmm3, rdx
    ucomisd xmm0, xmm3
    seta al
    mov BYTE [rbp - 18], al
    ; let e: bool = (a < b)
    ; (a < b)
    mov rax, QWORD [rbp - 8]
    movq xmm0, rax
    mov rdx, QWORD [rbp - 16]
    movq xmm3, rdx
    ucomisd xmm0, xmm3
    setb al
    mov BYTE [rbp - 19], al
    ; let f: bool = (5 < 3)
    ; (5 < 3)
    mov rax, __?float64?__(5.0)
    movq xmm0, rax
    mov rdx, __?float64?__(3.0)
    movq xmm3, rdx
    ucomisd xmm0, xmm3
    setb al
    mov BYTE [rbp - 20], al
    ; let g: bool = (a <= b)
    ; (a <= b)
    mov rax, QWORD [rbp - 8]
    movq xmm0, rax
    mov rdx, QWORD [rbp - 16]
    movq xmm3, rdx
    ucomisd xmm0, xmm3
    setbe al
    mov BYTE [rbp - 21], al
    ; let h: bool = (5 <= 3)
    ; (5 <= 3)
    mov rax, __?float64?__(5.0)
    movq xmm0, rax
    mov rdx, __?float64?__(3.0)
    movq xmm3, rdx
    ucomisd xmm0, xmm3
    setbe al
    mov BYTE [rbp - 22], al
    ; let i: bool = (a >= b)
    ; (a >= b)
    mov rax, QWORD [rbp - 8]
    movq xmm0, rax
    mov rdx, QWORD [rbp - 16]
    movq xmm3, rdx
    ucomisd xmm0, xmm3
    setae al
    mov BYTE [rbp - 23], al
    ; let j: bool = (5 >= 3)
    ; (5 >= 3)
    mov rax, __?float64?__(5.0)
    movq xmm0, rax
    mov rdx, __?float64?__(3.0)
    movq xmm3, rdx
    ucomisd xmm0, xmm3
    setae al
    mov BYTE [rbp - 24], al
    ; let k: bool = (a == b)
    ; (a == b)
    mov rax, QWORD [rbp - 8]
    movq xmm0, rax
    mov rdx, QWORD [rbp - 16]
    movq xmm3, rdx
    ucomisd xmm0, xmm3
    sete al
    mov BYTE [rbp - 25], al
    ; let l: bool = (5 == 3)
    ; (5 == 3)
    mov rax, __?float64?__(5.0)
    movq xmm0, rax
    mov rdx, __?float64?__(3.0)
    movq xmm3, rdx
    ucomisd xmm0, xmm3
    sete al
    mov BYTE [rbp - 26], al
    ; let m: bool = (a != b)
    ; (a != b)
    mov rax, QWORD [rbp - 8]
    movq xmm0, rax
    mov rdx, QWORD [rbp - 16]
    movq xmm3, rdx
    ucomisd xmm0, xmm3
    setne al
    mov BYTE [rbp - 27], al
    ; let n: bool = (5 != 3)
    ; (5 != 3)
    mov rax, __?float64?__(5.0)
    movq xmm0, rax
    mov rdx, __?float64?__(3.0)
    movq xmm3, rdx
    ucomisd xmm0, xmm3
    setne al
    mov BYTE [rbp - 28], al
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
fn compare_changed() -> anyhow::Result<()> {
    let code = r#"
    let a = 3.0 == 3.0 && 7.0 != 9.0;
    let b = 3.0_f64 == 3.0_f64 && 7.0_f64 != 9.0_f64;
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
    sub rsp, 64
    ; let a: bool = ((3 == 3) && (7 != 9))
    ; ((3 == 3) && (7 != 9))
    ; (3 == 3)
    mov eax, __?float32?__(3.0)
    movd xmm0, eax
    mov edx, __?float32?__(3.0)
    movd xmm3, edx
    ucomiss xmm0, xmm3
    sete al
    mov cl, al
    ; (7 != 9)
    mov eax, __?float32?__(7.0)
    movd xmm0, eax
    mov edx, __?float32?__(9.0)
    movd xmm3, edx
    ucomiss xmm0, xmm3
    setne al
    mov dil, al
    mov r14b, dl
    mov r13b, al
    mov r12b, cl
    mov al, dil
    mov dl, 0
    cmp cl, 0
    je .label0
    mov al, dil
    cmp al, 0
    je .label0
    mov eax, 1
    jmp .label1
.label0:
    mov eax, 0
.label1:
    mov cl, al
    mov dl, r14b
    mov al, r13b
    mov BYTE [rbp - 1], cl
    ; let b: bool = ((3 == 3) && (7 != 9))
    ; ((3 == 3) && (7 != 9))
    ; (3 == 3)
    mov rax, __?float64?__(3.0)
    movq xmm0, rax
    mov rdx, __?float64?__(3.0)
    movq xmm3, rdx
    ucomisd xmm0, xmm3
    sete al
    mov cl, al
    ; (7 != 9)
    mov rax, __?float64?__(7.0)
    movq xmm0, rax
    mov rdx, __?float64?__(9.0)
    movq xmm3, rdx
    ucomisd xmm0, xmm3
    setne al
    mov dil, al
    mov r14b, dl
    mov r13b, al
    mov r12b, cl
    mov al, dil
    mov dl, 0
    cmp cl, 0
    je .label2
    mov al, dil
    cmp al, 0
    je .label2
    mov eax, 1
    jmp .label3
.label2:
    mov eax, 0
.label3:
    mov cl, al
    mov dl, r14b
    mov al, r13b
    mov BYTE [rbp - 2], cl
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
fn compare_complex_f32() -> anyhow::Result<()> {
    let code = r#"
    let a: f32 = 5.0;
    let b: f32 = 3.0;
    let c: f32 = 7.0;
    let d: f32 = 9.0;

    let result = (a == b && c != d && a >= b) || (c <= d && a < b && c > d);
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
    sub rsp, 64
    ; let a: f32 = 5
    mov eax, __?float32?__(5.0)
    mov DWORD [rbp - 4], eax
    ; let b: f32 = 3
    mov eax, __?float32?__(3.0)
    mov DWORD [rbp - 8], eax
    ; let c: f32 = 7
    mov eax, __?float32?__(7.0)
    mov DWORD [rbp - 12], eax
    ; let d: f32 = 9
    mov eax, __?float32?__(9.0)
    mov DWORD [rbp - 16], eax
    ; let result: bool = ((((a == b) && (c != d)) && (a >= b)) || (((c <= d) && (a < b)) && (c > d)))
    ; ((((a == b) && (c != d)) && (a >= b)) || (((c <= d) && (a < b)) && (c > d)))
    ; (((a == b) && (c != d)) && (a >= b))
    ; ((a == b) && (c != d))
    ; (a == b)
    mov eax, DWORD [rbp - 4]
    movd xmm0, eax
    mov edx, DWORD [rbp - 8]
    movd xmm3, edx
    ucomiss xmm0, xmm3
    sete al
    mov cl, al
    ; (c != d)
    mov eax, DWORD [rbp - 12]
    movd xmm0, eax
    mov edx, DWORD [rbp - 16]
    movd xmm3, edx
    ucomiss xmm0, xmm3
    setne al
    mov dil, al
    mov r14b, dl
    mov r13b, al
    mov r12b, cl
    mov al, dil
    mov dl, 0
    cmp cl, 0
    je .label0
    mov al, dil
    cmp al, 0
    je .label0
    mov eax, 1
    jmp .label1
.label0:
    mov eax, 0
.label1:
    mov cl, al
    mov dl, r14b
    mov al, r13b
    mov dil, cl
    push rdi
    xor rdi, rdi
    ; (a >= b)
    mov eax, DWORD [rbp - 4]
    movd xmm0, eax
    mov edx, DWORD [rbp - 8]
    movd xmm3, edx
    ucomiss xmm0, xmm3
    setae al
    push rax
    xor rax, rax
    pop rdi
    pop rax
    mov r14b, dl
    mov r13b, al
    mov r12b, cl
    mov cl, al
    mov al, dil
    mov dl, 0
    cmp cl, 0
    je .label2
    mov al, dil
    cmp al, 0
    je .label2
    mov eax, 1
    jmp .label3
.label2:
    mov eax, 0
.label3:
    mov dl, r14b
    mov cl, r12b
    mov dil, al
    push rdi
    xor rdi, rdi
    ; (((c <= d) && (a < b)) && (c > d))
    ; ((c <= d) && (a < b))
    ; (c <= d)
    mov eax, DWORD [rbp - 12]
    movd xmm0, eax
    mov edx, DWORD [rbp - 16]
    movd xmm3, edx
    ucomiss xmm0, xmm3
    setbe al
    mov cl, al
    ; (a < b)
    mov eax, DWORD [rbp - 4]
    movd xmm0, eax
    mov edx, DWORD [rbp - 8]
    movd xmm3, edx
    ucomiss xmm0, xmm3
    setb al
    mov dil, al
    mov r14b, dl
    mov r13b, al
    mov r12b, cl
    mov al, dil
    mov dl, 0
    cmp cl, 0
    je .label4
    mov al, dil
    cmp al, 0
    je .label4
    mov eax, 1
    jmp .label5
.label4:
    mov eax, 0
.label5:
    mov cl, al
    mov dl, r14b
    mov al, r13b
    mov dil, cl
    push rdi
    xor rdi, rdi
    ; (c > d)
    mov eax, DWORD [rbp - 12]
    movd xmm0, eax
    mov edx, DWORD [rbp - 16]
    movd xmm3, edx
    ucomiss xmm0, xmm3
    seta al
    push rax
    xor rax, rax
    pop rdi
    pop rax
    mov r14b, dl
    mov r13b, al
    mov r12b, cl
    mov cl, al
    mov al, dil
    mov dl, 0
    cmp cl, 0
    je .label6
    mov al, dil
    cmp al, 0
    je .label6
    mov eax, 1
    jmp .label7
.label6:
    mov eax, 0
.label7:
    mov dl, r14b
    mov cl, r12b
    push rax
    xor rax, rax
    pop rdi
    pop rax
    mov r14b, dl
    mov r13b, al
    mov r12b, cl
    mov cl, al
    mov al, dil
    mov dl, 0
    cmp cl, 0
    jne .label8
    mov al, dil
    cmp al, 0
    je .label9
.label8:
    mov eax, 1
    jmp .label10
.label9:
    mov eax, 0
.label10:
    mov dl, r14b
    mov cl, r12b
    mov BYTE [rbp - 17], al
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
fn compare_complex_f64() -> anyhow::Result<()> {
    let code = r#"
    let a: f64 = 5.0;
    let b: f64 = 3.0;
    let c: f64 = 7.0;
    let d: f64 = 9.0;

    let result = (a == b && c != d && a >= b) || (c <= d && a < b && c > d);
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
    sub rsp, 128
    ; let a: f64 = 5
    mov rax, __?float64?__(5.0)
    mov QWORD [rbp - 8], rax
    ; let b: f64 = 3
    mov rax, __?float64?__(3.0)
    mov QWORD [rbp - 16], rax
    ; let c: f64 = 7
    mov rax, __?float64?__(7.0)
    mov QWORD [rbp - 24], rax
    ; let d: f64 = 9
    mov rax, __?float64?__(9.0)
    mov QWORD [rbp - 32], rax
    ; let result: bool = ((((a == b) && (c != d)) && (a >= b)) || (((c <= d) && (a < b)) && (c > d)))
    ; ((((a == b) && (c != d)) && (a >= b)) || (((c <= d) && (a < b)) && (c > d)))
    ; (((a == b) && (c != d)) && (a >= b))
    ; ((a == b) && (c != d))
    ; (a == b)
    mov rax, QWORD [rbp - 8]
    movq xmm0, rax
    mov rdx, QWORD [rbp - 16]
    movq xmm3, rdx
    ucomisd xmm0, xmm3
    sete al
    mov cl, al
    ; (c != d)
    mov rax, QWORD [rbp - 24]
    movq xmm0, rax
    mov rdx, QWORD [rbp - 32]
    movq xmm3, rdx
    ucomisd xmm0, xmm3
    setne al
    mov dil, al
    mov r14b, dl
    mov r13b, al
    mov r12b, cl
    mov al, dil
    mov dl, 0
    cmp cl, 0
    je .label0
    mov al, dil
    cmp al, 0
    je .label0
    mov eax, 1
    jmp .label1
.label0:
    mov eax, 0
.label1:
    mov cl, al
    mov dl, r14b
    mov al, r13b
    mov dil, cl
    push rdi
    xor rdi, rdi
    ; (a >= b)
    mov rax, QWORD [rbp - 8]
    movq xmm0, rax
    mov rdx, QWORD [rbp - 16]
    movq xmm3, rdx
    ucomisd xmm0, xmm3
    setae al
    push rax
    xor rax, rax
    pop rdi
    pop rax
    mov r14b, dl
    mov r13b, al
    mov r12b, cl
    mov cl, al
    mov al, dil
    mov dl, 0
    cmp cl, 0
    je .label2
    mov al, dil
    cmp al, 0
    je .label2
    mov eax, 1
    jmp .label3
.label2:
    mov eax, 0
.label3:
    mov dl, r14b
    mov cl, r12b
    mov dil, al
    push rdi
    xor rdi, rdi
    ; (((c <= d) && (a < b)) && (c > d))
    ; ((c <= d) && (a < b))
    ; (c <= d)
    mov rax, QWORD [rbp - 24]
    movq xmm0, rax
    mov rdx, QWORD [rbp - 32]
    movq xmm3, rdx
    ucomisd xmm0, xmm3
    setbe al
    mov cl, al
    ; (a < b)
    mov rax, QWORD [rbp - 8]
    movq xmm0, rax
    mov rdx, QWORD [rbp - 16]
    movq xmm3, rdx
    ucomisd xmm0, xmm3
    setb al
    mov dil, al
    mov r14b, dl
    mov r13b, al
    mov r12b, cl
    mov al, dil
    mov dl, 0
    cmp cl, 0
    je .label4
    mov al, dil
    cmp al, 0
    je .label4
    mov eax, 1
    jmp .label5
.label4:
    mov eax, 0
.label5:
    mov cl, al
    mov dl, r14b
    mov al, r13b
    mov dil, cl
    push rdi
    xor rdi, rdi
    ; (c > d)
    mov rax, QWORD [rbp - 24]
    movq xmm0, rax
    mov rdx, QWORD [rbp - 32]
    movq xmm3, rdx
    ucomisd xmm0, xmm3
    seta al
    push rax
    xor rax, rax
    pop rdi
    pop rax
    mov r14b, dl
    mov r13b, al
    mov r12b, cl
    mov cl, al
    mov al, dil
    mov dl, 0
    cmp cl, 0
    je .label6
    mov al, dil
    cmp al, 0
    je .label6
    mov eax, 1
    jmp .label7
.label6:
    mov eax, 0
.label7:
    mov dl, r14b
    mov cl, r12b
    push rax
    xor rax, rax
    pop rdi
    pop rax
    mov r14b, dl
    mov r13b, al
    mov r12b, cl
    mov cl, al
    mov al, dil
    mov dl, 0
    cmp cl, 0
    jne .label8
    mov al, dil
    cmp al, 0
    je .label9
.label8:
    mov eax, 1
    jmp .label10
.label9:
    mov eax, 0
.label10:
    mov dl, r14b
    mov cl, r12b
    mov BYTE [rbp - 33], al
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}
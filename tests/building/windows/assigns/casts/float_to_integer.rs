use monkey_language::core::code_generator::generator::ASMGenerator;
use monkey_language::core::code_generator::target_os::TargetOS;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::type_checker::static_type_checker::static_type_check;

#[test]
fn f32_to_u() -> anyhow::Result<()> {
    let code = r#"
    let a: f32 = 120.5;
    let b: u8 = (u8)a;

    let c: f32 = 120.5;
    let d: u16 = (u16)c;

    let e: f32 = 120.5;
    let f: u32 = (u32)e;

    let g: f32 = 120.5;
    let h: u64 = (u64)g;
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
    ; let a: f32 = 120.5
    mov eax, __?float32?__(120.5)
    mov DWORD [rbp - 4], eax
    ; let b: u8 = (u8)a
    ; Cast: (f32) -> (u8)
    mov eax, DWORD [rbp - 4]
    movd xmm7, eax
    cvtss2si eax, xmm7
    ; Cast: (i32) -> (u8)
    mov BYTE [rbp - 5], al
    ; let c: f32 = 120.5
    mov eax, __?float32?__(120.5)
    mov DWORD [rbp - 9], eax
    ; let d: u16 = (u16)c
    ; Cast: (f32) -> (u16)
    mov eax, DWORD [rbp - 9]
    movd xmm7, eax
    cvtss2si eax, xmm7
    ; Cast: (i32) -> (u16)
    mov WORD [rbp - 11], ax
    ; let e: f32 = 120.5
    mov eax, __?float32?__(120.5)
    mov DWORD [rbp - 15], eax
    ; let f: u32 = (u32)e
    ; Cast: (f32) -> (u32)
    mov eax, DWORD [rbp - 15]
    movd xmm7, eax
    cvtss2si eax, xmm7
    ; Cast: (i32) -> (u32)
    mov DWORD [rbp - 19], eax
    ; let g: f32 = 120.5
    mov eax, __?float32?__(120.5)
    mov DWORD [rbp - 23], eax
    ; let h: u64 = (u64)g
    ; Cast: (f32) -> (u64)
    mov eax, DWORD [rbp - 23]
    movd xmm7, eax
    cvtss2si eax, xmm7
    ; Cast: (i32) -> (u64)
    mov r14d, eax
    xor rax, rax
    mov eax, r14d
    mov QWORD [rbp - 31], rax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn f32_to_i() -> anyhow::Result<()> {
    let code = r#"
    let a: f32 = -120.5;
    let b: i8 = (i8)a;

    let c: f32 = -120.5;
    let d: i16 = (i16)c;

    let e: f32 = -120.5;
    let f: i32 = (i32)e;

    let g: f32 = -120.5;
    let h: i64 = (i64)g;
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
    ; let a: f32 = -120.5
    mov eax, __?float32?__(-120.5)
    mov DWORD [rbp - 4], eax
    ; let b: i8 = (i8)a
    ; Cast: (f32) -> (i8)
    mov eax, DWORD [rbp - 4]
    movd xmm7, eax
    cvtss2si eax, xmm7
    ; Cast: (i32) -> (i8)
    mov BYTE [rbp - 5], al
    ; let c: f32 = -120.5
    mov eax, __?float32?__(-120.5)
    mov DWORD [rbp - 9], eax
    ; let d: i16 = (i16)c
    ; Cast: (f32) -> (i16)
    mov eax, DWORD [rbp - 9]
    movd xmm7, eax
    cvtss2si eax, xmm7
    ; Cast: (i32) -> (i16)
    mov WORD [rbp - 11], ax
    ; let e: f32 = -120.5
    mov eax, __?float32?__(-120.5)
    mov DWORD [rbp - 15], eax
    ; let f: i32 = (i32)e
    ; Cast: (f32) -> (i32)
    mov eax, DWORD [rbp - 15]
    movd xmm7, eax
    cvtss2si eax, xmm7
    mov DWORD [rbp - 19], eax
    ; let g: f32 = -120.5
    mov eax, __?float32?__(-120.5)
    mov DWORD [rbp - 23], eax
    ; let h: i64 = (i64)g
    ; Cast: (f32) -> (i64)
    mov eax, DWORD [rbp - 23]
    movd xmm7, eax
    cvtss2si eax, xmm7
    ; Cast: (i32) -> (i64)
    movsx rax, eax
    mov QWORD [rbp - 31], rax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}



#[test]
fn f64_to_u() -> anyhow::Result<()> {
    let code = r#"
    let a: f64 = 120.5;
    let b: u8 = (u8)a;

    let c: f64 = 120.5;
    let d: u16 = (u16)c;

    let e: f64 = 120.5;
    let f: u32 = (u32)e;

    let g: f64 = 120.5;
    let h: u64 = (u64)g;
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
    sub rsp, 128
    ; let a: f64 = 120.5
    mov rax, __?float64?__(120.5)
    mov QWORD [rbp - 8], rax
    ; let b: u8 = (u8)a
    ; Cast: (f64) -> (u8)
    mov rax, QWORD [rbp - 8]
    movq xmm7, rax
    cvtsd2si rax, xmm7
    ; Cast: (i64) -> (u8)
    mov BYTE [rbp - 9], al
    ; let c: f64 = 120.5
    mov rax, __?float64?__(120.5)
    mov QWORD [rbp - 17], rax
    ; let d: u16 = (u16)c
    ; Cast: (f64) -> (u16)
    mov rax, QWORD [rbp - 17]
    movq xmm7, rax
    cvtsd2si rax, xmm7
    ; Cast: (i64) -> (u16)
    mov WORD [rbp - 19], ax
    ; let e: f64 = 120.5
    mov rax, __?float64?__(120.5)
    mov QWORD [rbp - 27], rax
    ; let f: u32 = (u32)e
    ; Cast: (f64) -> (u32)
    mov rax, QWORD [rbp - 27]
    movq xmm7, rax
    cvtsd2si rax, xmm7
    ; Cast: (i64) -> (u32)
    mov DWORD [rbp - 31], eax
    ; let g: f64 = 120.5
    mov rax, __?float64?__(120.5)
    mov QWORD [rbp - 39], rax
    ; let h: u64 = (u64)g
    ; Cast: (f64) -> (u64)
    mov rax, QWORD [rbp - 39]
    movq xmm7, rax
    cvtsd2si rax, xmm7
    ; Cast: (i64) -> (u64)
    mov QWORD [rbp - 47], rax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn f64_to_i() -> anyhow::Result<()> {
    let code = r#"
    let a: f64 = -120.5;
    let b: i8 = (i8)a;

    let c: f64 = -120.5;
    let d: i16 = (i16)c;

    let e: f64 = -120.5;
    let f: i32 = (i32)e;

    let g: f64 = -120.5;
    let h: i64 = (i64)g;
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
    sub rsp, 128
    ; let a: f64 = -120.5
    mov rax, __?float64?__(-120.5)
    mov QWORD [rbp - 8], rax
    ; let b: i8 = (i8)a
    ; Cast: (f64) -> (i8)
    mov rax, QWORD [rbp - 8]
    movq xmm7, rax
    cvtsd2si rax, xmm7
    ; Cast: (i64) -> (i8)
    mov BYTE [rbp - 9], al
    ; let c: f64 = -120.5
    mov rax, __?float64?__(-120.5)
    mov QWORD [rbp - 17], rax
    ; let d: i16 = (i16)c
    ; Cast: (f64) -> (i16)
    mov rax, QWORD [rbp - 17]
    movq xmm7, rax
    cvtsd2si rax, xmm7
    ; Cast: (i64) -> (i16)
    mov WORD [rbp - 19], ax
    ; let e: f64 = -120.5
    mov rax, __?float64?__(-120.5)
    mov QWORD [rbp - 27], rax
    ; let f: i32 = (i32)e
    ; Cast: (f64) -> (i32)
    mov rax, QWORD [rbp - 27]
    movq xmm7, rax
    cvtsd2si rax, xmm7
    ; Cast: (i64) -> (i32)
    mov DWORD [rbp - 31], eax
    ; let g: f64 = -120.5
    mov rax, __?float64?__(-120.5)
    mov QWORD [rbp - 39], rax
    ; let h: i64 = (i64)g
    ; Cast: (f64) -> (i64)
    mov rax, QWORD [rbp - 39]
    movq xmm7, rax
    cvtsd2si rax, xmm7
    mov QWORD [rbp - 47], rax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

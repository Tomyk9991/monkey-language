use monkey_language::core::code_generator::generator::ASMGenerator;
use monkey_language::core::code_generator::target_os::TargetOS;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::type_checker::static_type_checker::static_type_check;

#[test]
fn u_to_f32() -> anyhow::Result<()> {
    let code = r#"
    let a: u8 = 250;
    let b: f32 = (f32)a;

    let c: u16 = 250;
    let d: f32 = (f32)c;

    let e: u32 = 250;
    let f: f32 = (f32)e;

    let g: u64 = 250;
    let h: f32 = (f32)g;
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
    sub rsp, 63
    ; let a: u8 = 250
    mov BYTE [rbp - 1], 250
    ; let b: f32 = (f32)a
    movd eax, xmm0
    ; Cast: (u8) -> (f32)
    ; Cast: (u8) -> (u32)
    movzx eax, BYTE [rbp - 1]
    cvtsi2ss xmm7, eax
    movd eax, xmm7
    mov DWORD [rbp - 5], eax
    ; let c: u16 = 250
    mov WORD [rbp - 7], 250
    ; let d: f32 = (f32)c
    movd eax, xmm0
    ; Cast: (u16) -> (f32)
    ; Cast: (u16) -> (u32)
    movzx eax, WORD [rbp - 7]
    cvtsi2ss xmm7, eax
    movd eax, xmm7
    mov DWORD [rbp - 11], eax
    ; let e: u32 = 250
    mov DWORD [rbp - 15], 250
    ; let f: f32 = (f32)e
    movd eax, xmm0
    ; Cast: (u32) -> (f32)
    mov eax, DWORD [rbp - 15]
    cvtsi2ss xmm7, eax
    movd eax, xmm7
    mov DWORD [rbp - 19], eax
    ; let g: u64 = 250
    mov QWORD [rbp - 27], 250
    ; let h: f32 = (f32)g
    movd eax, xmm0
    ; Cast: (u64) -> (f32)
    ; Cast: (u64) -> (u32)
    mov rax, [rbp - 27]
    cvtsi2ss xmm7, eax
    movd eax, xmm7
    mov DWORD [rbp - 31], eax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn i_to_f32() -> anyhow::Result<()> {
    let code = r#"
    let a: i8 = -120;
    let b: f32 = (f32)a;

    let c: i16 = -120;
    let d: f32 = (f32)c;

    let e: i32 = -120;
    let f: f32 = (f32)e;

    let g: i64 = -120;
    let h: f32 = (f32)g;
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
    sub rsp, 63
    ; let a: i8 = -120
    mov BYTE [rbp - 1], -120
    ; let b: f32 = (f32)a
    movd eax, xmm0
    ; Cast: (i8) -> (f32)
    ; Cast: (i8) -> (u32)
    movsx eax, BYTE [rbp - 1]
    cvtsi2ss xmm7, eax
    movd eax, xmm7
    mov DWORD [rbp - 5], eax
    ; let c: i16 = -120
    mov WORD [rbp - 7], -120
    ; let d: f32 = (f32)c
    movd eax, xmm0
    ; Cast: (i16) -> (f32)
    ; Cast: (i16) -> (u32)
    movsx eax, WORD [rbp - 7]
    cvtsi2ss xmm7, eax
    movd eax, xmm7
    mov DWORD [rbp - 11], eax
    ; let e: i32 = -120
    mov DWORD [rbp - 15], -120
    ; let f: f32 = (f32)e
    movd eax, xmm0
    ; Cast: (i32) -> (f32)
    ; Cast: (i32) -> (u32)
    mov eax, DWORD [rbp - 15]
    cvtsi2ss xmm7, eax
    movd eax, xmm7
    mov DWORD [rbp - 19], eax
    ; let g: i64 = -120
    mov QWORD [rbp - 27], -120
    ; let h: f32 = (f32)g
    movd eax, xmm0
    ; Cast: (i64) -> (f32)
    ; Cast: (i64) -> (u32)
    mov rax, [rbp - 27]
    cvtsi2ss xmm7, eax
    movd eax, xmm7
    mov DWORD [rbp - 31], eax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn u_to_f64() -> anyhow::Result<()> {
    let code = r#"
    let a: u8 = 250;
    let b: f64 = (f64)a;

    let c: u16 = 250;
    let d: f64 = (f64)c;

    let e: u32 = 250;
    let f: f64 = (f64)e;

    let g: u64 = 250;
    let h: f64 = (f64)g;
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
    sub rsp, 79
    ; let a: u8 = 250
    mov BYTE [rbp - 1], 250
    ; let b: f64 = (f64)a
    movq rax, xmm0
    ; Cast: (u8) -> (f64)
    ; Cast: (u8) -> (u32)
    movzx eax, BYTE [rbp - 1]
    cvtsi2sd xmm7, eax
    movq rax, xmm7
    mov QWORD [rbp - 9], rax
    ; let c: u16 = 250
    mov WORD [rbp - 11], 250
    ; let d: f64 = (f64)c
    movq rax, xmm0
    ; Cast: (u16) -> (f64)
    ; Cast: (u16) -> (u32)
    movzx eax, WORD [rbp - 11]
    cvtsi2sd xmm7, eax
    movq rax, xmm7
    mov QWORD [rbp - 19], rax
    ; let e: u32 = 250
    mov DWORD [rbp - 23], 250
    ; let f: f64 = (f64)e
    movq rax, xmm0
    ; Cast: (u32) -> (f64)
    mov eax, DWORD [rbp - 23]
    cvtsi2sd xmm7, eax
    movq rax, xmm7
    mov QWORD [rbp - 31], rax
    ; let g: u64 = 250
    mov QWORD [rbp - 39], 250
    ; let h: f64 = (f64)g
    movq rax, xmm0
    ; Cast: (u64) -> (f64)
    ; Cast: (u64) -> (u32)
    mov rax, [rbp - 39]
    cvtsi2sd xmm7, eax
    movq rax, xmm7
    mov QWORD [rbp - 47], rax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}


#[test]
fn i_to_f64() -> anyhow::Result<()> {
    let code = r#"
    let a: i8 = -120;
    let b: f32 = (f32)a;

    let c: i16 = -120;
    let d: f32 = (f32)c;

    let e: i32 = -120;
    let f: f32 = (f32)e;

    let g: i64 = -120;
    let h: f32 = (f32)g;
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
    sub rsp, 63
    ; let a: i8 = -120
    mov BYTE [rbp - 1], -120
    ; let b: f32 = (f32)a
    movd eax, xmm0
    ; Cast: (i8) -> (f32)
    ; Cast: (i8) -> (u32)
    movsx eax, BYTE [rbp - 1]
    cvtsi2ss xmm7, eax
    movd eax, xmm7
    mov DWORD [rbp - 5], eax
    ; let c: i16 = -120
    mov WORD [rbp - 7], -120
    ; let d: f32 = (f32)c
    movd eax, xmm0
    ; Cast: (i16) -> (f32)
    ; Cast: (i16) -> (u32)
    movsx eax, WORD [rbp - 7]
    cvtsi2ss xmm7, eax
    movd eax, xmm7
    mov DWORD [rbp - 11], eax
    ; let e: i32 = -120
    mov DWORD [rbp - 15], -120
    ; let f: f32 = (f32)e
    movd eax, xmm0
    ; Cast: (i32) -> (f32)
    ; Cast: (i32) -> (u32)
    mov eax, DWORD [rbp - 15]
    cvtsi2ss xmm7, eax
    movd eax, xmm7
    mov DWORD [rbp - 19], eax
    ; let g: i64 = -120
    mov QWORD [rbp - 27], -120
    ; let h: f32 = (f32)g
    movd eax, xmm0
    ; Cast: (i64) -> (f32)
    ; Cast: (i64) -> (u32)
    mov rax, [rbp - 27]
    cvtsi2ss xmm7, eax
    movd eax, xmm7
    mov DWORD [rbp - 31], eax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}
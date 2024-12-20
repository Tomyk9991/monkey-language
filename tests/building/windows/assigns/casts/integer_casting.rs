use monkey_language::core::code_generator::generator::ASMGenerator;
use monkey_language::core::code_generator::target_os::TargetOS;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::type_checker::static_type_checker::static_type_check;

#[test]
fn u8_to_i() -> anyhow::Result<()> {
    let code = r#"
let a: u8 = 250;
let b: i8 = (i8) a;
let c: i16 = (i16) a;
let d: i32 = (i32) a;
let e: i64 = (i64) a;
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
    ; let a: u8 = 250
    mov BYTE [rbp - 1], 250
    ; let b: i8 = (i8)a
    ; Cast: (u8) -> (i8)
    mov al, BYTE [rbp - 1]
    mov BYTE [rbp - 2], al
    ; let c: i16 = (i16)a
    ; Cast: (u8) -> (i16)
    movzx ax, BYTE [rbp - 1]
    mov WORD [rbp - 4], ax
    ; let d: i32 = (i32)a
    ; Cast: (u8) -> (i32)
    movzx eax, BYTE [rbp - 1]
    mov DWORD [rbp - 8], eax
    ; let e: i64 = (i64)a
    ; Cast: (u8) -> (i64)
    movzx rax, BYTE [rbp - 1]
    mov QWORD [rbp - 16], rax
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn u8_to_u() -> anyhow::Result<()> {
    let code = r#"
let a: u8 = 250;
let b: u16 = (u16) a;
let c: u32 = (u32) a;
let d: u64 = (u64) a;
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
    ; let a: u8 = 250
    mov BYTE [rbp - 1], 250
    ; let b: u16 = (u16)a
    ; Cast: (u8) -> (u16)
    movzx ax, BYTE [rbp - 1]
    mov WORD [rbp - 3], ax
    ; let c: u32 = (u32)a
    ; Cast: (u8) -> (u32)
    movzx eax, BYTE [rbp - 1]
    mov DWORD [rbp - 7], eax
    ; let d: u64 = (u64)a
    ; Cast: (u8) -> (u64)
    movzx rax, BYTE [rbp - 1]
    mov QWORD [rbp - 15], rax
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}


#[test]
fn i8_to_i() -> anyhow::Result<()> {
    let code = r#"
let a: i8 = 120;
let b: i8 = a;
let c: i16 = (i16) a;
let d: i32 = (i32) a;
let e: i64 = (i64) a;
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
    ; let a: i8 = 120
    mov BYTE [rbp - 1], 120
    ; let b: i8 = a
    mov al, BYTE [rbp - 1]
    mov BYTE [rbp - 2], al
    ; let c: i16 = (i16)a
    ; Cast: (i8) -> (i16)
    movsx ax, BYTE [rbp - 1]
    mov WORD [rbp - 4], ax
    ; let d: i32 = (i32)a
    ; Cast: (i8) -> (i32)
    movsx eax, BYTE [rbp - 1]
    mov DWORD [rbp - 8], eax
    ; let e: i64 = (i64)a
    ; Cast: (i8) -> (i64)
    movsx rax, BYTE [rbp - 1]
    mov QWORD [rbp - 16], rax
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn i8_to_u() -> anyhow::Result<()> {
    let code = r#"
let a: i8 = 120;
let b: i8 = a;
let c: u16 = (u16) a;
let d: u32 = (u32) a;
let e: u64 = (u64) a;
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
    ; let a: i8 = 120
    mov BYTE [rbp - 1], 120
    ; let b: i8 = a
    mov al, BYTE [rbp - 1]
    mov BYTE [rbp - 2], al
    ; let c: u16 = (u16)a
    ; Cast: (i8) -> (u16)
    movsx ax, BYTE [rbp - 1]
    mov WORD [rbp - 4], ax
    ; let d: u32 = (u32)a
    ; Cast: (i8) -> (u32)
    movsx eax, BYTE [rbp - 1]
    mov DWORD [rbp - 8], eax
    ; let e: u64 = (u64)a
    ; Cast: (i8) -> (u64)
    movsx rax, BYTE [rbp - 1]
    mov QWORD [rbp - 16], rax
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}


#[test]
fn u16_to_i() -> anyhow::Result<()> {
    let code = r#"
let a: u16 = 250;
let b: i8 = (i8) a;
let c: i16 = (i16) a;
let d: i32 = (i32) a;
let e: i64 = (i64) a;
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
    ; let a: u16 = 250
    mov WORD [rbp - 2], 250
    ; let b: i8 = (i8)a
    ; Cast: (u16) -> (i8)
    mov ax, [rbp - 2]
    mov BYTE [rbp - 3], al
    ; let c: i16 = (i16)a
    ; Cast: (u16) -> (i16)
    mov ax, WORD [rbp - 2]
    mov WORD [rbp - 5], ax
    ; let d: i32 = (i32)a
    ; Cast: (u16) -> (i32)
    movzx eax, WORD [rbp - 2]
    mov DWORD [rbp - 9], eax
    ; let e: i64 = (i64)a
    ; Cast: (u16) -> (i64)
    movzx rax, WORD [rbp - 2]
    mov QWORD [rbp - 17], rax
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn u16_to_u() -> anyhow::Result<()> {
    let code = r#"
let a: u16 = 250;
let e: u8 = (u8) a;
let b: u16 = a;
let c: u32 = (u32) a;
let d: u64 = (u64) a;
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
    ; let a: u16 = 250
    mov WORD [rbp - 2], 250
    ; let e: u8 = (u8)a
    ; Cast: (u16) -> (u8)
    mov ax, [rbp - 2]
    mov BYTE [rbp - 3], al
    ; let b: u16 = a
    mov ax, WORD [rbp - 2]
    mov WORD [rbp - 5], ax
    ; let c: u32 = (u32)a
    ; Cast: (u16) -> (u32)
    movzx eax, WORD [rbp - 2]
    mov DWORD [rbp - 9], eax
    ; let d: u64 = (u64)a
    ; Cast: (u16) -> (u64)
    movzx rax, WORD [rbp - 2]
    mov QWORD [rbp - 17], rax
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}


#[test]
fn i16_to_i() -> anyhow::Result<()> {
    let code = r#"
let a: i16 = 120;
let c: i8 = (i8) a;
let b: i16 = a;
let d: i32 = (i32) a;
let e: i64 = (i64) a;
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
    ; let a: i16 = 120
    mov WORD [rbp - 2], 120
    ; let c: i8 = (i8)a
    ; Cast: (i16) -> (i8)
    mov ax, [rbp - 2]
    mov BYTE [rbp - 3], al
    ; let b: i16 = a
    mov ax, WORD [rbp - 2]
    mov WORD [rbp - 5], ax
    ; let d: i32 = (i32)a
    ; Cast: (i16) -> (i32)
    movsx eax, WORD [rbp - 2]
    mov DWORD [rbp - 9], eax
    ; let e: i64 = (i64)a
    ; Cast: (i16) -> (i64)
    movsx rax, WORD [rbp - 2]
    mov QWORD [rbp - 17], rax
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn i16_to_u() -> anyhow::Result<()> {
    let code = r#"
let a: i16 = 120;
let b: u8 = (u8) a;
let c: u16 = (u16) a;
let d: u32 = (u32) a;
let e: u64 = (u64) a;
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
    ; let a: i16 = 120
    mov WORD [rbp - 2], 120
    ; let b: u8 = (u8)a
    ; Cast: (i16) -> (u8)
    mov ax, [rbp - 2]
    mov BYTE [rbp - 3], al
    ; let c: u16 = (u16)a
    ; Cast: (i16) -> (u16)
    mov ax, WORD [rbp - 2]
    mov WORD [rbp - 5], ax
    ; let d: u32 = (u32)a
    ; Cast: (i16) -> (u32)
    movsx eax, WORD [rbp - 2]
    mov DWORD [rbp - 9], eax
    ; let e: u64 = (u64)a
    ; Cast: (i16) -> (u64)
    movsx rax, WORD [rbp - 2]
    mov QWORD [rbp - 17], rax
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}


#[test]
fn u32_to_i() -> anyhow::Result<()> {
    let code = r#"
let a: u32 = 250;
let b: i8 = (i8) a;
let c: i16 = (i16) a;
let d: i32 = (i32) a;
let e: i64 = (i64) a;
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
    ; let a: u32 = 250
    mov DWORD [rbp - 4], 250
    ; let b: i8 = (i8)a
    ; Cast: (u32) -> (i8)
    mov eax, [rbp - 4]
    mov BYTE [rbp - 5], al
    ; let c: i16 = (i16)a
    ; Cast: (u32) -> (i16)
    mov eax, [rbp - 4]
    mov WORD [rbp - 7], ax
    ; let d: i32 = (i32)a
    ; Cast: (u32) -> (i32)
    mov eax, DWORD [rbp - 4]
    mov DWORD [rbp - 11], eax
    ; let e: i64 = (i64)a
    ; Cast: (u32) -> (i64)
    mov r14d, DWORD [rbp - 4]
    xor rax, rax
    mov eax, r14d
    mov QWORD [rbp - 19], rax
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn u32_to_u() -> anyhow::Result<()> {
    let code = r#"
let a: u32 = 250;
let b: u8 = (u8) a;
let c: u16 = (u16) a;
let d: u32 = a;
let e: u64 = (u64) a;
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
    ; let a: u32 = 250
    mov DWORD [rbp - 4], 250
    ; let b: u8 = (u8)a
    ; Cast: (u32) -> (u8)
    mov eax, [rbp - 4]
    mov BYTE [rbp - 5], al
    ; let c: u16 = (u16)a
    ; Cast: (u32) -> (u16)
    mov eax, [rbp - 4]
    mov WORD [rbp - 7], ax
    ; let d: u32 = a
    mov eax, DWORD [rbp - 4]
    mov DWORD [rbp - 11], eax
    ; let e: u64 = (u64)a
    ; Cast: (u32) -> (u64)
    mov r14d, DWORD [rbp - 4]
    xor rax, rax
    mov eax, r14d
    mov QWORD [rbp - 19], rax
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}


#[test]
fn i32_to_i() -> anyhow::Result<()> {
    let code = r#"
let a: i32 = 120;
let c: i8 = (i8) a;
let b: i16 = (i16) a;
let d: i32 = a;
let e: i64 = (i64) a;
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
    ; let a: i32 = 120
    mov DWORD [rbp - 4], 120
    ; let c: i8 = (i8)a
    ; Cast: (i32) -> (i8)
    mov eax, [rbp - 4]
    mov BYTE [rbp - 5], al
    ; let b: i16 = (i16)a
    ; Cast: (i32) -> (i16)
    mov eax, [rbp - 4]
    mov WORD [rbp - 7], ax
    ; let d: i32 = a
    mov eax, DWORD [rbp - 4]
    mov DWORD [rbp - 11], eax
    ; let e: i64 = (i64)a
    ; Cast: (i32) -> (i64)
    movsx rax, DWORD [rbp - 4]
    mov QWORD [rbp - 19], rax
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn i32_to_u() -> anyhow::Result<()> {
    let code = r#"
let a: i32 = 120;
let b: u8 = (u8) a;
let c: u16 = (u16) a;
let d: u32 = (u32) a;
let e: u64 = (u64) a;
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
    ; let a: i32 = 120
    mov DWORD [rbp - 4], 120
    ; let b: u8 = (u8)a
    ; Cast: (i32) -> (u8)
    mov eax, [rbp - 4]
    mov BYTE [rbp - 5], al
    ; let c: u16 = (u16)a
    ; Cast: (i32) -> (u16)
    mov eax, [rbp - 4]
    mov WORD [rbp - 7], ax
    ; let d: u32 = (u32)a
    ; Cast: (i32) -> (u32)
    mov eax, DWORD [rbp - 4]
    mov DWORD [rbp - 11], eax
    ; let e: u64 = (u64)a
    ; Cast: (i32) -> (u64)
    mov r14d, DWORD [rbp - 4]
    xor rax, rax
    mov eax, r14d
    mov QWORD [rbp - 19], rax
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}


#[test]
fn u64_to_i() -> anyhow::Result<()> {
    let code = r#"
let a: u64 = 250;
let b: i8 = (i8) a;
let c: i16 = (i16) a;
let d: i32 = (i32) a;
let e: i64 = (i64) a;
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
    ; let a: u64 = 250
    mov QWORD [rbp - 8], 250
    ; let b: i8 = (i8)a
    ; Cast: (u64) -> (i8)
    mov rax, [rbp - 8]
    mov BYTE [rbp - 9], al
    ; let c: i16 = (i16)a
    ; Cast: (u64) -> (i16)
    mov rax, [rbp - 8]
    mov WORD [rbp - 11], ax
    ; let d: i32 = (i32)a
    ; Cast: (u64) -> (i32)
    mov rax, [rbp - 8]
    mov DWORD [rbp - 15], eax
    ; let e: i64 = (i64)a
    ; Cast: (u64) -> (i64)
    mov rax, QWORD [rbp - 8]
    mov QWORD [rbp - 23], rax
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn u64_to_u() -> anyhow::Result<()> {
    let code = r#"
let a: u64 = 250;
let b: u8 = (u8) a;
let c: u16 = (u16) a;
let d: u32 = (u32) a;
let e: u64 = a;
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
    ; let a: u64 = 250
    mov QWORD [rbp - 8], 250
    ; let b: u8 = (u8)a
    ; Cast: (u64) -> (u8)
    mov rax, [rbp - 8]
    mov BYTE [rbp - 9], al
    ; let c: u16 = (u16)a
    ; Cast: (u64) -> (u16)
    mov rax, [rbp - 8]
    mov WORD [rbp - 11], ax
    ; let d: u32 = (u32)a
    ; Cast: (u64) -> (u32)
    mov rax, [rbp - 8]
    mov DWORD [rbp - 15], eax
    ; let e: u64 = a
    mov rax, QWORD [rbp - 8]
    mov QWORD [rbp - 23], rax
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}


#[test]
fn i64_to_i() -> anyhow::Result<()> {
    let code = r#"
let a: i64 = 120;
let c: i8 = (i8) a;
let b: i16 = (i16) a;
let d: i32 = (i32) a;
let e: i64 = a;
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
    ; let a: i64 = 120
    mov QWORD [rbp - 8], 120
    ; let c: i8 = (i8)a
    ; Cast: (i64) -> (i8)
    mov rax, [rbp - 8]
    mov BYTE [rbp - 9], al
    ; let b: i16 = (i16)a
    ; Cast: (i64) -> (i16)
    mov rax, [rbp - 8]
    mov WORD [rbp - 11], ax
    ; let d: i32 = (i32)a
    ; Cast: (i64) -> (i32)
    mov rax, [rbp - 8]
    mov DWORD [rbp - 15], eax
    ; let e: i64 = a
    mov rax, QWORD [rbp - 8]
    mov QWORD [rbp - 23], rax
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn i64_to_u() -> anyhow::Result<()> {
    let code = r#"
let a: i64 = 120;
let b: u8 = (u8) a;
let c: u16 = (u16) a;
let d: u32 = (u32) a;
let e: u64 = (u64) a;
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
    ; let a: i64 = 120
    mov QWORD [rbp - 8], 120
    ; let b: u8 = (u8)a
    ; Cast: (i64) -> (u8)
    mov rax, [rbp - 8]
    mov BYTE [rbp - 9], al
    ; let c: u16 = (u16)a
    ; Cast: (i64) -> (u16)
    mov rax, [rbp - 8]
    mov WORD [rbp - 11], ax
    ; let d: u32 = (u32)a
    ; Cast: (i64) -> (u32)
    mov rax, [rbp - 8]
    mov DWORD [rbp - 15], eax
    ; let e: u64 = (u64)a
    ; Cast: (i64) -> (u64)
    mov rax, QWORD [rbp - 8]
    mov QWORD [rbp - 23], rax
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}
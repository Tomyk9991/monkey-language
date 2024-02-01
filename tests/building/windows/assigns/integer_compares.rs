use monkey_language::core::code_generator::generator::ASMGenerator;
use monkey_language::core::code_generator::target_os::TargetOS;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::type_checker::static_type_checker::static_type_check;

#[test]
fn simple_compare_u8() -> anyhow::Result<()> {
    let code = r#"
    let a: u8 = 5;
    let b: u8 = 3;

    let c: bool = a > b;
    let d: bool = 5 > 3;

    let e: bool = a < b;
    let f: bool = 5 < 3;

    let g: bool = a <= b;
    let h: bool = 5 <= 3;

    let i: bool = a >= b;
    let j: bool = 5 >= 3;

    let k: bool = a == b;
    let l: bool = 5 == 3;

    let m: bool = a != b;
    let n: bool = 5 != 3;
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
    sub rsp, 46
    ; let a: u8 = 5
    mov BYTE [rbp - 1], 5
    ; let b: u8 = 3
    mov BYTE [rbp - 2], 3
    ; let c: bool = (a > b)
    ; (a > b)
    mov al, BYTE [rbp - 1]
    cmp al, BYTE [rbp - 2]
    setg al
    mov BYTE [rbp - 3], al
    ; let d: bool = (5 > 3)
    ; (5 > 3)
    mov eax, 5
    cmp eax, 3
    setg al
    mov BYTE [rbp - 4], al
    ; let e: bool = (a < b)
    ; (a < b)
    mov al, BYTE [rbp - 1]
    cmp al, BYTE [rbp - 2]
    setl al
    mov BYTE [rbp - 5], al
    ; let f: bool = (5 < 3)
    ; (5 < 3)
    mov eax, 5
    cmp eax, 3
    setl al
    mov BYTE [rbp - 6], al
    ; let g: bool = (a <= b)
    ; (a <= b)
    mov al, BYTE [rbp - 1]
    cmp al, BYTE [rbp - 2]
    setle al
    mov BYTE [rbp - 7], al
    ; let h: bool = (5 <= 3)
    ; (5 <= 3)
    mov eax, 5
    cmp eax, 3
    setle al
    mov BYTE [rbp - 8], al
    ; let i: bool = (a >= b)
    ; (a >= b)
    mov al, BYTE [rbp - 1]
    cmp al, BYTE [rbp - 2]
    setge al
    mov BYTE [rbp - 9], al
    ; let j: bool = (5 >= 3)
    ; (5 >= 3)
    mov eax, 5
    cmp eax, 3
    setge al
    mov BYTE [rbp - 10], al
    ; let k: bool = (a == b)
    ; (a == b)
    mov al, BYTE [rbp - 1]
    cmp al, BYTE [rbp - 2]
    sete al
    mov BYTE [rbp - 11], al
    ; let l: bool = (5 == 3)
    ; (5 == 3)
    mov eax, 5
    cmp eax, 3
    sete al
    mov BYTE [rbp - 12], al
    ; let m: bool = (a != b)
    ; (a != b)
    mov al, BYTE [rbp - 1]
    cmp al, BYTE [rbp - 2]
    setne al
    mov BYTE [rbp - 13], al
    ; let n: bool = (5 != 3)
    ; (5 != 3)
    mov eax, 5
    cmp eax, 3
    setne al
    mov BYTE [rbp - 14], al
    leave
    ret
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}


#[test]
fn simple_compare_i8() -> anyhow::Result<()> {
    let code = r#"
    let a: i8 = 5;
    let b: i8 = 3;

    let c: bool = a > b;
    let d: bool = 5 > 3;

    let e: bool = a < b;
    let f: bool = 5 < 3;

    let g: bool = a <= b;
    let h: bool = 5 <= 3;

    let i: bool = a >= b;
    let j: bool = 5 >= 3;

    let k: bool = a == b;
    let l: bool = 5 == 3;

    let m: bool = a != b;
    let n: bool = 5 != 3;
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
    sub rsp, 46
    ; let a: i8 = 5
    mov BYTE [rbp - 1], 5
    ; let b: i8 = 3
    mov BYTE [rbp - 2], 3
    ; let c: bool = (a > b)
    ; (a > b)
    mov al, BYTE [rbp - 1]
    cmp al, BYTE [rbp - 2]
    setg al
    mov BYTE [rbp - 3], al
    ; let d: bool = (5 > 3)
    ; (5 > 3)
    mov eax, 5
    cmp eax, 3
    setg al
    mov BYTE [rbp - 4], al
    ; let e: bool = (a < b)
    ; (a < b)
    mov al, BYTE [rbp - 1]
    cmp al, BYTE [rbp - 2]
    setl al
    mov BYTE [rbp - 5], al
    ; let f: bool = (5 < 3)
    ; (5 < 3)
    mov eax, 5
    cmp eax, 3
    setl al
    mov BYTE [rbp - 6], al
    ; let g: bool = (a <= b)
    ; (a <= b)
    mov al, BYTE [rbp - 1]
    cmp al, BYTE [rbp - 2]
    setle al
    mov BYTE [rbp - 7], al
    ; let h: bool = (5 <= 3)
    ; (5 <= 3)
    mov eax, 5
    cmp eax, 3
    setle al
    mov BYTE [rbp - 8], al
    ; let i: bool = (a >= b)
    ; (a >= b)
    mov al, BYTE [rbp - 1]
    cmp al, BYTE [rbp - 2]
    setge al
    mov BYTE [rbp - 9], al
    ; let j: bool = (5 >= 3)
    ; (5 >= 3)
    mov eax, 5
    cmp eax, 3
    setge al
    mov BYTE [rbp - 10], al
    ; let k: bool = (a == b)
    ; (a == b)
    mov al, BYTE [rbp - 1]
    cmp al, BYTE [rbp - 2]
    sete al
    mov BYTE [rbp - 11], al
    ; let l: bool = (5 == 3)
    ; (5 == 3)
    mov eax, 5
    cmp eax, 3
    sete al
    mov BYTE [rbp - 12], al
    ; let m: bool = (a != b)
    ; (a != b)
    mov al, BYTE [rbp - 1]
    cmp al, BYTE [rbp - 2]
    setne al
    mov BYTE [rbp - 13], al
    ; let n: bool = (5 != 3)
    ; (5 != 3)
    mov eax, 5
    cmp eax, 3
    setne al
    mov BYTE [rbp - 14], al
    leave
    ret
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn simple_compare_u16() -> anyhow::Result<()> {
    let code = r#"
    let a: u16 = 5;
    let b: u16 = 3;

    let c: bool = a > b;
    let d: bool = 5 > 3;

    let e: bool = a < b;
    let f: bool = 5 < 3;

    let g: bool = a <= b;
    let h: bool = 5 <= 3;

    let i: bool = a >= b;
    let j: bool = 5 >= 3;

    let k: bool = a == b;
    let l: bool = 5 == 3;

    let m: bool = a != b;
    let n: bool = 5 != 3;
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
    sub rsp, 48
    ; let a: u16 = 5
    mov WORD [rbp - 2], 5
    ; let b: u16 = 3
    mov WORD [rbp - 4], 3
    ; let c: bool = (a > b)
    ; (a > b)
    mov ax, WORD [rbp - 2]
    cmp ax, WORD [rbp - 4]
    setg al
    mov BYTE [rbp - 5], al
    ; let d: bool = (5 > 3)
    ; (5 > 3)
    mov eax, 5
    cmp eax, 3
    setg al
    mov BYTE [rbp - 6], al
    ; let e: bool = (a < b)
    ; (a < b)
    mov ax, WORD [rbp - 2]
    cmp ax, WORD [rbp - 4]
    setl al
    mov BYTE [rbp - 7], al
    ; let f: bool = (5 < 3)
    ; (5 < 3)
    mov eax, 5
    cmp eax, 3
    setl al
    mov BYTE [rbp - 8], al
    ; let g: bool = (a <= b)
    ; (a <= b)
    mov ax, WORD [rbp - 2]
    cmp ax, WORD [rbp - 4]
    setle al
    mov BYTE [rbp - 9], al
    ; let h: bool = (5 <= 3)
    ; (5 <= 3)
    mov eax, 5
    cmp eax, 3
    setle al
    mov BYTE [rbp - 10], al
    ; let i: bool = (a >= b)
    ; (a >= b)
    mov ax, WORD [rbp - 2]
    cmp ax, WORD [rbp - 4]
    setge al
    mov BYTE [rbp - 11], al
    ; let j: bool = (5 >= 3)
    ; (5 >= 3)
    mov eax, 5
    cmp eax, 3
    setge al
    mov BYTE [rbp - 12], al
    ; let k: bool = (a == b)
    ; (a == b)
    mov ax, WORD [rbp - 2]
    cmp ax, WORD [rbp - 4]
    sete al
    mov BYTE [rbp - 13], al
    ; let l: bool = (5 == 3)
    ; (5 == 3)
    mov eax, 5
    cmp eax, 3
    sete al
    mov BYTE [rbp - 14], al
    ; let m: bool = (a != b)
    ; (a != b)
    mov ax, WORD [rbp - 2]
    cmp ax, WORD [rbp - 4]
    setne al
    mov BYTE [rbp - 15], al
    ; let n: bool = (5 != 3)
    ; (5 != 3)
    mov eax, 5
    cmp eax, 3
    setne al
    mov BYTE [rbp - 16], al
    leave
    ret
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn simple_compare_i16() -> anyhow::Result<()> {
    let code = r#"
    let a: i16 = 5;
    let b: i16 = 3;

    let c: bool = a > b;
    let d: bool = 5 > 3;

    let e: bool = a < b;
    let f: bool = 5 < 3;

    let g: bool = a <= b;
    let h: bool = 5 <= 3;

    let i: bool = a >= b;
    let j: bool = 5 >= 3;

    let k: bool = a == b;
    let l: bool = 5 == 3;

    let m: bool = a != b;
    let n: bool = 5 != 3;
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
    sub rsp, 48
    ; let a: i16 = 5
    mov WORD [rbp - 2], 5
    ; let b: i16 = 3
    mov WORD [rbp - 4], 3
    ; let c: bool = (a > b)
    ; (a > b)
    mov ax, WORD [rbp - 2]
    cmp ax, WORD [rbp - 4]
    setg al
    mov BYTE [rbp - 5], al
    ; let d: bool = (5 > 3)
    ; (5 > 3)
    mov eax, 5
    cmp eax, 3
    setg al
    mov BYTE [rbp - 6], al
    ; let e: bool = (a < b)
    ; (a < b)
    mov ax, WORD [rbp - 2]
    cmp ax, WORD [rbp - 4]
    setl al
    mov BYTE [rbp - 7], al
    ; let f: bool = (5 < 3)
    ; (5 < 3)
    mov eax, 5
    cmp eax, 3
    setl al
    mov BYTE [rbp - 8], al
    ; let g: bool = (a <= b)
    ; (a <= b)
    mov ax, WORD [rbp - 2]
    cmp ax, WORD [rbp - 4]
    setle al
    mov BYTE [rbp - 9], al
    ; let h: bool = (5 <= 3)
    ; (5 <= 3)
    mov eax, 5
    cmp eax, 3
    setle al
    mov BYTE [rbp - 10], al
    ; let i: bool = (a >= b)
    ; (a >= b)
    mov ax, WORD [rbp - 2]
    cmp ax, WORD [rbp - 4]
    setge al
    mov BYTE [rbp - 11], al
    ; let j: bool = (5 >= 3)
    ; (5 >= 3)
    mov eax, 5
    cmp eax, 3
    setge al
    mov BYTE [rbp - 12], al
    ; let k: bool = (a == b)
    ; (a == b)
    mov ax, WORD [rbp - 2]
    cmp ax, WORD [rbp - 4]
    sete al
    mov BYTE [rbp - 13], al
    ; let l: bool = (5 == 3)
    ; (5 == 3)
    mov eax, 5
    cmp eax, 3
    sete al
    mov BYTE [rbp - 14], al
    ; let m: bool = (a != b)
    ; (a != b)
    mov ax, WORD [rbp - 2]
    cmp ax, WORD [rbp - 4]
    setne al
    mov BYTE [rbp - 15], al
    ; let n: bool = (5 != 3)
    ; (5 != 3)
    mov eax, 5
    cmp eax, 3
    setne al
    mov BYTE [rbp - 16], al
    leave
    ret
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn simple_compare_u32() -> anyhow::Result<()> {
    let code = r#"
    let a: u32 = 5;
    let b: u32 = 3;

    let c: bool = a > b;
    let d: bool = 5 > 3;

    let e: bool = a < b;
    let f: bool = 5 < 3;

    let g: bool = a <= b;
    let h: bool = 5 <= 3;

    let i: bool = a >= b;
    let j: bool = 5 >= 3;

    let k: bool = a == b;
    let l: bool = 5 == 3;

    let m: bool = a != b;
    let n: bool = 5 != 3;
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
    sub rsp, 52
    ; let a: u32 = 5
    mov DWORD [rbp - 4], 5
    ; let b: u32 = 3
    mov DWORD [rbp - 8], 3
    ; let c: bool = (a > b)
    ; (a > b)
    mov eax, DWORD [rbp - 4]
    cmp eax, DWORD [rbp - 8]
    setg al
    mov BYTE [rbp - 9], al
    ; let d: bool = (5 > 3)
    ; (5 > 3)
    mov eax, 5
    cmp eax, 3
    setg al
    mov BYTE [rbp - 10], al
    ; let e: bool = (a < b)
    ; (a < b)
    mov eax, DWORD [rbp - 4]
    cmp eax, DWORD [rbp - 8]
    setl al
    mov BYTE [rbp - 11], al
    ; let f: bool = (5 < 3)
    ; (5 < 3)
    mov eax, 5
    cmp eax, 3
    setl al
    mov BYTE [rbp - 12], al
    ; let g: bool = (a <= b)
    ; (a <= b)
    mov eax, DWORD [rbp - 4]
    cmp eax, DWORD [rbp - 8]
    setle al
    mov BYTE [rbp - 13], al
    ; let h: bool = (5 <= 3)
    ; (5 <= 3)
    mov eax, 5
    cmp eax, 3
    setle al
    mov BYTE [rbp - 14], al
    ; let i: bool = (a >= b)
    ; (a >= b)
    mov eax, DWORD [rbp - 4]
    cmp eax, DWORD [rbp - 8]
    setge al
    mov BYTE [rbp - 15], al
    ; let j: bool = (5 >= 3)
    ; (5 >= 3)
    mov eax, 5
    cmp eax, 3
    setge al
    mov BYTE [rbp - 16], al
    ; let k: bool = (a == b)
    ; (a == b)
    mov eax, DWORD [rbp - 4]
    cmp eax, DWORD [rbp - 8]
    sete al
    mov BYTE [rbp - 17], al
    ; let l: bool = (5 == 3)
    ; (5 == 3)
    mov eax, 5
    cmp eax, 3
    sete al
    mov BYTE [rbp - 18], al
    ; let m: bool = (a != b)
    ; (a != b)
    mov eax, DWORD [rbp - 4]
    cmp eax, DWORD [rbp - 8]
    setne al
    mov BYTE [rbp - 19], al
    ; let n: bool = (5 != 3)
    ; (5 != 3)
    mov eax, 5
    cmp eax, 3
    setne al
    mov BYTE [rbp - 20], al
    leave
    ret
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn simple_compare_i32() -> anyhow::Result<()> {
    let code = r#"
    let a: i32 = 5;
    let b: i32 = 3;

    let c: bool = a > b;
    let d: bool = 5 > 3;

    let e: bool = a < b;
    let f: bool = 5 < 3;

    let g: bool = a <= b;
    let h: bool = 5 <= 3;

    let i: bool = a >= b;
    let j: bool = 5 >= 3;

    let k: bool = a == b;
    let l: bool = 5 == 3;

    let m: bool = a != b;
    let n: bool = 5 != 3;
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
    sub rsp, 52
    ; let a: i32 = 5
    mov DWORD [rbp - 4], 5
    ; let b: i32 = 3
    mov DWORD [rbp - 8], 3
    ; let c: bool = (a > b)
    ; (a > b)
    mov eax, DWORD [rbp - 4]
    cmp eax, DWORD [rbp - 8]
    setg al
    mov BYTE [rbp - 9], al
    ; let d: bool = (5 > 3)
    ; (5 > 3)
    mov eax, 5
    cmp eax, 3
    setg al
    mov BYTE [rbp - 10], al
    ; let e: bool = (a < b)
    ; (a < b)
    mov eax, DWORD [rbp - 4]
    cmp eax, DWORD [rbp - 8]
    setl al
    mov BYTE [rbp - 11], al
    ; let f: bool = (5 < 3)
    ; (5 < 3)
    mov eax, 5
    cmp eax, 3
    setl al
    mov BYTE [rbp - 12], al
    ; let g: bool = (a <= b)
    ; (a <= b)
    mov eax, DWORD [rbp - 4]
    cmp eax, DWORD [rbp - 8]
    setle al
    mov BYTE [rbp - 13], al
    ; let h: bool = (5 <= 3)
    ; (5 <= 3)
    mov eax, 5
    cmp eax, 3
    setle al
    mov BYTE [rbp - 14], al
    ; let i: bool = (a >= b)
    ; (a >= b)
    mov eax, DWORD [rbp - 4]
    cmp eax, DWORD [rbp - 8]
    setge al
    mov BYTE [rbp - 15], al
    ; let j: bool = (5 >= 3)
    ; (5 >= 3)
    mov eax, 5
    cmp eax, 3
    setge al
    mov BYTE [rbp - 16], al
    ; let k: bool = (a == b)
    ; (a == b)
    mov eax, DWORD [rbp - 4]
    cmp eax, DWORD [rbp - 8]
    sete al
    mov BYTE [rbp - 17], al
    ; let l: bool = (5 == 3)
    ; (5 == 3)
    mov eax, 5
    cmp eax, 3
    sete al
    mov BYTE [rbp - 18], al
    ; let m: bool = (a != b)
    ; (a != b)
    mov eax, DWORD [rbp - 4]
    cmp eax, DWORD [rbp - 8]
    setne al
    mov BYTE [rbp - 19], al
    ; let n: bool = (5 != 3)
    ; (5 != 3)
    mov eax, 5
    cmp eax, 3
    setne al
    mov BYTE [rbp - 20], al
    leave
    ret
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn simple_compare_u64() -> anyhow::Result<()> {
    let code = r#"
    let a: u64 = 5;
    let b: u64 = 3;

    let c: bool = a > b;
    let d: bool = 5 > 3;

    let e: bool = a < b;
    let f: bool = 5 < 3;

    let g: bool = a <= b;
    let h: bool = 5 <= 3;

    let i: bool = a >= b;
    let j: bool = 5 >= 3;

    let k: bool = a == b;
    let l: bool = 5 == 3;

    let m: bool = a != b;
    let n: bool = 5 != 3;
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
    sub rsp, 60
    ; let a: u64 = 5
    mov QWORD [rbp - 8], 5
    ; let b: u64 = 3
    mov QWORD [rbp - 16], 3
    ; let c: bool = (a > b)
    ; (a > b)
    mov rax, QWORD [rbp - 8]
    cmp rax, QWORD [rbp - 16]
    setg al
    mov BYTE [rbp - 17], al
    ; let d: bool = (5 > 3)
    ; (5 > 3)
    mov eax, 5
    cmp eax, 3
    setg al
    mov BYTE [rbp - 18], al
    ; let e: bool = (a < b)
    ; (a < b)
    mov rax, QWORD [rbp - 8]
    cmp rax, QWORD [rbp - 16]
    setl al
    mov BYTE [rbp - 19], al
    ; let f: bool = (5 < 3)
    ; (5 < 3)
    mov eax, 5
    cmp eax, 3
    setl al
    mov BYTE [rbp - 20], al
    ; let g: bool = (a <= b)
    ; (a <= b)
    mov rax, QWORD [rbp - 8]
    cmp rax, QWORD [rbp - 16]
    setle al
    mov BYTE [rbp - 21], al
    ; let h: bool = (5 <= 3)
    ; (5 <= 3)
    mov eax, 5
    cmp eax, 3
    setle al
    mov BYTE [rbp - 22], al
    ; let i: bool = (a >= b)
    ; (a >= b)
    mov rax, QWORD [rbp - 8]
    cmp rax, QWORD [rbp - 16]
    setge al
    mov BYTE [rbp - 23], al
    ; let j: bool = (5 >= 3)
    ; (5 >= 3)
    mov eax, 5
    cmp eax, 3
    setge al
    mov BYTE [rbp - 24], al
    ; let k: bool = (a == b)
    ; (a == b)
    mov rax, QWORD [rbp - 8]
    cmp rax, QWORD [rbp - 16]
    sete al
    mov BYTE [rbp - 25], al
    ; let l: bool = (5 == 3)
    ; (5 == 3)
    mov eax, 5
    cmp eax, 3
    sete al
    mov BYTE [rbp - 26], al
    ; let m: bool = (a != b)
    ; (a != b)
    mov rax, QWORD [rbp - 8]
    cmp rax, QWORD [rbp - 16]
    setne al
    mov BYTE [rbp - 27], al
    ; let n: bool = (5 != 3)
    ; (5 != 3)
    mov eax, 5
    cmp eax, 3
    setne al
    mov BYTE [rbp - 28], al
    leave
    ret
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn simple_compare_i64() -> anyhow::Result<()> {
    let code = r#"
    let a: i64 = 5;
    let b: i64 = 3;

    let c: bool = a > b;
    let d: bool = 5 > 3;

    let e: bool = a < b;
    let f: bool = 5 < 3;

    let g: bool = a <= b;
    let h: bool = 5 <= 3;

    let i: bool = a >= b;
    let j: bool = 5 >= 3;

    let k: bool = a == b;
    let l: bool = 5 == 3;

    let m: bool = a != b;
    let n: bool = 5 != 3;
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
    sub rsp, 60
    ; let a: i64 = 5
    mov QWORD [rbp - 8], 5
    ; let b: i64 = 3
    mov QWORD [rbp - 16], 3
    ; let c: bool = (a > b)
    ; (a > b)
    mov rax, QWORD [rbp - 8]
    cmp rax, QWORD [rbp - 16]
    setg al
    mov BYTE [rbp - 17], al
    ; let d: bool = (5 > 3)
    ; (5 > 3)
    mov eax, 5
    cmp eax, 3
    setg al
    mov BYTE [rbp - 18], al
    ; let e: bool = (a < b)
    ; (a < b)
    mov rax, QWORD [rbp - 8]
    cmp rax, QWORD [rbp - 16]
    setl al
    mov BYTE [rbp - 19], al
    ; let f: bool = (5 < 3)
    ; (5 < 3)
    mov eax, 5
    cmp eax, 3
    setl al
    mov BYTE [rbp - 20], al
    ; let g: bool = (a <= b)
    ; (a <= b)
    mov rax, QWORD [rbp - 8]
    cmp rax, QWORD [rbp - 16]
    setle al
    mov BYTE [rbp - 21], al
    ; let h: bool = (5 <= 3)
    ; (5 <= 3)
    mov eax, 5
    cmp eax, 3
    setle al
    mov BYTE [rbp - 22], al
    ; let i: bool = (a >= b)
    ; (a >= b)
    mov rax, QWORD [rbp - 8]
    cmp rax, QWORD [rbp - 16]
    setge al
    mov BYTE [rbp - 23], al
    ; let j: bool = (5 >= 3)
    ; (5 >= 3)
    mov eax, 5
    cmp eax, 3
    setge al
    mov BYTE [rbp - 24], al
    ; let k: bool = (a == b)
    ; (a == b)
    mov rax, QWORD [rbp - 8]
    cmp rax, QWORD [rbp - 16]
    sete al
    mov BYTE [rbp - 25], al
    ; let l: bool = (5 == 3)
    ; (5 == 3)
    mov eax, 5
    cmp eax, 3
    sete al
    mov BYTE [rbp - 26], al
    ; let m: bool = (a != b)
    ; (a != b)
    mov rax, QWORD [rbp - 8]
    cmp rax, QWORD [rbp - 16]
    setne al
    mov BYTE [rbp - 27], al
    ; let n: bool = (5 != 3)
    ; (5 != 3)
    mov eax, 5
    cmp eax, 3
    setne al
    mov BYTE [rbp - 28], al
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
    let a = 3 == 3 && 7 != 9;
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
    sub rsp, 33
    ; let a: bool = ((3 == 3) && (7 != 9))
    ; ((3 == 3) && (7 != 9))
    ; (3 == 3)
    mov eax, 3
    cmp eax, 3
    sete al
    mov ch, al
    ; (7 != 9)
    mov eax, 7
    cmp eax, 9
    setne al
    mov dil, al
    mov r14b, dl
    mov r13b, al
    mov r12b, cl
    mov cl, dil
    mov al, ch
    mov dl, 0
    cmp ch, 0
    je .label0
    mov al, dil
    cmp al, 0
    je .label0
    mov eax, 1
    jmp .label1
.label0:
    mov eax, 0
.label1:
    mov ch, al
    mov dl, r14b
    mov al, r13b
    mov cl, r12b
    mov al, ch
    mov BYTE [rbp - 1], al
    leave
    ret
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn compare_complex() -> anyhow::Result<()> {
    let code = r#"
    let a: i32 = 5;
    let b: i32 = 3;
    let c: i32 = 7;
    let d: i32 = 9;

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
    sub rsp, 49
    ; let a: i32 = 5
    mov DWORD [rbp - 4], 5
    ; let b: i32 = 3
    mov DWORD [rbp - 8], 3
    ; let c: i32 = 7
    mov DWORD [rbp - 12], 7
    ; let d: i32 = 9
    mov DWORD [rbp - 16], 9
    ; let result: bool = ((((a == b) && (c != d)) && (a >= b)) || (((c <= d) && (a < b)) && (c > d)))
    ; ((((a == b) && (c != d)) && (a >= b)) || (((c <= d) && (a < b)) && (c > d)))
    ; (((a == b) && (c != d)) && (a >= b))
    ; ((a == b) && (c != d))
    ; (a == b)
    mov eax, DWORD [rbp - 4]
    cmp eax, DWORD [rbp - 8]
    sete al
    mov ch, al
    ; (c != d)
    mov eax, DWORD [rbp - 12]
    cmp eax, DWORD [rbp - 16]
    setne al
    mov dil, al
    mov r14b, dl
    mov r13b, al
    mov r12b, cl
    mov cl, dil
    mov al, ch
    mov dl, 0
    cmp ch, 0
    je .label0
    mov al, dil
    cmp al, 0
    je .label0
    mov eax, 1
    jmp .label1
.label0:
    mov eax, 0
.label1:
    mov ch, al
    mov dl, r14b
    mov al, r13b
    mov cl, r12b
    mov al, ch
    push rax
    xor rax, rax
    ; (a >= b)
    mov eax, DWORD [rbp - 4]
    cmp eax, DWORD [rbp - 8]
    setge al
    push rax
    xor rax, rax
    pop rdi
    pop rax
    mov r14b, dl
    mov r13b, al
    mov r12b, cl
    mov cl, dil
    mov dl, 0
    cmp al, 0
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
    push rax
    xor rax, rax
    ; (((c <= d) && (a < b)) && (c > d))
    ; ((c <= d) && (a < b))
    ; (c <= d)
    mov eax, DWORD [rbp - 12]
    cmp eax, DWORD [rbp - 16]
    setle al
    mov ch, al
    ; (a < b)
    mov eax, DWORD [rbp - 4]
    cmp eax, DWORD [rbp - 8]
    setl al
    mov dil, al
    mov r14b, dl
    mov r13b, al
    mov r12b, cl
    mov cl, dil
    mov al, ch
    mov dl, 0
    cmp ch, 0
    je .label4
    mov al, dil
    cmp al, 0
    je .label4
    mov eax, 1
    jmp .label5
.label4:
    mov eax, 0
.label5:
    mov ch, al
    mov dl, r14b
    mov al, r13b
    mov cl, r12b
    mov al, ch
    push rax
    xor rax, rax
    ; (c > d)
    mov eax, DWORD [rbp - 12]
    cmp eax, DWORD [rbp - 16]
    setg al
    push rax
    xor rax, rax
    pop rdi
    pop rax
    mov r14b, dl
    mov r13b, al
    mov r12b, cl
    mov cl, dil
    mov dl, 0
    cmp al, 0
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
    mov cl, dil
    mov dl, 0
    cmp al, 0
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
    leave
    ret
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}
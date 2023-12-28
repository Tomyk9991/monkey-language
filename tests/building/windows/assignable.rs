use monkey_language::core::code_generator::generator::{ASMGenerator, Stack};
use monkey_language::core::code_generator::MetaInfo;
use monkey_language::core::code_generator::target_os::TargetOS;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::type_checker::static_type_checker::static_type_check;
use monkey_language::core::code_generator::ToASM;
use monkey_language::core::lexer::token::Token;
use monkey_language::core::lexer::tokens::assignable_token::AssignableToken;

#[test]
fn string_assign() -> anyhow::Result<()> {
    let code = r#"
    let a: *string = "Hallo";
    "#;

    let asm_result = asm_from_assign_code(&code)?;

    let expected = r#"
.label0:
    db "Hallo", 0
    ; let a: *string = "Hallo"
    mov QWORD [rbp - 8], .label0
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn expression_assign() -> anyhow::Result<()> {
    let code = r#"
    let a: i32 = 5 + 3;
    "#;

    let asm_result = asm_from_assign_code(&code)?;


    println!("{}", asm_result);

    let expected = r#"
; let a: i32 = (5 Add 3)
    ; (5 Add 3)
    mov eax, 5
    add eax, 3
    mov eax, eax
    mov DWORD [rbp - 4], eax
    "#;

    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: i32 = (5 + 2) + 8;
    "#;

    let asm_result = asm_from_assign_code(&code)?;


    let expected = r#"
; let a: i32 = ((5 Add 2) Add 8)
    ; ((5 Add 2) Add 8)
    ; (5 Add 2)
    mov eax, 5
    add eax, 2
    mov eax, eax
    add edx, 8
    mov DWORD [rbp - 4], eax
    "#;

    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: i32 = 5 + (2 + 8);
    "#;

    let asm_result = asm_from_assign_code(&code)?;


    let expected = r#"
    ; let a: i32 = (5 Add (2 Add 8))
    ; (5 Add (2 Add 8))
    ; (2 Add 8)
    mov eax, 2
    add eax, 8
    mov eax, eax
    add eax, 5
    mov DWORD [rbp - 4], eax
    "#;

    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: i32 = (5 + 3) + (2 + 8);
    "#;

    let asm_result = asm_from_assign_code(&code)?;


    let expected = r#"
    ; let a: i32 = ((5 Add 3) Add (2 Add 8))
    ; ((5 Add 3) Add (2 Add 8))
    ; (5 Add 3)
    mov eax, 5
    add eax, 3
    mov edx, eax
    ; (2 Add 8)
    mov eax, 2
    add eax, 8
    mov eax, eax
    add eax, edx
    mov DWORD [rbp - 4], eax
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: i32 = 6;
    "#;

    let asm_result = asm_from_assign_code(&code)?;


    let expected = r#"
    ; let a: i32 = 6
    mov DWORD [rbp - 4], 6
    "#;

    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: i32 = (6);
    "#;

    let asm_result = asm_from_assign_code(&code)?;


    let expected = r#"
    ; let a: i32 = 6
    mov eax, 6
    mov DWORD [rbp - 4], eax
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn pointer_assign_test() -> anyhow::Result<()> {
    let code = r#"
    let a: i32 = 5;
    let b: *i32 = &a;
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    static_type_check(&top_level_scope)?;

    let mut code_generator = ASMGenerator::from((top_level_scope, TargetOS::Windows));
    let asm_result = code_generator.generate()?;

    println!("\n{asm_result}");

    let expected = r#"
; This assembly is targeted for the Windows Operating System
segment .text
global main


main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 44
    ; let a: i32 = 5
    mov DWORD [rbp - 4], 5
    ; let b: *i32 = &a
    lea rax, [rbp - 4]
    mov QWORD [rbp - 12], rax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}


#[test]
fn pointer_assign_multiple_test() -> anyhow::Result<()> {
    let code = r#"
extern fn printf(format: *string, value: i32): void;
extern fn ExitProcess(exitCode: i32): void;

let a: i32 = 5;
let b: *i32 = &a;
let c: **i32 = &b;
let d: *i32 = *c;

let ref: **i32 = c;
let f: i32 = *d;
let g: i32 = **c;

let format: *string = "Das ist ein Test %d";
printf(format, *b);

ExitProcess(*b);
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

extern printf
extern ExitProcess

.label0:
    db "Das ist ein Test %d", 0
main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 84
    ; let a: i32 = 5
    mov DWORD [rbp - 4], 5
    ; let b: *i32 = &a
    lea rax, [rbp - 4]
    mov QWORD [rbp - 12], rax
    ; let c: **i32 = &b
    lea rax, [rbp - 12]
    mov QWORD [rbp - 20], rax
    ; let d: *i32 = *c
    mov rax, QWORD [rbp - 20]
    mov QWORD [rbp - 28], rax
    ; let ref: **i32 = c
    mov rax, QWORD [rbp - 20]
    mov QWORD [rbp - 36], rax
    ; let f: i32 = *d
    mov rax, QWORD [rbp - 28]
    mov DWORD [rbp - 40], eax
    ; let g: i32 = **c
    mov rax, QWORD [rbp - 20]
    mov rax, QWORD [rax]
    mov DWORD [rbp - 44], eax
    ; let format: *string = "Das ist ein Test %d"
    mov QWORD [rbp - 52], .label0
    mov rcx, QWORD [rbp - 52] ; Parameter (format)
    mov rax, QWORD [rbp - 12]
    mov rax, QWORD [rax]
    mov rdx, rax ; Parameter (*b)
    ; printf(format, *b)
    call printf
    mov rax, QWORD [rbp - 12]
    mov rax, QWORD [rax]
    mov rcx, rax ; Parameter (*b)
    ; ExitProcess(*b)
    call ExitProcess
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}


#[test]
fn single_expression_test() -> anyhow::Result<()> {
    let code = r#"
    let a: i32 = 5;
    let b: i32 = a;
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
    sub rsp, 40
    ; let a: i32 = 5
    mov DWORD [rbp - 4], 5
    ; let b: i32 = a
    mov eax, DWORD [rbp - 4]
    mov DWORD [rbp - 8], eax
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: i32 = 5;
    let b: i32 = (a);
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    static_type_check(&top_level_scope)?;

    let mut code_generator = ASMGenerator::from((top_level_scope, TargetOS::Windows));

    let asm_result = code_generator.generate()?;

    assert_eq!(expected.trim(), asm_result.trim());

    Ok(())
}

#[test]
fn i32_assign() -> anyhow::Result<()> {
    let code = r#"
    let a: i32 = 512;
    "#;

    let asm_result = asm_from_assign_code(&code)?;

    let expected = r#"
; let a: i32 = 512
    mov DWORD [rbp - 4], 512
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn full_program_assignable() -> anyhow::Result<()> {
    let code = r#"
    let a: *string = "Testing string";
    let b: i32 = 512;
    let c = b;
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    static_type_check(&top_level_scope)?;
    let mut code_generator = ASMGenerator::from((top_level_scope, TargetOS::Windows));
    let asm_result = String::from(code_generator.generate()?.trim());

    println!("{}", asm_result);

    let expected = r#"; This assembly is targeted for the Windows Operating System
segment .text
global main


.label0:
    db "Testing string", 0
main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 48
    ; let a: *string = "Testing string"
    mov QWORD [rbp - 8], .label0
    ; let b: i32 = 512
    mov DWORD [rbp - 12], 512
    ; let c: i32 = b
    mov eax, DWORD [rbp - 12]
    mov DWORD [rbp - 16], eax
    leave
    ret"#;

    assert_eq!(expected, asm_result);

    Ok(())
}

fn asm_from_assign_code(code: &str) -> anyhow::Result<String> {
    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    static_type_check(&top_level_scope)?;

    let mut asm_result = String::new();

    if let [token] = &top_level_scope.tokens[..] {
        let mut stack = Stack::default();
        let mut meta = MetaInfo {
            code_line: Default::default(),
            target_os: TargetOS::Windows,
            static_type_information: Default::default(),
        };

        if let Token::Variable(variable_token) = token {
            let asm = token.to_asm(&mut stack, &mut meta)?;

            if let AssignableToken::String(string) = &variable_token.assignable {
                let s = string.before_label(&mut stack, &mut meta);
                if let Some(s) = s {
                    asm_result += &s?;
                }
            }

            asm_result += &asm;
        }
    }


    return Ok(asm_result);
}
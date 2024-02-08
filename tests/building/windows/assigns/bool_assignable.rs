use monkey_language::core::code_generator::generator::{ASMGenerator, Stack};
use monkey_language::core::code_generator::MetaInfo;
use monkey_language::core::code_generator::target_os::TargetOS;
use monkey_language::core::code_generator::ToASM;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::token::Token;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::lexer::tokens::assignable_token::AssignableToken;
use monkey_language::core::type_checker::static_type_checker::static_type_check;

#[test]
fn expression_assign() -> anyhow::Result<()> {
    let code = r#"
    let a: bool = true;
    let b: bool = false;
    let c: bool = a | b;
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
    sub rsp, 35
    ; let a: bool = true
    mov BYTE [rbp - 1], 1
    ; let b: bool = false
    mov BYTE [rbp - 2], 0
    ; let c: bool = (a | b)
    ; (a | b)
    mov al, BYTE [rbp - 1]
    or al, BYTE [rbp - 2]
    mov BYTE [rbp - 3], al
    leave
    ret
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: bool = true | false;
    "#;

    let asm_result = asm_from_assign_code(&code)?;


    let expected = r#"
    ; let a: bool = (true | false)
    ; (true | false)
    mov al, 1
    or al, 0
    mov BYTE [rbp - 1], al
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: bool = (true | true) | false;
    "#;

    let asm_result = asm_from_assign_code(&code)?;


    let expected = r#"
        ; let a: bool = ((true | true) | false)
    ; ((true | true) | false)
    ; (true | true)
    mov al, 1
    or al, 1
    or al, 0
    mov BYTE [rbp - 1], al
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: bool = false | (true | true);
    "#;

    let asm_result = asm_from_assign_code(&code)?;

    let expected = r#"
    ; let a: bool = (false | (true | true))
    ; (false | (true | true))
    ; (true | true)
    mov al, 1
    or al, 1
    mov dl, al
    mov al, 0
    or al, dl
    mov BYTE [rbp - 1], al
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: bool = (true | true) | (false | false);
    "#;

    let asm_result = asm_from_assign_code(&code)?;


    let expected = r#"
    ; let a: bool = ((true | true) | (false | false))
    ; ((true | true) | (false | false))
    ; (true | true)
    mov al, 1
    or al, 1
    mov ch, al
    ; (false | false)
    mov al, 0
    or al, 0
    mov dil, al
    or ch, dil
    mov al, ch
    mov BYTE [rbp - 1], al
    "#;


    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: bool = true;
    "#;

    let asm_result = asm_from_assign_code(&code)?;


    let expected = r#"
    ; let a: bool = true
    mov BYTE [rbp - 1], 1
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: bool = (true);
    "#;

    let asm_result = asm_from_assign_code(&code)?;


    let expected = r#"
    ; let a: bool = true
    mov al, 1
    mov BYTE [rbp - 1], al
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn pointer_assign_test() -> anyhow::Result<()> {
    let code = r#"
    let a: bool = true;
    let b: *bool = &a;
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
    sub rsp, 41
    ; let a: bool = true
    mov BYTE [rbp - 1], 1
    ; let b: *bool = &a
    lea rax, BYTE [rbp - 1]
    mov QWORD [rbp - 9], rax
    leave
    ret
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}


#[test]
fn pointer_assign_multiple_test() -> anyhow::Result<()> {
    let code = r#"
extern fn printf(format: *string, value: i32): void;
extern fn ExitProcess(exitCode: i32): void;

let a: bool = true;
let b: *bool = &a;
let c: **bool = &b;
let d: *bool = *c;

let ref: **bool = c;
let f: bool = *d;
let g: bool = **c;

let format: *string = "Das ist ein Test %f";
printf(format, (i32)*b);

ExitProcess(0);
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

extern printf
extern ExitProcess

.label0:
    db "Das ist ein Test %f", 0
main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 75
    ; let a: bool = true
    mov BYTE [rbp - 1], 1
    ; let b: *bool = &a
    lea rax, BYTE [rbp - 1]
    mov QWORD [rbp - 9], rax
    ; let c: **bool = &b
    lea rax, [rbp - 9]
    mov QWORD [rbp - 17], rax
    ; let d: *bool = *c
    mov rax, QWORD [rbp - 17]
    mov rax, QWORD [rax]
    mov QWORD [rbp - 25], rax
    ; let ref: **bool = c
    mov rax, QWORD [rbp - 17]
    mov QWORD [rbp - 33], rax
    ; let f: bool = *d
    mov rax, QWORD [rbp - 25]
    mov rax, QWORD [rax]
    mov BYTE [rbp - 34], al
    ; let g: bool = **c
    mov rax, QWORD [rbp - 17]
    mov rax, QWORD [rax]
    mov rax, QWORD [rax]
    mov BYTE [rbp - 35], al
    ; let format: *string = "Das ist ein Test %f"
    mov QWORD [rbp - 43], .label0
    mov rcx, QWORD [rbp - 43] ; Parameter (format)
    mov rax, QWORD [rbp - 9]
    mov rax, QWORD [rax]
    ; Cast: (bool) -> (i32)
    ; Cast: (u8) -> (i32)
    movzx eax, al
    mov edx, eax ; Parameter ((i32)*b)
    ; printf(format, (i32)*b)
    call printf
    mov ecx, 0 ; Parameter (0)
    ; ExitProcess(0)
    call ExitProcess
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn pointer_deref_operation_lhs() -> anyhow::Result<()> {
    let code = r#"
let a: bool = true;
let b: *bool = &a;
let and = *b & false;
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
    sub rsp, 42
    ; let a: bool = true
    mov BYTE [rbp - 1], 1
    ; let b: *bool = &a
    lea rax, BYTE [rbp - 1]
    mov QWORD [rbp - 9], rax
    ; let and: bool = (*b & false)
    ; (*b & false)
    mov rax, QWORD [rbp - 9]
    mov rax, QWORD [rax]
    and al, 0
    mov BYTE [rbp - 10], al
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
let a: bool = false;
let b: *bool = &a;
let addition = true | *b;
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
    sub rsp, 42
    ; let a: bool = false
    mov BYTE [rbp - 1], 0
    ; let b: *bool = &a
    lea rax, BYTE [rbp - 1]
    mov QWORD [rbp - 9], rax
    ; let addition: bool = (true | *b)
    ; (true | *b)
    mov al, 1
    mov rdx, QWORD [rbp - 9]
    mov rdx, QWORD [rdx]
    or al, dl
    mov BYTE [rbp - 10], al
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}


#[test]
fn pointer_deref_operation_lhs_rhs() -> anyhow::Result<()> {
    let code = r#"
let a: bool = true;
let b: *bool = &a;
let addition = *b | *b;
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
    sub rsp, 42
    ; let a: bool = true
    mov BYTE [rbp - 1], 1
    ; let b: *bool = &a
    lea rax, BYTE [rbp - 1]
    mov QWORD [rbp - 9], rax
    ; let addition: bool = (*b | *b)
    ; (*b | *b)
    mov rax, QWORD [rbp - 9]
    mov rax, QWORD [rax]
    mov rdx, QWORD [rbp - 9]
    mov rdx, QWORD [rdx]
    or al, dl
    mov BYTE [rbp - 10], al
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn pointer_deref_operation_lhs_expression() -> anyhow::Result<()> {
    let code = r#"
let a: bool = true;
let b: *bool = &a;
let addition = *b | (false | true);
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
    sub rsp, 42
    ; let a: bool = true
    mov BYTE [rbp - 1], 1
    ; let b: *bool = &a
    lea rax, BYTE [rbp - 1]
    mov QWORD [rbp - 9], rax
    ; let addition: bool = (*b | (false | true))
    ; (*b | (false | true))
    ; (false | true)
    mov al, 0
    or al, 1
    mov dl, al
    mov rax, QWORD [rbp - 9]
    mov rax, QWORD [rax]
    or al, dl
    mov BYTE [rbp - 10], al
    leave
    ret
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn pointer_deref_operation_expression_rhs() -> anyhow::Result<()> {
    let code = r#"
let a: bool = true;
let b: *bool = &a;
let addition = (false | true) | *b;
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
    sub rsp, 42
    ; let a: bool = true
    mov BYTE [rbp - 1], 1
    ; let b: *bool = &a
    lea rax, BYTE [rbp - 1]
    mov QWORD [rbp - 9], rax
    ; let addition: bool = ((false | true) | *b)
    ; ((false | true) | *b)
    ; (false | true)
    mov al, 0
    or al, 1
    mov rdx, QWORD [rbp - 9]
    mov rdx, QWORD [rdx]
    or al, dl
    mov BYTE [rbp - 10], al
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn pointer_deref_operation_expression_expression() -> anyhow::Result<()> {
    let code = r#"
let a: bool = true;
let b: *bool = &a;
let addition = (*b | *b) & (*b | *b);
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
    sub rsp, 42
    ; let a: bool = true
    mov BYTE [rbp - 1], 1
    ; let b: *bool = &a
    lea rax, BYTE [rbp - 1]
    mov QWORD [rbp - 9], rax
    ; let addition: bool = ((*b | *b) & (*b | *b))
    ; ((*b | *b) & (*b | *b))
    ; (*b | *b)
    mov rax, QWORD [rbp - 9]
    mov rax, QWORD [rax]
    mov rdx, QWORD [rbp - 9]
    mov rdx, QWORD [rdx]
    or al, dl
    mov ch, al
    ; (*b | *b)
    mov rax, QWORD [rbp - 9]
    mov rax, QWORD [rax]
    mov rdx, QWORD [rbp - 9]
    mov rdx, QWORD [rdx]
    or al, dl
    mov dil, al
    and ch, dil
    mov al, ch
    mov BYTE [rbp - 10], al
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}

#[test]
fn pointer_deref_operation_complex_expression_expression() -> anyhow::Result<()> {
    let code = r#"
let a: bool = true;
let b: *bool = &a;

let c: bool = true;
let d: *bool = &c;

let addition = (((*d | *b) | (*b | *d)) | (*b | *b)) | ((*b | (*b | *b)) | (*b | (*d | *b)));
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
    sub rsp, 51
    ; let a: bool = true
    mov BYTE [rbp - 1], 1
    ; let b: *bool = &a
    lea rax, BYTE [rbp - 1]
    mov QWORD [rbp - 9], rax
    ; let c: bool = true
    mov BYTE [rbp - 10], 1
    ; let d: *bool = &c
    lea rax, BYTE [rbp - 10]
    mov QWORD [rbp - 18], rax
    ; let addition: bool = ((((*d | *b) | (*b | *d)) | (*b | *b)) | ((*b | (*b | *b)) | (*b | (*d | *b))))
    ; ((((*d | *b) | (*b | *d)) | (*b | *b)) | ((*b | (*b | *b)) | (*b | (*d | *b))))
    ; (((*d | *b) | (*b | *d)) | (*b | *b))
    ; ((*d | *b) | (*b | *d))
    ; (*d | *b)
    mov rax, QWORD [rbp - 18]
    mov rax, QWORD [rax]
    mov rdx, QWORD [rbp - 9]
    mov rdx, QWORD [rdx]
    or al, dl
    mov ch, al
    ; (*b | *d)
    mov rax, QWORD [rbp - 9]
    mov rax, QWORD [rax]
    mov rdx, QWORD [rbp - 18]
    mov rdx, QWORD [rdx]
    or al, dl
    mov dil, al
    or ch, dil
    mov al, ch
    push rax
    xor rax, rax
    ; (*b | *b)
    mov rax, QWORD [rbp - 9]
    mov rax, QWORD [rax]
    mov rdx, QWORD [rbp - 9]
    mov rdx, QWORD [rdx]
    or al, dl
    push rax
    xor rax, rax
    pop rdi
    pop rax
    or al, dil
    push rax
    xor rax, rax
    ; ((*b | (*b | *b)) | (*b | (*d | *b)))
    ; (*b | (*b | *b))
    ; (*b | *b)
    mov rax, QWORD [rbp - 9]
    mov rax, QWORD [rax]
    mov rdx, QWORD [rbp - 9]
    mov rdx, QWORD [rdx]
    or al, dl
    mov dl, al
    mov rax, QWORD [rbp - 9]
    mov rax, QWORD [rax]
    or al, dl
    mov dil, al
    push rdi
    xor rdi, rdi
    ; (*b | (*d | *b))
    ; (*d | *b)
    mov rax, QWORD [rbp - 18]
    mov rax, QWORD [rax]
    mov rdx, QWORD [rbp - 9]
    mov rdx, QWORD [rdx]
    or al, dl
    mov dl, al
    mov rax, QWORD [rbp - 9]
    mov rax, QWORD [rax]
    or al, dl
    push rax
    xor rax, rax
    pop rdi
    pop rax
    or al, dil
    push rax
    xor rax, rax
    pop rdi
    pop rax
    or al, dil
    mov BYTE [rbp - 19], al
    leave
    ret
    "#;

    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}


#[test]
fn single_expression_test() -> anyhow::Result<()> {
    let code = r#"
    let a: bool = true;
    let b: bool = a;
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
    sub rsp, 34
    ; let a: bool = true
    mov BYTE [rbp - 1], 1
    ; let b: bool = a
    mov al, BYTE [rbp - 1]
    mov BYTE [rbp - 2], al
    leave
    ret
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());

    let code = r#"
    let a: bool = true;
    let b: bool = (a);
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

#[test]
fn bool_assign() -> anyhow::Result<()> {
    let code = r#"
    let a: bool = false;
    "#;

    let asm_result = asm_from_assign_code(&code)?;

    let expected = r#"
    ; let a: bool = false
    mov BYTE [rbp - 1], 0
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
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
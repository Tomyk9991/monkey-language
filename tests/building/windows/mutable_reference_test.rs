use monkey_language::core::code_generator::generator::ASMGenerator;
use monkey_language::core::code_generator::target_os::TargetOS;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::parser::Lexer;
use monkey_language::core::type_checker::static_type_checker::static_type_check;

#[test]
pub fn mutable_ref_test() -> anyhow::Result<()> {
    let code = r#"
        fn mut_ref(x: mut *i32): void {
        *x = *x + 1;
    }

    let mut a: i32 = 5;
    mut_ref(&a);
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(code);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.parse()?;

    static_type_check(&top_level_scope)?;

    let mut code_generator = ASMGenerator::from((top_level_scope, TargetOS::Windows));
    let asm_result = code_generator.generate()?;

    let expected = r#"
    ; This assembly is targeted for the Windows Operating System
segment .text
global main


.mut_ref_ptri32~void:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 32
    mov QWORD [rbp - 8], rcx
    ; *x = (*x + 1)
    mov rdx, QWORD [rbp - 8]
    ; (*x + 1)
    mov rax, QWORD [rbp - 8]
    mov rax, QWORD [rax]
    add eax, 1
    mov DWORD [rdx], eax
    leave
    ret
main:
    push rbp
    mov rbp, rsp
    ; Reserve stack space as MS convention. Shadow stacking
    sub rsp, 64
    ; let a: i32 = 5
    mov DWORD [rbp - 4], 5
    lea rax, [rbp - 4]
    push rax
    pop rcx
    ; mut_ref(&a)
    call .mut_ref_ptri32~void
    ; return 0
    mov eax, 0
    leave
    ret
    "#;

    println!("{}", asm_result);
    assert_eq!(expected.trim(), asm_result.trim());
    Ok(())
}
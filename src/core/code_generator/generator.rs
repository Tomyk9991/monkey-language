use std::collections::{HashMap};
use crate::core::code_generator::ToASM;
use crate::core::lexer::scope::Scope;
use crate::core::code_generator::{Error};

pub type StackLocation = u32;
pub struct Stack {
    pub stack_position: StackLocation,
    pub variables: HashMap<String, StackLocation>
}

impl Stack {
    pub fn push_stack(&mut self, register: &str) -> String {
        self.stack_position += 1;
        format!("    push {register}\n")
    }

    pub fn pop_stack(&mut self, register: &str) -> String {
        self.stack_position -= 1;
        format!("    pop {register}\n")
    }

}

pub struct Generator {
    top_level_scope: Scope,
    pub stack: Stack,
}

impl Generator {
    pub fn generate(&mut self) -> Result<String, Error> {

        let mut result = String::new();

        result.push_str("global main\n");
        result.push_str("extern printf\n");
        result.push_str("section .data\n");
        result.push_str("hello db 'Hello, World!', 0\n");
        result.push_str("section .text\n");
        result.push_str("main:\n");

        for token in &self.top_level_scope.tokens {
            let generated_asm = token.to_asm(&mut self.stack)?;
            result.push_str(&generated_asm);
        }

        result.push_str("    sub rsp, 20h\n");
        result.push_str("    lea rcx, [hello]\n");
        result.push_str("    mov rdx, 0\n");
        result.push_str("    call printf\n");
        result.push_str("    add rsp, 20h\n");
        result.push_str("    mov eax, 60\n");
        result.push_str("    xor edi, edi\n");
        result.push_str("    syscall\n");

        return Ok(result.to_string());

        // return Ok(result);

//         let result =
// r#"; create
// global main
// extern printf
//
// section .data
// hello db 'Hello, World!', 0
//
// section .text
// main:
//     sub rsp, 20h
//
//     lea rcx, [hello]
//     mov rdx, 0
//
//     call printf
//
//     add rsp, 20h
//
//     mov eax, 60
//     xor edi, edi
//     syscall"#;
//
//         return Ok(result.to_string());
    }
}

impl From<Scope> for Generator {
    fn from(value: Scope) -> Self {
        Generator {
            top_level_scope: value,
            stack: Stack {
                stack_position: 0,
                variables: Default::default(),
            }
        }
    }
}
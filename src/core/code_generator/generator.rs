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

        result.push_str("global _start\n");
        result.push_str("_start:\n");

        for token in &self.top_level_scope.tokens {
            let generated_asm = token.to_asm(&mut self.stack)?;
            result.push_str(&generated_asm);
        }

        result.push_str("    ; exit code\n");
        result.push_str("    mov rax, 60\n");
        result.push_str("    pop rdi\n");
        result.push_str("    syscall\n");

        Ok(result.to_string())
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
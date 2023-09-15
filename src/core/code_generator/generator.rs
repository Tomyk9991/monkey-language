use crate::core::code_generator::ToASM;
use crate::core::lexer::scope::Scope;
use crate::core::code_generator::{Error};
use crate::core::lexer::token::Token;
use crate::core::lexer::tokens::name_token::NameToken;

pub struct StackLocation {
    pub position: usize,
    pub name: NameToken
}

/// a struct representing the current stack pointer and variables in the stack
pub struct Stack {
    /// represents the current position on the stack
    pub stack_position: usize,
    /// represents a list of all defined scopes and the position on the stack where it starts
    scopes: Vec<usize>,
    /// represents a list of all available variables in the current scopes and above
    pub variables: Vec<StackLocation>,
    /// to create labels and avoid collisions in naming, a label count is used
    label_count: usize,
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

    pub fn create_label(&mut self) -> String {
        let value = format!("label{}", self.label_count);
        self.label_count += 1;
        return value;
    }

    pub fn generate_scope(&mut self, tokens: &Vec<Token>) -> Result<String, crate::core::code_generator::Error> {
        let mut target = String::new();

        self.begin_scope();

        for token in tokens {
            target.push_str(&token.to_asm(self)?);
        }

        target.push_str(&self.end_scope());
        Ok(target)
    }

    pub fn begin_scope(&mut self) {
        self.scopes.push(self.variables.len());
    }

    pub fn end_scope(&mut self) -> String {
        if let Some(last_element) = self.scopes.last() {
            let pop_count = self.variables.len() - *last_element;
            let target = format!("    add rsp, {}\n", pop_count * 8);
            self.stack_position -= pop_count;

            for _ in 0..pop_count {
                let _ = self.variables.pop();
            }

            let _ = self.scopes.pop();

            return target;
        }

        String::new()
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



        if let Some(last) = self.top_level_scope.tokens.last() {
            if let Token::MethodCall(method_call_token) = last {
                if method_call_token.name.name == "exit" {
                    return Ok(result.to_string());
                }
            }
        }

        result.push_str("    ; exit(last variable)\n");
        result.push_str("    mov rax, 60\n");
        result.push_str("    pop rdi\n");
        result.push_str("    syscall");

        Ok(result.to_string())
    }
}

impl From<Scope> for Generator {
    fn from(value: Scope) -> Self {
        Generator {
            top_level_scope: value,
            stack: Stack {
                stack_position: 0,
                scopes: vec![],
                variables: Default::default(),
                label_count: 0,
            }
        }
    }
}
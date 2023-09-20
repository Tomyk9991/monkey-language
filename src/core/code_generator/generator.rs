use crate::core::code_generator::ToASM;
use crate::core::lexer::scope::Scope;
use crate::core::code_generator::{Error};
use crate::core::code_generator::target_os::TargetOS;
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
        value
    }

    pub fn generate_scope(&mut self, tokens: &Vec<Token>, target_os: &TargetOS) -> Result<String, crate::core::code_generator::Error> {
        let mut target = String::new();

        self.begin_scope();

        for token in tokens {
            target.push_str(&token.to_asm(self, target_os)?);
        }

        target.push_str(&self.end_scope());
        Ok(target)
    }

    fn begin_scope(&mut self) {
        self.scopes.push(self.variables.len());
    }

    fn end_scope(&mut self) -> String {
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

pub struct ASMGenerator {
    top_level_scope: Scope,
    pub stack: Stack,
    target_os: TargetOS,
}

impl ASMGenerator {
    pub fn generate(&mut self) -> Result<String, Error> {
        let mut result = String::new();
        result += &format!("; This assembly is targeted for the {} Operating System\n", self.target_os);

        let entry_point_label = if self.target_os == TargetOS::Windows {
            result.push_str("segment .text\n");
            String::from("main")
        } else {
            String::from("_start")
        };

        result.push_str(&format!("global {}\n\n", entry_point_label));

        if self.target_os == TargetOS::Windows {
            result.push_str("extern ExitProcess\n\n");
        }

        result.push_str(&format!("{}:\n", entry_point_label));


        for token in &self.top_level_scope.tokens {
            let generated_asm = token.to_asm(&mut self.stack, &self.target_os)?;
            result.push_str(&generated_asm);
        }


        if let Some(Token::MethodCall(method_call_token)) = self.top_level_scope.tokens.last() {
            if method_call_token.name.name == "exit" {
                return Ok(result.to_string());
            }
        }

        result.push_str("    ; exit(last variable)\n");
        result.push_str("    mov rax, 60\n");
        result.push_str("    pop rdi\n");
        result.push_str("    syscall");

        Ok(result.to_string())
    }
}

impl From<(Scope, TargetOS)> for ASMGenerator {
    fn from(value: (Scope, TargetOS)) -> Self {
        ASMGenerator {
            top_level_scope: value.0,
            stack: Stack {
                stack_position: 0,
                scopes: vec![],
                variables: Default::default(),
                label_count: 0,
            },
            target_os: value.1
        }
    }
}
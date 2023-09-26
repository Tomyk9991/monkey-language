use crate::core::code_generator::{MetaInfo, ToASM};
use crate::core::code_generator::ASMGenerateError;
use crate::core::code_generator::asm_builder::{ASMBuilder};
use crate::core::code_generator::target_os::TargetOS;
use crate::core::lexer::scope::Scope;
use crate::core::lexer::token::Token;
use crate::core::lexer::tokens::name_token::NameToken;

pub struct StackLocation {
    pub position: usize,
    pub name: NameToken,
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
    pub register_to_use: String,
}

impl Stack {
    pub fn _push_stack(&mut self, register: &str, size: usize) -> String {
        self.stack_position += size;
        format!("    push {register}\n")
    }

    pub fn _pop_stack(&mut self, register: &str, size: usize) -> String {
        self.stack_position -= size;
        format!("    pop {register}\n")
    }

    pub fn create_label(&mut self) -> String {
        let value = format!(".label{}", self.label_count);
        self.label_count += 1;
        value
    }

    pub fn generate_scope(&mut self, tokens: &Vec<Token>, meta: &MetaInfo) -> Result<String, ASMGenerateError> {
        let mut target = String::new();

        self.begin_scope();

        for token in tokens {
            target.push_str(&token.to_asm(self, meta)?);
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
    pub fn generate(&mut self) -> Result<String, ASMGenerateError> {
        let mut asb = String::new();
        asb += &ASMBuilder::line(&format!("; This assembly is targeted for the {} Operating System", self.target_os));

        let entry_point_label = if self.target_os == TargetOS::Windows {
            asb += &ASMBuilder::line("segment .text");

            String::from("main")
        } else {
            String::from("_start")
        };

        asb += &ASMBuilder::line(&format!("global {}", entry_point_label));

        if self.target_os == TargetOS::Windows {
            asb += &ASMBuilder::line("extern ExitProcess");
        }

        self.top_level_scope.tokens.iter().for_each(|a|
            if let Token::MethodDefinition(method_def) = a {
                if method_def.is_extern {
                    asb += &ASMBuilder::line(&format!("extern {}", &method_def.name.name));
                }
            }
        );

        asb += &ASMBuilder::line(&format!("{entry_point_label}:"));

        for token in &self.top_level_scope.tokens {
            let meta = MetaInfo {
                code_line: token.code_line(),
                target_os: self.target_os.clone(),
            };

            asb += &ASMBuilder::push(&token.to_asm(&mut self.stack, &meta)?);
        }


        if let Some(Token::MethodCall(method_call_token)) = self.top_level_scope.tokens.last() {
            if method_call_token.name.name == "exit" {
                return Ok(asb.to_string());
            }
        }

        asb += &ASMBuilder::line_ident("exit (last variable)");
        asb += &ASMBuilder::line_ident(" mov rax, 60");
        asb += &ASMBuilder::line_ident(" pop rdi");
        asb += &ASMBuilder::line_ident(" syscall");

        Ok(asb.to_string())
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
                register_to_use: String::from("eax"),
            },
            target_os: value.1,
        }
    }
}
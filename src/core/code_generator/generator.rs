use crate::core::code_generator::{MetaInfo, ToASM};
use crate::core::code_generator::ASMGenerateError;
use crate::core::code_generator::asm_builder::{ASMBuilder};
use crate::core::code_generator::target_os::TargetOS;
use crate::core::lexer::scope::Scope;
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::token::Token;
use crate::core::lexer::tokens::name_token::NameToken;

pub struct StackLocation {
    pub position: usize,
    pub size: usize,
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
    pub register_to_use: Vec<String>,
}

pub trait LastUnchecked<T> {
    fn last_unchecked(&self) -> T;
}

impl LastUnchecked<String> for Vec<String> {
    fn last_unchecked(&self) -> String {
        if let Some(last) = self.last() {
            last.to_string()
        } else {
            String::from("")
        }
    }
}

pub trait RegisterTransformation {
    fn is_64_bit_register(&self) -> bool;
    fn to_64_bit_register(&self) -> String;
}

impl RegisterTransformation for String {
    fn is_64_bit_register(&self) -> bool {
        self.starts_with(['r', 'R'])
    }

    fn to_64_bit_register(&self) -> String {
        self.replacen('e', "r", 1)
    }
}

impl Default for Stack {
    fn default() -> Self {
        Stack {
            stack_position: 0,
            scopes: vec![],
            variables: Default::default(),
            label_count: 0,
            register_to_use: vec!["eax".to_string()],
        }
    }
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

    pub fn get_latest_label(&self) -> String {
        format!(".label{}", self.label_count - 1)
    }

    pub fn generate_scope(&mut self, tokens: &Vec<Token>, meta: &mut MetaInfo) -> Result<String, ASMGenerateError> {
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
        let mut boiler_plate = String::new();
        boiler_plate += &ASMBuilder::line(&format!("; This assembly is targeted for the {} Operating System", self.target_os));

        let entry_point_label = if self.target_os == TargetOS::Windows {
            boiler_plate += &ASMBuilder::line("segment .text");

            String::from("main")
        } else {
            String::from("_start")
        };

        boiler_plate += &ASMBuilder::line(&format!("global {}", entry_point_label));
        boiler_plate += &ASMBuilder::line("");

        self.top_level_scope.tokens.iter().for_each(|a|
            if let Token::MethodDefinition(method_def) = a {
                if method_def.is_extern {
                    boiler_plate += &ASMBuilder::line(&format!("extern {}", &method_def.name.name));
                }
            }
        );

        boiler_plate += &ASMBuilder::line("");

        let mut prefix = String::new();

        let mut label_header: String = String::new();

        label_header += &ASMBuilder::line(&format!("{entry_point_label}:"));
        label_header += &ASMBuilder::ident_line("push rbp");
        label_header += &ASMBuilder::ident_line("mov rbp, rsp");

        label_header += &ASMBuilder::ident(&ASMBuilder::comment_line("Reserve stack space as MS convention. Shadow stacking"));

        let mut stack_allocation = 32; // per default microsoft convention requires 32 byte as a shadow stack
        let mut method_scope: String = String::new();


        for token in &self.top_level_scope.tokens {
            let mut meta = MetaInfo {
                code_line: token.code_line(),
                target_os: self.target_os.clone(),
                static_type_information: StaticTypeContext::new(&self.top_level_scope.tokens),
            };


            stack_allocation += token.byte_size(&mut meta);
            method_scope += &ASMBuilder::push(&token.to_asm(&mut self.stack, &mut meta)?);

            if let Some(prefix_asm) = token.before_label(&mut self.stack, &mut meta) {
                prefix += &ASMBuilder::push(&(prefix_asm?));
            }
        }

        if !method_scope.trim_end().ends_with("call ProcessExit") {
            method_scope += &ASMBuilder::ident_line("leave");
            method_scope += &ASMBuilder::ident_line("ret");
        }

        let stack_allocation_asm = ASMBuilder::ident_line(&format!("sub rsp, {}", stack_allocation));

        Ok(format!("{}{}{}{}{}", boiler_plate, prefix, label_header, stack_allocation_asm, method_scope))
    }
}

impl From<(Scope, TargetOS)> for ASMGenerator {
    fn from(value: (Scope, TargetOS)) -> Self {
        ASMGenerator {
            top_level_scope: value.0,
            stack: Stack::default(),
            target_os: value.1,
        }
    }
}
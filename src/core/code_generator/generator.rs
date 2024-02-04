use crate::core::code_generator::{MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::ASMGenerateError;
use crate::core::code_generator::registers::{Bit64, GeneralPurposeRegister};
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


#[derive(Default)]
/// a struct representing the current stack pointer and variables in the stack
pub struct Stack {
    /// represents the current position on the stack
    pub stack_position: usize,
    /// represents a list of all defined scopes and the position on the stack where it starts
    scopes: Vec<usize>,
    /// represents a list of all available variables in the current scopes and above
    pub variables: Vec<StackLocation>,
    /// to create labels and avoid collisions in naming, a label count is used
    pub label_count: usize,
    pub register_to_use: Vec<GeneralPurposeRegister>,
}

pub trait LastUnchecked<T> {
    type Error;
    fn last(&self) -> Result<T, Self::Error>;
}

impl LastUnchecked<GeneralPurposeRegister> for Vec<GeneralPurposeRegister> {
    type Error = ASMGenerateError;
    fn last(&self) -> Result<GeneralPurposeRegister, Self::Error> {
        if let [.., last] = &self[..] {
            Ok(last.clone())
        } else {
            Err(ASMGenerateError::InternalError(String::from("No register pushed to the general purpose register stack")))
        }
    }
}


impl Stack {
    pub fn _reset_registers(&self) -> String {
        static REGISTERS: [Bit64; 4] = [Bit64::Rax, Bit64::Rcx, Bit64::Rdx, Bit64::Rdi];
        let mut target = String::new();

        target += &ASMBuilder::ident_comment_line("Resetting registers");

        for register in &REGISTERS {
            target += &ASMBuilder::ident_line(&format!("xor {register}, {register}"));
        }

        target
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
        let mut prefix = String::new();

        self.begin_scope();

        for token in tokens {
            target += &ASMBuilder::push(&token.to_asm(self, meta)?);

            if let Some(Ok(prefix_asm)) = token.before_label(self, meta) {
                prefix += &ASMBuilder::push(&prefix_asm);
            }
        }

        target.push_str(&prefix);
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
    require_main: bool,
    target_os: TargetOS,
}

impl ASMGenerator {
    pub fn generate(&mut self) -> Result<String, ASMGenerateError> {
        let mut boiler_plate = String::new();
        boiler_plate += &ASMBuilder::line(&format!("; This assembly is targeted for the {} Operating System", self.target_os));
        let method_definitions = self.generate_method_definitions()?;

        let entry_point_label = if self.target_os == TargetOS::Windows {
            boiler_plate += &ASMBuilder::line("segment .text");

            String::from("main")
        } else {
            String::from("_start")
        };

        boiler_plate += &ASMBuilder::line(&format!("global {}", entry_point_label));
        boiler_plate += &ASMBuilder::line("");

        let mut added_extern_methods: Vec<&str> = vec![];
        self.top_level_scope.tokens.iter().for_each(|a|
            if let Token::MethodDefinition(method_def) = a {
                if method_def.is_extern && !added_extern_methods.contains(&method_def.name.name.as_str()) {
                    boiler_plate += &ASMBuilder::line(&format!("extern {}", &method_def.name.name));
                    added_extern_methods.push(&method_def.name.name);
                }
            }
        );

        boiler_plate += &ASMBuilder::line("");

        if self.require_main {
            // search for main function
            let main_entry = self.top_level_scope.tokens.iter().filter(|a| matches!(a, Token::MethodDefinition(md) if md.name.name == "main")).collect::<Vec<&Token>>();

            let value: Result<String, ASMGenerateError> = match main_entry.len() {
                0 => return Err(ASMGenerateError::EntryPointNotFound),
                1 => {
                    if let Some(Token::MethodDefinition(main)) = main_entry.first() {
                        if main.is_extern {
                            return Err(ASMGenerateError::EntryPointNotFound);
                        }

                        let mut meta = MetaInfo {
                            code_line: main.code_line.clone(),
                            target_os: self.target_os.clone(),
                            static_type_information: StaticTypeContext::new(&self.top_level_scope.tokens),
                        };

                        meta.static_type_information.merge(StaticTypeContext::new(&main.stack));

                        Ok(format!("{}{}{}", boiler_plate, method_definitions, &main.to_asm(&mut self.stack, &mut meta)?))
                    } else {
                        return Err(ASMGenerateError::EntryPointNotFound)
                    }
                }
                _ => return Err(ASMGenerateError::MultipleEntryPointsFound(main_entry.iter().map(|t| t.code_line()).collect::<Vec<_>>()))
            };

            Ok(value?)
        } else {
            let mut prefix = String::new();
            let mut label_header: String = String::new();

            label_header += &ASMBuilder::line(&format!("{entry_point_label}:"));
            label_header += &ASMBuilder::ident_line("push rbp");
            label_header += &ASMBuilder::mov_ident_line("rbp", "rsp");

            label_header += &ASMBuilder::ident(&ASMBuilder::comment_line("Reserve stack space as MS convention. Shadow stacking"));

            let mut stack_allocation = 32; // per default microsoft convention requires 32 byte as a shadow stack
            let mut method_scope: String = String::new();



            for token in &self.top_level_scope.tokens {
                let mut meta = MetaInfo {
                    code_line: token.code_line(),
                    target_os: self.target_os.clone(),
                    static_type_information: StaticTypeContext::new(&self.top_level_scope.tokens),
                };
                if let Token::MethodDefinition(_) = token {
                    continue;
                }

                stack_allocation += token.byte_size(&mut meta);
                method_scope += &ASMBuilder::push(&token.to_asm(&mut self.stack, &mut meta)?);

                if let Some(Ok(prefix_asm)) = token.before_label(&mut self.stack, &mut meta) {
                    prefix += &ASMBuilder::push(&(prefix_asm));
                }
            }

            if !method_scope.trim_end().ends_with("call ProcessExit") {
                method_scope += &ASMBuilder::ident_line("leave");
                method_scope += &ASMBuilder::ident_line("ret");
            }

            let stack_allocation_asm = ASMBuilder::ident_line(&format!("sub rsp, {}", stack_allocation));

            Ok(format!("{}{}{}{}{}{}", boiler_plate, prefix, method_definitions, label_header, stack_allocation_asm, method_scope))
        }
    }

    fn generate_method_definitions(&mut self) -> Result<String, ASMGenerateError> {
        let mut method_definitions = String::new();

        for token in &self.top_level_scope.tokens {
            let mut meta = MetaInfo {
                code_line: token.code_line(),
                target_os: self.target_os.clone(),
                static_type_information: StaticTypeContext::new(&self.top_level_scope.tokens),
            };

            if let Token::MethodDefinition(md) = token {
                if !md.is_extern && md.name.name != "main" {
                    meta.static_type_information.merge(StaticTypeContext::new(&md.stack));
                    method_definitions += &ASMBuilder::push(&md.to_asm(&mut self.stack, &mut meta)?);
                }

                continue;
            }
        }

        Ok(method_definitions.to_string())
    }
}

impl From<(Scope, TargetOS)> for ASMGenerator {
    fn from(value: (Scope, TargetOS)) -> Self {
        ASMGenerator {
            top_level_scope: value.0,
            stack: Stack::default(),
            require_main: false,
            target_os: value.1,
        }
    }
}

impl From<(Scope, TargetOS, bool)> for ASMGenerator {
    fn from(value: (Scope, TargetOS, bool)) -> Self {
        ASMGenerator {
            top_level_scope: value.0,
            stack: Stack::default(),
            require_main: value.2,
            target_os: value.1,
        }
    }
}
use crate::core::code_generator::{ASMOptions, ASMResult, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_result::{InterimResultOption};
use crate::core::code_generator::ASMGenerateError;
use crate::core::code_generator::conventions::calling_convention_from;
use crate::core::code_generator::registers::{GeneralPurposeRegister};
use crate::core::code_generator::target_os::TargetOS;
use crate::core::lexer::scope::Scope;
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::token::Token;
use crate::core::lexer::tokens::assignable_token::AssignableToken;
use crate::core::lexer::tokens::method_definition::MethodDefinition;
use crate::core::lexer::tokens::name_token::NameToken;
use crate::core::lexer::tokens::parameter_token::ParameterToken;
use crate::core::lexer::tokens::return_token::ReturnToken;
use crate::core::lexer::tokens::variable_token::VariableToken;
use crate::core::lexer::types::integer::Integer;
use crate::core::lexer::types::type_token::TypeToken;
use crate::core::model::data_section::DataSection;

#[derive(Debug)]
pub struct StackLocation {
    pub position: usize,
    pub size: usize,
    pub name: NameToken,
}

impl StackLocation {
    pub fn new_anonymous_stack_location(position: usize, size: usize) -> StackLocation {
        Self {
            position,
            size,
            name: NameToken::uuid(),
        }
    }
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
    /// represents the data section in the assembly language
    pub data_section: DataSection,
    /// to create labels and avoid collisions in naming, a label count is used
    pub label_count: usize,
    pub register_to_use: Vec<GeneralPurposeRegister>,
}

impl Stack {
    pub fn clear_stack(&mut self) {
        self.stack_position = 0;
        self.variables.clear();
    }
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
    pub fn create_label(&mut self) -> String {
        let value = format!(".label{}", self.label_count);
        self.label_count += 1;
        value
    }

    pub fn get_latest_label(&self) -> String {
        format!(".label{}", self.label_count - 1)
    }


    pub fn generate_scope<T: ASMOptions + 'static>(&mut self, tokens: &Vec<Token>, meta: &mut MetaInfo, options: Option<T>) -> Result<String, ASMGenerateError> {
        let mut target = String::new();

        self.begin_scope();

        for token in tokens {
            target += match &token.to_asm(self, meta, options.clone())? {
                ASMResult::Inline(t) => t,
                ASMResult::MultilineResulted(t, _) => t,
                ASMResult::Multiline(t) => t
            };
        }

        self.end_scope();
        Ok(target)
    }

    fn begin_scope(&mut self) {
        self.scopes.push(self.variables.len());
    }

    fn end_scope(&mut self) {
        if let Some(last_element) = self.scopes.last() {
            let pop_count = self.variables.len() - *last_element;
            self.stack_position -= pop_count;

            for _ in 0..pop_count {
                let _ = self.variables.pop();
            }

            let _ = self.scopes.pop();
        }
    }
}

pub struct ASMGenerator {
    top_level_scope: Scope,
    pub stack: Stack,
    /// Indicates, if a main method is required inside the source code
    require_main: bool,
    target_os: TargetOS,
}

impl ASMGenerator {
    pub fn generate(&mut self) -> Result<String, ASMGenerateError> {
        let mut boiler_plate = String::new();
        let compile_comment = &ASMBuilder::line(&format!("; This assembly is targeted for the {} Operating System", self.target_os));
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

                        self.stack.clear_stack();
                        let mut meta = MetaInfo {
                            code_line: main.code_line.clone(),
                            target_os: self.target_os.clone(),
                            static_type_information: StaticTypeContext::new(&self.top_level_scope.tokens),
                        };

                        meta.static_type_information.merge(StaticTypeContext::new(&main.stack));

                        let main_function_asm = main.to_asm::<InterimResultOption>(&mut self.stack, &mut meta, None)?;
                        let data_section = self.stack.data_section
                            .clone()
                            .to_asm::<InterimResultOption>(&mut self.stack, &mut meta, None)?;

                        Ok(format!("{}{}{}{}{}", compile_comment, data_section, boiler_plate, method_definitions, main_function_asm))
                    } else {
                        return Err(ASMGenerateError::EntryPointNotFound)
                    }
                }
                _ => return Err(ASMGenerateError::MultipleEntryPointsFound(main_entry.iter().map(|t| t.code_line()).collect::<Vec<_>>()))
            };

            Ok(value?)
        } else {
            self.require_main = true;

            let method_definitions = self.top_level_scope.tokens.iter().filter(|t| matches!(t, Token::MethodDefinition(_))).cloned().collect::<Vec<_>>();
            let mut main_stack = self.top_level_scope.tokens.iter().filter(|t| !matches!(t, Token::Import(_) | Token::MethodDefinition(_))).cloned().collect::<Vec<Token>>();
            // last element of main stack via pattern matching
            if let [.., last] = &main_stack[..] {
                if !matches!(last, Token::Return(_)) {
                     main_stack.push(Token::Return(ReturnToken::num_0()));
                }
            }
            let main_function = Token::MethodDefinition(MethodDefinition {
                name: NameToken { name: "main".to_string() },
                return_type: TypeToken::Integer(Integer::I32),
                arguments: vec![],
                stack: main_stack,
                is_extern: false,
                code_line: Default::default(),
            });

            self.top_level_scope.tokens.clear();
            let imports = self.top_level_scope.tokens.iter().filter(|t| matches!(t, Token::Import(_))).cloned().collect::<Vec<Token>>();

            method_definitions.iter().for_each(|t| self.top_level_scope.tokens.push(t.clone()));
            imports.iter().for_each(|t| self.top_level_scope.tokens.push(t.clone()));

            self.top_level_scope.tokens.push(main_function);
            self.generate()
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


            if let Token::MethodDefinition(method_definition) = token {
                if !method_definition.is_extern && method_definition.name.name != "main" {
                    self.stack.clear_stack();

                    let calling_convention = calling_convention_from(method_definition, &self.target_os);

                    for (index, (argument_name, argument_type)) in method_definition.arguments.iter().enumerate() {
                        let parameter_token = ParameterToken {
                            name_token: argument_name.clone(),
                            ty: argument_type.clone(),
                            register: calling_convention[index][0].clone(),
                            mutablility: false,
                            code_line: method_definition.code_line.clone(),
                        };

                        self.stack.variables.push(StackLocation {
                            position: self.stack.stack_position,
                            size: argument_type.byte_size(),
                            name: argument_name.clone(),
                        });

                        self.stack.stack_position += argument_type.byte_size();

                        meta.static_type_information.context.push(VariableToken {
                            name_token: argument_name.clone(),
                            mutability: false,
                            ty: Some(argument_type.clone()),
                            define: true,
                            assignable: AssignableToken::Parameter(parameter_token),
                            code_line: method_definition.code_line.clone(),
                        });
                    }

                    meta.static_type_information.merge(StaticTypeContext::new(&method_definition.stack));
                    method_definitions += match &method_definition.to_asm::<InterimResultOption>(&mut self.stack, &mut meta, None)? {
                        ASMResult::Inline(t) => t,
                        ASMResult::MultilineResulted(t, _) => t,
                        ASMResult::Multiline(t) => t
                    }
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
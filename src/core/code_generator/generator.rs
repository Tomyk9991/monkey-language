use crate::core::code_generator::{ASMResult, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_options::interim_result::InterimResultOption;
use crate::core::code_generator::ASMGenerateError;
use crate::core::code_generator::conventions::calling_convention_from;
use crate::core::code_generator::registers::{GeneralPurposeRegister};
use crate::core::code_generator::target_os::TargetOS;
use crate::core::lexer::token_with_span::FilePosition;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use crate::core::model::abstract_syntax_tree_nodes::method_definition::MethodDefinition;
use crate::core::model::abstract_syntax_tree_nodes::parameter::Parameter;
use crate::core::model::abstract_syntax_tree_nodes::ret::Return;
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::model::data_section::DataSection;
use crate::core::model::scope::Scope;
use crate::core::model::types::integer::IntegerType;
use crate::core::model::types::mutability::Mutability;
use crate::core::model::types::ty::Type;

#[derive(Debug)]
pub struct StackLocation {
    pub position: usize,
    pub size: usize,
    pub elements: usize,
    pub name: LValue,
}

impl StackLocation {
    pub fn new_anonymous_stack_location(position: usize, size: usize) -> StackLocation {
        Self {
            position,
            size,
            elements: 1,
            name: LValue::uuid(),
        }
    }
}


#[derive(Default, Debug)]
/// a struct representing the current stack pointer and variables in the stack
pub struct Stack {
    /// represents the current position on the stack
    pub stack_position: usize,
    /// represents a list of all defined scopes and the position on the stack where it starts
    scopes: Vec<usize>,
    /// represents a list of all available variables in the current scopes and above
    pub variables: Vec<StackLocation>,
    /// represents the current state, if an indexing is required
    pub indexing: Option<ASMResult>,
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
    fn last(&self, file_position: &FilePosition) -> Result<T, Self::Error>;
}

impl LastUnchecked<GeneralPurposeRegister> for Vec<GeneralPurposeRegister> {
    type Error = ASMGenerateError;
    fn last(&self, file_position: &FilePosition) -> Result<GeneralPurposeRegister, Self::Error> {
        if let [.., last] = &self[..] {
            Ok(last.clone())
        } else {
            Err(ASMGenerateError::InternalError(String::from("No register pushed to the general purpose register stack"), file_position.clone()))
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


    pub fn generate_scope<T: ASMOptions + 'static>(&mut self, nodes: &Vec<AbstractSyntaxTreeNode>, meta: &mut MetaInfo, options: Option<T>) -> Result<String, ASMGenerateError> {
        let mut target = String::new();

        self.begin_scope();

        for node in nodes {
            meta.file_position = node.file_position().clone();

            target += match &node.to_asm(self, meta, options.clone())? {
                ASMResult::Inline(t) => t,
                ASMResult::MultilineResulted(t, _) => t,
                ASMResult::Multiline(t) => t
            };

            node.data_section(self, meta);
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
    top_level_scope: Vec<AbstractSyntaxTreeNode>,
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

        let mut added_extern_methods: Vec<String> = vec![];
        self.top_level_scope.iter().for_each(|a|
            if let AbstractSyntaxTreeNode::MethodDefinition(method_def) = a {
                if method_def.is_extern && !added_extern_methods.contains(&method_def.identifier.identifier()) {
                    boiler_plate += &ASMBuilder::line(&format!("extern {}", &method_def.identifier.identifier()));
                    added_extern_methods.push(method_def.identifier.identifier());
                }
            }
        );

        boiler_plate += &ASMBuilder::line("");

        if self.require_main {
            // search for main function
            let main_entry = self.top_level_scope.iter().filter(|a| matches!(a, AbstractSyntaxTreeNode::MethodDefinition(md) if md.identifier.identifier() == "main")).collect::<Vec<&AbstractSyntaxTreeNode>>();

            let value: Result<String, ASMGenerateError> = match main_entry.len() {
                0 => return Err(ASMGenerateError::EntryPointNotFound),
                1 => {
                    if let Some(AbstractSyntaxTreeNode::MethodDefinition(main)) = main_entry.first() {
                        if main.is_extern {
                            return Err(ASMGenerateError::EntryPointNotFound);
                        }

                        self.stack.clear_stack();
                        let mut meta = MetaInfo {
                            file_position: main.file_position.clone(),
                            target_os: self.target_os.clone(),
                            static_type_information: StaticTypeContext::new(&self.top_level_scope),
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
                _ => return Err(ASMGenerateError::MultipleEntryPointsFound(main_entry.iter().map(|t| t.file_position()).collect::<Vec<_>>()))
            };

            Ok(value?)
        } else {
            self.require_main = true;

            let method_definitions = self.top_level_scope.iter().filter(|t| matches!(t, AbstractSyntaxTreeNode::MethodDefinition(_))).cloned().collect::<Vec<_>>();
            let mut main_stack = self.top_level_scope.iter().filter(|t| !matches!(t, AbstractSyntaxTreeNode::Import(_) | AbstractSyntaxTreeNode::MethodDefinition(_))).cloned().collect::<Vec<AbstractSyntaxTreeNode>>();
            // last element of main stack via pattern matching
            if let [.., last] = &main_stack[..] {
                if !matches!(last, AbstractSyntaxTreeNode::Return(_)) {
                     main_stack.push(AbstractSyntaxTreeNode::Return(Return::num_0()));
                }
            }
            let main_function = AbstractSyntaxTreeNode::MethodDefinition(MethodDefinition {
                identifier: LValue::Identifier(Identifier { name: "main".to_string() }),
                return_type: Type::Integer(IntegerType::I32, Mutability::Immutable),
                arguments: vec![],
                stack: main_stack,
                is_extern: false,
                file_position: FilePosition::default(),
            });

            self.top_level_scope.clear();
            let imports = self.top_level_scope.iter().filter(|t| matches!(t, AbstractSyntaxTreeNode::Import(_))).cloned().collect::<Vec<AbstractSyntaxTreeNode>>();

            method_definitions.iter().for_each(|t| self.top_level_scope.push(t.clone()));
            imports.iter().for_each(|t| self.top_level_scope.push(t.clone()));

            self.top_level_scope.push(main_function);
            self.generate()
        }
    }

    fn generate_method_definitions(&mut self) -> Result<String, ASMGenerateError> {
        let mut method_definitions = String::new();

        for node in &self.top_level_scope {
            let mut meta = MetaInfo {
                file_position: node.file_position().clone(),
                target_os: self.target_os.clone(),
                static_type_information: StaticTypeContext::new(&self.top_level_scope),
            };


            if let AbstractSyntaxTreeNode::MethodDefinition(method_definition) = node {
                if !method_definition.is_extern && method_definition.identifier.identifier() != "main" {
                    self.stack.clear_stack();

                    let calling_convention = calling_convention_from(method_definition, &self.target_os);

                    for (index, argument) in method_definition.arguments.iter().enumerate() {
                        let parameter = Parameter {
                            identifier: argument.identifier.clone(),
                            ty: argument.ty.clone(),
                            register: calling_convention[index][0].clone(),
                            mutability: argument.ty.mutable(),
                            file_position: method_definition.file_position.clone(),
                        };

                        self.stack.variables.push(StackLocation {
                            position: self.stack.stack_position,
                            size: argument.ty.byte_size(),
                            elements: 1,
                            name: argument.identifier.clone(),
                        });

                        self.stack.stack_position += argument.ty.byte_size();

                        meta.static_type_information.context.push(Variable {
                            l_value: argument.identifier.clone(),
                            mutability: parameter.mutability,
                            ty: Some(argument.ty.clone()),
                            define: true,
                            assignable: Assignable::Parameter(parameter),
                            file_position: method_definition.file_position.clone(),
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

impl From<(Vec<AbstractSyntaxTreeNode>, TargetOS)> for ASMGenerator {
    fn from(value: (Vec<AbstractSyntaxTreeNode>, TargetOS)) -> Self {
        ASMGenerator {
            top_level_scope: value.0,
            stack: Stack::default(),
            require_main: false,
            target_os: value.1,
        }
    }
}

impl From<(Vec<AbstractSyntaxTreeNode>, TargetOS, bool)> for ASMGenerator {
    fn from(value: (Vec<AbstractSyntaxTreeNode>, TargetOS, bool)) -> Self {
        ASMGenerator {
            top_level_scope: value.0,
            stack: Stack::default(),
            require_main: value.2,
            target_os: value.1,
        }
    }
}
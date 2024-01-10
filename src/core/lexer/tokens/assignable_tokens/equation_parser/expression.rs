use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::generator::{LastUnchecked, Stack};
use crate::core::code_generator::registers::{Bit32, Bit64, GeneralPurposeRegister};
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::tokens::assignable_token::AssignableToken;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;
use crate::core::lexer::tokens::name_token::NameToken;
use crate::core::lexer::type_token::{Float, InferTypeError, Integer, TypeToken};

#[derive(Clone, PartialEq, Debug)]
pub enum PointerArithmetic {
    /// *
    Asterics,
    /// &
    Ampersand,
}

#[derive(Clone, PartialEq, Debug)]
pub enum PrefixArithmetic {
    #[allow(unused)]
    Operation(Operator),
    // For example the "-" like let a = -5;
    PointerArithmetic(PointerArithmetic),
    Cast(TypeToken),
}

#[derive(Clone, PartialEq)]
#[allow(unused)]
pub struct Expression {
    pub lhs: Option<Box<Expression>>,
    pub rhs: Option<Box<Expression>>,
    pub operator: Operator,
    pub prefix_arithmetic: Option<PrefixArithmetic>,
    pub value: Option<Box<AssignableToken>>,
    pub positive: bool,
}

impl Expression {
    pub fn pointers(&self) -> Vec<PointerArithmetic> {
        let mut pointer_arithmetic = vec![];
        for prefix in &self.prefix_arithmetic {
            if let PrefixArithmetic::PointerArithmetic(p) = &prefix {
                pointer_arithmetic.push(p.clone());
            }
        }

        pointer_arithmetic
    }

    pub fn pointer(&self) -> Option<PointerArithmetic> {
        if let Some(PrefixArithmetic::PointerArithmetic(a)) = &self.prefix_arithmetic {
            return Some(a.clone());
        }

        None
    }
}

impl Default for Expression {
    fn default() -> Self {
        Self {
            lhs: None,
            rhs: None,
            operator: Operator::Noop,
            value: None,
            positive: true,
            prefix_arithmetic: None,
        }
    }
}

impl Debug for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct_formatter = f.debug_struct("Expression");

        if let Some(lhs) = &self.lhs {
            debug_struct_formatter.field("lhs", lhs);
        }

        debug_struct_formatter.field("operator", &self.operator);

        if let Some(rhs) = &self.rhs {
            debug_struct_formatter.field("rhs", rhs);
        }

        if let Some(value) = &self.value {
            debug_struct_formatter.field("value", value);
        }

        debug_struct_formatter.field("positive", &self.positive);
        let prefix_arithmetic = self.prefix_arithmetic.iter().map(|a| a.to_string()).collect::<String>();

        debug_struct_formatter.field("prefix_arithmetic", &prefix_arithmetic);

        debug_struct_formatter.finish()
    }
}

impl Display for PrefixArithmetic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PrefixArithmetic::Operation(operation) => operation.to_string(),
            PrefixArithmetic::PointerArithmetic(p) => p.to_string(),
            PrefixArithmetic::Cast(c) => format!("({c})")
        })
    }
}

impl Display for PointerArithmetic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PointerArithmetic::Asterics => "*".to_string(),
            PointerArithmetic::Ampersand => "&".to_string()
        })
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let sign = if self.positive { "".to_string() } else { "-".to_string() };
        let prefix_arithmetic = self.prefix_arithmetic.iter().rev().map(|a| a.to_string()).collect::<String>();

        match (&self.lhs, &self.rhs) {
            (Some(lhs), Some(rhs)) => {
                write!(f, "{}{}({} {} {})", prefix_arithmetic, sign, lhs, &self.operator, rhs)
            }
            _ => {
                if let Some(ass) = &self.value {
                    write!(f, "{}{}{}", prefix_arithmetic, sign, ass)
                } else {
                    write!(f, "Some error. No lhs and rhs and no value found")
                }
            }
        }
    }
}


impl From<Option<Box<AssignableToken>>> for Expression {
    fn from(value: Option<Box<AssignableToken>>) -> Self {
        Expression {
            value,
            ..Default::default()
        }
    }
}

impl ToASM for Expression {
    fn to_asm(&self, stack: &mut Stack, meta: &mut MetaInfo) -> Result<String, ASMGenerateError> {
        if let Some(value) = &self.value { // no lhs and rhs
            let mut target = String::new();

            if let Some(pointer) = self.pointer() {
                target += &ASMBuilder::push(&Self::pointer_arithmetic_to_asm(pointer, value, stack, meta)?);
            } else {
                target += &value.to_asm(stack, meta)?;
            }


            return Ok(target);
        }

        let mut target = String::new();
        target += &ASMBuilder::ident(&ASMBuilder::comment_line(&format!("{}", self)));

        match (&self.lhs, &self.rhs) {
            (Some(lhs), Some(rhs)) => {
                // first optimization. use every register
                if let (Some(inner_lhs_l), Some(inner_lhs_r), Some(inner_rhs_l), Some(inner_rhs_r)) = (&lhs.lhs, &lhs.rhs, &rhs.lhs, &rhs.rhs) {
                    if let (Some(_), Some(_), Some(_), Some(_)) = (
                        &inner_lhs_l.value,
                        &inner_lhs_r.value,
                        &inner_rhs_l.value,
                        &inner_rhs_r.value) {
                        // two expressions containing two values
                        stack.register_to_use.push(Bit32::Ecx.into());
                        target += &ASMBuilder::push(&lhs.to_asm(stack, meta)?.to_string());
                        stack.register_to_use.pop();

                        stack.register_to_use.push(Bit32::Edi.into());
                        target += &ASMBuilder::push(&rhs.to_asm(stack, meta)?.to_string());
                        stack.register_to_use.pop();

                        target += &ASMBuilder::ident_line(&format!("{} ecx, edi", self.operator.to_asm(stack, meta)?));
                        target += &ASMBuilder::mov_ident_line(Bit32::Eax, Bit32::Ecx);

                        return Ok(target);
                    }
                }
                match (&lhs.value, &rhs.value) {
                    (Some(_), Some(_)) => { // 2 + 3
                        stack.register_to_use.push(Bit32::Eax.into());
                        let destination_register = stack.register_to_use.last()?;
                        if lhs.is_pointer() {
                            target += &ASMBuilder::push(&lhs.to_asm(stack, meta)?);
                        } else {
                            target += &ASMBuilder::mov_ident_line(&destination_register, lhs.to_asm(stack, meta)?);
                        };
                        stack.register_to_use.pop();

                        stack.register_to_use.push(Bit32::Edx.into());
                        let target_register = stack.register_to_use.last()?;
                        if rhs.is_pointer() {
                            target += &ASMBuilder::push(&rhs.to_asm(stack, meta)?);
                            target += &ASMBuilder::ident_line(&format!("{} {destination_register}, {target_register}", self.operator.to_asm(stack, meta)?));
                        } else {
                            target += &ASMBuilder::ident_line(&format!("{} {destination_register}, {}", self.operator.to_asm(stack, meta)?, &rhs.to_asm(stack, meta)?));
                        };
                        stack.register_to_use.pop();
                        target += &ASMBuilder::mov_ident_line(stack.register_to_use.last()?, &destination_register);
                    }
                    (None, Some(_)) => { // (3 + 2) + 5
                        stack.register_to_use.push(Bit32::Eax.into());
                        let destination_register = stack.register_to_use.last()?;
                        target += &ASMBuilder::push(&lhs.to_asm(stack, meta)?.to_string());
                        stack.register_to_use.pop();

                        stack.register_to_use.push(Bit32::Edx.into());
                        let target_register = stack.register_to_use.last()?;
                        if rhs.is_pointer() {
                            target += &ASMBuilder::push(&rhs.to_asm(stack, meta)?);
                            target += &ASMBuilder::ident_line(&format!("{} {}, {}", self.operator.to_asm(stack, meta)?, Bit32::Eax, target_register));
                        } else {
                            target += &ASMBuilder::ident_line(&format!("{} {destination_register}, {}", self.operator.to_asm(stack, meta)?, rhs.to_asm(stack, meta)?));
                        };
                        stack.register_to_use.pop();
                        target += &ASMBuilder::mov_ident_line(stack.register_to_use.last()?, destination_register);
                    }
                    (Some(_), None) => { // 5 + (3 + 2)
                        stack.register_to_use.push(Bit32::Edx.into());
                        let target_register = stack.register_to_use.last()?;
                        target += &ASMBuilder::push(&rhs.to_asm(stack, meta)?.to_string());
                        stack.register_to_use.pop();

                        stack.register_to_use.push(Bit32::Eax.into());
                        let destination_register = stack.register_to_use.last()?;
                        if lhs.is_pointer() {
                            target += &ASMBuilder::push(&lhs.to_asm(stack, meta)?);
                            target += &ASMBuilder::ident_line(&format!("{} {}, {}", self.operator.to_asm(stack, meta)?, Bit32::Eax, Bit32::Edx));
                        } else {
                            target += &ASMBuilder::mov_ident_line(&destination_register, lhs.to_asm(stack, meta)?);
                            target += &ASMBuilder::ident_line(&format!("{} {destination_register}, {}", self.operator.to_asm(stack, meta)?, target_register));
                        };
                        stack.register_to_use.pop();
                        target += &ASMBuilder::mov_ident_line(stack.register_to_use.last()?, Bit32::Eax);
                    }
                    (None, None) => { // ((1 + 2) + (3 + 4)) + ((5 + 6) + (7 + 8)) // any depth
                        stack.register_to_use.push(Bit32::Edi.into());
                        target += &ASMBuilder::push(&lhs.to_asm(stack, meta)?.to_string());
                        stack.register_to_use.pop();

                        let register_to_push: GeneralPurposeRegister = if let Some(last_instruction) = extract_last_instruction(&target) {
                            let mut r = Bit64::Rdi.into();

                            if let Some(space_index) = last_instruction.chars().position(|a| a == ' ') {
                                if let Some(comma_index) = last_instruction.chars().position(|a| a == ',') {
                                    r = GeneralPurposeRegister::from_str(&last_instruction[space_index + 1..comma_index])?.to_64_bit_register();
                                }
                            }

                            r
                        } else {
                            Bit64::Rdi.into()
                        };

                        target += &ASMBuilder::ident_line(&format!("push {register_to_push}"));
                        target += &ASMBuilder::ident_line(&format!("xor {register_to_push}, {register_to_push}"));

                        stack.register_to_use.push(Bit32::Eax.into());
                        target += &ASMBuilder::push(&rhs.to_asm(stack, meta)?.to_string());
                        stack.register_to_use.pop();

                        target += &ASMBuilder::ident_line("push rax");
                        target += &ASMBuilder::ident_line("xor rax, rax");

                        target += &ASMBuilder::ident_line("pop rdi");
                        target += &ASMBuilder::ident_line("pop rax");

                        target += &ASMBuilder::ident_line(&format!("{} eax, edi", self.operator.to_asm(stack, meta)?));
                    }
                }
            }
            (_, _) => return Err(ASMGenerateError::NotImplemented { token: "Something went wrong. Neither rhs nor lhs are valid".to_string() })
        }

        Ok(target)
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        true
    }

    fn byte_size(&self, meta: &mut MetaInfo) -> usize {
        if let Some(ty) = self.traverse_type(meta) {
            return ty.byte_size();
        }

        0
    }

    fn before_label(&self, _stack: &mut Stack, _meta: &mut MetaInfo) -> Option<Result<String, ASMGenerateError>> {
        None
    }
}

fn extract_last_instruction(current_asm: &str) -> Option<String> {
    let last_instruction = current_asm.lines()
        .map(|a| a.trim())
        .filter(|a| !a.starts_with(';'))
        .last();

    if let Some(last_instruction) = last_instruction {
        return Some(last_instruction.to_string());
    }

    None
}

#[allow(unused)]
impl Expression {
    pub fn new(lhs: Option<Box<Expression>>, operator: Operator, rhs: Option<Box<Expression>>, value: Option<Box<AssignableToken>>) -> Self {
        Self {
            lhs,
            rhs,
            operator,
            prefix_arithmetic: None,
            value,
            positive: true,
        }
    }

    fn pointer_arithmetic_to_asm(pointer_arithmetic: PointerArithmetic, value: &AssignableToken, stack: &mut Stack, meta: &mut MetaInfo) -> Result<String, ASMGenerateError> {
        let mut target = String::new();
        let mut inner_source = String::new();
        let register_to_use = stack.register_to_use.last()?.to_64_bit_register().to_string();


        let mut child_is_pointer = false;

        if let Some(pointer) = value.pointer() {
            if let AssignableToken::ArithmeticEquation(a) = value {
                if let Some(child) = &a.value {
                    // must be met, if the value has a pointer itself
                    target += &ASMBuilder::push(&Self::pointer_arithmetic_to_asm(pointer, child, stack, meta)?);
                    inner_source = format!("QWORD [{}]", register_to_use);
                    child_is_pointer = true;
                }
            }
        } else {
            inner_source = value.to_asm(stack, meta)?;
        }

        match pointer_arithmetic {
            PointerArithmetic::Asterics => {
                target += &ASMBuilder::mov_ident_line(&register_to_use, inner_source);
                if !child_is_pointer {
                    target += &ASMBuilder::mov_ident_line(&register_to_use, format!("QWORD [{}]", register_to_use));
                }
            }
            PointerArithmetic::Ampersand => {
                target += &ASMBuilder::ident_line(
                    &format!("lea {}, {}", register_to_use, inner_source.replace("QWORD ", "").replace("DWORD ", ""))
                );
            }
        }


        Ok(target)
    }

    pub fn is_pointer(&self) -> bool {
        !self.pointers().is_empty()
    }

    pub fn traverse_type(&self, meta: &MetaInfo) -> Option<TypeToken> {
        self.traverse_type_resulted(&meta.static_type_information, &meta.code_line).ok()
    }

    pub fn traverse_type_resulted(&self, context: &StaticTypeContext, code_line: &CodeLine) -> Result<TypeToken, InferTypeError> {
        if let Some(value) = &self.value {
            let value_type = value.infer_type_with_context(context, code_line);
            let has_prefix_arithmetics = self.prefix_arithmetic.is_some();

            return if let (true, Ok(value_type)) = (has_prefix_arithmetics, &value_type) {
                let mut current_pointer_arithmetic: String = match value_type {
                    TypeToken::Custom(name) if name.name.starts_with(['*', '&']) => {
                        if let Some(index) = name.name.chars().position(|m| m.is_ascii_alphanumeric()) {
                            name.name[..index].to_string()
                        } else {
                            "".to_string()
                        }
                    }
                    _ => "".to_string()
                };

                let mut value_type = value_type.clone();

                for prefix_arithmetic in self.prefix_arithmetic.iter().rev() {
                    match prefix_arithmetic {
                        PrefixArithmetic::PointerArithmetic(PointerArithmetic::Asterics) if current_pointer_arithmetic.ends_with('*') => {
                            if let Some(new_ty) = value_type.pop_pointer() {
                                value_type = new_ty;
                                current_pointer_arithmetic = current_pointer_arithmetic.chars().collect::<Vec<char>>()[..current_pointer_arithmetic.len() - 1].iter().collect::<String>();
                            } else {
                                return Err(InferTypeError::IllegalDereference(*value.clone(), code_line.clone()));
                            }
                        }
                        PrefixArithmetic::PointerArithmetic(PointerArithmetic::Ampersand) => {
                            value_type = value_type.push_pointer();
                        }
                        PrefixArithmetic::PointerArithmetic(PointerArithmetic::Asterics) => {
                            // just using & in front of non pointer types is illegal. Dereferencing non pointers doesnt make any sense
                            return Err(InferTypeError::IllegalDereference(*value.clone(), code_line.clone()));
                        }
                        PrefixArithmetic::Cast(casting_to) => {
                            value_type = TypeToken::from_str(&casting_to.to_string())?;
                            // value_type = TypeToken::from_str(&format!("{current_pointer_arithmetic}{}", casting_to))?;
                        }
                        PrefixArithmetic::Operation(_) => {}
                    }
                }

                if value_type.is_pointer() {
                    Ok(TypeToken::Custom(NameToken { name: format!("{}", value_type) }))
                } else {
                    Ok(value_type)
                }
            } else {
                value_type
            };
        }

        Self::check_operator_compatibility(self.to_string(), &self.lhs, self.operator.clone(), &self.rhs, context, code_line)
    }

    fn check_operator_compatibility(error_message: String, lhs: &Option<Box<Expression>>, operator: Operator, rhs: &Option<Box<Expression>>, context: &StaticTypeContext, code_line: &CodeLine) -> Result<TypeToken, InferTypeError> {
        if let Some(lhs) = &lhs {
            if let Some(rhs) = &rhs {
                let lhs_type = lhs.traverse_type_resulted(context, code_line)?;
                let rhs_type = rhs.traverse_type_resulted(context, code_line)?;

                let mut base_type_matrix: HashMap<(TypeToken, Operator, TypeToken), TypeToken> = HashMap::new();
                base_type_matrix.insert((TypeToken::Custom(NameToken { name: "string".to_string() }), Operator::Add, TypeToken::Custom(NameToken { name: "string".to_string() })), TypeToken::Custom(NameToken { name: "*string".to_string() }));

                let integer_operation_matrix = Integer::operation_matrix();

                for row in integer_operation_matrix {
                    base_type_matrix.insert((row.0, row.1, row.2), row.3);
                }

                let float_operation_matrix = Float::operation_matrix();

                for row in float_operation_matrix {
                    base_type_matrix.insert((row.0, row.1, row.2), row.3);
                }

                base_type_matrix.insert((TypeToken::Bool, Operator::Add, TypeToken::Bool), TypeToken::Bool);
                base_type_matrix.insert((TypeToken::Bool, Operator::Sub, TypeToken::Bool), TypeToken::Bool);
                base_type_matrix.insert((TypeToken::Bool, Operator::Mul, TypeToken::Bool), TypeToken::Bool);
                base_type_matrix.insert((TypeToken::Bool, Operator::Div, TypeToken::Bool), TypeToken::Bool);

                if let Some(result_type) = base_type_matrix.get(&(lhs_type.clone(), operator.clone(), rhs_type.clone())) {
                    return Ok(result_type.clone());
                }

                return Err(InferTypeError::TypesNotCalculable(lhs_type, operator, rhs_type, code_line.clone()));
            }
        }

        Err(InferTypeError::UnresolvedReference(error_message, code_line.clone()))
    }

    pub fn set(&mut self, lhs: Option<Box<Expression>>, operation: Operator, rhs: Option<Box<Expression>>, value: Option<Box<AssignableToken>>) {
        self.lhs = lhs;
        self.rhs = rhs;
        self.operator = operation;
        self.value = value;
        self.prefix_arithmetic = None;
    }

    // pub fn set_keep_arithmetic(&mut self, lhs: Option<Box<Expression>>, operation: Operator, rhs: Option<Box<Expression>>, value: Option<Box<AssignableToken>>, positive: bool, prefix_arithmetic: Vec<PrefixArithmetic>) {
    //     self.lhs = lhs;
    //     self.rhs = rhs;
    //     self.operator = operation;
    //     self.positive = positive;
    //     self.value = value;
    //
    //     for prefix in prefix_arithmetic {
    //         self.prefix_arithmetic.push(prefix);
    //     }
    // }

    pub fn flip_value(&mut self) {
        if let Some(v) = &mut self.value {
            self.positive = !self.positive;
        }
    }
}
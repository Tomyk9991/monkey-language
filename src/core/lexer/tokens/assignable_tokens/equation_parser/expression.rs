use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::generator::Stack;
use crate::core::code_generator::register_destination::from_byte_size;
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::static_type_context::StaticTypeContext;
use crate::core::lexer::tokens::assignable_token::AssignableToken;
use crate::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;
use crate::core::lexer::tokens::name_token::NameToken;
use crate::core::lexer::type_token::{InferTypeError, TypeToken};

#[derive(Clone, PartialEq, Debug)]
pub enum PointerArithmetic {
    /// *
    Asterics,
    /// &
    Ampersand
}
#[derive(Clone, PartialEq)]
#[allow(unused)]
pub struct Expression {
    pub lhs: Option<Box<Expression>>,
    pub rhs: Option<Box<Expression>>,
    pub operator: Operator,
    pub pointer_arithmetic: Vec<PointerArithmetic>,
    pub value: Option<Box<AssignableToken>>,
    pub positive: bool,
}

impl Default for Expression {
    fn default() -> Self {
        Self {
            lhs: None,
            rhs: None,
            operator: Operator::Noop,
            pointer_arithmetic: vec![],
            value: None,
            positive: true,
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
        let pointer_arithmetic = self.pointer_arithmetic.iter().map(|a| match a {
            PointerArithmetic::Asterics => '*',
            PointerArithmetic::Ampersand => '&'
        }).collect::<String>();

        debug_struct_formatter.field("pointer_arithmetic", &pointer_arithmetic);

        debug_struct_formatter.finish()
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let sign =  if self.positive { "".to_string() } else { "-".to_string() };
        let pointer_prefix = self.pointer_arithmetic.iter().rev().map(|a| match a {
            PointerArithmetic::Asterics => '*',
            PointerArithmetic::Ampersand => '&'
        }).collect::<String>();

        match (&self.lhs, &self.rhs) {
            (Some(lhs), Some(rhs)) => {
                write!(f, "{}{}({} {} {})", pointer_prefix, sign, lhs, &self.operator, rhs)
            }
            _ => {
                if let Some(ass) = &self.value {
                    write!(f, "{}{}{}", pointer_prefix, sign, ass)
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
        if let Some(value) = &self.value { // this means, no children are provided. this is the actual value
            let pointer_iter = &self.pointer_arithmetic;
            let mut pointed = false;
            let mut inner_source = value.to_asm(stack, meta)?;
            let mut target = String::new();

            for arithmetic in pointer_iter.iter().rev() {
                pointed = true;
                match arithmetic {
                    PointerArithmetic::Asterics => {
                        target += &ASMBuilder::ident_line(
                            &format!("mov rax, {}", inner_source)
                        );
                    }
                    PointerArithmetic::Ampersand => {
                        target += &ASMBuilder::ident_line(
                            &format!("lea rax, {}", value.to_asm(stack, meta)?.replace("QWORD ", "").replace("DWORD ", ""))
                        );
                    }
                }

                inner_source = "QWORD [rax]".to_string();
            }

            if !pointed {
                target += &ASMBuilder::push(&format!("{}", value.to_asm(stack, meta)?));
            }

            return Ok(target)
        }

        let mut target = String::new();
        target += &ASMBuilder::ident(&ASMBuilder::comment_line(&format!("{}", self)));

        match (&self.rhs, &self.lhs) {
            (Some(lhs), Some(rhs)) => {
                match (&lhs.value, &rhs.value) {
                    (Some(_), Some(_)) => { // 2 + 3
                        let register_to_use_rhs = from_byte_size(rhs.byte_size(meta));
                        let register_to_use_lhs = from_byte_size(lhs.byte_size(meta));
                        assert_eq!(register_to_use_lhs, register_to_use_rhs); // todo: in type check make sure this is correct by checking if two types have the same byte length

                        let target_register = register_to_use_rhs;

                        target += &ASMBuilder::ident_line(&format!("mov {}, {}", target_register, rhs.to_asm(stack, meta)?));
                        target += &ASMBuilder::ident_line(&format!("{} {}, {}", self.operator.to_asm(stack, meta)?, target_register, lhs.to_asm(stack, meta)?));
                        target += &ASMBuilder::ident_line(&format!("mov {}, {}", stack.register_to_use, target_register));
                    }
                    (None, Some(_)) => { // (3 + 2) + 5
                        target += &ASMBuilder::push(&lhs.to_asm(stack, meta)?.to_string());
                        let register_to_use_rhs = from_byte_size(rhs.byte_size(meta));
                        target += &ASMBuilder::ident_line(&format!("{} {}, {}", self.operator.to_asm(stack, meta)?, register_to_use_rhs, rhs.to_asm(stack, meta)?));
                    }
                    (Some(_), None) => { // 5 + (3 + 2)
                        target += &ASMBuilder::push(&rhs.to_asm(stack, meta)?.to_string());
                        let register_to_use_lhs = from_byte_size(lhs.byte_size(meta)).replace('a', "d");
                        target += &ASMBuilder::ident_line(&format!("{} {}, {}", self.operator.to_asm(stack, meta)?, register_to_use_lhs, lhs.to_asm(stack, meta)?));
                    }
                    (None, None) => { // (5 + 3) + (9 + 8)
                        let register_to_use_rhs = from_byte_size(rhs.byte_size(meta)).replace('a', "d");
                        let register_to_use_lhs = from_byte_size(lhs.byte_size(meta));


                        stack.register_to_use = register_to_use_rhs.clone();
                        target += &ASMBuilder::push(&rhs.to_asm(stack, meta)?.to_string());

                        stack.register_to_use = register_to_use_lhs.clone();
                        target += &ASMBuilder::push(&lhs.to_asm(stack, meta)?.to_string());
                        stack.register_to_use = String::from("");

                        target += &ASMBuilder::ident_line(&format!("{} {}, {}", self.operator.to_asm(stack, meta)?, register_to_use_lhs, register_to_use_rhs));
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

#[allow(unused)]
impl Expression {
    pub fn new(lhs: Option<Box<Expression>>, operator: Operator, rhs: Option<Box<Expression>>, value: Option<Box<AssignableToken>>) -> Self {
        Self {
            lhs,
            rhs,
            operator,
            pointer_arithmetic: vec![],
            value,
            positive: true,
        }
    }

    pub fn traverse_type(&self, meta: &MetaInfo) -> Option<TypeToken> {
        self.traverse_type_resulted(&meta.static_type_information, &meta.code_line).ok()
    }

    pub fn traverse_type_resulted(&self, context: &StaticTypeContext, code_line: &CodeLine) -> Result<TypeToken, InferTypeError> {
        if let Some(value) = &self.value {
            let value_type = value.infer_type_with_context(context, code_line);
            let has_pointer_arithmetic = !self.pointer_arithmetic.is_empty();

            return if let (true, Ok(value_type)) = (has_pointer_arithmetic, &value_type) {
                let mut current_pointer_arithmetic: String = match value_type {
                    TypeToken::Custom(name) if name.name.starts_with(['*', '&']) => {
                        if let Some(index) = name.name.chars().position(|m| m.is_ascii_alphanumeric()) {
                            name.name[..index].to_string()
                        } else {
                            "".to_string()
                        }
                    },
                    _ => "".to_string()
                };

                let mut value_type = value_type.clone();
                for pointer_arithmetic in self.pointer_arithmetic.iter().rev() {
                    match pointer_arithmetic {
                        PointerArithmetic::Asterics if current_pointer_arithmetic.ends_with('*') => {
                            if let Some(new_ty) = value_type.pop_pointer() {
                                value_type = new_ty;
                            } else {
                                return Err(InferTypeError::IllegalDereference(*value.clone() ,code_line.clone()));
                            }
                        },
                        PointerArithmetic::Ampersand => {
                            value_type = value_type.push_pointer();
                        },
                        PointerArithmetic::Asterics => {
                            // just using & in front of non pointer types is illegal. Dereferencing non pointers doesnt make any sense
                            return Err(InferTypeError::IllegalDereference(*value.clone() ,code_line.clone()));
                        }
                    }
                }

                if value_type.is_pointer() {
                    Ok(TypeToken::Custom(NameToken { name: format!("{}", value_type) }))
                } else {
                    Ok(value_type)
                }
            } else {
                value_type
            }
        }

        if let Some(lhs) = &self.lhs {
            if let Some(rhs) = &self.rhs {
                let lhs_type = lhs.traverse_type_resulted(context, code_line)?;
                let rhs_type = rhs.traverse_type_resulted(context, code_line)?;

                let mut base_type_matrix: HashMap<(TypeToken, Operator, TypeToken), TypeToken> = HashMap::new();
                base_type_matrix.insert((TypeToken::Custom(NameToken { name: "string".to_string() }), Operator::Add, TypeToken::Custom(NameToken { name: "string".to_string() })), TypeToken::Custom(NameToken { name: "*string".to_string() }));

                base_type_matrix.insert((TypeToken::I32, Operator::Add, TypeToken::I32), TypeToken::I32);
                base_type_matrix.insert((TypeToken::I32, Operator::Sub, TypeToken::I32), TypeToken::I32);
                base_type_matrix.insert((TypeToken::I32, Operator::Mul, TypeToken::I32), TypeToken::I32);
                base_type_matrix.insert((TypeToken::I32, Operator::Div, TypeToken::I32), TypeToken::F32);

                base_type_matrix.insert((TypeToken::F32, Operator::Add, TypeToken::F32), TypeToken::F32);
                base_type_matrix.insert((TypeToken::F32, Operator::Sub, TypeToken::F32), TypeToken::F32);
                base_type_matrix.insert((TypeToken::F32, Operator::Mul, TypeToken::F32), TypeToken::F32);
                base_type_matrix.insert((TypeToken::F32, Operator::Div, TypeToken::F32), TypeToken::F32);

                base_type_matrix.insert((TypeToken::F32, Operator::Add, TypeToken::I32), TypeToken::F32);
                base_type_matrix.insert((TypeToken::F32, Operator::Sub, TypeToken::I32), TypeToken::F32);
                base_type_matrix.insert((TypeToken::F32, Operator::Mul, TypeToken::I32), TypeToken::F32);
                base_type_matrix.insert((TypeToken::F32, Operator::Div, TypeToken::I32), TypeToken::F32);

                base_type_matrix.insert((TypeToken::I32, Operator::Add, TypeToken::F32), TypeToken::F32);
                base_type_matrix.insert((TypeToken::I32, Operator::Sub, TypeToken::F32), TypeToken::F32);
                base_type_matrix.insert((TypeToken::I32, Operator::Mul, TypeToken::F32), TypeToken::F32);
                base_type_matrix.insert((TypeToken::I32, Operator::Div, TypeToken::F32), TypeToken::F32);

                base_type_matrix.insert((TypeToken::Bool, Operator::Add, TypeToken::Bool), TypeToken::Bool);
                base_type_matrix.insert((TypeToken::Bool, Operator::Sub, TypeToken::Bool), TypeToken::Bool);
                base_type_matrix.insert((TypeToken::Bool, Operator::Mul, TypeToken::Bool), TypeToken::Bool);
                base_type_matrix.insert((TypeToken::Bool, Operator::Div, TypeToken::Bool), TypeToken::Bool);

                if let Some(result_type) = base_type_matrix.get(&(lhs_type.clone(), self.operator.clone(), rhs_type.clone())) {
                    return Ok(result_type.clone());
                }

                return Err(InferTypeError::TypesNotCalculable(lhs_type, self.operator.clone(), rhs_type, code_line.clone()));
            }
        }

        Err(InferTypeError::UnresolvedReference(self.to_string(), code_line.clone()))
    }

    pub fn set(&mut self, lhs: Option<Box<Expression>>, operation: Operator, rhs: Option<Box<Expression>>, value: Option<Box<AssignableToken>>) {
        self.lhs = lhs;
        self.rhs = rhs;
        self.operator = operation;
        self.value = value;
    }

    pub fn flip_value(&mut self) {
        if let Some(v) = &mut self.value {
            self.positive = !self.positive;
        }
    }
}
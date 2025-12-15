use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::str::FromStr;

use crate::core::code_generator::{MetaInfo};
use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::operator::Operator;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::prefix_arithmetic::{PointerArithmetic, PrefixArithmetic};
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::types::float::FloatType;
use crate::core::model::types::integer::IntegerType;
use crate::core::model::types::mutability::Mutability;
use crate::core::model::types::ty::Type;
use crate::core::parser::abstract_syntax_tree_nodes::assignables::equation_parser::Equation;
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::boolean::Boolean;
use crate::core::parser::types::r#type::{InferTypeError};

impl Expression {
    /// identifier expects a variable name. Expressions per se dont have variables names, but the identifier function is called on a l_value
    pub fn identifier(&self) -> Option<String> {
        if let Some(value) = &self.value {
            return value.identifier();
        }

        None
    }
}

impl Parse for Expression {
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, Error> where Self: Sized {
        let mut s: Equation<'_> = Equation::new(tokens);
        let f = s.parse()?;

        Ok(ParseResult {
            result: *f.result,
            consumed: f.consumed,
        })
    }
}

impl Expression {
    pub fn set(&mut self, lhs: Option<Box<Expression>>, operation: Operator, rhs: Option<Box<Expression>>, value: Option<Box<Assignable>>) {
        self.lhs = lhs;
        self.rhs = rhs;
        self.operator = operation;
        self.positive = true;
        self.value = value;
        self.prefix_arithmetic = None;
    }

    pub fn flip_value(&mut self) {
        if let Some(Assignable::Integer(i)) = &mut self.value.as_deref_mut() {
            i.value = "-".to_string() + &i.value;
        }

        if let Some(Assignable::Float(f)) = &mut self.value.as_deref_mut() {
            f.value *= -1.0;
        }

        if self.value.is_some() {
            self.positive = !self.positive;
        }
    }
}
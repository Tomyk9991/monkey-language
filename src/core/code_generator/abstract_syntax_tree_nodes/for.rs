use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use crate::core::code_generator::{ASMGenerateError, MetaInfo, ToASM};
use crate::core::code_generator::asm_builder::ASMBuilder;
use crate::core::code_generator::asm_options::ASMOptions;
use crate::core::code_generator::asm_result::{ASMResult, ASMResultError, ASMResultVariance};
use crate::core::code_generator::generator::Stack;
use crate::core::io::code_line::CodeLine;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::model::abstract_syntax_tree_nodes::for_::{For, ForErr};
use crate::core::model::abstract_syntax_tree_nodes::variable::Variable;
use crate::core::model::scope::Scope;
use crate::core::model::types::mutability::Mutability;
use crate::core::model::types::ty::Type;
use crate::core::scanner::errors::EmptyIteratorErr;
use crate::core::scanner::scope::{PatternNotMatchedError, ScopeError};
use crate::core::scanner::static_type_context::StaticTypeContext;
use crate::core::scanner::abstract_syntax_tree_nodes::assignables::method_call::{dyck_language, DyckError};
use crate::core::scanner::abstract_syntax_tree_nodes::variable::{ParseVariableErr};
use crate::core::scanner::{Lines, TryParse};
use crate::core::scanner::types::r#type::{InferTypeError};
use crate::core::semantics::type_checker::{InferType, StaticTypeCheck};
use crate::core::semantics::type_checker::static_type_checker::{static_type_check, static_type_check_rec, StaticTypeCheckError};

impl PatternNotMatchedError for ForErr {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, ForErr::PatternNotMatched { .. })
    }
}


impl From<DyckError> for ForErr {
    fn from(s: DyckError) -> Self {
        ForErr::DyckLanguageErr { target_value: s.target_value, ordering: s.ordering }
    }
}

impl From<ParseVariableErr> for ForErr {
    fn from(value: ParseVariableErr) -> Self {
        ForErr::ParseVariableErr(value)
    }
}

impl From<AssignableError> for ForErr {
    fn from(value: AssignableError) -> Self {
        ForErr::AssignableErr(value)
    }
}

impl ToASM for For {
    fn to_asm<T: ASMOptions + 'static>(&self, stack: &mut Stack, meta: &mut MetaInfo, options: Option<T>) -> Result<ASMResult, ASMGenerateError> {
        let label1 = stack.create_label();
        let label2 = stack.create_label();
        let mut target = String::new();

        target += &ASMBuilder::ident(&ASMBuilder::comment_line(&format!("for ({}; {}; {})", self.initialization, self.condition, self.update)));
        let _ = self.initialization.to_asm(stack, meta, options.clone())?
            .apply_with(&mut target)
            .allow(ASMResultVariance::Inline)
            .allow(ASMResultVariance::MultilineResulted)
            .allow(ASMResultVariance::Multiline)
            .ast_node("for")
            .finish()?;

        target += &ASMBuilder::ident_line(&format!("jmp {label1}"));


        target += &ASMBuilder::line(&format!("{label2}:"));

        target += &stack.generate_scope(&self.stack, meta, options.clone())?;

        let _ = self.update.to_asm(stack, meta, options.clone())?
            .apply_with(&mut target)
            .allow(ASMResultVariance::Inline)
            .allow(ASMResultVariance::MultilineResulted)
            .allow(ASMResultVariance::Multiline)
            .ast_node("for")
            .finish()?;

        target += &ASMBuilder::line(&format!("{label1}:"));
        let general_purpose_register = self.condition.to_asm(stack, meta, options.clone())?
            .apply_with(&mut target)
            .allow(ASMResultVariance::MultilineResulted)
            .ast_node("for")
            .finish()?;

        if let Some(general_purpose_register) = general_purpose_register {
            target += &ASMBuilder::ident_line(&format!("cmp {general_purpose_register}, 0"));
            target += &ASMBuilder::ident_line(&format!("jne {label2}"));
        } else {
            return Err(ASMGenerateError::ASMResult(ASMResultError::UnexpectedVariance {
                expected: vec![ASMResultVariance::MultilineResulted],
                actual: ASMResultVariance::from(&self.condition.to_asm(stack, meta, options)?),
                ast_node: "for".to_string(),
            }));
        }


        Ok(ASMResult::Multiline(target))
    }

    fn is_stack_look_up(&self, _stack: &mut Stack, _meta: &MetaInfo) -> bool {
        false
    }

    fn byte_size(&self, meta: &mut MetaInfo) -> usize {
        self.initialization.byte_size(meta)
    }

    fn data_section(&self, stack: &mut Stack, meta: &mut MetaInfo) -> bool {
        let mut has_before_label_asm = false;
        let count_before = stack.label_count;

        for node in &self.stack {
            if node.data_section(stack, meta) {
                has_before_label_asm = true;
                stack.label_count -= 1;
            }
        }

        if self.initialization.data_section(stack, meta) {
            has_before_label_asm = true;
            stack.label_count -= 1;
        }

        if self.condition.data_section(stack, meta) {
            has_before_label_asm = true;
            stack.label_count -= 1;
        }

        if self.update.data_section(stack, meta) {
            has_before_label_asm = true;
            stack.label_count -= 1;
        }


        stack.label_count = count_before;


        has_before_label_asm
    }
}
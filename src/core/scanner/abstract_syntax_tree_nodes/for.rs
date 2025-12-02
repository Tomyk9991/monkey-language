use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use crate::core::lexer::token::Token;
use crate::core::io::code_line::CodeLine;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token_match::MatchResult;
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::abstract_syntax_tree_nodes::assignable::{Assignable, AssignableError};
use crate::core::model::abstract_syntax_tree_nodes::for_::For;
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
use crate::pattern;

#[derive(Debug)]
pub enum ForErr {
    PatternNotMatched { target_value: String },
    AssignableErr(AssignableError),
    ParseVariableErr(ParseVariableErr),
    ScopeErrorErr(ScopeError),
    DyckLanguageErr { target_value: String, ordering: Ordering },
    EmptyIterator(EmptyIteratorErr),
}

impl PatternNotMatchedError for ForErr {
    fn is_pattern_not_matched_error(&self) -> bool {
        matches!(self, ForErr::PatternNotMatched { .. })
    }
}

impl InferType for For {
    fn infer_type(&mut self, type_context: &mut StaticTypeContext) -> Result<(), InferTypeError> {
        Scope::infer_type(&mut self.stack, type_context)?;

        Ok(())
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

impl Error for ForErr {}

impl Display for ForErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ForErr::PatternNotMatched { target_value } =>
                format!("Pattern mot matched for: `{target_value}`\n\t for (initializiation; condition; update) {{}}"),
            ForErr::AssignableErr(a) => a.to_string(),
            ForErr::ParseVariableErr(a) => a.to_string(),
            ForErr::ScopeErrorErr(a) => a.to_string(),
            ForErr::EmptyIterator(e) => e.to_string(),
            ForErr::DyckLanguageErr { target_value, ordering } =>
                {
                    let error: String = match ordering {
                        Ordering::Less => String::from("Expected `)`"),
                        Ordering::Equal => String::from("Expected expression between `,`"),
                        Ordering::Greater => String::from("Expected `(`")
                    };
                    format!("\"{target_value}\": {error}")
                }
        })
    }
}

impl StaticTypeCheck for For {
    fn static_type_check(&self, type_context: &mut StaticTypeContext) -> Result<(), StaticTypeCheckError> {
        // add for header variables
        type_context.context.push(self.initialization.clone());

        let variables_len = type_context.context.len();
        let condition_type = self.condition.infer_type_with_context(type_context, &self.code_line)?;

        if !matches!(condition_type, Type::Bool(_)) {
            return Err(StaticTypeCheckError::InferredError(InferTypeError::MismatchedTypes {
                expected: Type::Bool(Mutability::Immutable),
                actual: condition_type,
                code_line: self.code_line.clone(),
            }));
        }

        static_type_check(&Scope {
            ast_nodes: vec![
                AbstractSyntaxTreeNode::Variable(self.initialization.clone()),
                AbstractSyntaxTreeNode::Variable(self.update.clone()),
            ],
        })?;

        if self.update.define {
            return Err(StaticTypeCheckError::InferredError(InferTypeError::DefineNotAllowed(self.update.clone(), self.code_line.clone())));
        }

        static_type_check_rec(&self.stack, type_context)?;

        let amount_pop = type_context.context.len() - variables_len;

        for _ in 0..amount_pop {
            let _ = type_context.context.pop();
        }
        
        Ok(())
    }
}

impl Parse for For {
    fn parse(tokens: &[TokenWithSpan], _: ParseOptions) -> Result<ParseResult<Self>, crate::core::lexer::error::Error> where Self: Sized, Self: Default {
        if let Some((MatchResult::Parse(variable))) = pattern!(tokens, For, ParenthesisOpen, @ parse Variable::<'=', ';'>,) {
            if let Some((MatchResult::Parse(condition))) = pattern!(&tokens[variable.consumed + 2..], @ parse Assignable,) {
                println!("{:#?}", variable);
                println!("{:#?}", condition);
            }
        }

        Err(crate::core::lexer::error::Error::first_unexpected_token(&tokens[0..1], &vec![Token::For.into()]))
    }
}

impl TryParse for For {
    type Output = For;
    type Err = ForErr;

    fn try_parse(code_lines_iterator: &mut Lines<'_>) -> anyhow::Result<Self::Output, Self::Err> {
        let for_header = *code_lines_iterator
            .peek()
            .ok_or(ForErr::EmptyIterator(EmptyIteratorErr))?;

        let split_alloc = for_header.split(vec![' ']);
        let split_ref = split_alloc.iter().map(|a| a.as_str()).collect::<Vec<_>>();
        let split_values = dyck_language(&split_ref.join(" ").to_string(), [vec![], vec![';'], vec![]])?;

        if split_values.len() != 3 {
            return Err(ForErr::PatternNotMatched {
                target_value: for_header.line.clone(),
            })
        }

        let mut split_ref: Vec<&str> = vec![];
        let split = split_values[0].splitn(3, ' ').collect::<Vec<_>>();

        split.iter().for_each(|a| split_ref.push(a));
        split_ref.push(";");

        split_ref.push(&split_values[1]);
        split_ref.push(";");

        let mut split = split_values[2].rsplitn(3, ' ').collect::<Vec<_>>();
        split.reverse();
        split.iter().for_each(|a| split_ref.push(a));

        let mut nodes = vec![];
        if let ["for", "(", initialization, ";", condition, ";", update, ")", "{"] = &split_ref[..] {
            let initialization = Variable::<'=', ';'>::try_parse(&CodeLine::imaginary(&format!("{} ;", initialization)))?;
            let condition = Assignable::from_str(condition)?;
            let update = Variable::<'=', ';'>::try_parse(&CodeLine::imaginary(&format!("{} ;", update)))?;

            // consume the header
            let _ = code_lines_iterator.next();
            while code_lines_iterator.peek().is_some() {
                let node = Scope::try_parse(code_lines_iterator).map_err(ForErr::ScopeErrorErr)?;

                if let AbstractSyntaxTreeNode::ScopeEnding(_) = node {
                    break;
                }

                nodes.push(node);
            }

            return Ok(For {
                initialization,
                condition,
                update,
                stack: nodes,
                code_line: for_header.clone(),
            });
        }

        Err(ForErr::PatternNotMatched {
            target_value: for_header.line.to_string(),
        })
    }
}
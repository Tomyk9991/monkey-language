use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseOptions, ParseResult};
use crate::core::lexer::token_with_span::TokenWithSpan;
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::prefix_arithmetic::PrefixArithmetic;
use crate::core::model::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use crate::core::model::abstract_syntax_tree_nodes::assignables::object::Object;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::types::array::Array;
use crate::core::model::types::boolean::Boolean;
use crate::core::model::types::float::FloatAST;
use crate::core::model::types::integer::IntegerAST;
use crate::core::model::types::static_string::StaticString;

impl TryFrom<Result<ParseResult<Self>, Error>> for Assignable {
    type Error = Error;

    fn try_from(value: Result<ParseResult<Self>, crate::core::lexer::error::Error>,
    ) -> Result<Self, Self::Error> {
        match value {
            Ok(value) => Ok(value.result),
            Err(err) => Err(err),
        }
    }
}

impl Parse for Assignable {
    fn parse(tokens: &[TokenWithSpan], parse_options: ParseOptions) -> Result<ParseResult<Self>, Error> where Self: Sized, Self: Default {
        let expression_index = 1;

        let mut parsers = vec![
            |tokens: &[TokenWithSpan]| Object::parse(tokens, ParseOptions::default()).map(|r| ParseResult { result: Assignable::Object(r.result), consumed: r.consumed }),
            |tokens: &[TokenWithSpan]| StaticString::parse(tokens, ParseOptions::default()).map(|r| ParseResult { result: Assignable::String(r.result), consumed: r.consumed }),
            |tokens: &[TokenWithSpan]| FloatAST::parse(tokens, ParseOptions::default()).map(|r| ParseResult { result: Assignable::Float(r.result), consumed: r.consumed }),
            |tokens: &[TokenWithSpan]| IntegerAST::parse(tokens, ParseOptions::default()).map(|r| ParseResult { result: Assignable::Integer(r.result), consumed: r.consumed }),
            |tokens: &[TokenWithSpan]| MethodCall::parse(tokens, ParseOptions::default()).map(|r| ParseResult { result: Assignable::MethodCall(r.result), consumed: r.consumed }),
            |tokens: &[TokenWithSpan]| Boolean::parse(tokens, ParseOptions::default()).map(|r| ParseResult { result: Assignable::Boolean(r.result), consumed: r.consumed }),
            |tokens: &[TokenWithSpan]| Array::parse(tokens, ParseOptions::default()).map(|r| ParseResult { result: Assignable::Array(r.result), consumed: r.consumed }),
            |tokens: &[TokenWithSpan]| Identifier::parse(tokens, ParseOptions::default()).map(|r| ParseResult { result: Assignable::Identifier(r.result), consumed: r.consumed }),
        ];

        if !parse_options.ignore_expression {
            parsers.insert(expression_index, |tokens: &[TokenWithSpan]| Expression::parse(tokens, ParseOptions::default()).map(|r| ParseResult { result: Assignable::Expression(r.result), consumed: r.consumed }),);
        }

        if tokens.is_empty() {
            return Err(Error::UnexpectedEOF);
        }

        for (index, parser) in parsers.iter().enumerate() {
            match parser(tokens) {
                Ok(assignable) => {
                    if !parse_options.ignore_expression && index == expression_index {
                        // we parsed an expression, wrap it in an arithmetic equation
                        let expr = match &assignable.result {
                            Assignable::Expression(e) => e,
                            _ => unreachable!(),
                        };

                        if let (Some(value), None, None, None, None) = (&expr.value, &expr.prefix_arithmetic, &expr.index_operator, &expr.lhs, &expr.rhs) {
                            // it's just a single value, return that directly
                            let s = *(*value).clone();
                            return Ok(ParseResult {
                                result: s,
                                consumed: assignable.consumed,
                            });
                        }
                    }
                    return Ok(assignable)
                }
                Err(err) => match &err {
                    Error::InvalidCharacter(_) => {}
                    Error::UnexpectedToken(_) => {}
                    Error::ExpectedToken(_) => {}
                    Error::UnexpectedEOF => {}
                    Error::Callstack(_) => {
                        return Err(err)
                    }
                    Error::WithContext { .. } => {
                        return Err(err)
                    }
                }
            }
        }

        Err(Error::UnexpectedToken(tokens[0].clone()))
    }
}

impl Assignable {
    pub fn identifier(&self) -> Option<String> {
        match self {
            Assignable::Identifier(identifier) => Some(identifier.name.clone()),
            Assignable::Expression(value) => value.identifier(),
            _ => None,
        }
    }

    pub fn prefix_arithmetic(&self) -> Option<PrefixArithmetic> {
        match self {
            Assignable::Expression(a) => a.prefix_arithmetic.clone(),
            _ => None,
        }
    }
}
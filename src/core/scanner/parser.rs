use std::fmt::{Display, Formatter};
use crate::core::code_generator::conventions::calling_convention_from;
use crate::core::code_generator::target_os::TargetOS;
use crate::core::io::monkey_file::{MonkeyFile, MonkeyFileNew};
use crate::core::lexer::error::Error;
use crate::core::lexer::parse::{Parse, ParseResult};
use crate::core::lexer::token::Token;
use crate::core::lexer::token_with_span::{FilePosition, TokenWithSpan};
use crate::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use crate::core::model::scope::Scope;
use crate::core::scanner::static_type_context::StaticTypeContext;
use crate::core::scanner::types::r#type::InferTypeError;

#[derive(Clone, Debug)]
pub struct ASTParser {
    program: Vec<AbstractSyntaxTreeNode>
}

impl Display for ASTParser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        for astn in &self.program {
            buffer.push_str(&astn.to_string());
        }

        write!(f, "{}", buffer)
    }
}

impl Default for ASTParser {
    fn default() -> Self {
        Self {
            program: vec![],
        }
    }
}

impl ASTParser {
    pub fn parse(tokens: &[TokenWithSpan]) -> Result<ParseResult<Self>, Error> where Self: Sized + Clone {
        let mut tokens = tokens.to_vec();

        tokens.insert(0, TokenWithSpan {token: Token::CurlyBraceOpen, span: FilePosition::default() });
        tokens.push(TokenWithSpan {token: Token::CurlyBraceClose, span: FilePosition::default() });

        let scope = Scope::parse(&tokens)?;

        assert_eq!(scope.consumed - 2, tokens.len() - 2);

        Ok(ParseResult {
            result: ASTParser {
                program: scope.result.ast_nodes,
            },
            consumed: scope.consumed - 2, // reduce the open and close scope. those virtual tokens are not part of the program
        })


        // while iterator.peek().is_some() {
        //     let ast_node = Scope::try_parse(&mut iterator)?;
        //
        //     if let AbstractSyntaxTreeNode::Import(imported_monkey_file) = ast_node {
        //         let inner_scope = ASTParser::from(imported_monkey_file.monkey_file.clone()).parse()?;
        //
        //         // todo: this could result in collisions.
        //         for t in inner_scope.ast_nodes {
        //             program.ast_nodes.push(t);
        //         }
        //
        //         program.ast_nodes.push(AbstractSyntaxTreeNode::Import(imported_monkey_file));
        //     } else {
        //         program.ast_nodes.push(ast_node)
        //     }
        // }

        // top level type context. top level variables and all methods are visible
        // let mut type_context: StaticTypeContext = StaticTypeContext::new(&program.ast_nodes);
        //
        // let mut methods: Vec<*mut MethodDefinition> = Vec::new();
        //
        // for node in &mut program.ast_nodes {
        //     if let AbstractSyntaxTreeNode::MethodDefinition(method_ref) = node {
        //         methods.push(method_ref);
        //     }
        // }
        //
        // Self::infer_types(&mut program.ast_nodes, &mut type_context)?;
        //
        // for method in methods.iter_mut() {
        //     let calling_convention = calling_convention_from(unsafe { &(*(*method)) }, &TargetOS::Windows);
        //
        //     for (index, argument) in unsafe { &(*(*method)) }.arguments.iter().enumerate() {
        //         let parameter = Parameter {
        //             identifier: argument.name.clone(),
        //             ty: argument.ty.clone(),
        //             register: calling_convention[index][0].clone(),
        //             mutability: argument.ty.mutable(),
        //             code_line: unsafe { &(*(*method)) }.code_line.clone(),
        //         };
        //
        //         type_context.context.push(Variable {
        //             l_value: LValue::Identifier(argument.name.clone()),
        //             mutability: argument.ty.mutable(),
        //             ty: Some(argument.ty.clone()),
        //             define: true,
        //             assignable: Assignable::Parameter(parameter),
        //             code_line: unsafe { &(*(*method)) }.code_line.clone(),
        //         });
        //     }
        //
        //     Scope::infer_type(unsafe { &mut (*(*method)).stack }, &mut type_context)?;
        //
        //     for _ in 0..unsafe { &(*(*method)) }.arguments.len() {
        //         type_context.context.pop();
        //     }
        // }
        //
        // Ok(program)
    }
}

impl ASTParser {
    pub fn infer_types(scope: &mut Vec<AbstractSyntaxTreeNode>, type_context: &mut StaticTypeContext) -> Result<(), InferTypeError> {
        for node in scope {
            node.infer_type(type_context)?;
        }

        Ok(())
    }
}

// impl ASTParser {
//     /// Build scope ast node with the current file
//     /// # Returns
//     /// A `Scope` containing all the abstract_syntax_tree_nodes
//     /// # Errors
//     /// - If the file is empty
//     /// - If the file contains an invalid ast parts
//     pub fn parse(&mut self) -> Result<Scope, ScopeError> {
//         let mut scope = Scope {
//             ast_nodes: vec![],
//         };
//
//         let mut iterator = self.current_file.lines.iter().peekable();
//
//         while iterator.peek().is_some() {
//             let ast_node = Scope::try_parse(&mut iterator)?;
//
//             if let AbstractSyntaxTreeNode::Import(imported_monkey_file) = ast_node {
//                 let inner_scope = ASTParser::from(imported_monkey_file.monkey_file.clone()).parse()?;
//
//                 // todo: this could result in collisions.
//                 for t in inner_scope.ast_nodes {
//                     scope.ast_nodes.push(t);
//                 }
//
//                 scope.ast_nodes.push(AbstractSyntaxTreeNode::Import(imported_monkey_file));
//             } else {
//                 scope.ast_nodes.push(ast_node)
//             }
//         }
//
//         // top level type context. top level variables and all methods are visible
//         let mut type_context: StaticTypeContext = StaticTypeContext::new(&scope.ast_nodes);
//
//         let mut methods: Vec<*mut MethodDefinition> = Vec::new();
//
//         for node in &mut scope.ast_nodes {
//             if let AbstractSyntaxTreeNode::MethodDefinition(method_ref) = node {
//                 methods.push(method_ref);
//             }
//         }
//
//         Self::infer_types(&mut scope.ast_nodes, &mut type_context)?;
//
//         for method in methods.iter_mut() {
//             let calling_convention = calling_convention_from(unsafe { &(*(*method)) }, &TargetOS::Windows);
//
//             for (index, argument) in unsafe { &(*(*method)) }.arguments.iter().enumerate() {
//                 let parameter = Parameter {
//                     identifier: argument.name.clone(),
//                     ty: argument.ty.clone(),
//                     register: calling_convention[index][0].clone(),
//                     mutability: argument.ty.mutable(),
//                     code_line: unsafe { &(*(*method)) }.code_line.clone(),
//                 };
//
//                 type_context.context.push(Variable {
//                     l_value: LValue::Identifier(argument.name.clone()),
//                     mutability: argument.ty.mutable(),
//                     ty: Some(argument.ty.clone()),
//                     define: true,
//                     assignable: Assignable::Parameter(parameter),
//                     code_line: unsafe { &(*(*method)) }.code_line.clone(),
//                 });
//             }
//
//             Scope::infer_type(unsafe { &mut (*(*method)).stack }, &mut type_context)?;
//
//             for _ in 0..unsafe { &(*(*method)) }.arguments.len() {
//                 type_context.context.pop();
//             }
//         }
//
//         Ok(scope)
//     }
//
//     pub fn infer_types(scope: &mut Vec<AbstractSyntaxTreeNode>, type_context: &mut StaticTypeContext) -> Result<(), InferTypeError> {
//         for node in scope {
//             node.infer_type(type_context)?;
//         }
//
//         Ok(())
//     }
// }
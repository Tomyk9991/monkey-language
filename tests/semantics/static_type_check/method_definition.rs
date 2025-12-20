use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::token_with_span::FilePosition;
use monkey_language::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use monkey_language::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use monkey_language::core::model::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use monkey_language::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use monkey_language::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use monkey_language::core::model::abstract_syntax_tree_nodes::method_definition::{
    MethodArgument, MethodDefinition,
};
use monkey_language::core::model::abstract_syntax_tree_nodes::ret::Return;
use monkey_language::core::model::abstract_syntax_tree_nodes::variable::Variable;
use monkey_language::core::model::types::integer::{IntegerAST, IntegerType};
use monkey_language::core::model::types::mutability::Mutability;
use monkey_language::core::model::types::static_string::StaticString;
use monkey_language::core::model::types::ty::Type;
use monkey_language::core::parser::ast_parser::ASTParser;
use monkey_language::core::parser::types::r#type::InferTypeError;
use monkey_language::core::semantics::static_type_check::static_type_checker::{static_type_check, StaticTypeCheckError};
use monkey_language::core::semantics::type_infer::type_inferer::infer_type;

#[test]
fn static_type_check_method_definition() -> anyhow::Result<()> {
    let program = r#"
    fn test(test: *string): i32 {
        let a = test;
        return 5;
    }

    let result: i32 = test("test");
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(program)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;

    infer_type(&mut top_level_scope.result.program)?;
    static_type_check(&top_level_scope.result.program)?;

    println!("{:#?}", top_level_scope.result.program);

    let expected: Vec<AbstractSyntaxTreeNode> = vec![
        AbstractSyntaxTreeNode::MethodDefinition(MethodDefinition {
            identifier: LValue::Identifier(Identifier {
                name: "test".to_string(),
            }),
            return_type: Type::Integer(IntegerType::I32, Mutability::Immutable),
            arguments: vec![MethodArgument {
                identifier: LValue::Identifier(Identifier {
                    name: "test".to_string(),
                }),
                ty: Type::Custom(
                    Identifier {
                        name: "*string".to_string(),
                    },
                    Mutability::Immutable,
                ),
            }],
            stack: vec![
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier {
                        name: "a".to_string(),
                    }),
                    mutability: false,
                    ty: Some(Type::Custom(
                        Identifier {
                            name: "*string".to_string(),
                        },
                        Mutability::Immutable,
                    )),
                    define: true,
                    assignable: Assignable::Identifier(Identifier {
                        name: "test".to_string(),
                    }),
                    file_position: FilePosition {
                        line: 3..=3,
                        column: 9..=21,
                    },
                }),
                AbstractSyntaxTreeNode::Return(Return {
                    assignable: Some(Assignable::Integer(IntegerAST {
                        value: "5".to_string(),
                        ty: IntegerType::I32,
                    })),
                    file_position: FilePosition {
                        line: 4..=4,
                        column: 9..=17,
                    },
                }),
            ],
            is_extern: false,
            file_position: FilePosition {
                line: 2..=5,
                column: 5..=5,
            },
        }),
        AbstractSyntaxTreeNode::Variable(Variable {
            l_value: LValue::Identifier(Identifier {
                name: "result".to_string(),
            }),
            mutability: false,
            ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
            define: true,
            assignable: Assignable::MethodCall(MethodCall {
                identifier: LValue::Identifier(Identifier {
                    name: "test".to_string(),
                }),
                arguments: vec![Assignable::String(StaticString {
                    value: "\"test\"".to_string(),
                })],
                file_position: FilePosition {
                    line: 7..=7,
                    column: 23..=34,
                },
            }),
            file_position: FilePosition {
                line: 7..=7,
                column: 5..=35,
            },
        }),
    ];

    assert_eq!(expected, top_level_scope.result.program);

    Ok(())
}

#[test]
fn static_type_check_return_mismatch() -> anyhow::Result<()> {
    let program = r#"
    fn test(test: *string): i32 {
        let a = test;
        return a;
    }

    let result: i32 = test("test");
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(program)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;

    infer_type(&mut top_level_scope.result.program)?;
    let result = static_type_check(&top_level_scope.result.program);

    assert!(result.is_err());

    println!("{:#?}", result);

    if let Err(StaticTypeCheckError::InferredError(s)) = result {
        let b = *s;
        assert!(matches!(b, InferTypeError::MethodReturnArgumentTypeMismatch {
            expected: Type::Integer(IntegerType::I32, Mutability::Immutable),
            ..
        }));
    }

    Ok(())
}

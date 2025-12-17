use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::token_with_span::FilePosition;
use monkey_language::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use monkey_language::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use monkey_language::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use monkey_language::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use monkey_language::core::model::abstract_syntax_tree_nodes::method_definition::{MethodArgument, MethodDefinition};
use monkey_language::core::model::abstract_syntax_tree_nodes::variable::Variable;
use monkey_language::core::model::types::integer::{IntegerAST, IntegerType};
use monkey_language::core::model::types::mutability::Mutability;
use monkey_language::core::model::types::static_string::StaticString;
use monkey_language::core::model::types::ty::Type;
use monkey_language::core::parser::parser::ASTParser;
use monkey_language::core::semantics::type_infer::type_inferer::infer_type;

#[test]
fn function_test() -> anyhow::Result<()> {
    let function = r#"
    fn method_name(variable: i32, variable: i32): void {
        let function_variable_one = 10;
        let function_variable_two = 2;
    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;

    let expected = vec![
        AbstractSyntaxTreeNode::MethodDefinition(MethodDefinition {
            identifier: LValue::Identifier(Identifier { name: "method_name".to_string() }),
            return_type: Type::Void,
            arguments: vec![
                MethodArgument {
                    identifier: LValue::Identifier(Identifier { name: "variable".to_string() }),
                    ty: Type::Integer(IntegerType::I32, Mutability::Immutable),
                },
                MethodArgument {
                    identifier: LValue::Identifier(Identifier { name: "variable".to_string() }),
                    ty: Type::Integer(IntegerType::I32, Mutability::Immutable),
                },
            ],
            stack: vec![
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier { name: "function_variable_one".to_string() }),
                    mutability: false,
                    ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Integer(IntegerAST { value: "10".to_string(), ty: IntegerType::I32 }),
                    file_position: FilePosition { line: 3..=3, column: 9..=39 },
                }),
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier { name: "function_variable_two".to_string() }),
                    mutability: false,
                    ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Integer(IntegerAST { value: "2".to_string(), ty: IntegerType::I32 }),
                    file_position: FilePosition { line: 4..=4, column: 9..=38 },
                })],
            is_extern: false,
            file_position: FilePosition { line: 2..=5, column: 5..=5 }
        }),
    ];

    assert_eq!(expected, top_level_scope.result.program);
    Ok(())
}

#[test]
fn multiple_functions_test() -> anyhow::Result<()> {
    let function = r#"
    fn f(variable1: i32, variable2: Data): void
    {
        let function_variable_one = 10;
    }

    fn method_name(variable1: bool, variable2: *string): void {
        let function_variable_one = 10;
        let function_variable_two = 2;
    }


    fn method_without_parameters(): void {

    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;

    let expected = vec![
        AbstractSyntaxTreeNode::MethodDefinition(MethodDefinition {
            identifier: LValue::Identifier(Identifier { name: "f".to_string() }),
            return_type: Type::Void,
            arguments: vec![
                MethodArgument {
                    identifier: LValue::Identifier(Identifier { name: "variable1".to_string() }),
                    ty: Type::Integer(IntegerType::I32, Mutability::Immutable),
                },
                MethodArgument {
                    identifier: LValue::Identifier(Identifier { name: "variable2".to_string() }),
                    ty: Type::Custom(Identifier { name: "Data".to_string() }, Mutability::Immutable),
                },
            ],
            stack: vec![AbstractSyntaxTreeNode::Variable(Variable {
                l_value: LValue::Identifier(Identifier { name: "function_variable_one".to_string() }),
                mutability: false,
                ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                define: true,
                assignable: Assignable::Integer(IntegerAST { value: "10".to_string(), ty: IntegerType::I32 }),
                file_position: FilePosition { line: 4..=4, column: 9..=39 }
            })],
            is_extern: false,
            file_position: FilePosition { line: 2..=5, column: 5..=5 },
        }),
        AbstractSyntaxTreeNode::MethodDefinition(MethodDefinition {
            identifier: LValue::Identifier(Identifier { name: "method_name".to_string() }),
            return_type: Type::Void,
            arguments: vec![
                MethodArgument {
                    identifier: LValue::Identifier(Identifier { name: "variable1".to_string() }),
                    ty: Type::Bool(Mutability::Immutable),
                },
                MethodArgument {
                    identifier: LValue::Identifier(Identifier { name: "variable2".to_string() }),
                    ty: Type::Custom(Identifier { name: "*string".to_string() }, Mutability::Immutable),
                },
            ],
            stack: vec![
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier { name: "function_variable_one".to_string() }),
                    mutability: false,
                    ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Integer(IntegerAST { value: "10".to_string(), ty: IntegerType::I32 }),
                    file_position: FilePosition { line: 8..=8, column: 9..=39 }
                }),
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier { name: "function_variable_two".to_string() }),
                    mutability: false,
                    ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Integer(IntegerAST { value: "2".to_string(), ty: IntegerType::I32 }),
                    file_position: FilePosition { line: 9..=9, column: 9..=38 }
                })],
            is_extern: false,
            file_position: FilePosition { line: 7..=10, column: 5..=5 },
        }),
        AbstractSyntaxTreeNode::MethodDefinition(MethodDefinition {
            identifier: LValue::Identifier(Identifier { name: "method_without_parameters".to_string() }),
            return_type: Type::Void,
            arguments: vec![],
            stack: vec![],
            is_extern: false,
            file_position: FilePosition { line: 13..=15, column: 5..=5 }
        }),
    ];

    assert_eq!(expected, top_level_scope.result.program);
    Ok(())
}

#[test]
fn function_different_return_type_test() -> anyhow::Result<()> {
    let function = r#"
    fn f(variable1: i32, variable2: i32): void
    {
        let function_variable_zero = "Hallo";
    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;

    println!("{:#?}", top_level_scope);

    let expected = vec![
        AbstractSyntaxTreeNode::MethodDefinition(MethodDefinition {
            identifier: LValue::Identifier(Identifier { name: "f".to_string() }),
            return_type: Type::Void,
            arguments: vec![
                MethodArgument {
                    identifier: LValue::Identifier(Identifier { name: "variable1".to_string() }),
                    ty: Type::Integer(IntegerType::I32, Mutability::Immutable),
                },
                MethodArgument {
                    identifier: LValue::Identifier(Identifier { name: "variable2".to_string() }),
                    ty: Type::Integer(IntegerType::I32, Mutability::Immutable),
                },
            ],
            stack: vec![
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier { name: "function_variable_zero".to_string() }),
                    mutability: false,
                    ty: Some(Type::Custom(Identifier { name: String::from("*string") }, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::String(StaticString { value: "\"Hallo\"".to_string() }),
                    file_position: FilePosition { line: 4..=4, column: 9..=45 }
                }),
            ],
            is_extern: false,
            file_position: FilePosition { line: 2..=5, column: 5..=5 },
        }),
    ];

    assert_eq!(expected, top_level_scope.result.program);
    Ok(())
}

#[test]
fn function_in_function_test() -> anyhow::Result<()> {
    let function = r#"
    fn f(variable1: i32, variable2: i32): void
    {
        let function_variable_zero = "Hallo";
        fn method_name(variable1: i32, variable2: i32): void {
            let function_variable_one = 10;
            let function_variable_two = 2;
        }
    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;

    let expected = vec![
        AbstractSyntaxTreeNode::MethodDefinition(MethodDefinition {
            identifier: LValue::Identifier(Identifier { name: "f".to_string() }),
            return_type: Type::Void,
            arguments: vec![
                MethodArgument {
                    identifier: LValue::Identifier(Identifier { name: "variable1".to_string() }),
                    ty: Type::Integer(IntegerType::I32, Mutability::Immutable),
                },
                MethodArgument {
                    identifier: LValue::Identifier(Identifier { name: "variable2".to_string() }),
                    ty: Type::Integer(IntegerType::I32, Mutability::Immutable),
                }
            ],
            stack: vec![
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier { name: "function_variable_zero".to_string() }),
                    mutability: false,
                    ty: Some(Type::Custom(Identifier { name: String::from("*string") }, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::String(StaticString { value: "\"Hallo\"".to_string() }),
                    file_position: FilePosition { line: 4..=4, column: 9..=45 }
                }),
                AbstractSyntaxTreeNode::MethodDefinition(MethodDefinition {
                    identifier: LValue::Identifier(Identifier { name: "method_name".to_string() }),
                    return_type: Type::Void,
                    arguments: vec![
                        MethodArgument {
                            identifier: LValue::Identifier(Identifier { name: "variable1".to_string() }),
                            ty: Type::Integer(IntegerType::I32, Mutability::Immutable),
                        }, MethodArgument {
                            identifier: LValue::Identifier(Identifier { name: "variable2".to_string() }),
                            ty: Type::Integer(IntegerType::I32, Mutability::Immutable),
                        }
                    ],
                    stack: vec![
                        AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "function_variable_one".to_string() }), mutability: false, ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "10".to_string(), ty: IntegerType::I32 }), file_position: FilePosition { line: 6..=6, column: 13..=43 } }),
                        AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "function_variable_two".to_string() }), mutability: false, ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "2".to_string(), ty: IntegerType::I32 }), file_position: FilePosition { line: 7..=7, column: 13..=42 } }),
                    ],
                    is_extern: false,
                    file_position: FilePosition { line: 5..=8, column: 9..=9 },
                }),
            ],
            is_extern: false,
            file_position: FilePosition { line: 2..=9, column: 5..=5 },
        }),
    ];

    assert_eq!(expected, top_level_scope.result.program);
    Ok(())
}
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::token_with_span::FilePosition;
use monkey_language::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use monkey_language::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use monkey_language::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use monkey_language::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::operator::Operator;
use monkey_language::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use monkey_language::core::model::abstract_syntax_tree_nodes::if_::If;
use monkey_language::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use monkey_language::core::model::abstract_syntax_tree_nodes::variable::Variable;
use monkey_language::core::model::types::integer::{IntegerAST, IntegerType};
use monkey_language::core::model::types::mutability::Mutability;
use monkey_language::core::model::types::static_string::StaticString;
use monkey_language::core::model::types::ty::Type;
use monkey_language::core::parser::parser::ASTParser;
use monkey_language::core::semantics::type_infer::type_inferer::infer_type;

#[test]
fn if_test() -> anyhow::Result<()> {
    let program = r#"
    let variable = 5;
    if (variable) {
        let mut if_variable_one = 10;
        let if_variable_two = 2;
    }
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(program)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;

    let expected = vec![
        AbstractSyntaxTreeNode::Variable(Variable {
            l_value: LValue::Identifier(Identifier {
                name: String::from("variable"),
            }),
            mutability: false,
            ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
            define: true,
            assignable: Assignable::Integer(IntegerAST {
                value: "5".to_string(),
                ty: IntegerType::I32,
            }),
            file_position: FilePosition {
                line: 2..=2,
                column: 5..=21,
            },
        }),
        AbstractSyntaxTreeNode::If(If {
            condition: Assignable::Identifier(Identifier {
                name: String::from("variable"),
            }),
            if_stack: vec![
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier {
                        name: "if_variable_one".to_string(),
                    }),
                    mutability: true,
                    ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Integer(IntegerAST {
                        value: "10".to_string(),
                        ty: IntegerType::I32,
                    }),
                    file_position: FilePosition {
                        line: 4..=4,
                        column: 9..=37,
                    },
                }),
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier {
                        name: "if_variable_two".to_string(),
                    }),
                    mutability: false,
                    ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Integer(IntegerAST {
                        value: "2".to_string(),
                        ty: IntegerType::I32,
                    }),
                    file_position: FilePosition {
                        line: 5..=5,
                        column: 9..=32,
                    },
                }),
            ],
            else_stack: None,
            file_position: FilePosition {
                line: 3..=6,
                column: 5..=5,
            },
        }),
    ];

    println!("{:?}", top_level_scope.result.program);
    println!("{:?}", expected);

    assert_eq!(expected, top_level_scope.result.program);

    Ok(())
}

#[test]
fn multiple_if_test() -> anyhow::Result<()> {
    let program = r#"
    let variable1 = 5;
    let variable2 = 5;
    let variable3 = 5;
    if (variable1) {
        let if_variable_one = 10;
        let if_variable_two = 2;
    }

    if (variable2) {
        let if_variable_one = 10;
        let if_variable_two = 2;
    }


    if (variable3) {

    }
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(program)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;

    let expected = vec![
        AbstractSyntaxTreeNode::Variable(Variable {
            l_value: LValue::Identifier(Identifier {
                name: String::from("variable1"),
            }),
            mutability: false,
            ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
            define: true,
            assignable: Assignable::Integer(IntegerAST {
                value: "5".to_string(),
                ty: IntegerType::I32,
            }),
            file_position: FilePosition {
                line: 2..=2,
                column: 5..=22,
            },
        }),
        AbstractSyntaxTreeNode::Variable(Variable {
            l_value: LValue::Identifier(Identifier {
                name: String::from("variable2"),
            }),
            mutability: false,
            ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
            define: true,
            assignable: Assignable::Integer(IntegerAST {
                value: "5".to_string(),
                ty: IntegerType::I32,
            }),
            file_position: FilePosition {
                line: 3..=3,
                column: 5..=22,
            },
        }),
        AbstractSyntaxTreeNode::Variable(Variable {
            l_value: LValue::Identifier(Identifier {
                name: String::from("variable3"),
            }),
            mutability: false,
            ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
            define: true,
            assignable: Assignable::Integer(IntegerAST {
                value: "5".to_string(),
                ty: IntegerType::I32,
            }),
            file_position: FilePosition {
                line: 4..=4,
                column: 5..=22,
            },
        }),
        AbstractSyntaxTreeNode::If(If {
            condition: Assignable::Identifier(Identifier {
                name: String::from("variable1"),
            }),
            if_stack: vec![
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier {
                        name: "if_variable_one".to_string(),
                    }),
                    mutability: false,
                    ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Integer(IntegerAST {
                        value: "10".to_string(),
                        ty: IntegerType::I32,
                    }),
                    file_position: FilePosition {
                        line: 6..=6,
                        column: 9..=33,
                    },
                }),
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier {
                        name: "if_variable_two".to_string(),
                    }),
                    mutability: false,
                    ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Integer(IntegerAST {
                        value: "2".to_string(),
                        ty: IntegerType::I32,
                    }),
                    file_position: FilePosition {
                        line: 7..=7,
                        column: 9..=32,
                    },
                }),
            ],
            else_stack: None,
            file_position: FilePosition {
                line: 5..=8,
                column: 5..=5,
            },
        }),
        AbstractSyntaxTreeNode::If(If {
            condition: Assignable::Identifier(Identifier {
                name: String::from("variable2"),
            }),
            if_stack: vec![
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier {
                        name: "if_variable_one".to_string(),
                    }),
                    mutability: false,
                    ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Integer(IntegerAST {
                        value: "10".to_string(),
                        ty: IntegerType::I32,
                    }),
                    file_position: FilePosition {
                        line: 11..=11,
                        column: 9..=33,
                    },
                }),
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier {
                        name: "if_variable_two".to_string(),
                    }),
                    mutability: false,
                    ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Integer(IntegerAST {
                        value: "2".to_string(),
                        ty: IntegerType::I32,
                    }),
                    file_position: FilePosition {
                        line: 12..=12,
                        column: 9..=32,
                    },
                }),
            ],
            else_stack: None,
            file_position: FilePosition {
                line: 10..=13,
                column: 5..=5,
            },
        }),
        AbstractSyntaxTreeNode::If(If {
            condition: Assignable::Identifier(Identifier {
                name: String::from("variable3"),
            }),
            if_stack: vec![],
            else_stack: None,
            file_position: FilePosition {
                line: 16..=18,
                column: 5..=5,
            },
        }),
    ];

    assert_eq!(expected, top_level_scope.result.program);
    Ok(())
}

#[test]
fn if_else_test() -> anyhow::Result<()> {
    let program = r#"let variable = 1;
    if (variable) {
        let mut   if_variable_one = 10;
        let if_variable_two = 2;
    } else {
        let else_variable_one = 10;
        let mut else_variable_two = 2;
    }
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(program)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;

    let expected = vec![
        AbstractSyntaxTreeNode::Variable(Variable {
            l_value: LValue::Identifier(Identifier {
                name: String::from("variable"),
            }),
            mutability: false,
            ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
            define: true,
            assignable: Assignable::Integer(IntegerAST {
                value: "1".to_string(),
                ty: IntegerType::I32,
            }),
            file_position: FilePosition { line: 1..=1, column: 1..=17 },
        }),
        AbstractSyntaxTreeNode::If(If {
        condition: Assignable::Identifier(Identifier {
            name: String::from("variable"),
        }),
        if_stack: vec![
            AbstractSyntaxTreeNode::Variable(Variable {
                l_value: LValue::Identifier(Identifier {
                    name: "if_variable_one".to_string(),
                }),
                mutability: true,
                ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                define: true,
                assignable: Assignable::Integer(IntegerAST {
                    value: "10".to_string(),
                    ty: IntegerType::I32,
                }),
                file_position: FilePosition { line: 3..=3, column: 9..=39 },
            }),
            AbstractSyntaxTreeNode::Variable(Variable {
                l_value: LValue::Identifier(Identifier {
                    name: "if_variable_two".to_string(),
                }),
                mutability: false,
                ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                define: true,
                assignable: Assignable::Integer(IntegerAST {
                    value: "2".to_string(),
                    ty: IntegerType::I32,
                }),
                file_position: FilePosition { line: 4..=4, column: 9..=32 },
            }),
        ],
        else_stack: Some(vec![
            AbstractSyntaxTreeNode::Variable(Variable {
                l_value: LValue::Identifier(Identifier {
                    name: "else_variable_one".to_string(),
                }),
                mutability: false,
                ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                define: true,
                assignable: Assignable::Integer(IntegerAST {
                    value: "10".to_string(),
                    ty: IntegerType::I32,
                }),
                file_position: FilePosition { line: 6..=6, column: 9..=35 },
            }),
            AbstractSyntaxTreeNode::Variable(Variable {
                l_value: LValue::Identifier(Identifier {
                    name: "else_variable_two".to_string(),
                }),
                mutability: true,
                ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                define: true,
                assignable: Assignable::Integer(IntegerAST {
                    value: "2".to_string(),
                    ty: IntegerType::I32,
                }),
                file_position: FilePosition { line: 7..=7, column: 9..=38 },
            }),
        ]),
        file_position: FilePosition { line: 2..=8, column: 5..=5 },
    })];

    assert_eq!(expected, top_level_scope.result.program);
    Ok(())
}

#[test]
fn function_in_function_test() -> anyhow::Result<()> {
    let program = r#"
    let hallo = 5;
    if (hallo) {
        let if_stack_variable = 5 / 2;

        if(if_stack_variable) {
            let nested_if_stack_variable = 13;
        } else {let nested_else_stack_variable = "nice";}
    } else {
        let else_stack_variable = "hallo";
    }
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(program)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;

    let expected = vec![
        AbstractSyntaxTreeNode::Variable(Variable {
            l_value: LValue::Identifier(Identifier {
                name: String::from("hallo"),
            }),
            mutability: false,
            ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
            define: true,
            assignable: Assignable::Integer(IntegerAST {
                value: "5".to_string(),
                ty: IntegerType::I32,
            }),
            file_position: FilePosition { line: 2..=2, column: 5..=18 },
        }),
        AbstractSyntaxTreeNode::If(If {
        condition: Assignable::Identifier(Identifier {
            name: "hallo".to_string(),
        }),
        if_stack: vec![
            AbstractSyntaxTreeNode::Variable(Variable {
                l_value: LValue::Identifier(Identifier {
                    name: "if_stack_variable".to_string(),
                }),
                mutability: false,
                ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                define: true,
                assignable: Assignable::Expression(Expression {
                    lhs: Some(Box::new(Expression {
                        lhs: None,
                        rhs: None,
                        operator: Operator::Noop,
                        prefix_arithmetic: None,
                        value: Some(Box::new(Assignable::Integer(IntegerAST {
                            value: "5".to_string(),
                            ty: IntegerType::I32,
                        }))),
                        index_operator: None,
                        positive: true,
                    })),
                    operator: Operator::Div,
                    rhs: Some(Box::new(Expression {
                        lhs: None,
                        rhs: None,
                        operator: Operator::Noop,
                        prefix_arithmetic: None,
                        value: Some(Box::new(Assignable::Integer(IntegerAST {
                            value: "2".to_string(),
                            ty: IntegerType::I32,
                        }))),
                        index_operator: None,
                        positive: true,
                    })),
                    positive: true,
                    value: None,
                    prefix_arithmetic: None,
                    index_operator: None,
                }),
                file_position: FilePosition { line: 4..=4, column: 9..=38 },
            }),
            AbstractSyntaxTreeNode::If(If {
                condition: Assignable::Identifier(Identifier {
                    name: "if_stack_variable".to_string(),
                }),
                if_stack: vec![AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier {
                        name: "nested_if_stack_variable".to_string(),
                    }),
                    mutability: false,
                    ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Integer(IntegerAST {
                        value: "13".to_string(),
                        ty: IntegerType::I32,
                    }),
                    file_position: FilePosition { line: 7..=7, column: 13..=46 },
                })],
                else_stack: Some(vec![AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier {
                        name: "nested_else_stack_variable".to_string(),
                    }),
                    mutability: false,
                    ty: Some(Type::Custom(
                        Identifier {
                            name: String::from("*string"),
                        },
                        Mutability::Immutable,
                    )),
                    define: true,
                    assignable: Assignable::String(StaticString {
                        value: "\"nice\"".to_string(),
                    }),
                    file_position: FilePosition { line: 8..=8, column: 17..=56 },
                })]),
                file_position: FilePosition { line: 6..=8, column: 9..=57 },
            }),
        ],
        else_stack: Some(vec![AbstractSyntaxTreeNode::Variable(Variable {
            l_value: LValue::Identifier(Identifier {
                name: "else_stack_variable".to_string(),
            }),
            mutability: false,
            ty: Some(Type::Custom(
                Identifier {
                    name: String::from("*string"),
                },
                Mutability::Immutable,
            )),
            define: true,
            assignable: Assignable::String(StaticString {
                value: "\"hallo\"".to_string(),
            }),
            file_position: FilePosition { line: 10..=10, column: 9..=42 },
        })]),
        file_position: FilePosition { line: 3..=11, column: 5..=5 },
    })];

    assert_eq!(expected, top_level_scope.result.program);
    Ok(())
}

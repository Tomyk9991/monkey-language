use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::token_with_span::FilePosition;
use monkey_language::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use monkey_language::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use monkey_language::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use monkey_language::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::operator::Operator;
use monkey_language::core::model::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use monkey_language::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use monkey_language::core::model::abstract_syntax_tree_nodes::if_::If;
use monkey_language::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use monkey_language::core::model::abstract_syntax_tree_nodes::method_definition::MethodDefinition;
use monkey_language::core::model::abstract_syntax_tree_nodes::ret::Return;
use monkey_language::core::model::abstract_syntax_tree_nodes::variable::Variable;
use monkey_language::core::model::types::boolean::Boolean;
use monkey_language::core::model::types::integer::{IntegerAST, IntegerType};
use monkey_language::core::model::types::mutability::Mutability;
use monkey_language::core::model::types::ty::Type;
use monkey_language::core::parser::ast_parser::ASTParser;
use monkey_language::core::semantics::type_infer::type_inferer::infer_type;

#[test]
fn infer_type_assignment() -> anyhow::Result<()> {
    let function = r#"
        let a = 1;
        let c = a;
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;

    let expected: Vec<AbstractSyntaxTreeNode> = vec![
        AbstractSyntaxTreeNode::Variable(Variable {
            l_value: LValue::Identifier(Identifier { name: "a".to_string() }),
            mutability: false,
            ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
            define: true,
            assignable: Assignable::Integer(IntegerAST { value: "1".to_string(), ty: IntegerType::I32 }),
            file_position: FilePosition { line: 2..=2, column: 9..=18 },
        }),
        AbstractSyntaxTreeNode::Variable(Variable {
            l_value: LValue::Identifier(Identifier { name: "c".to_string() }),
            mutability: false,
            ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
            define: true,
            assignable: Assignable::Identifier(Identifier { name: "a".to_string() }),
            file_position: FilePosition { line: 3..=3, column: 9..=18 },
        }),
    ];

    assert_eq!(expected, top_level_scope.result.program);
    Ok(())
}

#[test]
fn infer_type_assignment_in_scope() -> anyhow::Result<()> {
    let function = r#"
        if (true) {
            let a = 1;
            let c = a;
        }
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;


    let expected: Vec<AbstractSyntaxTreeNode> = vec![
        AbstractSyntaxTreeNode::If(If {
            condition: Assignable::Boolean(Boolean { value: true }),
            if_stack: vec![
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier { name: "a".to_string() }),
                    mutability: false,
                    ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Integer(IntegerAST { value: "1".to_string(), ty: IntegerType::I32 }),
                    file_position: FilePosition { line: 3..=3, column: 13..=22 },
                }),
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier { name: "c".to_string() }),
                    mutability: false,
                    ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Identifier(Identifier { name: "a".to_string() }),
                    file_position: FilePosition { line: 4..=4, column: 13..=22 },
                }),
            ],
            else_stack: None,
            file_position: FilePosition { line: 2..=5, column: 9..=9 },
        })
    ];

    assert_eq!(expected, top_level_scope.result.program);
    Ok(())
}

#[test]
fn infer_type_assignment_in_scope_complex() -> anyhow::Result<()> {
    let function = r#"
    fn constant_1(): i32 { return 5; }
    let a: i32 = 5;
    if (true) {
        let a = a / constant_1();
        let c = a;
    }
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;

    let expected: Vec<AbstractSyntaxTreeNode> = vec![
        AbstractSyntaxTreeNode::MethodDefinition(MethodDefinition {
            identifier: LValue::Identifier(Identifier { name: "constant_1".to_string() }),
            return_type: Type::Integer(IntegerType::I32, Mutability::Immutable),
            arguments: vec![],
            stack: vec![AbstractSyntaxTreeNode::Return(Return {
                assignable: Some(Assignable::Integer(IntegerAST { value: "5".to_string(), ty: IntegerType::I32 })),
                file_position: FilePosition { line: 2..=2, column: 28..=36 },
            })],
            is_extern: false,
            file_position: FilePosition { line: 2..=2, column: 5..=38 },
        }),
        AbstractSyntaxTreeNode::Variable(Variable {
            l_value: LValue::Identifier(Identifier { name: "a".to_string() }),
            mutability: false,
            ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
            define: true,
            assignable: Assignable::Integer(IntegerAST { value: "5".to_string(), ty: IntegerType::I32 }),
            file_position: FilePosition { line: 3..=3, column: 5..=19 },
        }),
        AbstractSyntaxTreeNode::If(If {
            condition: Assignable::Boolean(Boolean { value: true }),
            if_stack: vec![
                AbstractSyntaxTreeNode::Variable(Variable::<'=', ';'> {
                    l_value: LValue::Identifier(Identifier { name: "a".to_string() }),
                    mutability: false,
                    ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                    define: true,
                    file_position: FilePosition { line: 5..=5, column: 9..=33 },
                    assignable: Assignable::Expression(Expression {
                        lhs: Some(Box::new(Expression {
                            lhs: None,
                            rhs: None,
                            operator: Operator::Noop,
                            prefix_arithmetic: None,
                            value: Some(Box::new(Assignable::Identifier(Identifier { name: "a".to_string() }))),
                            index_operator: None,
                            positive: true,
                        })),
                        rhs: Some(Box::new(Expression {
                            lhs: None,
                            rhs: None,
                            operator: Operator::Noop,
                            prefix_arithmetic: None,
                            value: Some(Box::new(Assignable::MethodCall(MethodCall {
                                identifier: LValue::Identifier(Identifier { name: "constant_1".to_string() }),
                                arguments: vec![],
                                file_position: FilePosition { line: 5..=5, column: 21..=32 },
                            }))),
                            index_operator: None,
                            positive: true,
                        })),
                        operator: Operator::Div,
                        value: None,
                        index_operator: None,
                        positive: true,
                        prefix_arithmetic: None,
                    }),
                }),
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier { name: "c".to_string() }),
                    mutability: false,
                    ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Identifier(Identifier { name: "a".to_string() }),
                    file_position: FilePosition { line: 6..=6, column: 9..=18 },
                }),
            ],
            else_stack: None,
            file_position: FilePosition { line: 4..=7, column: 5..=5 },
        })
    ];

    assert_eq!(expected, top_level_scope.result.program);
    Ok(())
}

#[test]
fn infer_type_assignment_in_scope_complex_in_method() -> anyhow::Result<()> {
    let function = r#"
    fn constant_1(): i32 { return 5; }
    fn test(): i32 {
        if (true) {
            let a = a / constant_1();
            let c = a;
        }

        return 0;
    }

    let a: i32 = 5;
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;

    println!("{:#?}", top_level_scope);

    let expected: Vec<AbstractSyntaxTreeNode> = vec![
        AbstractSyntaxTreeNode::MethodDefinition(MethodDefinition {
            identifier: LValue::Identifier(Identifier { name: "constant_1".to_string() }),
            return_type: Type::Integer(IntegerType::I32, Mutability::Immutable),
            arguments: vec![],
            stack: vec![AbstractSyntaxTreeNode::Return(Return {
                assignable: Some(Assignable::Integer(IntegerAST { value: "5".to_string(), ty: IntegerType::I32 })),
                file_position: FilePosition { line: 2..=2, column: 28..=36 },
            })],
            is_extern: false,
            file_position: FilePosition { line: 2..=2, column: 5..=38 },
        }),
        AbstractSyntaxTreeNode::MethodDefinition(MethodDefinition {
            identifier: LValue::Identifier(Identifier { name: "test".to_string() }),
            return_type: Type::Integer(IntegerType::I32, Mutability::Immutable),
            arguments: vec![],
            stack: vec![
                AbstractSyntaxTreeNode::If(If {
                    condition: Assignable::Boolean(Boolean { value: true }),
                    if_stack: vec![
                        AbstractSyntaxTreeNode::Variable(Variable::<'=', ';'> {
                            l_value: LValue::Identifier(Identifier { name: "a".to_string() }),
                            mutability: false,
                            ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                            define: true,
                            file_position: FilePosition { line: 5..=5, column: 13..=37 },
                            assignable: Assignable::Expression(Expression {
                                lhs: Some(Box::new(Expression {
                                    lhs: None,
                                    rhs: None,
                                    operator: Operator::Noop,
                                    prefix_arithmetic: None,
                                    value: Some(Box::new(Assignable::Identifier(Identifier { name: "a".to_string() }))),
                                    index_operator: None,
                                    positive: true,
                                })),
                                rhs: Some(Box::new(Expression {
                                    lhs: None,
                                    rhs: None,
                                    operator: Operator::Noop,
                                    prefix_arithmetic: None,
                                    value: Some(Box::new(Assignable::MethodCall(MethodCall {
                                        identifier: LValue::Identifier(Identifier { name: "constant_1".to_string() }),
                                        arguments: vec![],
                                        file_position: FilePosition { line: 5..=5, column: 25..=36 },
                                    }))),
                                    index_operator: None,
                                    positive: true,
                                })),
                                operator: Operator::Div,
                                prefix_arithmetic: None,
                                value: None,
                                index_operator: None,
                                positive: true,
                            }),
                        }),
                        AbstractSyntaxTreeNode::Variable(Variable {
                            l_value: LValue::Identifier(Identifier { name: "c".to_string() }),
                            mutability: false,
                            ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                            define: true,
                            assignable: Assignable::Identifier(Identifier { name: "a".to_string() }),
                            file_position: FilePosition { line: 6..=6, column: 13..=22 },
                        }),
                    ],
                    else_stack: None,
                    file_position: FilePosition { line: 4..=7, column: 9..=9 },
                }),
                AbstractSyntaxTreeNode::Return(Return {
                    assignable: Some(Assignable::Integer(IntegerAST { value: "0".to_string(), ty: IntegerType::I32 })),
                    file_position: FilePosition { line: 9..=9, column: 9..=17 },
                })
            ],
            is_extern: false,
            file_position: FilePosition { line: 3..=10, column: 5..=5 },
        }),
        AbstractSyntaxTreeNode::Variable(Variable {
            l_value: LValue::Identifier(Identifier { name: "a".to_string() }),
            mutability: false,
            ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
            define: true,
            assignable: Assignable::Integer(IntegerAST { value: "5".to_string(), ty: IntegerType::I32 }),
            file_position: FilePosition { line: 12..=12, column: 5..=19 },
        }),
    ];

    assert_eq!(expected, top_level_scope.result.program);
    Ok(())
}
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::token_with_span::FilePosition;
use monkey_language::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use monkey_language::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use monkey_language::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use monkey_language::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::operator::Operator;
use monkey_language::core::model::abstract_syntax_tree_nodes::for_::For;
use monkey_language::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use monkey_language::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use monkey_language::core::model::abstract_syntax_tree_nodes::variable::Variable;
use monkey_language::core::model::types::integer::{IntegerAST, IntegerType};
use monkey_language::core::model::types::mutability::Mutability;
use monkey_language::core::model::types::ty::Type;
use monkey_language::core::parser::ast_parser::ASTParser;
use monkey_language::core::semantics::static_type_check::static_type_checker::{static_type_check, StaticTypeCheckError};
use monkey_language::core::semantics::type_infer::type_inferer::infer_type;

#[test]
fn static_type_check_for_loop() -> anyhow::Result<()> {
    let program = r#"
    for (let mut i: i32 = 0; i < 10; i = i + 1;) {
        let a: i32 = i;
    }
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(program)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;

    infer_type(&mut top_level_scope.result.program)?;
    static_type_check(&top_level_scope.result.program)?;

    println!("{:#?}", top_level_scope.result.program);

    let expected: Vec<AbstractSyntaxTreeNode> = vec![
        AbstractSyntaxTreeNode::For(For {
            initialization: Variable {
                l_value: LValue::Identifier(Identifier { name: "i".to_string() }),
                mutability: true,
                ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                define: true,
                assignable: Assignable::Integer(IntegerAST { value: "0".to_string(), ty: IntegerType::I32 }),
                file_position: FilePosition { line: 2..=2, column: 10..=28 },
            },
            condition: Assignable::Expression(Expression {
                lhs: Some(Box::new(Expression {
                    lhs: None,
                    rhs: None,
                    operator: Operator::Noop,
                    prefix_arithmetic: None,
                    value: Some(Box::new(Assignable::Identifier(Identifier { name: "i".to_string() }))),
                    index_operator: None,
                    positive: true,
                })),
                rhs: Some(Box::new(Expression {
                    lhs: None,
                    rhs: None,
                    operator: Operator::Noop,
                    prefix_arithmetic: None,
                    value: Some(Box::new(Assignable::Integer(IntegerAST { value: "10".to_string(), ty: IntegerType::I32 }))),
                    index_operator: None,
                    positive: true,
                })),
                operator: Operator::LessThan,
                prefix_arithmetic: None,
                value: None,
                index_operator: None,
                positive: true,
            }),
            update: Variable {
                l_value: LValue::Identifier(Identifier { name: "i".to_string() }),
                mutability: false,
                ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                define: false,
                assignable: Assignable::Expression(Expression {
                    lhs: Some(Box::new(Expression {
                        lhs: None,
                        rhs: None,
                        operator: Operator::Noop,
                        prefix_arithmetic: None,
                        value: Some(Box::new(Assignable::Identifier(Identifier { name: "i".to_string() }))),
                        index_operator: None,
                        positive: true,
                    })),
                    rhs: Some(Box::new(Expression {
                        lhs: None,
                        rhs: None,
                        operator: Operator::Noop,
                        prefix_arithmetic: None,
                        value: Some(Box::new(Assignable::Integer(IntegerAST { value: "1".to_string(), ty: IntegerType::I32 }))),
                        index_operator: None,
                        positive: true,
                    })),
                    operator: Operator::Add,
                    prefix_arithmetic: None,
                    value: None,
                    index_operator: None,
                    positive: true,
                }),
                file_position: FilePosition { line: 2..=2, column: 38..=47 },
            },
            stack: vec![
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier { name: "a".to_string() }),
                    mutability: false,
                    ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Identifier(Identifier { name: "i".to_string() }),
                    file_position: FilePosition { line: 3..=3, column: 9..=23 },
                })
            ],
            file_position: FilePosition { line: 2..=4, column: 5..=5 },
        })
    ];

    assert_eq!(expected, top_level_scope.result.program);

    Ok(())
}

#[test]
fn static_type_check_for_loop_should_fail_missing_mutability() -> anyhow::Result<()> {
    let program = r#"
    for (let i: i32 = 0; i < 10; i = i + 1;) {
        let a: i32 = i;
    }
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(program)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;

    infer_type(&mut top_level_scope.result.program)?;
    let static_analysis_result = static_type_check(&top_level_scope.result.program);

    assert!(static_analysis_result.is_err());
    assert!(matches!(static_analysis_result, Err(StaticTypeCheckError::ImmutabilityViolated { .. })));
    
    Ok(())
}
use monkey_language::core::io::code_line::CodeLine;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::scanner::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::assignable::Assignable;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::assignables::boolean::Boolean;
use monkey_language::core::scanner::parser::ASTParser;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::assignables::equation_parser::operator::Operator;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::assignables::integer::IntegerAST;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::identifier::Identifier;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::r#if::If;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::l_value::LValue;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::method_definition::MethodDefinition;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::r#return::Return;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::variable::Variable;
use monkey_language::core::scanner::types::integer::Integer;
use monkey_language::core::scanner::types::r#type::{Mutability, Type};
use monkey_language::core::semantics::type_checker::static_type_checker::static_type_check;

#[test]
fn infer_type_assignment() -> anyhow::Result<()> {
    let function = r#"
        let a = 1;
        let c = a;
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = ASTParser::from(monkey_file);
    let top_level_scope = lexer.parse()?;

    static_type_check(&top_level_scope)?;

    let expected: Vec<AbstractSyntaxTreeNode> = vec![
        AbstractSyntaxTreeNode::Variable(Variable {
            l_value: LValue::Identifier(Identifier { name: "a".to_string() }),
            mutability: false,
            ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)),
            define: true,
            assignable: Assignable::Integer(IntegerAST { value: "1".to_string(), ty: Integer::I32 }),
            code_line: CodeLine {
                line: "let a = 1 ;".to_string(),
                actual_line_number: 2..2,
                virtual_line_number: 1,
            },
        }),
        AbstractSyntaxTreeNode::Variable(Variable {
            l_value: LValue::Identifier(Identifier { name: "c".to_string() }),
            mutability: false,
            ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)),
            define: true,
            assignable: Assignable::Identifier(Identifier { name: "a".to_string() }),
            code_line: CodeLine {
                line: "let c = a ;".to_string(),
                actual_line_number: 3..3,
                virtual_line_number: 2,
            },
        }),
    ];

    assert_eq!(expected, top_level_scope.ast_nodes);
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


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = ASTParser::from(monkey_file);
    let top_level_scope = lexer.parse()?;

    static_type_check(&top_level_scope)?;

    let expected: Vec<AbstractSyntaxTreeNode> = vec![
        AbstractSyntaxTreeNode::If(If {
            condition: Assignable::Boolean(Boolean { value: true }),
            if_stack: vec![
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier { name: "a".to_string() }),
                    mutability: false,
                    ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Integer(IntegerAST { value: "1".to_string(), ty: Integer::I32 }),
                    code_line: CodeLine {
                        line: "let a = 1 ;".to_string(),
                        actual_line_number: 3..3,
                        virtual_line_number: 2,
                    },
                }),
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier { name: "c".to_string() }),
                    mutability: false,
                    ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Identifier(Identifier { name: "a".to_string() }),
                    code_line: CodeLine {
                        line: "let c = a ;".to_string(),
                        actual_line_number: 4..4,
                        virtual_line_number: 3,
                    },
                }),
            ],
            else_stack: None,
            file_position: CodeLine { line: "if  ( true )  {".to_string(), actual_line_number: 2..2, virtual_line_number: 1 },
        })
    ];

    assert_eq!(expected, top_level_scope.ast_nodes);
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


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = ASTParser::from(monkey_file);
    let top_level_scope = lexer.parse()?;

    static_type_check(&top_level_scope)?;

    let expected: Vec<AbstractSyntaxTreeNode> = vec![
        AbstractSyntaxTreeNode::MethodDefinition(MethodDefinition {
            identifier: Identifier { name: "constant_1".to_string() },
            return_type: Type::Integer(Integer::I32, Mutability::Immutable),
            arguments: vec![],
            stack: vec![AbstractSyntaxTreeNode::Return(Return {
                assignable: Some(Assignable::Integer(IntegerAST { value: "5".to_string(), ty: Integer::I32 })),
                code_line: CodeLine { line: "return 5 ;".to_string(), actual_line_number: 2..2, virtual_line_number: 2 },
            })],
            is_extern: false,
            code_line: CodeLine { line: "fn constant_1 (  )  :  i32 {".to_string(), actual_line_number: 2..2, virtual_line_number: 1 },
        }),
        AbstractSyntaxTreeNode::Variable(Variable {
            l_value: LValue::Identifier(Identifier { name: "a".to_string() }),
            mutability: false,
            ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)),
            define: true,
            assignable: Assignable::Integer(IntegerAST { value: "5".to_string(), ty: Integer::I32 }),
            code_line: CodeLine { line: "let a :  i32 = 5 ;".to_string(),
                actual_line_number: 3..3,
                virtual_line_number: 4,
            },
        }),
        AbstractSyntaxTreeNode::If(If {
            condition: Assignable::Boolean(Boolean { value: true }),
            if_stack: vec![
                AbstractSyntaxTreeNode::Variable(Variable::<'=', ';'> {
                    l_value: LValue::Identifier(Identifier { name: "a".to_string() }),
                    mutability: false,
                    ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)),
                    define: true,
                    code_line: CodeLine {
                        line: "let a = a / constant_1 (  )  ;".to_string(),
                        actual_line_number: 5..5,
                        virtual_line_number: 6,
                    },
                    assignable: Assignable::ArithmeticEquation(Expression {
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
                                identifier: Identifier { name: "constant_1".to_string() },
                                arguments: vec![],
                                code_line: CodeLine {
                                    line: "constant_1  (   ) ;".to_string(),
                                    actual_line_number: 0..0,
                                    virtual_line_number: 0,
                                },
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
                    ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Identifier(Identifier { name: "a".to_string() }),
                    code_line: CodeLine {
                        line: "let c = a ;".to_string(),
                        actual_line_number: 6..6,
                        virtual_line_number: 7,
                    },
                }),
            ],
            else_stack: None,
            file_position: CodeLine { line: "if  ( true )  {".to_string(), actual_line_number: 4..4, virtual_line_number: 5 },
        })
    ];

    assert_eq!(expected, top_level_scope.ast_nodes);
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


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = ASTParser::from(monkey_file);
    let top_level_scope = lexer.parse()?;

    println!("{:#?}", top_level_scope);

    static_type_check(&top_level_scope)?;

    let expected: Vec<AbstractSyntaxTreeNode> = vec![
        AbstractSyntaxTreeNode::MethodDefinition(MethodDefinition {
            identifier: Identifier { name: "constant_1".to_string() },
            return_type: Type::Integer(Integer::I32, Mutability::Immutable),
            arguments: vec![],
            stack: vec![AbstractSyntaxTreeNode::Return(Return {
                assignable: Some(Assignable::Integer(IntegerAST { value: "5".to_string(), ty: Integer::I32 })),
                code_line: CodeLine { line: "return 5 ;".to_string(), actual_line_number: 2..2, virtual_line_number: 2 },
            })],
            is_extern: false,
            code_line: CodeLine { line: "fn constant_1 (  )  :  i32 {".to_string(), actual_line_number: 2..2, virtual_line_number: 1 },
        }),
        AbstractSyntaxTreeNode::MethodDefinition(MethodDefinition {
            identifier: Identifier { name: "test".to_string() },
            return_type: Type::Integer(Integer::I32, Mutability::Immutable),
            arguments: vec![],
            stack: vec![
                AbstractSyntaxTreeNode::If(If {
                    condition: Assignable::Boolean(Boolean { value: true }),
                    if_stack: vec![
                        AbstractSyntaxTreeNode::Variable(Variable::<'=', ';'> {
                            l_value: LValue::Identifier(Identifier { name: "a".to_string() }),
                            mutability: false,
                            ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)),
                            define: true,
                            code_line: CodeLine {
                                line: "let a = a / constant_1 (  )  ;".to_string(),
                                actual_line_number: 5..5,
                                virtual_line_number: 6,
                            },
                            assignable: Assignable::ArithmeticEquation(Expression {
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
                                        identifier: Identifier { name: "constant_1".to_string() },
                                        arguments: vec![],
                                        code_line: CodeLine {
                                            line: "constant_1  (   ) ;".to_string(),
                                            actual_line_number: 0..0,
                                            virtual_line_number: 0,
                                        },
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
                            ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)),
                            define: true,
                            assignable: Assignable::Identifier(Identifier { name: "a".to_string() }),
                            code_line: CodeLine {
                                line: "let c = a ;".to_string(),
                                actual_line_number: 6..6,
                                virtual_line_number: 7,
                            },
                        }),
                    ],
                    else_stack: None,
                    file_position: CodeLine { line: "if  ( true )  {".to_string(), actual_line_number: 4..4, virtual_line_number: 5 },
                }),
                AbstractSyntaxTreeNode::Return(Return {
                    assignable: Some(Assignable::Integer(IntegerAST { value: "0".to_string(), ty: Integer::I32 })),
                    code_line: CodeLine { line: "return 0 ;".to_string(),
                        actual_line_number: 9..9,
                        virtual_line_number: 9,
                    },
                })
            ],
            is_extern: false,
            code_line: CodeLine {
                line: "fn test (  )  :  i32 {".to_string(),
                actual_line_number: 3..3,
                virtual_line_number: 4,
            },
        }),
        AbstractSyntaxTreeNode::Variable(Variable {
            l_value: LValue::Identifier(Identifier { name: "a".to_string() }),
            mutability: false,
            ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)),
            define: true,
            assignable: Assignable::Integer(IntegerAST { value: "5".to_string(), ty: Integer::I32 }),
            code_line: CodeLine { line: "let a :  i32 = 5 ;".to_string(),
                actual_line_number: 12..12,
                virtual_line_number: 11,
            },
        }),
    ];

    assert_eq!(expected, top_level_scope.ast_nodes);
    Ok(())
}
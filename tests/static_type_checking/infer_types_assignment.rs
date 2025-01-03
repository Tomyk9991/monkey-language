use monkey_language::core::io::code_line::CodeLine;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::token::Token;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::lexer::tokens::assignable_token::AssignableToken;
use monkey_language::core::lexer::tokens::assignable_tokens::boolean_token::BooleanToken;
use monkey_language::core::lexer::tokens::assignable_tokens::equation_parser::expression::Expression;
use monkey_language::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;
use monkey_language::core::lexer::tokens::assignable_tokens::integer_token::IntegerToken;
use monkey_language::core::lexer::tokens::assignable_tokens::method_call_token::MethodCallToken;
use monkey_language::core::lexer::tokens::if_token::IfToken;
use monkey_language::core::lexer::tokens::l_value::LValue;
use monkey_language::core::lexer::tokens::method_definition::MethodDefinition;
use monkey_language::core::lexer::tokens::name_token::NameToken;
use monkey_language::core::lexer::tokens::return_token::ReturnToken;
use monkey_language::core::lexer::tokens::variable_token::VariableToken;
use monkey_language::core::lexer::types::integer::Integer;
use monkey_language::core::lexer::types::type_token::{Mutability, TypeToken};
use monkey_language::core::type_checker::static_type_checker::static_type_check;

#[test]
fn infer_type_assignment() -> anyhow::Result<()> {
    let function = r#"
        let a = 1;
        let c = a;
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    static_type_check(&top_level_scope)?;

    let expected: Vec<Token> = vec![
        Token::Variable(VariableToken {
            l_value: LValue::Name(NameToken { name: "a".to_string() }),
            mutability: false,
            ty: Some(TypeToken::Integer(Integer::I32, Mutability::Immutable)),
            define: true,
            assignable: AssignableToken::IntegerToken(IntegerToken { value: "1".to_string(), ty: Integer::I32 }),
            code_line: CodeLine {
                line: "let a = 1 ;".to_string(),
                actual_line_number: 2..2,
                virtual_line_number: 1,
            },
        }),
        Token::Variable(VariableToken {
            l_value: LValue::Name(NameToken { name: "c".to_string() }),
            mutability: false,
            ty: Some(TypeToken::Integer(Integer::I32, Mutability::Immutable)),
            define: true,
            assignable: AssignableToken::NameToken(NameToken { name: "a".to_string() }),
            code_line: CodeLine {
                line: "let c = a ;".to_string(),
                actual_line_number: 3..3,
                virtual_line_number: 2,
            },
        }),
    ];

    assert_eq!(expected, top_level_scope.tokens);
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
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    static_type_check(&top_level_scope)?;

    let expected: Vec<Token> = vec![
        Token::If(IfToken {
            condition: AssignableToken::BooleanToken(BooleanToken { value: true }),
            if_stack: vec![
                Token::Variable(VariableToken {
                    l_value: LValue::Name(NameToken { name: "a".to_string() }),
                    mutability: false,
                    ty: Some(TypeToken::Integer(Integer::I32, Mutability::Immutable)),
                    define: true,
                    assignable: AssignableToken::IntegerToken(IntegerToken { value: "1".to_string(), ty: Integer::I32 }),
                    code_line: CodeLine {
                        line: "let a = 1 ;".to_string(),
                        actual_line_number: 3..3,
                        virtual_line_number: 2,
                    },
                }),
                Token::Variable(VariableToken {
                    l_value: LValue::Name(NameToken { name: "c".to_string() }),
                    mutability: false,
                    ty: Some(TypeToken::Integer(Integer::I32, Mutability::Immutable)),
                    define: true,
                    assignable: AssignableToken::NameToken(NameToken { name: "a".to_string() }),
                    code_line: CodeLine {
                        line: "let c = a ;".to_string(),
                        actual_line_number: 4..4,
                        virtual_line_number: 3,
                    },
                }),
            ],
            else_stack: None,
            code_line: CodeLine { line: "if  ( true )  {".to_string(), actual_line_number: 2..2, virtual_line_number: 1 },
        })
    ];

    assert_eq!(expected, top_level_scope.tokens);
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
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    static_type_check(&top_level_scope)?;

    let expected: Vec<Token> = vec![
        Token::MethodDefinition(MethodDefinition {
            name: NameToken { name: "constant_1".to_string() },
            return_type: TypeToken::Integer(Integer::I32, Mutability::Immutable),
            arguments: vec![],
            stack: vec![Token::Return(ReturnToken {
                assignable: Some(AssignableToken::IntegerToken(IntegerToken { value: "5".to_string(), ty: Integer::I32 })),
                code_line: CodeLine { line: "return 5 ;".to_string(), actual_line_number: 2..2, virtual_line_number: 2 },
            })],
            is_extern: false,
            code_line: CodeLine { line: "fn constant_1 (  )  :  i32 {".to_string(), actual_line_number: 2..2, virtual_line_number: 1 },
        }),
        Token::Variable(VariableToken {
            l_value: LValue::Name(NameToken { name: "a".to_string() }),
            mutability: false,
            ty: Some(TypeToken::Integer(Integer::I32, Mutability::Immutable)),
            define: true,
            assignable: AssignableToken::IntegerToken(IntegerToken { value: "5".to_string(), ty: Integer::I32 }),
            code_line: CodeLine { line: "let a :  i32 = 5 ;".to_string(),
                actual_line_number: 3..3,
                virtual_line_number: 4,
            },
        }),
        Token::If(IfToken {
            condition: AssignableToken::BooleanToken(BooleanToken { value: true }),
            if_stack: vec![
                Token::Variable(VariableToken::<'=', ';'> {
                    l_value: LValue::Name(NameToken { name: "a".to_string() }),
                    mutability: false,
                    ty: Some(TypeToken::Integer(Integer::I32, Mutability::Immutable)),
                    define: true,
                    code_line: CodeLine {
                        line: "let a = a / constant_1 (  )  ;".to_string(),
                        actual_line_number: 5..5,
                        virtual_line_number: 6,
                    },
                    assignable: AssignableToken::ArithmeticEquation(Expression {
                        lhs: Some(Box::new(Expression {
                            lhs: None,
                            rhs: None,
                            operator: Operator::Noop,
                            prefix_arithmetic: None,
                            value: Some(Box::new(AssignableToken::NameToken(NameToken { name: "a".to_string() }))),
                            index_operator: None,
                            positive: true,
                        })),
                        rhs: Some(Box::new(Expression {
                            lhs: None,
                            rhs: None,
                            operator: Operator::Noop,
                            prefix_arithmetic: None,
                            value: Some(Box::new(AssignableToken::MethodCallToken(MethodCallToken {
                                name: NameToken { name: "constant_1".to_string() },
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
                Token::Variable(VariableToken {
                    l_value: LValue::Name(NameToken { name: "c".to_string() }),
                    mutability: false,
                    ty: Some(TypeToken::Integer(Integer::I32, Mutability::Immutable)),
                    define: true,
                    assignable: AssignableToken::NameToken(NameToken { name: "a".to_string() }),
                    code_line: CodeLine {
                        line: "let c = a ;".to_string(),
                        actual_line_number: 6..6,
                        virtual_line_number: 7,
                    },
                }),
            ],
            else_stack: None,
            code_line: CodeLine { line: "if  ( true )  {".to_string(), actual_line_number: 4..4, virtual_line_number: 5 },
        })
    ];

    assert_eq!(expected, top_level_scope.tokens);
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
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    println!("{:#?}", top_level_scope);

    static_type_check(&top_level_scope)?;

    let expected: Vec<Token> = vec![
        Token::MethodDefinition(MethodDefinition {
            name: NameToken { name: "constant_1".to_string() },
            return_type: TypeToken::Integer(Integer::I32, Mutability::Immutable),
            arguments: vec![],
            stack: vec![Token::Return(ReturnToken {
                assignable: Some(AssignableToken::IntegerToken(IntegerToken { value: "5".to_string(), ty: Integer::I32 })),
                code_line: CodeLine { line: "return 5 ;".to_string(), actual_line_number: 2..2, virtual_line_number: 2 },
            })],
            is_extern: false,
            code_line: CodeLine { line: "fn constant_1 (  )  :  i32 {".to_string(), actual_line_number: 2..2, virtual_line_number: 1 },
        }),
        Token::MethodDefinition(MethodDefinition {
            name: NameToken { name: "test".to_string() },
            return_type: TypeToken::Integer(Integer::I32, Mutability::Immutable),
            arguments: vec![],
            stack: vec![
                Token::If(IfToken {
                    condition: AssignableToken::BooleanToken(BooleanToken { value: true }),
                    if_stack: vec![
                        Token::Variable(VariableToken::<'=', ';'> {
                            l_value: LValue::Name(NameToken { name: "a".to_string() }),
                            mutability: false,
                            ty: Some(TypeToken::Integer(Integer::I32, Mutability::Immutable)),
                            define: true,
                            code_line: CodeLine {
                                line: "let a = a / constant_1 (  )  ;".to_string(),
                                actual_line_number: 5..5,
                                virtual_line_number: 6,
                            },
                            assignable: AssignableToken::ArithmeticEquation(Expression {
                                lhs: Some(Box::new(Expression {
                                    lhs: None,
                                    rhs: None,
                                    operator: Operator::Noop,
                                    prefix_arithmetic: None,
                                    value: Some(Box::new(AssignableToken::NameToken(NameToken { name: "a".to_string() }))),
                                    index_operator: None,
                                    positive: true,
                                })),
                                rhs: Some(Box::new(Expression {
                                    lhs: None,
                                    rhs: None,
                                    operator: Operator::Noop,
                                    prefix_arithmetic: None,
                                    value: Some(Box::new(AssignableToken::MethodCallToken(MethodCallToken {
                                        name: NameToken { name: "constant_1".to_string() },
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
                        Token::Variable(VariableToken {
                            l_value: LValue::Name(NameToken { name: "c".to_string() }),
                            mutability: false,
                            ty: Some(TypeToken::Integer(Integer::I32, Mutability::Immutable)),
                            define: true,
                            assignable: AssignableToken::NameToken(NameToken { name: "a".to_string() }),
                            code_line: CodeLine {
                                line: "let c = a ;".to_string(),
                                actual_line_number: 6..6,
                                virtual_line_number: 7,
                            },
                        }),
                    ],
                    else_stack: None,
                    code_line: CodeLine { line: "if  ( true )  {".to_string(), actual_line_number: 4..4, virtual_line_number: 5 },
                }),
                Token::Return(ReturnToken {
                    assignable: Some(AssignableToken::IntegerToken(IntegerToken { value: "0".to_string(), ty: Integer::I32 })),
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
        Token::Variable(VariableToken {
            l_value: LValue::Name(NameToken { name: "a".to_string() }),
            mutability: false,
            ty: Some(TypeToken::Integer(Integer::I32, Mutability::Immutable)),
            define: true,
            assignable: AssignableToken::IntegerToken(IntegerToken { value: "5".to_string(), ty: Integer::I32 }),
            code_line: CodeLine { line: "let a :  i32 = 5 ;".to_string(),
                actual_line_number: 12..12,
                virtual_line_number: 11,
            },
        }),
    ];

    assert_eq!(expected, top_level_scope.tokens);
    Ok(())
}
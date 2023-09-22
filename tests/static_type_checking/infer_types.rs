use monkey_language::core::io::code_line::CodeLine;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::token::Token;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::lexer::tokens::assignable_token::AssignableToken;
use monkey_language::core::lexer::tokens::assignable_tokens::boolean_token::BooleanToken;
use monkey_language::core::lexer::tokens::assignable_tokens::double_token::FloatToken;
use monkey_language::core::lexer::tokens::assignable_tokens::equation_parser::expression::Expression;
use monkey_language::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;
use monkey_language::core::lexer::tokens::assignable_tokens::integer_token::IntegerToken;
use monkey_language::core::lexer::tokens::assignable_tokens::string_token::StringToken;
use monkey_language::core::lexer::tokens::if_definition::IfDefinition;
use monkey_language::core::lexer::tokens::name_token::NameToken;
use monkey_language::core::lexer::tokens::variable_token::VariableToken;
use monkey_language::core::lexer::type_token::TypeToken;
use monkey_language::core::type_checker::static_type_checker::static_type_check;

#[test]
fn infer_type() -> anyhow::Result<()> {
    let function = r#"
        let a = 1;
        let b = 2.0;
        let c = true;
        let d = "KEKW";
        let e = 1 + 1.0;
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    static_type_check(&top_level_scope)?;

    let expected: Vec<Token> = vec![
        Token::Variable(VariableToken {
            name_token: NameToken { name: "a".to_string() },
            mutability: false,
            ty: Some(TypeToken::I32),
            define: true,
            assignable: AssignableToken::IntegerToken(IntegerToken { value: 1 }),
            code_line: CodeLine {
                line: "let a = 1 ;".to_string(),
                actual_line_number: 2..2,
                virtual_line_number: 1,
            },
        }),
        Token::Variable(VariableToken {
            name_token: NameToken { name: "b".to_string() },
            mutability: false,
            ty: Some(TypeToken::F32),
            define: true,
            assignable: AssignableToken::FloatToken(FloatToken { value: 2.0 }),
            code_line: CodeLine {
                line: "let b = 2.0 ;".to_string(),
                actual_line_number: 3..3,
                virtual_line_number: 2,
            },
        }),
        Token::Variable(VariableToken {
            name_token: NameToken { name: "c".to_string() },
            mutability: false,
            ty: Some(TypeToken::Bool),
            define: true,
            assignable: AssignableToken::BooleanToken(BooleanToken { value: true }),
            code_line: CodeLine {
                line: "let c = true ;".to_string(),
                actual_line_number: 4..4,
                virtual_line_number: 3,
            },
        }),
        Token::Variable(VariableToken {
            name_token: NameToken { name: "d".to_string() },
            mutability: false,
            ty: Some(TypeToken::String),
            define: true,
            assignable: AssignableToken::String(StringToken { value: "\"KEKW\"".to_string() }),
            code_line: CodeLine {
                line: "let d = \"KEKW\" ;".to_string(),
                actual_line_number: 5..5,
                virtual_line_number: 4,
            },
        }),
        Token::Variable(VariableToken {
            name_token: NameToken { name: "e".to_string() },
            mutability: false,
            ty: Some(TypeToken::F32),
            define: true,
            assignable: AssignableToken::ArithmeticEquation(
                Expression {
                    lhs: Some(Box::new(Expression {
                        lhs: None,
                        rhs: None,
                        operator: Operator::Noop,
                        value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 1 }))),
                        positive: true,
                    })),
                    rhs: Some(Box::new(Expression {
                        lhs: None,
                        rhs: None,
                        operator: Operator::Noop,
                        value: Some(Box::new(AssignableToken::FloatToken(FloatToken { value: 1.0 }))),
                        positive: true,
                    })),
                    operator: Operator::Add,
                    value: None,
                    positive: true,
                }
            ),
            code_line: CodeLine {
                line: "let e = 1 + 1.0 ;".to_string(),
                actual_line_number: 6..6,
                virtual_line_number: 5,
            },
        })
    ];

    assert_eq!(expected, top_level_scope.tokens);
    Ok(())
}

#[test]
fn infer_type_in_scope() -> anyhow::Result<()> {
    let function = r#"
        if (1) {
            let a = 1;
            let b = 2.0;
            let c = true;
            let d = "KEKW";
            let e = 1 + 1.0;
        }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    static_type_check(&top_level_scope)?;

    let expected: Vec<Token> = vec![
        Token::IfDefinition(IfDefinition {
            condition: AssignableToken::IntegerToken(IntegerToken { value: 1 }),
            if_stack: vec![
                Token::Variable(VariableToken {
                    name_token: NameToken { name: "a".to_string() },
                    mutability: false,
                    ty: Some(TypeToken::I32),
                    define: true,
                    assignable: AssignableToken::IntegerToken(IntegerToken { value: 1 }),
                    code_line: CodeLine {
                        line: "let a = 1 ;".to_string(),
                        actual_line_number: 3..3,
                        virtual_line_number: 2,
                    },
                }),
                Token::Variable(VariableToken {
                    name_token: NameToken { name: "b".to_string() },
                    mutability: false,
                    ty: Some(TypeToken::F32),
                    define: true,
                    assignable: AssignableToken::FloatToken(FloatToken { value: 2.0 }),
                    code_line: CodeLine {
                        line: "let b = 2.0 ;".to_string(),
                        actual_line_number: 4..4,
                        virtual_line_number: 3,
                    },
                }),
                Token::Variable(VariableToken {
                    name_token: NameToken { name: "c".to_string() },
                    mutability: false,
                    ty: Some(TypeToken::Bool),
                    define: true,
                    assignable: AssignableToken::BooleanToken(BooleanToken { value: true }),
                    code_line: CodeLine {
                        line: "let c = true ;".to_string(),
                        actual_line_number: 5..5,
                        virtual_line_number: 4,
                    },
                }),
                Token::Variable(VariableToken {
                    name_token: NameToken { name: "d".to_string() },
                    mutability: false,
                    ty: Some(TypeToken::String),
                    define: true,
                    assignable: AssignableToken::String(StringToken { value: "\"KEKW\"".to_string() }),
                    code_line: CodeLine {
                        line: "let d = \"KEKW\" ;".to_string(),
                        actual_line_number: 6..6,
                        virtual_line_number: 5,
                    },
                }),
                Token::Variable(VariableToken {
                    name_token: NameToken { name: "e".to_string() },
                    mutability: false,
                    ty: Some(TypeToken::F32),
                    define: true,
                    assignable: AssignableToken::ArithmeticEquation(
                        Expression {
                            lhs: Some(Box::new(Expression {
                                lhs: None,
                                rhs: None,
                                operator: Operator::Noop,
                                value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 1 }))),
                                positive: true,
                            })),
                            rhs: Some(Box::new(Expression {
                                lhs: None,
                                rhs: None,
                                operator: Operator::Noop,
                                value: Some(Box::new(AssignableToken::FloatToken(FloatToken { value: 1.0 }))),
                                positive: true,
                            })),
                            operator: Operator::Add,
                            value: None,
                            positive: true,
                        }
                    ),
                    code_line: CodeLine {
                        line: "let e = 1 + 1.0 ;".to_string(),
                        actual_line_number: 7..7,
                        virtual_line_number: 6,
                    },
                })
            ],
            else_stack: None,
            code_line: CodeLine { line: "if  ( 1 )  {".to_string(), actual_line_number: 2..2, virtual_line_number: 1 },
        })
    ];

    assert_eq!(expected, top_level_scope.tokens);
    Ok(())
}
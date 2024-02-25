use monkey_language::core::io::code_line::CodeLine;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::token::Token;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::lexer::tokens::assignable_token::AssignableToken;
use monkey_language::core::lexer::tokens::assignable_tokens::boolean_token::BooleanToken;
use monkey_language::core::lexer::tokens::assignable_tokens::float_token::FloatToken;
use monkey_language::core::lexer::tokens::assignable_tokens::integer_token::IntegerToken;
use monkey_language::core::lexer::tokens::assignable_tokens::string_token::StringToken;
use monkey_language::core::lexer::tokens::if_definition::IfToken;
use monkey_language::core::lexer::tokens::name_token::NameToken;
use monkey_language::core::lexer::tokens::variable_token::VariableToken;
use monkey_language::core::lexer::types::float::Float;
use monkey_language::core::lexer::types::integer::Integer;
use monkey_language::core::lexer::types::type_token::TypeToken;
use monkey_language::core::type_checker::static_type_checker::static_type_check;

#[test]
fn infer_type() -> anyhow::Result<()> {
    let function = r#"
        let a = 1;
        let b = 2.0;
        let c = true;
        let d = "KEKW";
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    static_type_check(&top_level_scope)?;

    let expected: Vec<Token> = vec![
        Token::Variable(VariableToken {
            name_token: NameToken { name: "a".to_string() },
            mutability: false,
            ty: Some(TypeToken::Integer(Integer::I32)),
            define: true,
            assignable: AssignableToken::IntegerToken(IntegerToken { value: "1".to_string(), ty: Integer::I32 }),
            code_line: CodeLine {
                line: "let a = 1 ;".to_string(),
                actual_line_number: 2..2,
                virtual_line_number: 1,
            },
        }),
        Token::Variable(VariableToken {
            name_token: NameToken { name: "b".to_string() },
            mutability: false,
            ty: Some(TypeToken::Float(Float::Float32)),
            define: true,
            assignable: AssignableToken::FloatToken(FloatToken { value: 2.0, ty: Float::Float32 }),
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
            ty: Some(TypeToken::Custom(NameToken { name: String::from("*string") })),
            define: true,
            assignable: AssignableToken::String(StringToken { value: "\"KEKW\"".to_string() }),
            code_line: CodeLine {
                line: "let d = \"KEKW\" ;".to_string(),
                actual_line_number: 5..5,
                virtual_line_number: 4,
            },
        }),
    ];

    assert_eq!(expected, top_level_scope.tokens);
    Ok(())
}

#[test]
fn infer_type_in_scope() -> anyhow::Result<()> {
    let function = r#"
        if (true) {
            let a = 1;
            let b = 2.0;
            let c = true;
            let d = "KEKW";
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
                    name_token: NameToken { name: "a".to_string() },
                    mutability: false,
                    ty: Some(TypeToken::Integer(Integer::I32)),
                    define: true,
                    assignable: AssignableToken::IntegerToken(IntegerToken { value: "1".to_string(), ty: Integer::I32  }),
                    code_line: CodeLine {
                        line: "let a = 1 ;".to_string(),
                        actual_line_number: 3..3,
                        virtual_line_number: 2,
                    },
                }),
                Token::Variable(VariableToken {
                    name_token: NameToken { name: "b".to_string() },
                    mutability: false,
                    ty: Some(TypeToken::Float(Float::Float32)),
                    define: true,
                    assignable: AssignableToken::FloatToken(FloatToken { value: 2.0, ty: Float::Float32 }),
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
                    ty: Some(TypeToken::Custom(NameToken { name: String::from("*string") })),
                    define: true,
                    assignable: AssignableToken::String(StringToken { value: "\"KEKW\"".to_string() }),
                    code_line: CodeLine {
                        line: "let d = \"KEKW\" ;".to_string(),
                        actual_line_number: 6..6,
                        virtual_line_number: 5,
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
use monkey_language::core::io::code_line::CodeLine;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::token::Token;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::lexer::tokens::assignable_token::AssignableToken;
use monkey_language::core::lexer::tokens::assignable_tokens::integer_token::IntegerToken;
use monkey_language::core::lexer::tokens::assignable_tokens::string_token::StringToken;
use monkey_language::core::lexer::tokens::method_definition::MethodDefinition;
use monkey_language::core::lexer::tokens::name_token::NameToken;
use monkey_language::core::lexer::tokens::variable_token::VariableToken;
use monkey_language::core::lexer::type_token::TypeToken;

#[test]
fn function_test() -> anyhow::Result<()> {
    let function = r#"
    fn method_name(variable, variable): void {
        let function_variable_one = 10;
        let function_variable_two = 2;
    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    let expected = vec![
        Token::MethodDefinition(MethodDefinition {
            name: NameToken { name: "method_name".to_string() },
            return_type: TypeToken::Void,
            arguments: vec![AssignableToken::Variable(NameToken { name: "variable".to_string() }), AssignableToken::Variable(NameToken { name: "variable".to_string() })],
            stack: vec![
                Token::Variable(VariableToken {
                    name_token: NameToken { name: "function_variable_one".to_string() },
                    mutability: false,
                    ty: Some(TypeToken::I32),
                    define: true,
                    assignable: AssignableToken::IntegerToken(IntegerToken { value: 10 }),
                    code_line: CodeLine { line: "let function_variable_one = 10 ;".to_string(), actual_line_number: 3..3, virtual_line_number: 2 },
                }),
                Token::Variable(VariableToken {
                    name_token: NameToken { name: "function_variable_two".to_string() },
                    mutability: false,
                    ty: Some(TypeToken::I32),
                    define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 2 }),
                    code_line: CodeLine { line: "let function_variable_two = 2 ;".to_string(), actual_line_number: 4..4, virtual_line_number: 3 },
                })],
            code_line: CodeLine { line: "fn method_name ( variable ,  variable )  :  void {".to_string(), actual_line_number: 2..2, virtual_line_number: 1 },
        }),
    ];

    assert_eq!(expected, top_level_scope.tokens);
    Ok(())
}

#[test]
fn multiple_functions_test() -> anyhow::Result<()> {
    let function = r#"
    fn f(variable, variable): void
    {
        let function_variable_one = 10;
    }
    
    fn method_name(variable, variable): void {
        let function_variable_one = 10;
        let function_variable_two = 2;
    }
    

    fn method_without_parameters( ): void {

    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    let expected = vec![
        Token::MethodDefinition(MethodDefinition { name: NameToken { name: "f".to_string() }, return_type: TypeToken::Void, arguments: vec![AssignableToken::Variable(NameToken { name: "variable".to_string() }), AssignableToken::Variable(NameToken { name: "variable".to_string() })], stack: vec![Token::Variable(VariableToken { name_token: NameToken { name: "function_variable_one".to_string() }, mutability: false, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 10 }), code_line: CodeLine { line: "let function_variable_one = 10 ;".to_string(), actual_line_number: 4..4, virtual_line_number: 2 } })], code_line: CodeLine { line: "fn f ( variable ,  variable )  :  void {".to_string(), actual_line_number: 2..3, virtual_line_number: 1 } }),
        Token::MethodDefinition(MethodDefinition { name: NameToken { name: "method_name".to_string() }, return_type: TypeToken::Void, arguments: vec![AssignableToken::Variable(NameToken { name: "variable".to_string() }), AssignableToken::Variable(NameToken { name: "variable".to_string() })], stack: vec![Token::Variable(VariableToken { name_token: NameToken { name: "function_variable_one".to_string() }, mutability: false, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 10 }), code_line: CodeLine { line: "let function_variable_one = 10 ;".to_string(), actual_line_number: 8..8, virtual_line_number: 5 } }), Token::Variable(VariableToken { name_token: NameToken { name: "function_variable_two".to_string() }, mutability: false, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 2 }), code_line: CodeLine { line: "let function_variable_two = 2 ;".to_string(), actual_line_number: 9..9, virtual_line_number: 6 } })], code_line: CodeLine { line: "fn method_name ( variable ,  variable )  :  void {".to_string(), actual_line_number: 7..7, virtual_line_number: 4 } }),
        Token::MethodDefinition(MethodDefinition { name: NameToken { name: "method_without_parameters".to_string() }, return_type: TypeToken::Void, arguments: vec![], stack: vec![], code_line: CodeLine { line: "fn method_without_parameters (   )  :  void {".to_string(), actual_line_number: 13..13, virtual_line_number: 8 } }),
    ];

    assert_eq!(expected, top_level_scope.tokens);
    Ok(())
}

#[test]
fn function_different_return_type_test() -> anyhow::Result<()> {
    let function = r#"
    fn f(variable, variable): string
    {
        let function_variable_zero = "Hallo";
    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    println!("{:#?}", top_level_scope);

    let expected = vec![
        Token::MethodDefinition(MethodDefinition {
            name: NameToken { name: "f".to_string() },
            return_type: TypeToken::String,
            arguments: vec![AssignableToken::Variable(NameToken { name: "variable".to_string() }), AssignableToken::Variable(NameToken { name: "variable".to_string() })],
            stack: vec![
                Token::Variable(VariableToken { name_token: NameToken { name: "function_variable_zero".to_string() }, mutability: false, ty: Some(TypeToken::String), define: true, assignable: AssignableToken::String(StringToken { value: "\"Hallo\"".to_string() }), code_line: CodeLine { line: "let function_variable_zero = \"Hallo\" ;".to_string(), actual_line_number: 4..4, virtual_line_number: 2 } }),
            ],
            code_line: CodeLine { line: "fn f ( variable ,  variable )  :  string {".to_string(), actual_line_number: 2..3, virtual_line_number: 1 },
        }),
    ];

    assert_eq!(expected, top_level_scope.tokens);
    Ok(())
}

#[test]
fn function_in_function_test() -> anyhow::Result<()> {
    let function = r#"
    fn f(variable, variable): void
    {
        let function_variable_zero = "Hallo";
        fn method_name(variable, variable): void {
            let function_variable_one = 10;
            let function_variable_two = 2;
        }
    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    let expected = vec![
        Token::MethodDefinition(MethodDefinition {
            name: NameToken { name: "f".to_string() },
            return_type: TypeToken::Void,
            arguments: vec![
                AssignableToken::Variable(NameToken { name: "variable".to_string() }),
                AssignableToken::Variable(NameToken { name: "variable".to_string() }),
            ],
            stack: vec![
                Token::Variable(VariableToken { name_token: NameToken { name: "function_variable_zero".to_string() }, mutability: false, ty: Some(TypeToken::String), define: true, assignable: AssignableToken::String(StringToken { value: "\"Hallo\"".to_string() }), code_line: CodeLine { line: "let function_variable_zero = \"Hallo\" ;".to_string(), actual_line_number: 4..4, virtual_line_number: 2 } }),
                Token::MethodDefinition(MethodDefinition {
                    name: NameToken { name: "method_name".to_string() },
                    return_type: TypeToken::Void,
                    arguments: vec![
                        AssignableToken::Variable(NameToken { name: "variable".to_string() }),
                        AssignableToken::Variable(NameToken { name: "variable".to_string() })
                    ],
                    stack: vec![
                        Token::Variable(VariableToken { name_token: NameToken { name: "function_variable_one".to_string() }, mutability: false, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 10 }), code_line: CodeLine { line: "let function_variable_one = 10 ;".to_string(), actual_line_number: 6..6, virtual_line_number: 4 } }),
                        Token::Variable(VariableToken { name_token: NameToken { name: "function_variable_two".to_string() }, mutability: false, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 2 }), code_line: CodeLine { line: "let function_variable_two = 2 ;".to_string(), actual_line_number: 7..7, virtual_line_number: 5 } })
                    ], code_line: CodeLine { line: "fn method_name ( variable ,  variable )  :  void {".to_string(), actual_line_number: 5..5, virtual_line_number: 3 } }),
            ],
            code_line: CodeLine { line: "fn f ( variable ,  variable )  :  void {".to_string(), actual_line_number: 2..3, virtual_line_number: 1 },
        }),
    ];

    assert_eq!(expected, top_level_scope.tokens);
    Ok(())
}
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::token::Token;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::lexer::tokens::assignable_token::AssignableToken;
use monkey_language::core::lexer::tokens::assignable_tokens::integer_token::IntegerToken;
use monkey_language::core::lexer::tokens::assignable_tokens::string_token::StringToken;
use monkey_language::core::lexer::tokens::method_definition::MethodDefinition;
use monkey_language::core::lexer::tokens::name_token::NameToken;
use monkey_language::core::lexer::tokens::variable_token::VariableToken;

#[test]
fn function_test() -> anyhow::Result<()> {
    let function = r#"
    fn method_name(variable, variable): void {
        function_variable_one = 10;
        function_variable_two = 2;
    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    let expected = vec![
        Token::MethodDefinition(MethodDefinition { name: NameToken { name: "method_name".to_string() }, return_type: NameToken { name: "void".to_string() }, arguments: vec![AssignableToken::Variable(NameToken { name: "variable".to_string() }), AssignableToken::Variable(NameToken { name: "variable".to_string() })], stack: vec![Token::Variable(VariableToken { name_token: NameToken { name: "function_variable_one".to_string() }, assignable: AssignableToken::IntegerToken(IntegerToken { value: 10 }) }), Token::Variable(VariableToken { name_token: NameToken { name: "function_variable_two".to_string() }, assignable: AssignableToken::IntegerToken(IntegerToken { value: 2 }) })] }),
    ];

    assert_eq!(expected, top_level_scope.tokens);
    Ok(())
}

#[test]
fn multiple_functions_test() -> anyhow::Result<()> {
    let function = r#"
    fn f(variable, variable): void
    {
        function_variable_one = 10;
    }
    
    fn method_name(variable, variable): void {
        function_variable_one = 10;
        function_variable_two = 2;
    }
    

    fn method_without_parameters( ): void {

    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    let expected = vec![
        Token::MethodDefinition(MethodDefinition { name: NameToken { name: "f".to_string() }, return_type: NameToken { name: "void".to_string() }, arguments: vec![AssignableToken::Variable(NameToken { name: "variable".to_string() }), AssignableToken::Variable(NameToken { name: "variable".to_string() })], stack: vec![Token::Variable(VariableToken { name_token: NameToken { name: "function_variable_one".to_string() }, assignable: AssignableToken::IntegerToken(IntegerToken { value: 10 }) })] }),
        Token::MethodDefinition(MethodDefinition { name: NameToken { name: "method_name".to_string() }, return_type: NameToken { name: "void".to_string() }, arguments: vec![AssignableToken::Variable(NameToken { name: "variable".to_string() }), AssignableToken::Variable(NameToken { name: "variable".to_string() })], stack: vec![Token::Variable(VariableToken { name_token: NameToken { name: "function_variable_one".to_string() }, assignable: AssignableToken::IntegerToken(IntegerToken { value: 10 }) }), Token::Variable(VariableToken { name_token: NameToken { name: "function_variable_two".to_string() }, assignable: AssignableToken::IntegerToken(IntegerToken { value: 2 }) })] }),
        Token::MethodDefinition(MethodDefinition { name: NameToken { name: "method_without_parameters".to_string() }, return_type: NameToken { name: "void".to_string() }, arguments: vec![], stack: vec![] }),
    ];

    assert_eq!(expected, top_level_scope.tokens);
    Ok(())
}

#[test]
fn function_different_return_type_test() -> anyhow::Result<()> {
    let function = r#"
    fn f(variable, variable): String
    {
        function_variable_zero = "Hallo";
    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    let expected = vec![
        Token::MethodDefinition(MethodDefinition {
            name: NameToken { name: "f".to_string() },
            return_type: NameToken { name: "String".to_string() },
            arguments: vec![AssignableToken::Variable(NameToken { name: "variable".to_string() }), AssignableToken::Variable(NameToken { name: "variable".to_string() })]
            ,
            stack: vec![
                Token::Variable(VariableToken { name_token: NameToken { name: "function_variable_zero".to_string() }, assignable: AssignableToken::String(StringToken { value: "\"Hallo\"".to_string() }) }),
            ],
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
        function_variable_zero = "Hallo";
        fn method_name(variable, variable): void {
            function_variable_one = 10;
            function_variable_two = 2;
        }
    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    let expected = vec![
        Token::MethodDefinition(MethodDefinition {
            name: NameToken { name: "f".to_string() },
            return_type: NameToken { name: "void".to_string() },
            arguments: vec![
                AssignableToken::Variable(NameToken { name: "variable".to_string() }),
                AssignableToken::Variable(NameToken { name: "variable".to_string() }),
            ],
            stack: vec![
                Token::Variable(VariableToken { name_token: NameToken { name: "function_variable_zero".to_string() }, assignable: AssignableToken::String(StringToken { value: "\"Hallo\"".to_string() }) }),
                Token::MethodDefinition(MethodDefinition { name: NameToken { name: "method_name".to_string() }, return_type: NameToken { name: "void".to_string() }, arguments: vec![AssignableToken::Variable(NameToken { name: "variable".to_string() }), AssignableToken::Variable(NameToken { name: "variable".to_string() })], stack: vec![Token::Variable(VariableToken { name_token: NameToken { name: "function_variable_one".to_string() }, assignable: AssignableToken::IntegerToken(IntegerToken { value: 10 }) }), Token::Variable(VariableToken { name_token: NameToken { name: "function_variable_two".to_string() }, assignable: AssignableToken::IntegerToken(IntegerToken { value: 2 }) })] }),
            ],
        }),
    ];

    assert_eq!(expected, top_level_scope.tokens);
    Ok(())
}
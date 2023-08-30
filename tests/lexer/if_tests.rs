use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::token::Token;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::lexer::tokens::assignable_token::AssignableToken;
use monkey_language::core::lexer::tokens::assignable_tokens::integer_token::IntegerToken;
use monkey_language::core::lexer::tokens::assignable_tokens::string_token::StringToken;
use monkey_language::core::lexer::tokens::if_definition::IfDefinition;
use monkey_language::core::lexer::tokens::name_token::NameToken;
use monkey_language::core::lexer::tokens::variable_token::VariableToken;

#[test]
fn if_test() -> anyhow::Result<()> {
    let function = r#"
    if (variable) {
        if_variable_one = 10;
        if_variable_two = 2;
    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    let expected = vec![
        Token::IfDefinition(IfDefinition { condition: AssignableToken::Variable(NameToken { name: String::from("variable")}), if_stack: vec![Token::Variable(VariableToken { name_token: NameToken { name: "if_variable_one".to_string() }, assignable: AssignableToken::IntegerToken(IntegerToken { value: 10 }) }), Token::Variable(VariableToken { name_token: NameToken { name: "if_variable_two".to_string() }, assignable: AssignableToken::IntegerToken(IntegerToken { value: 2 }) })], else_stack: None}),
    ];

    assert_eq!(expected, top_level_scope.tokens);

    let function = r#"
    if(variable){
    if_variable_one = 10;
        if_variable_two = 2;
    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    assert_eq!(expected, top_level_scope.tokens);

    let function = r#"
    if(variable){if_variable_one = 10;
        if_variable_two = 2;
    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    assert_eq!(expected, top_level_scope.tokens);
    Ok(())
}

#[test]
fn multiple_if_test() -> anyhow::Result<()> {
    let function = r#"
    if (variable1) {
        if_variable_one = 10;
        if_variable_two = 2;
    }

    if (variable2) {
        if_variable_one = 10;
        if_variable_two = 2;
    }


    if (variable3) {

    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    let expected = vec![
        Token::IfDefinition(IfDefinition { condition: AssignableToken::Variable(NameToken { name: String::from("variable1")}), if_stack: vec![Token::Variable(VariableToken { name_token: NameToken { name: "if_variable_one".to_string() }, assignable: AssignableToken::IntegerToken(IntegerToken { value: 10 }) }), Token::Variable(VariableToken { name_token: NameToken { name: "if_variable_two".to_string() }, assignable: AssignableToken::IntegerToken(IntegerToken { value: 2 }) })], else_stack: None}),
        Token::IfDefinition(IfDefinition { condition: AssignableToken::Variable(NameToken { name: String::from("variable2")}), if_stack: vec![Token::Variable(VariableToken { name_token: NameToken { name: "if_variable_one".to_string() }, assignable: AssignableToken::IntegerToken(IntegerToken { value: 10 }) }), Token::Variable(VariableToken { name_token: NameToken { name: "if_variable_two".to_string() }, assignable: AssignableToken::IntegerToken(IntegerToken { value: 2 }) })], else_stack: None}),
        Token::IfDefinition(IfDefinition { condition: AssignableToken::Variable(NameToken { name: String::from("variable3")}), if_stack: vec![], else_stack: None})
    ];

    assert_eq!(expected, top_level_scope.tokens);
    Ok(())
}

#[test]
fn if_else_test() -> anyhow::Result<()> {
    let function = r#"
    if (variable) {
        if_variable_one = 10;
        if_variable_two = 2;
    } else {
        else_variable_one = 10;
        else_variable_two = 2;
    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    let expected = vec![
        Token::IfDefinition(IfDefinition {
            condition: AssignableToken::Variable(NameToken { name: String::from("variable")}),
            if_stack: vec![Token::Variable(VariableToken { name_token: NameToken { name: "if_variable_one".to_string() }, assignable: AssignableToken::IntegerToken(IntegerToken { value: 10 }) }), Token::Variable(VariableToken { name_token: NameToken { name: "if_variable_two".to_string() }, assignable: AssignableToken::IntegerToken(IntegerToken { value: 2 }) })],
            else_stack: Some(vec![Token::Variable(VariableToken { name_token: NameToken { name: "else_variable_one".to_string() }, assignable: AssignableToken::IntegerToken(IntegerToken { value: 10 }) }), Token::Variable(VariableToken { name_token: NameToken { name: "else_variable_two".to_string() }, assignable: AssignableToken::IntegerToken(IntegerToken { value: 2 }) })])
        }),
    ];

    assert_eq!(expected, top_level_scope.tokens);

    let function = r#"
    if (variable) {if_variable_one = 10; if_variable_two = 2; } else {
        else_variable_one = 10;
        else_variable_two = 2;
    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    assert_eq!(expected, top_level_scope.tokens);

    let function = r#"
    if (variable) {
        if_variable_one = 10;
        if_variable_two = 2;
    }

    else { else_variable_one = 10; else_variable_two = 2; }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    assert_eq!(expected, top_level_scope.tokens);
    Ok(())
}

#[test]
fn function_in_function_test() -> anyhow::Result<()> {
    let function = r#"
    if (hallo) {
        if_stack_variable = 5;

        if(if_stack_variable) {
            nested_if_stack_variable = 13;
        }else{nested_else_stack_variable = "nice";}
    } else {
        else_stack_variable = "hallo";
    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    let expected = vec![
        Token::IfDefinition(IfDefinition {
            condition: AssignableToken::Variable(NameToken { name: "hallo".to_string() }),
            if_stack: vec![
                Token::Variable(VariableToken { name_token: NameToken { name: "if_stack_variable".to_string() }, assignable: AssignableToken::IntegerToken(IntegerToken { value: 5 }) }),
                Token::IfDefinition(IfDefinition {
                    condition: AssignableToken::Variable(NameToken { name: "if_stack_variable".to_string() }),
                    if_stack: vec![
                        Token::Variable(VariableToken { name_token: NameToken { name: "nested_if_stack_variable".to_string() }, assignable: AssignableToken::IntegerToken(IntegerToken { value: 13 }) })
                    ],
                    else_stack: Some(vec![
                        Token::Variable(VariableToken { name_token: NameToken { name: "nested_else_stack_variable".to_string() }, assignable: AssignableToken::String(StringToken { value: "\"nice\"".to_string() }) })
                    ]),
                })
            ],
            else_stack: Some(vec![
                Token::Variable(VariableToken { name_token: NameToken { name: "else_stack_variable".to_string() }, assignable: AssignableToken::String(StringToken { value: "\"hallo\"".to_string() }) })
            ]),
        })
    ];

    assert_eq!(expected, top_level_scope.tokens);
    Ok(())
}
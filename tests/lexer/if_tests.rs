use monkey_language::core::io::code_line::CodeLine;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::token::Token;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::lexer::tokens::assignable_token::AssignableToken;
use monkey_language::core::lexer::tokens::assignable_tokens::equation_parser::expression::Expression;
use monkey_language::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;
use monkey_language::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator::Div;
use monkey_language::core::lexer::tokens::assignable_tokens::integer_token::IntegerToken;
use monkey_language::core::lexer::tokens::assignable_tokens::string_token::StringToken;
use monkey_language::core::lexer::tokens::if_definition::IfDefinition;
use monkey_language::core::lexer::tokens::name_token::NameToken;
use monkey_language::core::lexer::tokens::variable_token::VariableToken;
use monkey_language::core::lexer::type_token::TypeToken;

#[test]
fn if_test() -> anyhow::Result<()> {
    let function = r#"
    if (variable) {
        let mut if_variable_one = 10;
        let if_variable_two = 2;
    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    let expected = vec![
        Token::IfDefinition(IfDefinition {
            condition: AssignableToken::Variable(NameToken { name: String::from("variable")}),
            if_stack: vec![
                Token::Variable(VariableToken { name_token: NameToken { name: "if_variable_one".to_string() }, mutability: true, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 10 }), code_line: CodeLine { line: "let mut if_variable_one = 10 ;".to_string(), actual_line_number: 3..3, virtual_line_number: 2 } }),
                Token::Variable(VariableToken { name_token: NameToken { name: "if_variable_two".to_string() }, mutability: false, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 2 }), code_line: CodeLine { line: "let if_variable_two = 2 ;".to_string(), actual_line_number: 4..4, virtual_line_number: 3 } })
            ],
            else_stack: None,
            code_line: CodeLine { line: "if  ( variable )  {".to_string(), actual_line_number: 2..2, virtual_line_number: 1 } }),
    ];

    println!("{:?}", top_level_scope.tokens);
    println!("{:?}", expected);

    assert_eq!(expected, top_level_scope.tokens);

    let function = r#"
    if(variable){
    let mut if_variable_one = 10;
        let if_variable_two = 2;
    }
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    assert_eq!(expected, top_level_scope.tokens);

    let function = r#"
    if(variable){let mut if_variable_one = 10;
        let if_variable_two = 2;
    }
    "#;


    let expected = vec![
        Token::IfDefinition(IfDefinition {
            condition: AssignableToken::Variable(NameToken { name: String::from("variable")}),
            if_stack: vec![
                Token::Variable(VariableToken { name_token: NameToken { name: "if_variable_one".to_string() }, mutability: true, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 10 }), code_line: CodeLine { line: "let mut if_variable_one = 10 ;".to_string(), actual_line_number: 2..2, virtual_line_number: 2 } }),
                Token::Variable(VariableToken { name_token: NameToken { name: "if_variable_two".to_string() }, mutability: false, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 2 }), code_line: CodeLine { line: "let if_variable_two = 2 ;".to_string(), actual_line_number: 3..3, virtual_line_number: 3 } })
            ],
            else_stack: None,
            code_line: CodeLine { line: "if  ( variable )  {".to_string(), actual_line_number: 2..2, virtual_line_number: 1 } }),
    ];

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


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    let expected = vec![
        Token::IfDefinition(IfDefinition { condition: AssignableToken::Variable(NameToken { name: String::from("variable1")}), if_stack: vec![Token::Variable(VariableToken { name_token: NameToken { name: "if_variable_one".to_string() }, mutability: false, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 10 }), code_line: CodeLine { line: "let if_variable_one = 10 ;".to_string(), actual_line_number: 3..3, virtual_line_number: 2 } }), Token::Variable(VariableToken { name_token: NameToken { name: "if_variable_two".to_string() }, mutability: false, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 2 }), code_line: CodeLine { line: "let if_variable_two = 2 ;".to_string(), actual_line_number: 4..4, virtual_line_number: 3 } })], else_stack: None, code_line: CodeLine { line: "if  ( variable1 )  {".to_string(), actual_line_number: 2..2, virtual_line_number: 1 } }),
        Token::IfDefinition(IfDefinition { condition: AssignableToken::Variable(NameToken { name: String::from("variable2")}), if_stack: vec![Token::Variable(VariableToken { name_token: NameToken { name: "if_variable_one".to_string() }, mutability: false, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 10 }), code_line: CodeLine { line: "let if_variable_one = 10 ;".to_string(), actual_line_number: 8..8, virtual_line_number: 6 } }), Token::Variable(VariableToken { name_token: NameToken { name: "if_variable_two".to_string() }, mutability: false, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 2 }), code_line: CodeLine { line: "let if_variable_two = 2 ;".to_string(), actual_line_number: 9..9, virtual_line_number: 7 } })], else_stack: None, code_line: CodeLine { line: "if  ( variable2 )  {".to_string(), actual_line_number: 7..7, virtual_line_number: 5 } }),
        Token::IfDefinition(IfDefinition { condition: AssignableToken::Variable(NameToken { name: String::from("variable3")}), if_stack: vec![], else_stack: None, code_line: CodeLine { line: "if  ( variable3 )  {".to_string(), actual_line_number: 13..13, virtual_line_number: 9 } })
    ];

    assert_eq!(expected, top_level_scope.tokens);
    Ok(())
}

#[test]
fn if_else_test() -> anyhow::Result<()> {
    let function = r#"if (variable) {
        let mut   if_variable_one = 10;
        let if_variable_two = 2;
    } else {
        let else_variable_one = 10;
        let mut else_variable_two = 2;
    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    let expected = vec![
        Token::IfDefinition(IfDefinition {
            condition: AssignableToken::Variable(NameToken { name: String::from("variable")}),
            if_stack: vec![
                Token::Variable(VariableToken { name_token: NameToken { name: "if_variable_one".to_string() }, mutability: true, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 10 }), code_line: CodeLine { line: "let mut if_variable_one = 10 ;".to_string(), actual_line_number: 2..2, virtual_line_number: 2 } }),
                Token::Variable(VariableToken { name_token: NameToken { name: "if_variable_two".to_string() }, mutability: false, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 2 }), code_line: CodeLine { line: "let if_variable_two = 2 ;".to_string(), actual_line_number: 3..3, virtual_line_number: 3 } })
            ],
            else_stack: Some(vec![
                Token::Variable(VariableToken { name_token: NameToken { name: "else_variable_one".to_string() }, mutability: false, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 10 }), code_line: CodeLine { line: "let else_variable_one = 10 ;".to_string(), actual_line_number: 5..5, virtual_line_number: 6 } }),
                Token::Variable(VariableToken { name_token: NameToken { name: "else_variable_two".to_string() }, mutability: true, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 2 }), code_line: CodeLine { line: "let mut else_variable_two = 2 ;".to_string(), actual_line_number: 6..6, virtual_line_number: 7 } })
            ]),
            code_line: CodeLine { line: "if  ( variable )  {".to_string(), actual_line_number: 1..1, virtual_line_number: 1 },
        }),
    ];

    assert_eq!(expected, top_level_scope.tokens);


    let function = r#"
    if (variable) {let mut if_variable_one = 10; let if_variable_two = 2; } else {
        let else_variable_one = 10;
        let   mut else_variable_two = 2;
    }
    "#;


    let expected = vec![
        Token::IfDefinition(IfDefinition {
            condition: AssignableToken::Variable(NameToken { name: String::from("variable")}),
            if_stack: vec![
                Token::Variable(VariableToken { name_token: NameToken { name: "if_variable_one".to_string() }, mutability: true, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 10 }), code_line: CodeLine { line: "let mut if_variable_one = 10 ;".to_string(), actual_line_number: 2..2, virtual_line_number: 2 } }),
                Token::Variable(VariableToken { name_token: NameToken { name: "if_variable_two".to_string() }, mutability: false, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 2 }), code_line: CodeLine { line: "let if_variable_two = 2 ;".to_string(), actual_line_number: 2..2, virtual_line_number: 3 } })
            ],
            else_stack: Some(vec![
                Token::Variable(VariableToken { name_token: NameToken { name: "else_variable_one".to_string() }, mutability: false, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 10 }), code_line: CodeLine { line: "let else_variable_one = 10 ;".to_string(), actual_line_number: 3..3, virtual_line_number: 6 } }),
                Token::Variable(VariableToken { name_token: NameToken { name: "else_variable_two".to_string() }, mutability: true, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 2 }), code_line: CodeLine { line: "let mut else_variable_two = 2 ;".to_string(), actual_line_number: 4..4, virtual_line_number: 7 } })
            ]),
            code_line: CodeLine { line: "if  ( variable )  {".to_string(), actual_line_number: 2..2, virtual_line_number: 1 },
        }),
    ];

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    assert_eq!(expected, top_level_scope.tokens);


    let function = r#"
    if (variable) {
        let mut if_variable_one = 10;
        let if_variable_two = 2;
    }

    else { let else_variable_one = 10; let mut else_variable_two = 2; }
    "#;

    let expected = vec![
        Token::IfDefinition(IfDefinition {
            condition: AssignableToken::Variable(NameToken { name: String::from("variable")}),
            if_stack: vec![
                Token::Variable(VariableToken { name_token: NameToken { name: "if_variable_one".to_string() }, mutability: true, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 10 }), code_line: CodeLine { line: "let mut if_variable_one = 10 ;".to_string(), actual_line_number: 3..3, virtual_line_number: 2 } }),
                Token::Variable(VariableToken { name_token: NameToken { name: "if_variable_two".to_string() }, mutability: false, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 2 }), code_line: CodeLine { line: "let if_variable_two = 2 ;".to_string(), actual_line_number: 4..4, virtual_line_number: 3 } })
            ],
            else_stack: Some(vec![
                Token::Variable(VariableToken { name_token: NameToken { name: "else_variable_one".to_string() }, mutability: false, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 10 }), code_line: CodeLine { line: "let else_variable_one = 10 ;".to_string(), actual_line_number: 7..7, virtual_line_number: 6 } }),
                Token::Variable(VariableToken { name_token: NameToken { name: "else_variable_two".to_string() }, mutability: true, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 2 }), code_line: CodeLine { line: "let mut else_variable_two = 2 ;".to_string(), actual_line_number: 7..7, virtual_line_number: 7 } })
            ]),
            code_line: CodeLine { line: "if  ( variable )  {".to_string(), actual_line_number: 2..2, virtual_line_number: 1 },
        }),
    ];

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
        let if_stack_variable = 5 / 2;

        if(if_stack_variable) {
            let nested_if_stack_variable = 13;
        } else {let nested_else_stack_variable = "nice";}
    } else {
        let else_stack_variable = "hallo";
    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    let expected = vec![
        Token::IfDefinition(IfDefinition {
            condition: AssignableToken::Variable(NameToken { name: "hallo".to_string() }),
            if_stack: vec![
                Token::Variable(VariableToken { name_token: NameToken { name: "if_stack_variable".to_string() }, mutability: false, ty: Some(TypeToken::F32), define: true, assignable: AssignableToken::ArithmeticEquation(
                    Expression { lhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 5 }))), positive: true })), operator: Div, rhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 2 }))), positive: true })), positive: true, value: None }
                ),
                    code_line: CodeLine { line: "let if_stack_variable = 5 / 2 ;".to_string(), actual_line_number: 3..3, virtual_line_number: 2 },
                }),
                Token::IfDefinition(IfDefinition {
                    condition: AssignableToken::Variable(NameToken { name: "if_stack_variable".to_string() }),
                    if_stack: vec![
                        Token::Variable(VariableToken { name_token: NameToken { name: "nested_if_stack_variable".to_string() }, mutability: false, ty: Some(TypeToken::I32), define: true, assignable: AssignableToken::IntegerToken(IntegerToken { value: 13 }), code_line: CodeLine { line: "let nested_if_stack_variable = 13 ;".to_string(), actual_line_number: 6..6, virtual_line_number: 4 } })
                    ],
                    else_stack: Some(vec![
                        Token::Variable(VariableToken { name_token: NameToken { name: "nested_else_stack_variable".to_string() }, mutability: false, ty: Some(TypeToken::String), define: true, assignable: AssignableToken::String(StringToken { value: "\"nice\"".to_string() }), code_line: CodeLine { line: "let nested_else_stack_variable = \"nice\" ;".to_string(), actual_line_number: 7..7, virtual_line_number: 7 } })
                    ]),
                    code_line: CodeLine { line: "if  ( if_stack_variable )  {".to_string(), actual_line_number: 5..5, virtual_line_number: 3 },
                })
            ],
            else_stack: Some(vec![
                Token::Variable(VariableToken { name_token: NameToken { name: "else_stack_variable".to_string() }, mutability: false, ty: Some(TypeToken::String), define: true, assignable: AssignableToken::String(StringToken { value: "\"hallo\"".to_string() }), code_line: CodeLine { line: "let else_stack_variable = \"hallo\" ;".to_string(), actual_line_number: 9..9, virtual_line_number: 11 } })
            ]),
            code_line: CodeLine { line: "if  ( hallo )  {".to_string(), actual_line_number: 2..2, virtual_line_number: 1 },
        })
    ];

    assert_eq!(expected, top_level_scope.tokens);
    Ok(())
}
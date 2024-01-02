use monkey_language::core::io::code_line::CodeLine;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::token::Token;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::lexer::tokens::assignable_token::AssignableToken;
use monkey_language::core::lexer::tokens::assignable_tokens::equation_parser::expression::{Expression, PointerArithmetic};
use monkey_language::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;
use monkey_language::core::lexer::tokens::assignable_tokens::integer_token::IntegerToken;
use monkey_language::core::lexer::tokens::assignable_tokens::method_call_token::MethodCallToken;
use monkey_language::core::lexer::tokens::assignable_tokens::object_token::ObjectToken;
use monkey_language::core::lexer::tokens::assignable_tokens::string_token::StringToken;
use monkey_language::core::lexer::tokens::name_token::NameToken;
use monkey_language::core::lexer::tokens::variable_token::VariableToken;
use monkey_language::core::lexer::type_token::TypeToken;

#[test]
fn variable_test() -> anyhow::Result<()> {
    let variables = r#"
    let fisch = "Fische sind wirklich wirklich toll";
    let hallo = "Thomas"; let tschuess = 5;
    let mallo = "";
    let michi =
    Data {
        guten: "Hallo",
        ciau: 5,
        rofl: name(),
        mofl: name(nestedMethod("Hallo", moin("Ciao", 5)))
    };
    let value = 9;
    let ref_value = &value;
    let pointer_arithmetic = *ref_value + 1;
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(variables);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    let expected = vec![
        Token::Variable(
            VariableToken {
                name_token: NameToken { name: "fisch".to_string() },
                mutability: false,
                ty: Some(TypeToken::Custom(NameToken { name: String::from("*string") })),
                define: true,
                assignable: AssignableToken::String(StringToken { value: "\"Fische sind wirklich wirklich toll\"".to_string() }),
                code_line: CodeLine { line: "let fisch = \"Fische sind wirklich wirklich toll\" ;".to_string(), actual_line_number: 2..2, virtual_line_number: 1 },
            }
        ),
        Token::Variable(
            VariableToken {
                name_token: NameToken { name: "hallo".to_string() },
                mutability: false,
                ty: Some(TypeToken::Custom(NameToken { name: String::from("*string") })),
                define: true,
                assignable: AssignableToken::String(StringToken { value: "\"Thomas\"".to_string() }),
                code_line: CodeLine { line: "let hallo = \"Thomas\" ;".to_string(), actual_line_number: 3..3, virtual_line_number: 2 },
            }
        ),
        Token::Variable(
            VariableToken {
                name_token: NameToken { name: "tschuess".to_string() },
                mutability: false,
                ty: Some(TypeToken::I32),
                define: true,
                assignable: AssignableToken::IntegerToken(IntegerToken { value: 5 }),
                code_line: CodeLine { line: "let tschuess = 5 ;".to_string(), actual_line_number: 3..3, virtual_line_number: 3 },
            }
        ),
        Token::Variable(
            VariableToken {
                name_token: NameToken { name: "mallo".to_string() },
                mutability: false,
                ty: Some(TypeToken::Custom(NameToken { name: String::from("*string") })),
                define: true,
                assignable: AssignableToken::String(StringToken { value: "\"\"".to_string() }),
                code_line: CodeLine { line: "let mallo = \"\" ;".to_string(), actual_line_number: 4..4, virtual_line_number: 4 },
            }
        ),
        Token::Variable(
            VariableToken {
                name_token: NameToken { name: "michi".to_string() },
                mutability: false,
                ty: Some(TypeToken::Custom(NameToken { name: "Data".to_string() })),
                define: true,
                assignable: AssignableToken::Object(ObjectToken {
                    variables: vec![
                        VariableToken {
                            name_token: NameToken { name: "guten".to_string() },
                            mutability: false,
                            ty: Some(TypeToken::Custom(NameToken { name: String::from("*string") })),
                            define: false,
                            assignable: AssignableToken::String(StringToken { value: "\"Hallo\"".to_string() }),
                            code_line: CodeLine { line: "guten : \"Hallo\" ,".to_string(), actual_line_number: 0..0, virtual_line_number: 0 },
                        },
                        VariableToken {
                            name_token: NameToken { name: "ciau".to_string() },
                            mutability: false,
                            ty: Some(TypeToken::I32),
                            define: false,
                            assignable: AssignableToken::IntegerToken(IntegerToken { value: 5 }),
                            code_line: CodeLine { line: "ciau : 5 ,".to_string(), actual_line_number: 0..0, virtual_line_number: 0 },
                        },
                        VariableToken {
                            name_token: NameToken { name: "rofl".to_string() },
                            mutability: false,
                            ty: None,
                            define: false,
                            assignable: AssignableToken::MethodCallToken(
                                MethodCallToken {
                                    name: NameToken { name: "name".to_string() },
                                    arguments: vec![],
                                    code_line: CodeLine { line: "name ( ) ;".to_string(), actual_line_number: 0..0, virtual_line_number: 0 },
                                }
                            ),
                            code_line: CodeLine { line: "rofl : name ( ) ,".to_string(), actual_line_number: 0..0, virtual_line_number: 0 },
                        },
                        VariableToken {
                            name_token: NameToken { name: "mofl".to_string() },
                            mutability: false,
                            ty: None,
                            define: false,
                            assignable: AssignableToken::MethodCallToken(MethodCallToken {
                                name: NameToken { name: "name".to_string() },
                                arguments: vec![
                                    AssignableToken::MethodCallToken(MethodCallToken {
                                        name: NameToken { name: "nestedMethod".to_string() },
                                        arguments: vec![
                                            AssignableToken::String(StringToken { value: "\"Hallo\"".to_string() }),
                                            AssignableToken::MethodCallToken(MethodCallToken {
                                                name: NameToken { name: "moin".to_string() },
                                                arguments: vec![
                                                    AssignableToken::String(StringToken { value: "\"Ciao\"".to_string() }),
                                                    AssignableToken::IntegerToken(IntegerToken { value: 5 }),
                                                ],
                                                code_line: CodeLine { line: "moin ( \"Ciao\" , 5 ) ;".to_string(), actual_line_number: 0..0, virtual_line_number: 0 },
                                            }),
                                        ],
                                        code_line: CodeLine { line: "nestedMethod ( \"Hallo\" , moin ( \"Ciao\" , 5 ) ) ;".to_string(), actual_line_number: 0..0, virtual_line_number: 0 },
                                    })],
                                code_line: CodeLine { line: "name ( nestedMethod ( \"Hallo\" , moin ( \"Ciao\" , 5 ) ) ) ;".to_string(), actual_line_number: 0..0, virtual_line_number: 0 },
                            }),
                            code_line: CodeLine { line: "mofl : name ( nestedMethod ( \"Hallo\" , moin ( \"Ciao\" , 5 ) ) ) ,".to_string(), actual_line_number: 0..0, virtual_line_number: 0 },
                        }],
                    ty: TypeToken::Custom(NameToken { name: "Data".to_string() }),
                }),
                code_line: CodeLine { line: "let michi = Data {  guten :  \"Hallo\" ,  ciau :  5 ,  rofl :  name (  )  ,  mofl :  name ( nestedMethod ( \"Hallo\" ,  moin ( \"Ciao\" ,  5 )  )  )  }  ;".to_string(), actual_line_number: 5..11, virtual_line_number: 5 },
            }
        ),
        Token::Variable(
            VariableToken {
                name_token: NameToken { name: "value".to_string() },
                mutability: false,
                ty: Some(TypeToken::I32),
                define: true,
                assignable: AssignableToken::IntegerToken(IntegerToken { value: 9 }),
                code_line: CodeLine { line: "let value = 9 ;".to_string(), actual_line_number: 12..12, virtual_line_number: 6 },
            }
        ),
        Token::Variable(
            VariableToken {
                name_token: NameToken { name: "ref_value".to_string() },
                mutability: false,
                ty: Some(TypeToken::Custom(NameToken { name: "*i32".to_string() })),
                define: true,
                assignable: AssignableToken::ArithmeticEquation(Expression {
                    lhs: None,
                    rhs: None,
                    operator: Operator::Noop,
                    pointer_arithmetic: vec![PointerArithmetic::Ampersand],
                    value: Some(Box::new(AssignableToken::NameToken(NameToken { name: "value".to_string() }))),
                    positive: true,
                }),
                code_line: CodeLine { line: "let ref_value = &value ;".to_string(), actual_line_number: 13..13, virtual_line_number: 7 },
            }
        ),
        // let pointer_arithmetic = *ref_value + 1;
        Token::Variable(
            VariableToken {
                name_token: NameToken { name: "pointer_arithmetic".to_string() },
                mutability: false,
                ty: Some(TypeToken::I32),
                define: true,
                assignable: AssignableToken::ArithmeticEquation(Expression {
                    lhs: Some(Box::new(Expression {
                        value: Some(Box::new(AssignableToken::NameToken(NameToken { name: "ref_value".to_string() }))),
                        pointer_arithmetic: vec![PointerArithmetic::Asterics],
                        positive: true,
                        ..Default::default()
                    })),
                    rhs: Some(Box::new(Expression {
                        value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 1 }))),
                        positive: true,
                        ..Default::default()
                    })),
                    operator: Operator::Add,
                    pointer_arithmetic: vec![],
                    value: None,
                    positive: true,
                }),
                code_line: CodeLine { line: "let pointer_arithmetic = *ref_value + 1 ;".to_string(), actual_line_number: 14..14, virtual_line_number: 8 },
            }
        ),
    ];

    assert_eq!(expected, top_level_scope.tokens);

    Ok(())
}
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::token::Token;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::lexer::tokens::assignable_token::AssignableToken;
use monkey_language::core::lexer::tokens::assignable_tokens::integer_token::IntegerToken;
use monkey_language::core::lexer::tokens::assignable_tokens::method_call_token::MethodCallToken;
use monkey_language::core::lexer::tokens::assignable_tokens::object_token::ObjectToken;
use monkey_language::core::lexer::tokens::assignable_tokens::string_token::StringToken;
use monkey_language::core::lexer::tokens::name_token::NameToken;
use monkey_language::core::lexer::tokens::variable_token::VariableToken;

#[test]
fn variable_test() -> anyhow::Result<()> {
    let variables = r#"
    let fisch = "Fische sind wirklich wirklich toll";
    let hallo = "Thomas"; let tschuess = 5;
    let mallo = "";
    let michi =
    {
        guten: "Hallo",
        ciau: 5,
        rofl: name(),
        mofl: name(nestedMethod("Hallo", moin("Ciao", 5)))
    };
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(variables);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    let expected = vec![
        Token::Variable(
            VariableToken {
                name_token: NameToken { name: "fisch".to_string() },
                define: true,
                assignable: AssignableToken::String(StringToken { value: "\"Fische sind wirklich wirklich toll\"".to_string() }),
            }
        ),
        Token::Variable(
            VariableToken {
                name_token: NameToken { name: "hallo".to_string() },
                define: true,
                assignable: AssignableToken::String(StringToken { value: "\"Thomas\"".to_string() }),
            }
        ),
        Token::Variable(
            VariableToken {
                name_token: NameToken { name: "tschuess".to_string() },
                define: true,
                assignable: AssignableToken::IntegerToken(IntegerToken { value: 5 }),
            }
        ),
        Token::Variable(
            VariableToken {
                name_token: NameToken { name: "mallo".to_string() },
                define: true,
                assignable: AssignableToken::String(StringToken { value: "\"\"".to_string() }),
            }
        ),
        Token::Variable(
            VariableToken {
                name_token: NameToken { name: "michi".to_string() },
                define: true,
                assignable: AssignableToken::Object(ObjectToken {
                    variables: vec![
                        VariableToken {
                            name_token: NameToken { name: "guten".to_string() },
                            define: false,
                            assignable: AssignableToken::String(StringToken { value: "\"Hallo\"".to_string() }),
                        },
                        VariableToken {
                            name_token: NameToken { name: "ciau".to_string() },
                            define: false,
                            assignable: AssignableToken::IntegerToken(IntegerToken { value: 5 }),
                        },
                        VariableToken {
                            name_token: NameToken { name: "rofl".to_string() },
                            define: false,
                            assignable: AssignableToken::MethodCallToken(
                                MethodCallToken {
                                    name: NameToken { name: "name".to_string() },
                                    arguments: vec![],
                                }
                            ),
                        }, VariableToken {
                            name_token: NameToken { name: "mofl".to_string() },
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
                                            }),
                                        ],
                                    })],
                            }),
                        }]
                }),
            }),
    ];

    assert_eq!(expected, top_level_scope.tokens);


    Ok(())
}
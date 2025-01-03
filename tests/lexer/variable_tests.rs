use monkey_language::core::io::code_line::CodeLine;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::token::Token;
use monkey_language::core::lexer::tokenizer::Lexer;
use monkey_language::core::lexer::tokens::assignable_token::AssignableToken;
use monkey_language::core::lexer::tokens::assignable_tokens::equation_parser::expression::{Expression};
use monkey_language::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;
use monkey_language::core::lexer::tokens::assignable_tokens::equation_parser::prefix_arithmetic::{PointerArithmetic, PrefixArithmetic};
use monkey_language::core::lexer::tokens::assignable_tokens::integer_token::IntegerToken;
use monkey_language::core::lexer::tokens::assignable_tokens::method_call_token::MethodCallToken;
use monkey_language::core::lexer::tokens::assignable_tokens::object_token::ObjectToken;
use monkey_language::core::lexer::tokens::assignable_tokens::string_token::StringToken;
use monkey_language::core::lexer::tokens::l_value::LValue;
use monkey_language::core::lexer::tokens::name_token::NameToken;
use monkey_language::core::lexer::tokens::variable_token::VariableToken;
use monkey_language::core::lexer::types::float::Float;
use monkey_language::core::lexer::types::integer::Integer;
use monkey_language::core::lexer::types::type_token;
use monkey_language::core::lexer::types::type_token::{Mutability, TypeToken};

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
                l_value: LValue::Name(NameToken { name: "fisch".to_string() }),
                mutability: false,
                ty: Some(TypeToken::Custom(NameToken { name: String::from("*string") }, Mutability::Immutable)),
                define: true,
                assignable: AssignableToken::String(StringToken { value: "\"Fische sind wirklich wirklich toll\"".to_string() }),
                code_line: CodeLine { line: "let fisch = \"Fische sind wirklich wirklich toll\" ;".to_string(), actual_line_number: 2..2, virtual_line_number: 1 },
            }
        ),
        Token::Variable(
            VariableToken {
                l_value: LValue::Name(NameToken { name: "hallo".to_string() }),
                mutability: false,
                ty: Some(TypeToken::Custom(NameToken { name: String::from("*string") }, Mutability::Immutable)),
                define: true,
                assignable: AssignableToken::String(StringToken { value: "\"Thomas\"".to_string() }),
                code_line: CodeLine { line: "let hallo = \"Thomas\" ;".to_string(), actual_line_number: 3..3, virtual_line_number: 2 },
            }
        ),
        Token::Variable(
            VariableToken {
                l_value: LValue::Name(NameToken { name: "tschuess".to_string() }),
                mutability: false,
                ty: Some(TypeToken::Integer(Integer::I32, Mutability::Immutable)),
                define: true,
                assignable: AssignableToken::IntegerToken(IntegerToken { value: "5".to_string(), ty: Integer::I32 }),
                code_line: CodeLine { line: "let tschuess = 5 ;".to_string(), actual_line_number: 3..3, virtual_line_number: 3 },
            }
        ),
        Token::Variable(
            VariableToken {
                l_value: LValue::Name(NameToken { name: "mallo".to_string() }),
                mutability: false,
                ty: Some(TypeToken::Custom(NameToken { name: String::from("*string") }, Mutability::Immutable)),
                define: true,
                assignable: AssignableToken::String(StringToken { value: "\"\"".to_string() }),
                code_line: CodeLine { line: "let mallo = \"\" ;".to_string(), actual_line_number: 4..4, virtual_line_number: 4 },
            }
        ),
        Token::Variable(
            VariableToken {
                l_value: LValue::Name(NameToken { name: "michi".to_string() }),
                mutability: false,
                ty: Some(TypeToken::Custom(NameToken { name: "Data".to_string() }, Mutability::Immutable)),
                define: true,
                assignable: AssignableToken::Object(ObjectToken {
                    variables: vec![
                        VariableToken {
                            l_value: LValue::Name(NameToken { name: "guten".to_string() }),
                            mutability: false,
                            ty: Some(TypeToken::Custom(NameToken { name: String::from("*string") }, Mutability::Immutable)),
                            define: false,
                            assignable: AssignableToken::String(StringToken { value: "\"Hallo\"".to_string() }),
                            code_line: CodeLine { line: "guten : \"Hallo\" ,".to_string(), actual_line_number: 0..0, virtual_line_number: 0 },
                        },
                        VariableToken {
                            l_value: LValue::Name(NameToken { name: "ciau".to_string() }),
                            mutability: false,
                            ty: Some(TypeToken::Integer(Integer::I32, Mutability::Immutable)),
                            define: false,
                            assignable: AssignableToken::IntegerToken(IntegerToken { value: "5".to_string(), ty: Integer::I32 }),
                            code_line: CodeLine { line: "ciau : 5 ,".to_string(), actual_line_number: 0..0, virtual_line_number: 0 },
                        },
                        VariableToken {
                            l_value: LValue::Name(NameToken { name: "rofl".to_string() }),
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
                            l_value: LValue::Name(NameToken { name: "mofl".to_string() }),
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
                                                    AssignableToken::IntegerToken(IntegerToken { value: "5".to_string(), ty: Integer::I32 }),
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
                    ty: TypeToken::Custom(NameToken { name: "Data".to_string() }, Mutability::Immutable),
                }),
                code_line: CodeLine { line: "let michi = Data {  guten :  \"Hallo\" ,  ciau :  5 ,  rofl :  name (  )  ,  mofl :  name ( nestedMethod ( \"Hallo\" ,  moin ( \"Ciao\" ,  5 )  )  )  }  ;".to_string(), actual_line_number: 5..11, virtual_line_number: 5 },
            }
        ),
        Token::Variable(
            VariableToken {
                l_value: LValue::Name(NameToken { name: "value".to_string() }),
                mutability: false,
                ty: Some(TypeToken::Integer(Integer::I32, Mutability::Immutable)),
                define: true,
                assignable: AssignableToken::IntegerToken(IntegerToken { value: "9".to_string(), ty: Integer::I32 }),
                code_line: CodeLine { line: "let value = 9 ;".to_string(), actual_line_number: 12..12, virtual_line_number: 6 },
            }
        ),
        Token::Variable(
            VariableToken {
                l_value: LValue::Name(NameToken { name: "ref_value".to_string() }),
                mutability: false,
                ty: Some(TypeToken::Custom(NameToken { name: "*i32".to_string() }, Mutability::Immutable)),
                define: true,
                assignable: AssignableToken::ArithmeticEquation(Expression {
                    lhs: None,
                    rhs: None,
                    operator: Operator::Noop,
                    prefix_arithmetic: Some(PrefixArithmetic::PointerArithmetic(PointerArithmetic::Ampersand)),
                    value: Some(Box::new(AssignableToken::ArithmeticEquation(Expression {
                        lhs: None,
                        rhs: None,
                        operator: Operator::Noop,
                        prefix_arithmetic: None,
                        value: Some(Box::new(AssignableToken::NameToken(NameToken { name: "value".to_string() }))),
                        index_operator: None,
                        positive: true,
                    }))),
                    index_operator: None,
                    positive: true,
                }),
                code_line: CodeLine { line: "let ref_value = &value ;".to_string(), actual_line_number: 13..13, virtual_line_number: 7 },
            }
        ),
        // let pointer_arithmetic = *ref_value + 1;
        Token::Variable(
            VariableToken {
                l_value: LValue::Name(NameToken { name: "pointer_arithmetic".to_string() }),
                mutability: false,
                ty: Some(TypeToken::Integer(Integer::I32, Mutability::Immutable)),
                define: true,
                assignable: AssignableToken::ArithmeticEquation(Expression {
                    lhs: Some(Box::new(Expression {
                        value: Some(Box::new(AssignableToken::ArithmeticEquation(Expression {
                            lhs: None,
                            rhs: None,
                            operator: Operator::Noop,
                            prefix_arithmetic: None,
                            value: Some(Box::new(AssignableToken::NameToken(NameToken { name: "ref_value".to_string() }))),
                            index_operator: None,
                            positive: true,
                        }))),
                        positive: true,
                        prefix_arithmetic: Some(PrefixArithmetic::PointerArithmetic(PointerArithmetic::Asterics)),
                        ..Default::default()
                    })),
                    rhs: Some(Box::new(Expression {
                        value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: "1".to_string(), ty: Integer::I32 }))),
                        positive: true,
                        ..Default::default()
                    })),
                    operator: Operator::Add,
                    prefix_arithmetic: None,
                    value: None,
                    index_operator: None,
                    positive: true,
                }),
                code_line: CodeLine { line: "let pointer_arithmetic = *ref_value + 1 ;".to_string(), actual_line_number: 14..14, virtual_line_number: 8 },
            }
        ),
    ];

    assert_eq!(expected, top_level_scope.tokens);

    Ok(())
}

#[test]
fn variable_test_types() -> anyhow::Result<()> {
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
        type_token::common::string(),
        type_token::common::string(),
        TypeToken::Integer(Integer::I32, Mutability::Immutable),
        type_token::common::string(),
        TypeToken::Custom(NameToken { name: "Data".to_string() }, Mutability::Immutable),
        TypeToken::Integer(Integer::I32, Mutability::Immutable),
        TypeToken::Custom(NameToken { name: "*i32".to_string() }, Mutability::Immutable),
        TypeToken::Integer(Integer::I32, Mutability::Immutable),
    ];

    for (index, token) in top_level_scope.tokens.iter().enumerate() {
        match token {
            Token::Variable(v) if v.ty.is_some() => {
                if let Some(ty) = &v.ty {
                    assert_eq!(&expected[index], ty);
                } else {
                    assert!(false, "Didnt expect not inferred type");
                }
            },
            _ => assert!(false, "Didnt expect this type of token")
        }
    }

    Ok(())
}

#[test]
fn variable_test_casting() -> anyhow::Result<()> {
    let variables =
        r#"let a = (f32) 5;
    let b = (f32)((i32) 5);
    let f = 5;
    let c = (f32)(i32) 5;
    let c = (f32)((i32) 5);
    let d = 5.0 + 1.0;
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(variables);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;
    println!("{:?}", top_level_scope);

    let expected = vec![
        TypeToken::Float(Float::Float32, Mutability::Immutable),
        TypeToken::Float(Float::Float32, Mutability::Immutable),
        TypeToken::Integer(Integer::I32, Mutability::Immutable),
        TypeToken::Float(Float::Float32, Mutability::Immutable),
        TypeToken::Float(Float::Float32, Mutability::Immutable),
        TypeToken::Float(Float::Float32, Mutability::Immutable),
    ];

    for (index, token) in top_level_scope.tokens.iter().enumerate() {
        match token {
            Token::Variable(v) if v.ty.is_some() => {
                if let Some(ty) = &v.ty {
                    assert_eq!(&expected[index], ty, "Failed at: {}", v);
                } else {
                    assert!(false, "Didnt expect not inferred type");
                }
            },
            _ => assert!(false, "Didnt expect this type of token")
        }
    }

    Ok(())
}

#[test]
fn variable_test_double_casting() -> anyhow::Result<()> {
    let variables = r#"let b: f32 = (f32)(i32) 5;"#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(variables);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    let expected = vec![
        TypeToken::Float(Float::Float32, Mutability::Immutable),
    ];

    let s = None;

    for token in &top_level_scope.tokens {
        println!("{}", token);
        match token {
            Token::Variable(v) => {
                println!("{:?}", match &v.assignable {
                    AssignableToken::ArithmeticEquation(a) => {
                        &a.prefix_arithmetic
                    },
                    _ => { &s }
                });
            },
            _ => {}
        }
    }

    for (index, token) in top_level_scope.tokens.iter().enumerate() {
        match token {
            Token::Variable(v) if v.ty.is_some() => {
                if let Some(ty) = &v.ty {
                    assert_eq!(&expected[index], ty, "Failed at: {}", v);
                } else {
                    assert!(false, "Didnt expect not inferred type");
                }
            },
            _ => assert!(false, "Didnt expect this type of token")
        }
    }

    Ok(())
}


#[test]
fn variable_test_casting_complex_expression() -> anyhow::Result<()> {
    let variables = r#"let a: i32 = 5;
    let b: *i32 = &a;

    let c: i32 = 13;
    let d: *i32 = &c;

    let another_addition = (f32) (((1 + 2) + (3 + 4)) + (5 + 6)) + (f32) ((7 + (8 + 9)) + (10 + (11 + 12)));
    let addition1 = (f32) (((*d + *b) + (*b + *d)) + (*b + *b)) + (f32) ((*b + (*b + *b)) + (*b + (*d + *b)));
    let addition2 = ((((f32)*d + (f32) *b) + (f32)((*b + *d)) + (f32)(*b + *b))) + (f32)(((*b + (*b + *b)) + (*b + (*d + *b))));
    let addition3 = (((i32)((f32)*d + (f32)*b) + (*b + *d)) + (*b + *b)) + ((*b + (*b + *b)) + (*b + (*d + *b)));
    let addition4 = (((i32)((f32)*d + (f32)*b) + (*b + *d)) + (*b + *b)) + ((*b + (*b + *b)) + (*b + (*d + *b)));
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(variables);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    let expected = vec![
        TypeToken::Integer(Integer::I32, Mutability::Immutable),
        TypeToken::Custom(NameToken { name: "*i32".to_string() }, Mutability::Immutable),
        TypeToken::Integer(Integer::I32, Mutability::Immutable),
        TypeToken::Custom(NameToken { name: "*i32".to_string() }, Mutability::Immutable),
        TypeToken::Float(Float::Float32, Mutability::Immutable),
        TypeToken::Float(Float::Float32, Mutability::Immutable),
        TypeToken::Float(Float::Float32, Mutability::Immutable),
        TypeToken::Integer(Integer::I32, Mutability::Immutable),
        TypeToken::Integer(Integer::I32, Mutability::Immutable),
    ];

    for (index, token) in top_level_scope.tokens.iter().enumerate() {
        match token {
            Token::Variable(v) if v.ty.is_some() => {
                if let Some(ty) = &v.ty {
                    assert_eq!(&expected[index], ty, "FAILED AT: {token}");
                } else {
                    assert!(false, "Didnt expect not inferred type");
                }
            },
            _ => assert!(false, "Didnt expect this type of token")
        }
    }

    Ok(())
}

#[test]
fn variable_test_casting_complex() -> anyhow::Result<()> {
    let variables = r#"let f: i32 = 5;
    let r = &f;
    let p: i32 = ((i32)(f32)*r);
    let q = (i32)*(*f32)r;
    let k: i32 = (i32)(f32)(((i32)(f32)*r) + 2);
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(variables);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    println!("{:#?}", top_level_scope);

    let expected = vec![
        TypeToken::Integer(Integer::I32, Mutability::Immutable),
        TypeToken::Custom(NameToken { name: "*i32".to_string() }, Mutability::Immutable),
        TypeToken::Integer(Integer::I32, Mutability::Immutable),
        TypeToken::Integer(Integer::I32, Mutability::Immutable),
        TypeToken::Integer(Integer::I32, Mutability::Immutable),
    ];

    for token in &top_level_scope.tokens {
        println!("{}", token);
    }


    for (index, token) in top_level_scope.tokens.iter().enumerate() {
        match token {
            Token::Variable(v) if v.ty.is_some() => {
                if let Some(ty) = &v.ty {
                    assert_eq!(&expected[index], ty, "FAILED AT: {token}");
                } else {
                    assert!(false, "Didnt expect not inferred type");
                }
            },
            _ => assert!(false, "Didnt expect this type of token")
        }
    }

    Ok(())
}

#[test]
fn variable_test_integers() -> anyhow::Result<()> {
    let variables = r#"let a: i32 = 5;
    let b: i64 = 5;
    let c: i16 = 3;
    let d: i8 = 9;

    let e = (i64)a + b;

    let f: u8 = 2;
    let g: u16 = 3;
    let h: u32 = 4;
    let i: u64 = 5;

    let j = (u64)f + i;
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(variables);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    println!("{:#?}", top_level_scope);

    let expected = vec![
        TypeToken::Integer(Integer::I32, Mutability::Immutable),
        TypeToken::Integer(Integer::I64, Mutability::Immutable),
        TypeToken::Integer(Integer::I16, Mutability::Immutable),
        TypeToken::Integer(Integer::I8, Mutability::Immutable),
        TypeToken::Integer(Integer::I64, Mutability::Immutable),
        TypeToken::Integer(Integer::U8, Mutability::Immutable),
        TypeToken::Integer(Integer::U16, Mutability::Immutable),
        TypeToken::Integer(Integer::U32, Mutability::Immutable),
        TypeToken::Integer(Integer::U64, Mutability::Immutable),
        TypeToken::Integer(Integer::U64, Mutability::Immutable),
    ];

    for token in &top_level_scope.tokens {
        println!("{}", token);
    }


    for (index, token) in top_level_scope.tokens.iter().enumerate() {
        match token {
            Token::Variable(v) if v.ty.is_some() => {
                if let Some(ty) = &v.ty {
                    assert_eq!(&expected[index], ty, "FAILED AT: {token}");
                } else {
                    assert!(false, "Didnt expect not inferred type");
                }
            },
            _ => assert!(false, "Didnt expect this type of token")
        }
    }

    Ok(())
}

#[test]
fn variable_test_integers_assignable() -> anyhow::Result<()> {
    let variables = r#"let a: i32 = 5;
    let b: i64 = 5;
    let c: i16 = 3;
    let d: i8 = 9;

    let f: u8 = 2;
    let g: u16 = 3;
    let h: u32 = 4;
    let i: u64 = 5;
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(variables);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.tokenize()?;

    println!("{:#?}", top_level_scope);

    let expected = vec![
        TypeToken::Integer(Integer::I32, Mutability::Immutable),
        TypeToken::Integer(Integer::I64, Mutability::Immutable),
        TypeToken::Integer(Integer::I16, Mutability::Immutable),
        TypeToken::Integer(Integer::I8, Mutability::Immutable),
        TypeToken::Integer(Integer::U8, Mutability::Immutable),
        TypeToken::Integer(Integer::U16, Mutability::Immutable),
        TypeToken::Integer(Integer::U32, Mutability::Immutable),
        TypeToken::Integer(Integer::U64, Mutability::Immutable),
    ];

    for token in &top_level_scope.tokens {
        println!("{}", token);
    }


    for (index, token) in top_level_scope.tokens.iter().enumerate() {
        match token {
            Token::Variable(v) if v.ty.is_some() => {
                if let AssignableToken::IntegerToken(i) = &v.assignable {
                    assert_eq!(&expected[index], &TypeToken::Integer(i.ty.clone(), Mutability::Immutable), "FAILED AT: {token}");
                } else {
                    assert!(false, "Didnt expect not inferred type {}", v);
                }
            },
            _ => assert!(false, "Didnt expect this type of token")
        }
    }

    Ok(())
}
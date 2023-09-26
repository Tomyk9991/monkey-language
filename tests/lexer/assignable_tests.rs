use std::str::FromStr;
use monkey_language::core::io::code_line::CodeLine;

use monkey_language::core::lexer::tokens::assignable_token::AssignableToken;
use monkey_language::core::lexer::tokens::assignable_tokens::boolean_token::BooleanToken;
use monkey_language::core::lexer::tokens::assignable_tokens::double_token::FloatToken;
use monkey_language::core::lexer::tokens::assignable_tokens::equation_parser::equation_token_options::{ArithmeticEquationOptions, BooleanEquationOptions};
use monkey_language::core::lexer::tokens::assignable_tokens::equation_parser::EquationToken;
use monkey_language::core::lexer::tokens::assignable_tokens::equation_parser::expression::Expression;
use monkey_language::core::lexer::tokens::assignable_tokens::equation_parser::operator::Operator;
use monkey_language::core::lexer::tokens::assignable_tokens::integer_token::IntegerToken;
use monkey_language::core::lexer::tokens::assignable_tokens::method_call_token::MethodCallToken;
use monkey_language::core::lexer::tokens::assignable_tokens::object_token::ObjectToken;
use monkey_language::core::lexer::tokens::assignable_tokens::string_token::StringToken;
use monkey_language::core::lexer::tokens::name_token::NameToken;

#[test]
fn assignable_string() -> anyhow::Result<()> {
    let values: Vec<(bool, String)> = vec![
        (true, "\"This is a monkeystring\"".to_string()),
        (true, "\"\"".to_string()),
        (true, "\"2\"".to_string()),
        (true, "\" \"".to_string()),
        (true, "\"\"\"".to_string()),
        (false, "This is a not a monkeystring".to_string()),
        (false, "\"This".to_string()),
        (false, "This\"".to_string()),
        (false, "T\"his".to_string()),
        (false, "Thi\"s".to_string()),
        (false, "T\"his\"".to_string()),
    ];

    for (expected_result, value) in &values {
        let token = StringToken::from_str(value);
        if !*expected_result {
            if !*expected_result {
                println!("{}", token.err().unwrap());
            } else {
                token.unwrap();
            }
        }
    }

    Ok(())
}

#[test]
fn assignable_integer() -> anyhow::Result<()> {
    let values: Vec<(bool, String)> = vec![
        (false, "\"This is a monkeystring\"".to_string()),
        (false, "2\"\"".to_string()),
        (true, "2".to_string()),
        (true, "-0".to_string()),
        (true, "15".to_string()),
        (false, "-+12".to_string()),
        (true, "+0".to_string()),
        (true, "+12312".to_string()),
        (false, "2147483648".to_string()),
        (true, "2147483647".to_string()),
        (true, "-2147483648".to_string()),
        (false, "-2147483649".to_string()),
    ];

    for (expected_result, value) in &values {
        let token = IntegerToken::from_str(value);
        if !*expected_result {
            println!("{}", token.err().unwrap());
        } else {
            token.unwrap();
        }
    }

    Ok(())
}

#[test]
fn assignable_double() -> anyhow::Result<()> {
    let values: Vec<(bool, String)> = vec![
        (false, "\"This is a monkeystring\"".to_string()),
        (false, "2\"\"".to_string()),
        (false, "2".to_string()),
        (true, "3.14".to_string()),
        (true, "-0.5".to_string()),
        (true, ".25".to_string()),
        (false, "1,234.56".to_string()),
        (false, "3.14.159".to_string()),
        (false, "+1.0e-5".to_string()),
    ];

    for (expected_result, value) in &values {
        let token = FloatToken::from_str(value);
        if !*expected_result {
            println!("{}", token.err().unwrap());
        } else {
            token.unwrap();
        }
    }

    Ok(())
}

#[test]
fn assignable_object() -> anyhow::Result<()> {
    let values: Vec<(bool, String)> = vec![
        (true, "Data { key1 : \"value1\" , key2 : 1 }".to_string()),
        (false, "Data { 'key1' : \"value2\" , 'key2' : 2 }".to_string()),
        (true, "Data { }".to_string()),
        (false, "Data { :\"key1\" , \"key3\" }".to_string()),
        (false, "Data { \"key1\" , key4 : }".to_string()),
        (false, "Data { key1\"' : \"value5' , key2 : 3 }".to_string()),
        (false, "[ key1 : \"value6\" , key2\" : 4 }".to_string()),
        (true, "Data { key1 : Data { inner_key1 : \"value1\", inner_key2 : 5 }, key2 : 1 }".to_string()),
        (true, "Data { key1 : Data { inner_key1 : Data { inner_inner_key : \"value\" } }, key2 : 2 }".to_string()),
        (false, "Data { key1 : Data { 'inner_key1' : \"value2\", 'inner_key2' : 2 }, key2 : 2 }".to_string()),
        (false, "Data { key1 : Data { inner_key1 : \"value3\", : 3 }, key2 : 3 }".to_string()),
        (true, "Data { key1 : func1 ( param1, param2 ) , key2 : 1 }".to_string()),
        (false, "Data { key1 : 'func2 ( param1, param2 )' , key2 : 2 }".to_string()),
        (true, "Data { key1 : func3 ( func4 ( param1, param2 ), param3 ) , key2 : 1 }".to_string()),
        (true, "Data { key1 : Data { inner_key1 : func5 ( param1, param2 ), inner_key2 : 5 }, key2 : 1 }".to_string()),
        (false, "Data { key1 : Data { inner_key1 : 'func6 ( param1, param2 )', inner_key2 : 2 }, key2 : 2 }".to_string()),
        (true, "Data { key1 : Data { inner_key1 : Data { inner_inner_key : func7 ( param1 ) } }, key2 : 2 }".to_string()),
    ];

    for (expected_result, value) in &values {
        let token = ObjectToken::from_str(value);

        match *expected_result {
            true => assert!(token.is_ok(), "{:?}", token),
            false => assert!(token.is_err(), "{:?}", token)
        }
    }

    Ok(())
}


#[test]
fn assignable_imaginary_fn_calls() -> anyhow::Result<()> {
    let values: Vec<(bool, String)> = vec![
        (true, "imaginary_fn1 ( )".to_string()),
        (true, "imaginary_fn2 ( )".to_string()),
        (false, "imaginary_fn2 ( a * * b )".to_string()),
        (true, "imaginary_fn3 ( param1, param2 )".to_string()),
        (true, "imaginary_fn4 ( param1, imaginary_fn2 ( param2 ) )".to_string()),
        (false, "imaginary_fn5 (, param2 )".to_string()),
        (false, "imaginary_fn6 (param1,)".to_string()),
        (false, "imaginary_fn7 (param1 , 2 ,)".to_string()),
        (true, "imaginary_fn8 ( param1 , imaginary_fn2 ( param3 , param4 ) , param2 )".to_string()),
        (true, "imaginary_fn9 ( inner_fn1 ( param1 ) , inner_fn2 ( param2 ) )".to_string()),
        (true, "imaginary_fn10 ( inner_fn1 ( param1 ) , inner_fn2 ( param2 ) )".to_string()),
        (true, "imaginary_fn11 ( \"string_value\" ) ".to_string()),
        (true, "imaginary_fn12 ( 42 )".to_string()),
        (true, "imaginary_fn13 ( -42 )".to_string()),
        (true, "imaginary_fn14 ( 3.14 )".to_string()),
        (true, "imaginary_fn15 ( -3.14 )".to_string()),
        (true, "imaginary_fn16 ( 123 )".to_string()),
        (true, "imaginary_fn17 ( 31.4, test )".to_string()),
        (true, "imaginary_fn18 ( Data { key1 : \"value1\" , key2 : 1 } )".to_string()),
        (false, "imaginary_fn19 ( Data { 'key1' : \"value2\" , 'key2' : 2 } )".to_string()),
        (true, "imaginary_fn20 ( Data { } )".to_string()),
        (false, "imaginary_fn21 ( Data { key1 : Data { inner_key1 : 'func6 ( param1, param2 )', inner_key2 : 2 }, key2 : 2 } )".to_string()),
        (true, "imaginary_fn21 ( Data { key1 : Data { inner_key1 : Data { inner_inner_key : func7 ( param1 ) } }, key2 : 2 } )".to_string()),
        (true, "imaginary_fn22 ( Data { a : imaginary ( b ) } , imaginary ( Data { } ) ) ".to_string()),
    ];

    for (expected_result, value) in &values {
        let token = MethodCallToken::from_str(value);

        match *expected_result {
            true => assert!(token.is_ok(), "{:?}", value),
            false => assert!(token.is_err(), "{:?}", value)
        }
    }

    Ok(())
}

#[test]
fn assignable_booleans() -> anyhow::Result<()> {
    let values = [
        ("true", true),
        ("false", true),
        ("TRUE", false),
        ("FALSE", false),
        ("True", false),
        ("False", false),
        ("1", false),
        ("tru", false),
        ("falsey", false)
    ];

    for (value, expected_result) in &values {
        let token = BooleanToken::from_str(value);

        match *expected_result {
            true => assert!(token.is_ok(), "{:?}", value),
            false => assert!(token.is_err(), "{:?}", value)
        }
    }

    Ok(())
}

#[test]
fn assignable_arithmetic_equation() -> anyhow::Result<()> {
    let values: Vec<(bool, String, Option<Expression>)> = vec![
        (true, "a*b".to_string(), Some(Expression {
            lhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, value: Some(Box::new(AssignableToken::NameToken(NameToken { name: String::from("a") }))), positive: true })),
            operator: Operator::Mul,
            rhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, value: Some(Box::new(AssignableToken::NameToken(NameToken { name: String::from("b") }))), positive: true })),
            value: None,
            positive: true,
        })),
        (false, "a**b".to_string(), None),
        (false, "sqrt(b**c)".to_string(), None),
        (false, "sqrt(b*c)/".to_string(), None),
        (false, "a*sqrt(b**c)".to_string(), None),
        (true, "a+b*b".to_string(), Some(Expression {
            lhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, value: Some(Box::new(AssignableToken::NameToken(NameToken { name: String::from("a") }))), positive: true })),
            operator: Operator::Add,
            rhs: Some(Box::new(Expression {
                lhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, value: Some(Box::new(AssignableToken::NameToken(NameToken { name: String::from("b") }))), positive: true })),
                rhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, value: Some(Box::new(AssignableToken::NameToken(NameToken { name: String::from("b") }))), positive: true })),
                operator: Operator::Mul,
                value: None,
                positive: true,
            })),
            value: None,
            positive: true,
        })),
        (true, "1--1".to_string(), Some(Expression {
            lhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 1 }))), positive: true })),
            operator: Operator::Sub,
            rhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 1 }))), positive: false })),
            value: None,
            positive: true,
        })),
        (true, "1*-2".to_string(), Some(Expression {
            lhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 1 }))), positive: true })),
            operator: Operator::Mul,
            rhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 2 }))), positive: false })),
            value: None,
            positive: true,
        })),
        (true, "-(-1+-3)".to_string(), Some(Expression {
            lhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 1 }))), positive: false })),
            operator: Operator::Add,
            rhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 3 }))), positive: false })),
            value: None,
            positive: false,
        })),
        (true, "((4 - (2*3) * 5 + 1) * -(3*3+4*4)) / 2".to_string(), Some(Expression {
            lhs: Some(Box::new(Expression {
                lhs: Some(Box::new(Expression {
                    lhs: Some(Box::new(Expression {
                        lhs: Some(Box::new(Expression {
                            operator: Operator::Noop,
                            lhs: None,
                            rhs: None,
                            positive: true,
                            value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 4 }))),
                        })),
                        operator: Operator::Sub,
                        rhs: Some(Box::new(Expression {
                            lhs: Some(Box::new(Expression {
                                lhs: Some(Box::new(Expression {
                                    operator: Operator::Noop,
                                    lhs: None,
                                    rhs: None,
                                    positive: true,
                                    value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 2 }))),
                                })),
                                operator: Operator::Mul,
                                rhs: Some(Box::new(Expression {
                                    operator: Operator::Noop,
                                    lhs: None,
                                    rhs: None,
                                    positive: true,
                                    value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 3 }))),
                                })),
                                value: None,
                                positive: true,
                            })),
                            operator: Operator::Mul,
                            rhs: Some(Box::new(Expression {
                                operator: Operator::Noop,
                                lhs: None,
                                rhs: None,
                                positive: true,
                                value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 5 }))),
                            })),
                            value: None,
                            positive: true,
                        })),
                        value: None,
                        positive: true,
                    })),
                    operator: Operator::Add,
                    rhs: Some(Box::new(Expression {
                        value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 1 }))),
                        operator: Operator::Noop,
                        lhs: None,
                        rhs: None,
                        positive: true,
                    })),
                    value: None,
                    positive: true,
                })),
                operator: Operator::Mul,
                value: None,
                rhs: Some(Box::new(Expression {
                    lhs: Some(Box::new(Expression {
                        lhs: Some(Box::new(Expression {
                            operator: Operator::Noop,
                            lhs: None,
                            rhs: None,
                            positive: true,
                            value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 3 }))),
                        })),
                        operator: Operator::Mul,
                        value: None,
                        rhs: Some(Box::new(Expression {
                            operator: Operator::Noop,
                            lhs: None,
                            rhs: None,
                            positive: true,
                            value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 3 }))),
                        })),
                        positive: true,
                    })),
                    operator: Operator::Add,
                    value: None,
                    rhs: Some(Box::new(Expression {
                        lhs: Some(Box::new(Expression {
                            lhs: None,
                            rhs: None,
                            positive: true,
                            value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 4 }))),
                            operator: Operator::Noop,
                        })),
                        operator: Operator::Mul,
                        value: None,
                        rhs: Some(Box::new(Expression {
                            lhs: None,
                            rhs: None,
                            positive: true,
                            value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 4 }))),
                            operator: Operator::Noop,
                        })),
                        positive: true,
                    })),
                    positive: true,
                })),
                positive: true,
            })),
            operator: Operator::Div,
            rhs: Some(Box::new(Expression {
                operator: Operator::Noop,
                lhs: None,
                rhs: None,
                positive: true,
                value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 2 }))),
            })),
            value: None,
            positive: true,
        })),
        (true, "((4 - (2*3) * 5 + 1) * -sqrt) / 2".to_string(), Some(Expression {
            lhs: Some(Box::new(Expression {
                lhs: Some(Box::new(Expression {
                    lhs: Some(Box::new(Expression {
                        lhs: Some(Box::new(Expression {
                            operator: Operator::Noop,
                            lhs: None,
                            rhs: None,
                            positive: true,
                            value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 4 }))),
                        })),
                        operator: Operator::Sub,
                        rhs: Some(Box::new(Expression {
                            lhs: Some(Box::new(Expression {
                                lhs: Some(Box::new(Expression {
                                    operator: Operator::Noop,
                                    lhs: None,
                                    rhs: None,
                                    positive: true,
                                    value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 2 }))),
                                })),
                                operator: Operator::Mul,
                                rhs: Some(Box::new(Expression {
                                    operator: Operator::Noop,
                                    lhs: None,
                                    rhs: None,
                                    positive: true,
                                    value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 3 }))),
                                })),
                                value: None,
                                positive: true,
                            })),
                            operator: Operator::Mul,
                            rhs: Some(Box::new(Expression {
                                operator: Operator::Noop,
                                lhs: None,
                                rhs: None,
                                positive: true,
                                value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 5 }))),
                            })),
                            value: None,
                            positive: true,
                        })),
                        value: None,
                        positive: true,
                    })),
                    operator: Operator::Add,
                    rhs: Some(Box::new(Expression {
                        value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 1 }))),
                        operator: Operator::Noop,
                        lhs: None,
                        rhs: None,
                        positive: true,
                    })),
                    value: None,
                    positive: true,
                })),
                operator: Operator::Mul,
                value: None,
                rhs: Some(Box::new(Expression {
                    lhs: None,
                    operator: Operator::Noop,
                    value: Some(Box::new(AssignableToken::NameToken(NameToken { name: String::from("sqrt")}))),
                    rhs: None,
                    positive: false,
                })),
                positive: true,
            })),
            operator: Operator::Div,
            rhs: Some(Box::new(Expression {
                operator: Operator::Noop,
                lhs: None,
                rhs: None,
                positive: true,
                value: Some(Box::new(AssignableToken::IntegerToken(IntegerToken { value: 2 }))),
            })),
            value: None,
            positive: true,
        })),
        (true, "((4 - 2 * 3 + 1) * -sqrt(3*3+4*4)) / 2".to_string(), Some(Expression {
            lhs: Some(Box::new(
                Expression {
                    lhs: Some(Box::new(
                        Expression {
                            lhs: Some(Box::new(
                                Expression {
                                    lhs: Some(Box::new(
                                        Expression {
                                            lhs: None,
                                            operator: Operator::Noop,
                                            rhs: None,
                                            value: Some(Box::new(
                                                AssignableToken::IntegerToken(
                                                    IntegerToken {
                                                        value: 4,
                                                    },
                                                ),
                                            )),
                                            positive: true,
                                        },
                                    )),
                                    operator: Operator::Sub,
                                    rhs: Some(Box::new(
                                        Expression {
                                            lhs: Some(Box::new(
                                                Expression {
                                                    lhs: None,
                                                    operator: Operator::Noop,
                                                    rhs: None,
                                                    value: Some(Box::new(
                                                        AssignableToken::IntegerToken(
                                                            IntegerToken {
                                                                value: 2,
                                                            },
                                                        ),
                                                    )),
                                                    positive: true,
                                                },
                                            )),
                                            operator: Operator::Mul,
                                            rhs: Some(Box::new(
                                                Expression {
                                                    lhs: None,
                                                    operator: Operator::Noop,
                                                    rhs: None,
                                                    value: Some(Box::new(
                                                        AssignableToken::IntegerToken(
                                                            IntegerToken {
                                                                value: 3,
                                                            },
                                                        ),
                                                    )),
                                                    positive: true,
                                                },
                                            )),
                                            value: None,
                                            positive: true,
                                        },
                                    )),
                                    value: None,
                                    positive: true,
                                },
                            )),
                            operator: Operator::Add,
                            rhs: Some(Box::new(
                                Expression {
                                    lhs: None,
                                    operator: Operator::Noop,
                                    rhs: None,
                                    value: Some(Box::new(
                                        AssignableToken::IntegerToken(
                                            IntegerToken {
                                                value: 1,
                                            },
                                        )),
                                    ),
                                    positive: true,
                                },
                            )),
                            value: None,
                            positive: true,
                        },
                    )),
                    operator: Operator::Mul,
                    rhs: Some(Box::new(
                        Expression {
                            lhs: None,
                            operator: Operator::Noop,
                            rhs: None,
                            value: Some(Box::new(
                                AssignableToken::MethodCallToken(
                                    MethodCallToken {
                                        name: NameToken {
                                            name: String::from("sqrt"),
                                        },
                                        arguments: vec![
                                            AssignableToken::ArithmeticEquation(
                                                Expression {
                                                    lhs: Some(Box::new(
                                                        Expression {
                                                            lhs: Some(Box::new(
                                                                Expression {
                                                                    lhs: None,
                                                                    operator: Operator::Noop,
                                                                    rhs: None,
                                                                    value: Some(Box::new(
                                                                        AssignableToken::IntegerToken(
                                                                            IntegerToken {
                                                                                value: 3,
                                                                            },
                                                                        ),
                                                                    )),
                                                                    positive: true,
                                                                },
                                                            )),
                                                            operator: Operator::Mul,
                                                            rhs: Some(Box::new(
                                                                Expression {
                                                                    lhs: None,
                                                                    operator: Operator::Noop,
                                                                    rhs: None,
                                                                    value: Some(Box::new(
                                                                        AssignableToken::IntegerToken(
                                                                            IntegerToken {
                                                                                value: 3,
                                                                            },
                                                                        ),
                                                                    )),
                                                                    positive: true,
                                                                },
                                                            )),
                                                            value: None,
                                                            positive: true,
                                                        },
                                                    )),
                                                    operator: Operator::Add,
                                                    rhs: Some(Box::new(
                                                        Expression {
                                                            lhs: Some(Box::new(
                                                                Expression {
                                                                    lhs: None,
                                                                    operator: Operator::Noop,
                                                                    rhs: None,
                                                                    value: Some(Box::new(
                                                                        AssignableToken::IntegerToken(
                                                                            IntegerToken {
                                                                                value: 4,
                                                                            },
                                                                        ),
                                                                    )),
                                                                    positive: true,
                                                                },
                                                            )),
                                                            operator: Operator::Mul,
                                                            rhs: Some(Box::new(
                                                                Expression {
                                                                    lhs: None,
                                                                    operator: Operator::Noop,
                                                                    rhs: None,
                                                                    value: Some(Box::new(
                                                                        AssignableToken::IntegerToken(
                                                                            IntegerToken {
                                                                                value: 4,
                                                                            },
                                                                        ),
                                                                    )),
                                                                    positive: true,
                                                                },
                                                            )),
                                                            value: None,
                                                            positive: true,
                                                        },
                                                    )),
                                                    value: None,
                                                    positive: true,
                                                },
                                            ),
                                        ],
                                        code_line: CodeLine { line: "sqrt ( 3*3+4*4 ) ;".to_string(), actual_line_number: 0..0, virtual_line_number: 0},
                                    },
                                ),
                            )),
                            positive: false,
                        },
                    )),
                    value: None,
                    positive: true,
                }),
            ),
            operator: Operator::Div,
            rhs: Some(Box::new(
                Expression {
                    lhs: None,
                    operator: Operator::Noop,
                    rhs: None,
                    value: Some(Box::new(
                        AssignableToken::IntegerToken(
                            IntegerToken {
                                value: 2,
                            },
                        )),
                    ),
                    positive: true,
                },
            )),
            value: None,
            positive: true,
        })),
        (true, "a(b(c(d(e*f)))))".to_string(), Some(Expression {
            lhs: None,
            operator: Operator::Noop,
            rhs: None,
            value: Some(Box::new(
                AssignableToken::MethodCallToken(
                    MethodCallToken {
                        name: NameToken {
                            name: String::from("a"),
                        },
                        arguments: vec![
                            AssignableToken::MethodCallToken(
                                MethodCallToken {
                                    name: NameToken {
                                        name: String::from("b"),
                                    },
                                    arguments: vec![
                                        AssignableToken::MethodCallToken(
                                            MethodCallToken {
                                                name: NameToken {
                                                    name: String::from("c"),
                                                },
                                                arguments: vec![
                                                    AssignableToken::MethodCallToken(
                                                        MethodCallToken {
                                                            name: NameToken {
                                                                name: String::from("d"),
                                                            },
                                                            arguments: vec![
                                                                AssignableToken::ArithmeticEquation(
                                                                    Expression {
                                                                        lhs: Some(Box::new(
                                                                            Expression {
                                                                                lhs: None,
                                                                                operator: Operator::Noop,
                                                                                rhs: None,
                                                                                value: Some(Box::new(
                                                                                    AssignableToken::NameToken(
                                                                                        NameToken {
                                                                                            name: String::from("e"),
                                                                                        },
                                                                                    ),
                                                                                )),
                                                                                positive: true,
                                                                            },
                                                                        )),
                                                                        operator: Operator::Mul,
                                                                        rhs: Some(Box::new(
                                                                            Expression {
                                                                                lhs: None,
                                                                                operator: Operator::Noop,
                                                                                rhs: None,
                                                                                value: Some(Box::new(
                                                                                    AssignableToken::NameToken(
                                                                                        NameToken {
                                                                                            name: String::from("f"),
                                                                                        },
                                                                                    ),
                                                                                )),
                                                                                positive: true,
                                                                            },
                                                                        )),
                                                                        value: None,
                                                                        positive: true,
                                                                    },
                                                                ),
                                                            ],
                                                            code_line: CodeLine { line: "d ( e*f ) ;".to_string(), actual_line_number: 0..0, virtual_line_number: 0 },
                                                        },
                                                    ),
                                                ],
                                                code_line: CodeLine { line: "c ( d ( e*f ) ) ;".to_string(), actual_line_number: 0..0, virtual_line_number: 0},
                                            },
                                        ),
                                    ],
                                    code_line: CodeLine { line: "b ( c ( d ( e*f ) ) ) ;".to_string(), actual_line_number: 0..0, virtual_line_number: 0},
                                },
                            ),
                        ],
                        code_line: CodeLine { line: "a ( b ( c ( d ( e*f )  )  )  ) ;".to_string(), actual_line_number: 0..0, virtual_line_number: 0},
                    },
                ),
            )),
            positive: true,
        }
        )),
        (false, "((4 - 2 * ) -sqrt(3*3+4*4)) / 2".to_string(), None),
    ];

    for (expected_result, value, expected) in &values {
        let token = EquationToken::<ArithmeticEquationOptions>::from_str(value);


        match *expected_result {
            true => {
                if let Ok(token) = &token {
                    let s = expected.as_ref().unwrap();
                    assert_eq!(*s, *token);
                }

                assert!(token.is_ok(), "{value}, {:?}", token);
            }
            false => {
                if let Err(err) = &token {
                    println!("{:<5}{:?}", " ", err);
                }
                assert!(token.is_err(), "{:?}", value)
            }
        }
    }

    Ok(())
}

#[test]
fn assignable_boolean_equation() -> anyhow::Result<()> {
    let values: Vec<(bool, String)> = vec![
        (true, "a&b".to_string()),
        (true, "a|b&b".to_string()),
        (true, "a|b&b".to_string()),
        (true, "((true | (true&false) & true | false) & |(false&false|true&true)) & true".to_string()),
        (true, "((true | (true&false) & true | false) & |(false&false|true&true)) & true".to_string()),
        (true, "((true | (true&false) & true | false) & |(false&false|true&true)) & true".to_string()),
        (true, "((true | (true&false) & true | false) & |(false&false|true&true)) & true".to_string()),
        (true, "((true | (true&false) & true | false) & |sqrt) & true".to_string()),
        (true, "((true | true & false | false) & |sqrt(false&false|true&true)) & true".to_string()),
        (true, "((true | true&false | false) |sqrt(false&false|true&true)) & true".to_string()),
        (true, "a(b(c(d(e&f)))))".to_string()),
        (false, "((true | true & ) |sqrt(false&false|true&true)) & true".to_string()),
    ];

    for (expected_result, value) in &values {
        let token = EquationToken::<BooleanEquationOptions>::from_str(value);


        match *expected_result {
            true => {
                if let Ok(token) = &token {
                    println!("{}", token);
                }
                assert!(token.is_ok(), "{value}, {:?}", token);
            }
            false => assert!(token.is_err(), "{:?}", value)
        }
    }

    Ok(())
}
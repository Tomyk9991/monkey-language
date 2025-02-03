use std::str::FromStr;

use monkey_language::core::io::code_line::CodeLine;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::assignable::Assignable;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::assignables::boolean::Boolean;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::assignables::equation_parser::Equation;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::assignables::equation_parser::operator::Operator;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::assignables::equation_parser::prefix_arithmetic::{PointerArithmetic, PrefixArithmetic};
use monkey_language::core::lexer::abstract_syntax_tree_nodes::assignables::float::FloatAST;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::assignables::integer::IntegerAST;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::assignables::object::Object;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::assignables::string::StaticString;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::identifier::Identifier;
use monkey_language::core::lexer::types::integer::Integer;

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
        let static_string = StaticString::from_str(value);
        if !*expected_result {
            if !*expected_result {
                println!("{}", static_string.err().unwrap());
            } else {
                static_string.unwrap();
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
        (true, "2147483648".to_string()),
        (true, "2147483647".to_string()),
        (true, "-2147483648".to_string()),
        (true, "-2147483649".to_string()),
    ];

    for (expected_result, value) in &values {
        let integer = IntegerAST::from_str(value);
        if !*expected_result {
            println!("{}", integer.err().unwrap());
        } else {
            integer.unwrap();
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
        let float = FloatAST::from_str(value);
        println!("{}", value);
        if !*expected_result {
            println!("{}", float.err().unwrap());
        } else {
            float.unwrap();
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
        let object = Object::from_str(value);

        match *expected_result {
            true => assert!(object.is_ok(), "{:?}", object),
            false => assert!(object.is_err(), "{:?}", object)
        }
    }

    Ok(())
}


#[test]
fn assignable_imaginary_fn_calls() -> anyhow::Result<()> {
    let values: Vec<(bool, String)> = vec![
        (true, "imaginary_fn1 ( )".to_string()),
        (true, "imaginary_fn2 ( )".to_string()),
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
        let node = MethodCall::from_str(value);

        match *expected_result {
            true => assert!(node.is_ok(), "{:?}", value),
            false => assert!(node.is_err(), "{:?}", value)
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
        let node = Boolean::from_str(value);

        match *expected_result {
            true => assert!(node.is_ok(), "{:?}", value),
            false => assert!(node.is_err(), "{:?}", value)
        }
    }

    Ok(())
}

#[test]
fn assignable_arithmetic_equation() -> anyhow::Result<()> {
    let values: Vec<(bool, String, Option<Expression>)> = vec![
        (true, "a*b".to_string(), Some(Expression {
            lhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, prefix_arithmetic: None, value: Some(Box::new(Assignable::Identifier(Identifier { name: String::from("a") }))), index_operator: None, positive: true })),
            operator: Operator::Mul,
            rhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, value: Some(Box::new(Assignable::Identifier(Identifier { name: String::from("b") }))), index_operator: None, positive: true, prefix_arithmetic: None })),
            value: None,
            index_operator: None,
            positive: true,
            prefix_arithmetic: None,
        })),
        (true, "a**b".to_string(), Some(Expression {
            lhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, prefix_arithmetic: None, value: Some(Box::new(Assignable::Identifier(Identifier { name: String::from("a") }))), index_operator: None, positive: true })),
            operator: Operator::Mul,
            rhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, prefix_arithmetic: Some(PrefixArithmetic::PointerArithmetic(PointerArithmetic::Asterics)), value: Some(Box::new(Assignable::ArithmeticEquation(Expression {
                lhs: None,
                rhs: None,
                operator: Operator::Noop,
                prefix_arithmetic: None,
                value: Some(Box::new(Assignable::Identifier(Identifier { name: String::from("b") }))),
                index_operator: None,
                positive: true,
            }))),
                index_operator: None,
                positive: true })),
            value: None,
            index_operator: None,
            positive: true,
            prefix_arithmetic: None,
        })),
        (false, "sqrt(b*c)/".to_string(), None),
        (true, "a+b*b".to_string(), Some(Expression {
            lhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, value: Some(Box::new(Assignable::Identifier(Identifier { name: String::from("a") }))), index_operator: None, positive: true, prefix_arithmetic: None })),
            operator: Operator::Add,
            rhs: Some(Box::new(Expression {
                lhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, prefix_arithmetic: None, value: Some(Box::new(Assignable::Identifier(Identifier { name: String::from("b") }))), index_operator: None, positive: true })),
                rhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, prefix_arithmetic: None, value: Some(Box::new(Assignable::Identifier(Identifier { name: String::from("b") }))), index_operator: None, positive: true })),
                operator: Operator::Mul,
                prefix_arithmetic: None,
                value: None,
                index_operator: None,
                positive: true,
            })),
            value: None,
            index_operator: None,
            positive: true,
            prefix_arithmetic: None,
        })),
        (true, "1--1".to_string(), Some(Expression {
            lhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, prefix_arithmetic: None, value: Some(Box::new(Assignable::Integer(IntegerAST { value: "1".to_string(), ty: Integer::I32 }))), index_operator: None, positive: true })),
            operator: Operator::Sub,
            rhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, prefix_arithmetic: None, value: Some(Box::new(Assignable::Integer(IntegerAST { value: "-1".to_string(), ty: Integer::I32 }))), index_operator: None, positive: false })),
            value: None,
            index_operator: None,
            positive: true,
            prefix_arithmetic: None,
        })),
        (true, "1*-2".to_string(), Some(Expression {
            lhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, value: Some(Box::new(Assignable::Integer(IntegerAST { value: "1".to_string(), ty: Integer::I32 }))), index_operator: None, positive: true, prefix_arithmetic: None })),
            operator: Operator::Mul,
            rhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, prefix_arithmetic: None, value: Some(Box::new(Assignable::Integer(IntegerAST { value: "-2".to_string(), ty: Integer::I32 }))), index_operator: None, positive: false })),
            value: None,
            index_operator: None,
            positive: true,
            prefix_arithmetic: None,
        })),
        (true, "-(-1+-3)".to_string(), Some(Expression {
            lhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, prefix_arithmetic: None, value: Some(Box::new(Assignable::Integer(IntegerAST { value: "-1".to_string(), ty: Integer::I32 }))), index_operator: None, positive: false })),
            operator: Operator::Add,
            rhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, prefix_arithmetic: None, value: Some(Box::new(Assignable::Integer(IntegerAST { value: "-3".to_string(), ty: Integer::I32 }))), index_operator: None, positive: false })),
            value: None,
            index_operator: None,
            positive: false,
            prefix_arithmetic: None,
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
                            value: Some(Box::new(Assignable::Integer(IntegerAST { value: "4".to_string(), ty: Integer::I32 }))),
                            prefix_arithmetic: None,
                            index_operator: None,
                        })),
                        operator: Operator::Sub,
                        rhs: Some(Box::new(Expression {
                            lhs: Some(Box::new(Expression {
                                lhs: Some(Box::new(Expression {
                                    operator: Operator::Noop,
                                    lhs: None,
                                    rhs: None,
                                    positive: true,
                                    value: Some(Box::new(Assignable::Integer(IntegerAST { value: "2".to_string(), ty: Integer::I32 }))),
                                    prefix_arithmetic: None,
                                    index_operator: None,
                                })),
                                operator: Operator::Mul,
                                rhs: Some(Box::new(Expression {
                                    operator: Operator::Noop,
                                    lhs: None,
                                    rhs: None,
                                    positive: true,
                                    value: Some(Box::new(Assignable::Integer(IntegerAST { value: "3".to_string(), ty: Integer::I32 }))),
                                    prefix_arithmetic: None,
                                    index_operator: None,
                                })),
                                value: None,
                                index_operator: None,
                                positive: true,
                                prefix_arithmetic: None,
                            })),
                            operator: Operator::Mul,
                            rhs: Some(Box::new(Expression {
                                operator: Operator::Noop,
                                lhs: None,
                                rhs: None,
                                positive: true,
                                value: Some(Box::new(Assignable::Integer(IntegerAST { value: "5".to_string(), ty: Integer::I32 }))),
                                prefix_arithmetic: None,
                                index_operator: None,
                            })),
                            value: None,
                            index_operator: None,
                            positive: true,
                            prefix_arithmetic: None,
                        })),
                        value: None,
                        index_operator: None,
                        positive: true,
                        prefix_arithmetic: None,
                    })),
                    operator: Operator::Add,
                    rhs: Some(Box::new(Expression {
                        value: Some(Box::new(Assignable::Integer(IntegerAST { value: "1".to_string(), ty: Integer::I32 }))),
                        operator: Operator::Noop,
                        lhs: None,
                        rhs: None,
                        positive: true,
                        prefix_arithmetic: None,
                        index_operator: None,
                    })),
                    value: None,
                    index_operator: None,
                    positive: true,
                    prefix_arithmetic: None,
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
                            value: Some(Box::new(Assignable::Integer(IntegerAST { value: "3".to_string(), ty: Integer::I32 }))),
                            prefix_arithmetic: None,
                            index_operator: None,
                        })),
                        operator: Operator::Mul,
                        value: None,
                        rhs: Some(Box::new(Expression {
                            operator: Operator::Noop,
                            lhs: None,
                            rhs: None,
                            positive: true,
                            value: Some(Box::new(Assignable::Integer(IntegerAST { value: "3".to_string(), ty: Integer::I32 }))),
                            prefix_arithmetic: None,
                            index_operator: None,
                        })),
                        positive: true,
                        prefix_arithmetic: None,
                        index_operator: None,
                    })),
                    operator: Operator::Add,
                    value: None,
                    rhs: Some(Box::new(Expression {
                        lhs: Some(Box::new(Expression {
                            lhs: None,
                            rhs: None,
                            positive: true,
                            value: Some(Box::new(Assignable::Integer(IntegerAST { value: "4".to_string(), ty: Integer::I32 }))),
                            operator: Operator::Noop,
                            prefix_arithmetic: None,
                            index_operator: None,
                        })),
                        operator: Operator::Mul,
                        value: None,
                        rhs: Some(Box::new(Expression {
                            lhs: None,
                            rhs: None,
                            positive: true,
                            value: Some(Box::new(Assignable::Integer(IntegerAST { value: "4".to_string(), ty: Integer::I32 }))),
                            operator: Operator::Noop,
                            prefix_arithmetic: None,
                            index_operator: None,
                        })),
                        positive: true,
                        prefix_arithmetic: None,
                        index_operator: None,
                    })),
                    positive: true,
                    prefix_arithmetic: None,
                    index_operator: None,
                })),
                positive: true,
                prefix_arithmetic: None,
                index_operator: None,
            })),
            operator: Operator::Div,
            rhs: Some(Box::new(Expression {
                operator: Operator::Noop,
                lhs: None,
                rhs: None,
                positive: true,
                value: Some(Box::new(Assignable::Integer(IntegerAST { value: "2".to_string(), ty: Integer::I32 }))),
                prefix_arithmetic: None,
                index_operator: None,
            })),
            value: None,
            index_operator: None,
            positive: true,
            prefix_arithmetic: None,
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
                            value: Some(Box::new(Assignable::Integer(IntegerAST { value: "4".to_string(), ty: Integer::I32 }))),
                            prefix_arithmetic: None,
                            index_operator: None,
                        })),
                        operator: Operator::Sub,
                        rhs: Some(Box::new(Expression {
                            lhs: Some(Box::new(Expression {
                                lhs: Some(Box::new(Expression {
                                    operator: Operator::Noop,
                                    lhs: None,
                                    rhs: None,
                                    positive: true,
                                    value: Some(Box::new(Assignable::Integer(IntegerAST { value: "2".to_string(), ty: Integer::I32 }))),
                                    prefix_arithmetic: None,
                                    index_operator: None,
                                })),
                                operator: Operator::Mul,
                                rhs: Some(Box::new(Expression {
                                    operator: Operator::Noop,
                                    lhs: None,
                                    rhs: None,
                                    positive: true,
                                    value: Some(Box::new(Assignable::Integer(IntegerAST { value: "3".to_string(), ty: Integer::I32 }))),
                                    prefix_arithmetic: None,
                                    index_operator: None,
                                })),
                                value: None,
                                index_operator: None,
                                positive: true,
                                prefix_arithmetic: None,
                            })),
                            operator: Operator::Mul,
                            rhs: Some(Box::new(Expression {
                                operator: Operator::Noop,
                                lhs: None,
                                rhs: None,
                                positive: true,
                                value: Some(Box::new(Assignable::Integer(IntegerAST { value: "5".to_string(), ty: Integer::I32 }))),
                                prefix_arithmetic: None,
                                index_operator: None,
                            })),
                            value: None,
                            index_operator: None,
                            positive: true,
                            prefix_arithmetic: None,
                        })),
                        value: None,
                        index_operator: None,
                        positive: true,
                        prefix_arithmetic: None,
                    })),
                    operator: Operator::Add,
                    rhs: Some(Box::new(Expression {
                        value: Some(Box::new(Assignable::Integer(IntegerAST { value: "1".to_string(), ty: Integer::I32 }))),
                        operator: Operator::Noop,
                        lhs: None,
                        rhs: None,
                        positive: true,
                        prefix_arithmetic: None,
                        index_operator: None,
                    })),
                    value: None,
                    index_operator: None,
                    positive: true,
                    prefix_arithmetic: None,
                })),
                operator: Operator::Mul,
                value: None,
                rhs: Some(Box::new(Expression {
                    lhs: None,
                    operator: Operator::Noop,
                    prefix_arithmetic: None,
                    value: Some(Box::new(Assignable::Identifier(Identifier { name: String::from("sqrt") }))),
                    rhs: None,
                    positive: false,
                    index_operator: None,
                })),
                positive: true,
                prefix_arithmetic: None,
                index_operator: None,
            })),
            operator: Operator::Div,
            rhs: Some(Box::new(Expression {
                operator: Operator::Noop,
                lhs: None,
                rhs: None,
                positive: true,
                value: Some(Box::new(Assignable::Integer(IntegerAST { value: "2".to_string(), ty: Integer::I32 }))),
                prefix_arithmetic: None,
                index_operator: None,
            })),
            value: None,
            index_operator: None,
            positive: true,
            prefix_arithmetic: None,
        })),
        (true, "((4 - 2 * 3 + 1) * -sqrt(3*3+4*4)) / 2".to_string(), Some(Expression {
            lhs: Some(Box::new(Expression {
                lhs: Some(Box::new(Expression {
                    lhs: Some(Box::new(
                        Expression {
                            lhs: Some(Box::new(Expression {
                                lhs: None,
                                operator: Operator::Noop,
                                rhs: None,
                                value: Some(Box::new(
                                    Assignable::Integer(
                                        IntegerAST {
                                            value: "4".to_string(), ty: Integer::I32
                                        },
                                    ),
                                )),
                                index_operator: None,
                                positive: true,
                                prefix_arithmetic: None,
                            }, )),
                            operator: Operator::Sub,
                            rhs: Some(Box::new(Expression {
                                lhs: Some(Box::new(
                                    Expression {
                                        lhs: None,
                                        operator: Operator::Noop,
                                        rhs: None,
                                        value: Some(Box::new(
                                            Assignable::Integer(
                                                IntegerAST {
                                                    value: "2".to_string(), ty: Integer::I32
                                                },
                                            ),
                                        )),
                                        index_operator: None,
                                        positive: true,
                                        prefix_arithmetic: None,
                                    },
                                )),
                                operator: Operator::Mul,
                                rhs: Some(Box::new(Expression {
                                    lhs: None,
                                    operator: Operator::Noop,
                                    rhs: None,
                                    value: Some(Box::new(
                                        Assignable::Integer(
                                            IntegerAST {
                                                value: "3".to_string(), ty: Integer::I32
                                            },
                                        ),
                                    )),
                                    index_operator: None,
                                    positive: true,
                                    prefix_arithmetic: None,
                                }, )),
                                value: None,
                                index_operator: None,
                                positive: true,
                                prefix_arithmetic: None,
                            }, )),
                            value: None,
                            index_operator: None,
                            positive: true,
                            prefix_arithmetic: None,
                        },
                    )),
                    operator: Operator::Add,
                    rhs: Some(Box::new(
                        Expression {
                            lhs: None,
                            operator: Operator::Noop,
                            rhs: None,
                            value: Some(Box::new(
                                Assignable::Integer(
                                    IntegerAST {
                                        value: "1".to_string(), ty: Integer::I32
                                    },
                                )),
                            ),
                            index_operator: None,
                            positive: true,
                            prefix_arithmetic: None,
                        },
                    )),
                    value: None,
                    index_operator: None,
                    positive: true,
                    prefix_arithmetic: None,
                }, )),
                operator: Operator::Mul,
                rhs: Some(Box::new(Expression {
                    lhs: None,
                    operator: Operator::Noop,
                    rhs: None,
                    value: Some(Box::new(
                        Assignable::MethodCall(
                            MethodCall {
                                identifier: Identifier {
                                    name: String::from("sqrt"),
                                },
                                arguments: vec![
                                    Assignable::ArithmeticEquation(
                                        Expression {
                                            lhs: Some(Box::new(Expression {
                                                lhs: Some(Box::new(
                                                    Expression {
                                                        lhs: None,
                                                        operator: Operator::Noop,
                                                        rhs: None,
                                                        value: Some(Box::new(
                                                            Assignable::Integer(
                                                                IntegerAST {
                                                                    value: "3".to_string(), ty: Integer::I32
                                                                },
                                                            ),
                                                        )),
                                                        index_operator: None,
                                                        positive: true,
                                                        prefix_arithmetic: None,
                                                    },
                                                )),
                                                operator: Operator::Mul,
                                                rhs: Some(Box::new(
                                                    Expression {
                                                        lhs: None,
                                                        operator: Operator::Noop,
                                                        rhs: None,
                                                        value: Some(Box::new(
                                                            Assignable::Integer(
                                                                IntegerAST {
                                                                    value: "3".to_string(), ty: Integer::I32
                                                                },
                                                            ),
                                                        )),
                                                        index_operator: None,
                                                        positive: true,
                                                        prefix_arithmetic: None,
                                                    },
                                                )),
                                                value: None,
                                                index_operator: None,
                                                positive: true,
                                                prefix_arithmetic: None,
                                            }, )),
                                            operator: Operator::Add,
                                            rhs: Some(Box::new(
                                                Expression {
                                                    lhs: Some(Box::new(
                                                        Expression {
                                                            lhs: None,
                                                            operator: Operator::Noop,
                                                            rhs: None,
                                                            value: Some(Box::new(
                                                                Assignable::Integer(
                                                                    IntegerAST {
                                                                        value: "4".to_string(), ty: Integer::I32
                                                                    },
                                                                ),
                                                            )),
                                                            index_operator: None,
                                                            positive: true,
                                                            prefix_arithmetic: None,
                                                        },
                                                    )),
                                                    operator: Operator::Mul,
                                                    rhs: Some(Box::new(
                                                        Expression {
                                                            lhs: None,
                                                            operator: Operator::Noop,
                                                            rhs: None,
                                                            value: Some(Box::new(
                                                                Assignable::Integer(
                                                                    IntegerAST {
                                                                        value: "4".to_string(), ty: Integer::I32
                                                                    },
                                                                ),
                                                            )),
                                                            index_operator: None,
                                                            positive: true,
                                                            prefix_arithmetic: None,
                                                        },
                                                    )),
                                                    value: None,
                                                    index_operator: None,
                                                    positive: true,
                                                    prefix_arithmetic: None,
                                                },
                                            )),
                                            value: None,
                                            index_operator: None,
                                            positive: true,
                                            prefix_arithmetic: None,
                                        },
                                    ),
                                ],
                                code_line: CodeLine { line: "sqrt ( 3*3+4*4 ) ;".to_string(), actual_line_number: 0..0, virtual_line_number: 0 },
                            },
                        ),
                    )),
                    index_operator: None,
                    positive: false,
                    prefix_arithmetic: None,
                }, )),
                value: None,
                index_operator: None,
                positive: true,
                prefix_arithmetic: None,
            }),
            ),
            operator: Operator::Div,
            rhs: Some(Box::new(
                Expression {
                    lhs: None,
                    operator: Operator::Noop,
                    rhs: None,
                    value: Some(Box::new(
                        Assignable::Integer(
                            IntegerAST {
                                value: "2".to_string(), ty: Integer::I32
                            },
                        )),
                    ),
                    index_operator: None,
                    positive: true,
                    prefix_arithmetic: None,
                },
            )),
            value: None,
            index_operator: None,
            positive: true,
            prefix_arithmetic: None,
        })),
        (true, "a(b(c(d(e*f))))".to_string(), Some(Expression {
            lhs: None,
            operator: Operator::Noop,
            rhs: None,
            value: Some(Box::new(
                Assignable::MethodCall(
                    MethodCall {
                        identifier: Identifier {
                            name: String::from("a"),
                        },
                        arguments: vec![
                            Assignable::MethodCall(
                                MethodCall {
                                    identifier: Identifier {
                                        name: String::from("b"),
                                    },
                                    arguments: vec![
                                        Assignable::MethodCall(
                                            MethodCall {
                                                identifier: Identifier {
                                                    name: String::from("c"),
                                                },
                                                arguments: vec![
                                                    Assignable::MethodCall(
                                                        MethodCall {
                                                            identifier: Identifier {
                                                                name: String::from("d"),
                                                            },
                                                            arguments: vec![
                                                                Assignable::ArithmeticEquation(
                                                                    Expression {
                                                                        lhs: Some(Box::new(
                                                                            Expression {
                                                                                lhs: None,
                                                                                operator: Operator::Noop,
                                                                                rhs: None,
                                                                                value: Some(Box::new(
                                                                                    Assignable::Identifier(
                                                                                        Identifier {
                                                                                            name: String::from("e"),
                                                                                        },
                                                                                    ),
                                                                                )),
                                                                                index_operator: None,
                                                                                positive: true,
                                                                                prefix_arithmetic: None,
                                                                            },
                                                                        )),
                                                                        operator: Operator::Mul,
                                                                        rhs: Some(Box::new(
                                                                            Expression {
                                                                                lhs: None,
                                                                                operator: Operator::Noop,
                                                                                rhs: None,
                                                                                value: Some(Box::new(
                                                                                    Assignable::Identifier(
                                                                                        Identifier {
                                                                                            name: String::from("f"),
                                                                                        },
                                                                                    ),
                                                                                )),
                                                                                index_operator: None,
                                                                                positive: true,
                                                                                prefix_arithmetic: None,
                                                                            },
                                                                        )),
                                                                        value: None,
                                                                        index_operator: None,
                                                                        positive: true,
                                                                        prefix_arithmetic: None,
                                                                    },
                                                                ),
                                                            ],
                                                            code_line: CodeLine { line: "d ( e*f ) ;".to_string(), actual_line_number: 0..0, virtual_line_number: 0 },
                                                        },
                                                    ),
                                                ],
                                                code_line: CodeLine { line: "c ( d ( e*f ) ) ;".to_string(), actual_line_number: 0..0, virtual_line_number: 0 },
                                            },
                                        ),
                                    ],
                                    code_line: CodeLine { line: "b ( c ( d ( e*f ) ) ) ;".to_string(), actual_line_number: 0..0, virtual_line_number: 0 },
                                },
                            ),
                        ],
                        code_line: CodeLine { line: "a ( b ( c ( d ( e*f )  )  )  ) ;".to_string(), actual_line_number: 0..0, virtual_line_number: 0 },
                    },
                ),
            )),
            index_operator: None,
            positive: true,
            prefix_arithmetic: None,
        }
        )),
        (false, "((4 - 2 * ) -sqrt(3*3+4*4)) / 2".to_string(), None),
        (false, "r))".to_string(), None)
    ];

    for (expected_result, value, expected) in &values {
        let node = Equation::from_str(value);


        match *expected_result {
            true => {
                if let Ok(new_node) = &node {
                    let s = expected.as_ref().unwrap();
                    assert_eq!(*s, *new_node);
                }

                assert!(node.is_ok(), "{value}, {:?}", node);
            }
            false => {
                if let Err(err) = &node {
                    println!("{:<5}{:?}", " ", err);
                }
                assert!(node.is_err(), "{:?}", value)
            }
        }
    }

    Ok(())
}

#[test]
fn assignable_boolean_equation() -> anyhow::Result<()> {
    // let values: Vec<(bool, String)> = vec![
    //     (true, "a&b".to_string()),
    //     (true, "a|b&b".to_string()),
    //     (true, "a|b&b".to_string()),
    //     (true, "((true | (true&false) & true | false) & |(false&false|true&true)) & true".to_string()),
    //     (true, "((true | (true&false) & true | false) & |(false&false|true&true)) & true".to_string()),
    //     (true, "((true | (true&false) & true | false) & |(false&false|true&true)) & true".to_string()),
    //     (true, "((true | (true&false) & true | false) & |(false&false|true&true)) & true".to_string()),
    //     (true, "((true | (true&false) & true | false) & |sqrt) & true".to_string()),
    //     (true, "((true | true & false | false) & |sqrt(false&false|true&true)) & true".to_string()),
    //     (true, "((true | true&false | false) |sqrt(false&false|true&true)) & true".to_string()),
    //     (true, "a(b(c(d(e&f))))".to_string()),
    //     (false, "((true | true & ) |sqrt(false&false|true&true)) & true".to_string()),
    // ];

    // todo: make this work again lol
    // for (expected_result, value) in &values {
    //     let node = Equation::from_str(value);
    //
    //
    //     match *expected_result {
    //         true => {
    //             if let Ok(new_node) = &node {
    //                 println!("{}", new_node);
    //             }
    //             assert!(node.is_ok(), "{value}, {:?}", node);
    //         }
    //         false => assert!(node.is_err(), "{:?}", value)
    //     }
    // }
    //
    Ok(())
}
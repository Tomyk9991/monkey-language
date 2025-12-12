use std::str::FromStr;
use monkey_language::core::io::monkey_file::MonkeyFileNew;
use monkey_language::core::lexer::parse::{Parse, ParseOptions};
use monkey_language::core::lexer::token_with_span::FilePosition;
use monkey_language::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use monkey_language::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use monkey_language::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::operator::Operator;
use monkey_language::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::prefix_arithmetic::{PointerArithmetic, PrefixArithmetic};
use monkey_language::core::model::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use monkey_language::core::model::abstract_syntax_tree_nodes::assignables::object::Object;
use monkey_language::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use monkey_language::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use monkey_language::core::model::types::boolean::Boolean;
use monkey_language::core::model::types::float::FloatAST;
use monkey_language::core::model::types::integer::{IntegerAST, IntegerType};
use monkey_language::core::model::types::static_string::StaticString;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::assignables::equation_parser::Equation;
use monkey_language::core::scanner::parser::ASTParser;

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
        // (false, "\"This is a monkeystring\"".to_string()),
        // (false, "2\"\"".to_string()),
        // (true, "2".to_string()),
        (true, "-0".to_string()),
        // (true, "15".to_string()),
        // (false, "-+12".to_string()),
        // (true, "+0".to_string()),
        // (true, "+12312".to_string()),
        // (true, "2147483648".to_string()),
        // (true, "2147483647".to_string()),
        // (true, "-2147483648".to_string()),
        // (true, "-2147483649".to_string()),
    ];

    for (expected_result, value) in &values {
        let monkey_file: MonkeyFileNew = MonkeyFileNew::read_from_str(value)?;
        let integer = IntegerAST::parse(&monkey_file.tokens, ParseOptions::default());

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
        (false, "+1.0e-5".to_string()),
    ];

    for (expected_result, value) in &values {
        let monkey_file: MonkeyFileNew = MonkeyFileNew::read_from_str(value)?;
        let float = FloatAST::parse(&monkey_file.tokens, ParseOptions::default());
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
    // todo: fix object parser
    // let values: Vec<(bool, String)> = vec![
    //     (true, "Data { key1 : \"value1\" , key2 : 1 }".to_string()),
    //     (false, "Data { 'key1' : \"value2\" , 'key2' : 2 }".to_string()),
    //     (true, "Data { }".to_string()),
    //     (false, "Data { :\"key1\" , \"key3\" }".to_string()),
    //     (false, "Data { \"key1\" , key4 : }".to_string()),
    //     (false, "Data { key1\"' : \"value5' , key2 : 3 }".to_string()),
    //     (false, "[ key1 : \"value6\" , key2\" : 4 }".to_string()),
    //     (true, "Data { key1 : Data { inner_key1 : \"value1\", inner_key2 : 5 }, key2 : 1 }".to_string()),
    //     (true, "Data { key1 : Data { inner_key1 : Data { inner_inner_key : \"value\" } }, key2 : 2 }".to_string()),
    //     (false, "Data { key1 : Data { 'inner_key1' : \"value2\", 'inner_key2' : 2 }, key2 : 2 }".to_string()),
    //     (false, "Data { key1 : Data { inner_key1 : \"value3\", : 3 }, key2 : 3 }".to_string()),
    //     (true, "Data { key1 : func1 ( param1, param2 ) , key2 : 1 }".to_string()),
    //     (false, "Data { key1 : 'func2 ( param1, param2 )' , key2 : 2 }".to_string()),
    //     (true, "Data { key1 : func3 ( func4 ( param1, param2 ), param3 ) , key2 : 1 }".to_string()),
    //     (true, "Data { key1 : Data { inner_key1 : func5 ( param1, param2 ), inner_key2 : 5 }, key2 : 1 }".to_string()),
    //     (false, "Data { key1 : Data { inner_key1 : 'func6 ( param1, param2 )', inner_key2 : 2 }, key2 : 2 }".to_string()),
    //     (true, "Data { key1 : Data { inner_key1 : Data { inner_inner_key : func7 ( param1 ) } }, key2 : 2 }".to_string()),
    // ];
    //
    // for (expected_result, value) in &values {
    //     let object = Object::from_str(value);
    //
    //     match *expected_result {
    //         true => assert!(object.is_ok(), "{:?}", object),
    //         false => assert!(object.is_err(), "{:?}", object)
    //     }
    // }

    Ok(())
}


#[test]
fn assignable_imaginary_fn_calls() -> anyhow::Result<()> {
    let values: Vec<(bool, String)> = vec![
        (true, "imaginary_fn1()".to_string()),
        (true, "imaginary_fn2()".to_string()),
        (true, "imaginary_fn3(param1, param2)".to_string()),
        (true, "imaginary_fn4(param1, imaginary_fn2 (param2))".to_string()),
        (false, "imaginary_fn5(, param2)".to_string()),
        (false, "imaginary_fn6(param1,)".to_string()),
        (false, "imaginary_fn7(param1, 2, )".to_string()),
        (true, "imaginary_fn8(param1, imaginary_fn2(param3, param4), param2)".to_string()),
        (true, "imaginary_fn9(inner_fn1 (param1), inner_fn2 (param2))".to_string()),
        (true, "imaginary_fn10(inner_fn1 (param1), inner_fn2 (param2))".to_string()),
        (true, "imaginary_fn11(\"string_value\") ".to_string()),
        (true, "imaginary_fn12(42)".to_string()),
        (true, "imaginary_fn13(-42)".to_string()),
        (true, "imaginary_fn14(3.14)".to_string()),
        (true, "imaginary_fn15(-3.14)".to_string()),
        (true, "imaginary_fn16(123)".to_string()),
        (true, "imaginary_fn17(31.4, test)".to_string()),
        // todo fix with object parser
        // (true, "imaginary_fn20 (Data { })".to_string()),
        // (false, "imaginary_fn19 ( Data { 'key1' : \"value2\" , 'key2' : 2 } )".to_string()),
        // (true, "imaginary_fn18 ( Data { key1 : \"value1\" , key2 : 1 } )".to_string()),
        // (false, "imaginary_fn21 ( Data { key1 : Data { inner_key1 : 'func6 ( param1, param2 )', inner_key2 : 2 }, key2 : 2 } )".to_string()),
        // (true, "imaginary_fn21 ( Data { key1 : Data { inner_key1 : Data { inner_inner_key : func7 ( param1 ) } }, key2 : 2 } )".to_string()),
        // (true, "imaginary_fn22 ( Data { a : imaginary ( b ) } , imaginary ( Data { } ) ) ".to_string()),
    ];

    for (expected_result, value) in &values {
        let monkey_file: MonkeyFileNew = MonkeyFileNew::read_from_str(value)?;
        let node = MethodCall::parse(&monkey_file.tokens, ParseOptions::default());

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
    let expressions = vec![
        ("a*b", true),
        ("a**b", true),
        ("sqrt(b*c)/", false),
        ("a+b*b", true),
        ("1--1", true),
        ("1*-2", true),
        ("-(-1+-3)", true),
        ("((4 - (2*3) * 5 + 1) * -(3*3+4*4)) / 2", true),
        ("((4 - (2*3) * 5 + 1) * -sqrt) / 2", true),
        ("((4 - 2 * 3 + 1) * -sqrt(3*3+4*4)) / 2", true),
        ("a(b(c(d(e*f))))", true),
    ];

    for (value, expected_result) in &expressions {
        let monkey_file: MonkeyFileNew = MonkeyFileNew::read_from_str(value)?;
        let mut top_level_scope = Expression::parse(&monkey_file.tokens, ParseOptions::default());

        assert_eq!(top_level_scope.is_ok(), *expected_result);
    }



    Ok(())
}

#[test]
fn assignable_boolean_equation() -> anyhow::Result<()> {
    let values: Vec<(bool, String)> = vec![
        (true, "a&b".to_string()),
        (true, "a|b&b".to_string()),
        (true, "a|b&b".to_string()),
        (true, "((true | (true&false) & true | false) & (false&false|true&true)) & true".to_string()),
        (true, "((true | (true&false) & true | false) & sqrt) & true".to_string()),
        (true, "((true | true & false | false) & sqrt(false&false|true&true)) & true".to_string()),
        (true, "((true | true&false | false) |sqrt(false&false|true&true)) & true".to_string()),
        (true, "a(b(c(d(e&f))))".to_string()),
        (false, "((true | true & ))".to_string()),
        (false, "((true | true & ) |sqrt(false&false|true&true)) & true".to_string()),
    ];

    for (expected_result, value) in &values {
        let monkey_file: MonkeyFileNew = MonkeyFileNew::read_from_str(value)?;
        let node = Expression::parse(&monkey_file.tokens, ParseOptions::default());
        
        assert_eq!(node.is_ok(), *expected_result, "{:?}", value);
    }

    Ok(())
}
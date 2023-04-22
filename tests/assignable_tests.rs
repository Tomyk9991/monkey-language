use std::str::FromStr;
use monkey_language::interpreter::lexer::tokens::assignable_tokens::integer_token::{IntegerToken};
use monkey_language::interpreter::lexer::tokens::assignable_tokens::string_token::{StringToken, StringTokenErr};

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
        (false, "+1.0e-5".to_string())
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
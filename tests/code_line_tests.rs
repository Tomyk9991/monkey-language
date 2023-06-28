use std::ops::Range;

use monkey_language::core::io::monkey_file::MonkeyFile;

#[test]
fn code_line_test_1() -> anyhow::Result<()> {
    let source_code = r#"if (hallo) {

    }

    fn hallo(): void {

    }

    michi =
    {
        guten: "Hallo",
        ciau: 5,
        rofl: name(),
        mofl: name(nestedMethod("Hallo", moin("Ciao", 5)))
    };

    nestedMichi = {
        guten: "Hallo",
        ciau: 5,
        mofl: {
            guten: "Hallo",
            ciau: 5,
            rofl: name(),
            mofl: name(nestedMethod("Hallo", moin("Ciao", 5)))
        },
        rofl: name(),
    };

    moin = { test: "Hallo", nested: { integer: -51 } };
    variable_inside = { test: "Hallo" };

    fisch = "Fische sind wirklich wirklich toll";
    guten_tag = name();
    guten_tag = name("Guten Morgen", 5);
    name(nestedMethod("Hallo", moin("Ciao", 5)));
    hallo = "Thomas"; tschuess = 5;
    mallo = "";
    variable_with_another_variable_assignment = fisch;

    variable_with_another_variable_assignment = fisch;

    fn method_name(variable, variable): void {
        function_variable_one = 10;
    }

    fn f(variable, variable): void
    {
        function_variable_two = 10;
    }

    hallo = "Thomas"; tschuess = 5;
    mallo = "";
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(source_code);
    let actual_code_lines = monkey_file.lines
        .iter()
        .map(|code_line| code_line.actual_line_number.clone())
        .collect::<Vec<_>>();

    let virtual_code_lines = monkey_file.lines
        .iter()
        .map(|code_line| code_line.virtual_line_number.clone())
        .collect::<Vec<_>>();


    let expected: Vec<Range<usize>> = vec![
        1..1, 3..3, 5..5, 7..7, 9..15, 17..27, 29..29, 30..30, 32..32, 33..33, 34..34, 35..35, 36..36, 36..36, 37..37, 38..38, 40..40, 42..42, 43..43, 44..44, 46..47, 48..48, 49..49, 51..51, 51..51, 52..52,
    ];

    assert_eq!(expected, actual_code_lines);
    assert_eq!((1..=26).collect::<Vec<_>>(), virtual_code_lines);

    Ok(())
}

#[test]
fn code_line_test_2() -> anyhow::Result<()> {
    let source_code = r#"variable = ((4 - 2 * 3 + 1) * -sqrt(3*3+4*4)) / 2;

    if(hallo) {
        if_stack_variable = 5;

        if (if_stack_variable) {
            nested_if_stack_variable = 13;
        }
    }

    fn hallo(): void {

    }

    michi =
    {
        guten: "Hallo",
        ciau: 5,
        rofl: name(),
        mofl: name(nestedMethod("Hallo", moin("Ciao", 5)))
    };

    nestedMichi = {
        guten: "Hallo",
        ciau: 5,
        mofl: {
            guten: "Hallo",
            ciau: 5,
            rofl: name(),
            mofl: name(nestedMethod("Hallo", moin("Ciao", 5)))
        },
        rofl: name(),
    };

    moin = { test: "Hallo", nested: { integer: -51 } };
    variable_inside = { test: "Hallo" };

    fisch = "Fische sind wirklich wirklich toll";
    guten_tag = name();
    guten_tag = name("Guten Morgen", 5);
    name(nestedMethod("Hallo", moin("Ciao", 5)));
    hallo = "Thomas"; tschuess = 5;
    mallo = "";
    variable_with_another_variable_assignment = fisch;

    variable_with_another_variable_assignment = fisch;

    fn method_name(variable, variable): void {
        function_variable_one = 10;
    }

    fn f(variable, variable): void
    {
        function_variable_two = 10;
    }

    hallo = "Thomas"; tschuess = 5;
    mallo = "";
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(source_code);
    let actual_code_lines = monkey_file.lines
        .iter()
        .map(|code_line| code_line.actual_line_number.clone())
        .collect::<Vec<_>>();

    let virtual_code_lines = monkey_file.lines
        .iter()
        .map(|code_line| code_line.virtual_line_number.clone())
        .collect::<Vec<_>>();


    let expected: Vec<Range<usize>> = vec![
        1..1, 3..3, 4..4, 6..6, 7..7, 8..8, 9..9, 11..11, 13..13, 15..21, 23..33, 35..35, 36..36, 38..38, 39..39, 40..40, 41..41, 42..42, 42..42, 43..43, 44..44, 46..46, 48..48, 49..49, 50..50, 52..53, 54..54, 55..55, 57..57, 57..57, 58..58
    ];

    assert_eq!(expected, actual_code_lines);
    assert_eq!((1..=31).collect::<Vec<_>>(), virtual_code_lines);

    Ok(())
}
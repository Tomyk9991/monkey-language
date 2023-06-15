use std::ops::Range;

use monkey_language::interpreter::io::monkey_file::MonkeyFile;

#[test]
fn code_line_test() -> anyhow::Result<()> {
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
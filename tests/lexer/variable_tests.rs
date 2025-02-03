use monkey_language::core::io::code_line::CodeLine;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::assignable::Assignable;
use monkey_language::core::lexer::parser::Lexer;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::assignables::equation_parser::expression::{Expression};
use monkey_language::core::lexer::abstract_syntax_tree_nodes::assignables::equation_parser::operator::Operator;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::assignables::equation_parser::prefix_arithmetic::{PointerArithmetic, PrefixArithmetic};
use monkey_language::core::lexer::abstract_syntax_tree_nodes::assignables::integer::IntegerAST;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::assignables::object::Object;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::assignables::string::StaticString;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::identifier::Identifier;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::l_value::LValue;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::variable::Variable;
use monkey_language::core::lexer::types::float::Float;
use monkey_language::core::lexer::types::integer::Integer;
use monkey_language::core::lexer::types::r#type;
use monkey_language::core::lexer::types::r#type::{Mutability, Type};

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
    let top_level_scope = lexer.parse()?;

    let expected = vec![
        AbstractSyntaxTreeNode::Variable(
            Variable {
                l_value: LValue::Identifier(Identifier { name: "fisch".to_string() }),
                mutability: false,
                ty: Some(Type::Custom(Identifier { name: String::from("*string") }, Mutability::Immutable)),
                define: true,
                assignable: Assignable::String(StaticString { value: "\"Fische sind wirklich wirklich toll\"".to_string() }),
                code_line: CodeLine { line: "let fisch = \"Fische sind wirklich wirklich toll\" ;".to_string(), actual_line_number: 2..2, virtual_line_number: 1 },
            }
        ),
        AbstractSyntaxTreeNode::Variable(
            Variable {
                l_value: LValue::Identifier(Identifier { name: "hallo".to_string() }),
                mutability: false,
                ty: Some(Type::Custom(Identifier { name: String::from("*string") }, Mutability::Immutable)),
                define: true,
                assignable: Assignable::String(StaticString { value: "\"Thomas\"".to_string() }),
                code_line: CodeLine { line: "let hallo = \"Thomas\" ;".to_string(), actual_line_number: 3..3, virtual_line_number: 2 },
            }
        ),
        AbstractSyntaxTreeNode::Variable(
            Variable {
                l_value: LValue::Identifier(Identifier { name: "tschuess".to_string() }),
                mutability: false,
                ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)),
                define: true,
                assignable: Assignable::Integer(IntegerAST { value: "5".to_string(), ty: Integer::I32 }),
                code_line: CodeLine { line: "let tschuess = 5 ;".to_string(), actual_line_number: 3..3, virtual_line_number: 3 },
            }
        ),
        AbstractSyntaxTreeNode::Variable(
            Variable {
                l_value: LValue::Identifier(Identifier { name: "mallo".to_string() }),
                mutability: false,
                ty: Some(Type::Custom(Identifier { name: String::from("*string") }, Mutability::Immutable)),
                define: true,
                assignable: Assignable::String(StaticString { value: "\"\"".to_string() }),
                code_line: CodeLine { line: "let mallo = \"\" ;".to_string(), actual_line_number: 4..4, virtual_line_number: 4 },
            }
        ),
        AbstractSyntaxTreeNode::Variable(
            Variable {
                l_value: LValue::Identifier(Identifier { name: "michi".to_string() }),
                mutability: false,
                ty: Some(Type::Custom(Identifier { name: "Data".to_string() }, Mutability::Immutable)),
                define: true,
                assignable: Assignable::Object(Object {
                    variables: vec![
                        Variable {
                            l_value: LValue::Identifier(Identifier { name: "guten".to_string() }),
                            mutability: false,
                            ty: Some(Type::Custom(Identifier { name: String::from("*string") }, Mutability::Immutable)),
                            define: false,
                            assignable: Assignable::String(StaticString { value: "\"Hallo\"".to_string() }),
                            code_line: CodeLine { line: "guten : \"Hallo\" ,".to_string(), actual_line_number: 0..0, virtual_line_number: 0 },
                        },
                        Variable {
                            l_value: LValue::Identifier(Identifier { name: "ciau".to_string() }),
                            mutability: false,
                            ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)),
                            define: false,
                            assignable: Assignable::Integer(IntegerAST { value: "5".to_string(), ty: Integer::I32 }),
                            code_line: CodeLine { line: "ciau : 5 ,".to_string(), actual_line_number: 0..0, virtual_line_number: 0 },
                        },
                        Variable {
                            l_value: LValue::Identifier(Identifier { name: "rofl".to_string() }),
                            mutability: false,
                            ty: None,
                            define: false,
                            assignable: Assignable::MethodCall(
                                MethodCall {
                                    identifier: Identifier { name: "name".to_string() },
                                    arguments: vec![],
                                    code_line: CodeLine { line: "name ( ) ;".to_string(), actual_line_number: 0..0, virtual_line_number: 0 },
                                }
                            ),
                            code_line: CodeLine { line: "rofl : name ( ) ,".to_string(), actual_line_number: 0..0, virtual_line_number: 0 },
                        },
                        Variable {
                            l_value: LValue::Identifier(Identifier { name: "mofl".to_string() }),
                            mutability: false,
                            ty: None,
                            define: false,
                            assignable: Assignable::MethodCall(MethodCall {
                                identifier: Identifier { name: "name".to_string() },
                                arguments: vec![
                                    Assignable::MethodCall(MethodCall {
                                        identifier: Identifier { name: "nestedMethod".to_string() },
                                        arguments: vec![
                                            Assignable::String(StaticString { value: "\"Hallo\"".to_string() }),
                                            Assignable::MethodCall(MethodCall {
                                                identifier: Identifier { name: "moin".to_string() },
                                                arguments: vec![
                                                    Assignable::String(StaticString { value: "\"Ciao\"".to_string() }),
                                                    Assignable::Integer(IntegerAST { value: "5".to_string(), ty: Integer::I32 }),
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
                    ty: Type::Custom(Identifier { name: "Data".to_string() }, Mutability::Immutable),
                }),
                code_line: CodeLine { line: "let michi = Data {  guten :  \"Hallo\" ,  ciau :  5 ,  rofl :  name (  )  ,  mofl :  name ( nestedMethod ( \"Hallo\" ,  moin ( \"Ciao\" ,  5 )  )  )  }  ;".to_string(), actual_line_number: 5..11, virtual_line_number: 5 },
            }
        ),
        AbstractSyntaxTreeNode::Variable(
            Variable {
                l_value: LValue::Identifier(Identifier { name: "value".to_string() }),
                mutability: false,
                ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)),
                define: true,
                assignable: Assignable::Integer(IntegerAST { value: "9".to_string(), ty: Integer::I32 }),
                code_line: CodeLine { line: "let value = 9 ;".to_string(), actual_line_number: 12..12, virtual_line_number: 6 },
            }
        ),
        AbstractSyntaxTreeNode::Variable(
            Variable {
                l_value: LValue::Identifier(Identifier { name: "ref_value".to_string() }),
                mutability: false,
                ty: Some(Type::Custom(Identifier { name: "*i32".to_string() }, Mutability::Immutable)),
                define: true,
                assignable: Assignable::ArithmeticEquation(Expression {
                    lhs: None,
                    rhs: None,
                    operator: Operator::Noop,
                    prefix_arithmetic: Some(PrefixArithmetic::PointerArithmetic(PointerArithmetic::Ampersand)),
                    value: Some(Box::new(Assignable::ArithmeticEquation(Expression {
                        lhs: None,
                        rhs: None,
                        operator: Operator::Noop,
                        prefix_arithmetic: None,
                        value: Some(Box::new(Assignable::Identifier(Identifier { name: "value".to_string() }))),
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
        AbstractSyntaxTreeNode::Variable(
            Variable {
                l_value: LValue::Identifier(Identifier { name: "pointer_arithmetic".to_string() }),
                mutability: false,
                ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)),
                define: true,
                assignable: Assignable::ArithmeticEquation(Expression {
                    lhs: Some(Box::new(Expression {
                        value: Some(Box::new(Assignable::ArithmeticEquation(Expression {
                            lhs: None,
                            rhs: None,
                            operator: Operator::Noop,
                            prefix_arithmetic: None,
                            value: Some(Box::new(Assignable::Identifier(Identifier { name: "ref_value".to_string() }))),
                            index_operator: None,
                            positive: true,
                        }))),
                        positive: true,
                        prefix_arithmetic: Some(PrefixArithmetic::PointerArithmetic(PointerArithmetic::Asterics)),
                        ..Default::default()
                    })),
                    rhs: Some(Box::new(Expression {
                        value: Some(Box::new(Assignable::Integer(IntegerAST { value: "1".to_string(), ty: Integer::I32 }))),
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

    assert_eq!(expected, top_level_scope.ast_nodes);

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
    let top_level_scope = lexer.parse()?;

    let expected = vec![
        r#type::common::string(),
        r#type::common::string(),
        Type::Integer(Integer::I32, Mutability::Immutable),
        r#type::common::string(),
        Type::Custom(Identifier { name: "Data".to_string() }, Mutability::Immutable),
        Type::Integer(Integer::I32, Mutability::Immutable),
        Type::Custom(Identifier { name: "*i32".to_string() }, Mutability::Immutable),
        Type::Integer(Integer::I32, Mutability::Immutable),
    ];

    for (index, node) in top_level_scope.ast_nodes.iter().enumerate() {
        match node {
            AbstractSyntaxTreeNode::Variable(v) if v.ty.is_some() => {
                if let Some(ty) = &v.ty {
                    assert_eq!(&expected[index], ty);
                } else {
                    assert!(false, "Didnt expect not inferred type");
                }
            },
            _ => assert!(false, "Didnt expect this type of node")
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
    let top_level_scope = lexer.parse()?;
    println!("{:?}", top_level_scope);

    let expected = vec![
        Type::Float(Float::Float32, Mutability::Immutable),
        Type::Float(Float::Float32, Mutability::Immutable),
        Type::Integer(Integer::I32, Mutability::Immutable),
        Type::Float(Float::Float32, Mutability::Immutable),
        Type::Float(Float::Float32, Mutability::Immutable),
        Type::Float(Float::Float32, Mutability::Immutable),
    ];

    for (index, node) in top_level_scope.ast_nodes.iter().enumerate() {
        match node {
            AbstractSyntaxTreeNode::Variable(v) if v.ty.is_some() => {
                if let Some(ty) = &v.ty {
                    assert_eq!(&expected[index], ty, "Failed at: {}", v);
                } else {
                    assert!(false, "Didnt expect not inferred type");
                }
            },
            _ => assert!(false, "Didnt expect this type of node")
        }
    }

    Ok(())
}

#[test]
fn variable_test_double_casting() -> anyhow::Result<()> {
    let variables = r#"let b: f32 = (f32)(i32) 5;"#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(variables);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.parse()?;

    let expected = vec![
        Type::Float(Float::Float32, Mutability::Immutable),
    ];

    let s = None;

    for node in &top_level_scope.ast_nodes {
        println!("{}", node);
        match node {
            AbstractSyntaxTreeNode::Variable(v) => {
                println!("{:?}", match &v.assignable {
                    Assignable::ArithmeticEquation(a) => {
                        &a.prefix_arithmetic
                    },
                    _ => { &s }
                });
            },
            _ => {}
        }
    }

    for (index, node) in top_level_scope.ast_nodes.iter().enumerate() {
        match node {
            AbstractSyntaxTreeNode::Variable(v) if v.ty.is_some() => {
                if let Some(ty) = &v.ty {
                    assert_eq!(&expected[index], ty, "Failed at: {}", v);
                } else {
                    assert!(false, "Didnt expect not inferred type");
                }
            },
            _ => assert!(false, "Didnt expect this type of node")
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
    let top_level_scope = lexer.parse()?;

    let expected = vec![
        Type::Integer(Integer::I32, Mutability::Immutable),
        Type::Custom(Identifier { name: "*i32".to_string() }, Mutability::Immutable),
        Type::Integer(Integer::I32, Mutability::Immutable),
        Type::Custom(Identifier { name: "*i32".to_string() }, Mutability::Immutable),
        Type::Float(Float::Float32, Mutability::Immutable),
        Type::Float(Float::Float32, Mutability::Immutable),
        Type::Float(Float::Float32, Mutability::Immutable),
        Type::Integer(Integer::I32, Mutability::Immutable),
        Type::Integer(Integer::I32, Mutability::Immutable),
    ];

    for (index, node) in top_level_scope.ast_nodes.iter().enumerate() {
        match node {
            AbstractSyntaxTreeNode::Variable(v) if v.ty.is_some() => {
                if let Some(ty) = &v.ty {
                    assert_eq!(&expected[index], ty, "FAILED AT: {node}");
                } else {
                    assert!(false, "Didnt expect not inferred type");
                }
            },
            _ => assert!(false, "Didnt expect this type of node")
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
    let top_level_scope = lexer.parse()?;

    println!("{:#?}", top_level_scope);

    let expected = vec![
        Type::Integer(Integer::I32, Mutability::Immutable),
        Type::Custom(Identifier { name: "*i32".to_string() }, Mutability::Immutable),
        Type::Integer(Integer::I32, Mutability::Immutable),
        Type::Integer(Integer::I32, Mutability::Immutable),
        Type::Integer(Integer::I32, Mutability::Immutable),
    ];

    for node in &top_level_scope.ast_nodes {
        println!("{}", node);
    }


    for (index, node) in top_level_scope.ast_nodes.iter().enumerate() {
        match node {
            AbstractSyntaxTreeNode::Variable(v) if v.ty.is_some() => {
                if let Some(ty) = &v.ty {
                    assert_eq!(&expected[index], ty, "FAILED AT: {node}");
                } else {
                    assert!(false, "Didnt expect not inferred type");
                }
            },
            _ => assert!(false, "Didnt expect this type of node")
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
    let top_level_scope = lexer.parse()?;

    println!("{:#?}", top_level_scope);

    let expected = vec![
        Type::Integer(Integer::I32, Mutability::Immutable),
        Type::Integer(Integer::I64, Mutability::Immutable),
        Type::Integer(Integer::I16, Mutability::Immutable),
        Type::Integer(Integer::I8, Mutability::Immutable),
        Type::Integer(Integer::I64, Mutability::Immutable),
        Type::Integer(Integer::U8, Mutability::Immutable),
        Type::Integer(Integer::U16, Mutability::Immutable),
        Type::Integer(Integer::U32, Mutability::Immutable),
        Type::Integer(Integer::U64, Mutability::Immutable),
        Type::Integer(Integer::U64, Mutability::Immutable),
    ];

    for node in &top_level_scope.ast_nodes {
        println!("{}", node);
    }


    for (index, node) in top_level_scope.ast_nodes.iter().enumerate() {
        match node {
            AbstractSyntaxTreeNode::Variable(v) if v.ty.is_some() => {
                if let Some(ty) = &v.ty {
                    assert_eq!(&expected[index], ty, "FAILED AT: {node}");
                } else {
                    assert!(false, "Didnt expect not inferred type");
                }
            },
            _ => assert!(false, "Didnt expect this type of node")
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
    let top_level_scope = lexer.parse()?;

    println!("{:#?}", top_level_scope);

    let expected = vec![
        Type::Integer(Integer::I32, Mutability::Immutable),
        Type::Integer(Integer::I64, Mutability::Immutable),
        Type::Integer(Integer::I16, Mutability::Immutable),
        Type::Integer(Integer::I8, Mutability::Immutable),
        Type::Integer(Integer::U8, Mutability::Immutable),
        Type::Integer(Integer::U16, Mutability::Immutable),
        Type::Integer(Integer::U32, Mutability::Immutable),
        Type::Integer(Integer::U64, Mutability::Immutable),
    ];

    for node in &top_level_scope.ast_nodes {
        println!("{}", node);
    }


    for (index, node) in top_level_scope.ast_nodes.iter().enumerate() {
        match node {
            AbstractSyntaxTreeNode::Variable(v) if v.ty.is_some() => {
                if let Assignable::Integer(i) = &v.assignable {
                    assert_eq!(&expected[index], &Type::Integer(i.ty.clone(), Mutability::Immutable), "FAILED AT: {node}");
                } else {
                    assert!(false, "Didnt expect not inferred type {}", v);
                }
            },
            _ => assert!(false, "Didnt expect this type of node")
        }
    }

    Ok(())
}
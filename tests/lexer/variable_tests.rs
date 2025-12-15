use monkey_language::core::io::monkey_file::{MonkeyFile};
use monkey_language::core::lexer::token_with_span::FilePosition;
use monkey_language::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use monkey_language::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use monkey_language::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use monkey_language::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::operator::Operator;
use monkey_language::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::prefix_arithmetic::{PointerArithmetic, PrefixArithmetic};
use monkey_language::core::model::abstract_syntax_tree_nodes::assignables::method_call::MethodCall;
use monkey_language::core::model::abstract_syntax_tree_nodes::assignables::object::Object;
use monkey_language::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use monkey_language::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use monkey_language::core::model::abstract_syntax_tree_nodes::variable::Variable;
use monkey_language::core::model::types::float::FloatType;
use monkey_language::core::model::types::integer::{IntegerAST, IntegerType};
use monkey_language::core::model::types::mutability::Mutability;
use monkey_language::core::model::types::static_string::StaticString;
use monkey_language::core::model::types::ty::Type;
use monkey_language::core::parser::parser::ASTParser;
use monkey_language::core::parser::types::r#type;
use monkey_language::core::semantics::type_infer::type_inferer::infer_type;

#[test]
fn variable_test() -> anyhow::Result<()> {
    // let variables = r#"
    // let fisch = "Fische sind wirklich wirklich toll";
    // let hallo = "Thomas"; let tschuess = 5;
    // let mallo = "";
    // let michi =
    // Data {
    //     guten: "Hallo",
    //     ciau: 5,
    //     rofl: name(),
    //     mofl: name(nestedMethod("Hallo", moin("Ciao", 5)))
    // };
    // let value = 9;
    // let ref_value = &value;
    // let pointer_arithmetic = *ref_value + 1;
    // "#;

    let variables = r#"
    let fisch = "Fische sind wirklich wirklich toll";
    let hallo = "Thomas"; let tschuess = 5;
    let mallo = "";
    let value = 9;
    let ref_value = &value;
    let pointer_arithmetic = *ref_value + 1;
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(variables)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;

    let expected = vec![
        AbstractSyntaxTreeNode::Variable(
            Variable {
                l_value: LValue::Identifier(Identifier { name: "fisch".to_string() }),
                mutability: false,
                ty: Some(Type::Custom(Identifier { name: String::from("*string") }, Mutability::Immutable)),
                define: true,
                assignable: Assignable::String(StaticString { value: "\"Fische sind wirklich wirklich toll\"".to_string() }),
                file_position: FilePosition { line: 2..=2, column: 5..=53 },
            }
        ),
        AbstractSyntaxTreeNode::Variable(
            Variable {
                l_value: LValue::Identifier(Identifier { name: "hallo".to_string() }),
                mutability: false,
                ty: Some(Type::Custom(Identifier { name: String::from("*string") }, Mutability::Immutable)),
                define: true,
                assignable: Assignable::String(StaticString { value: "\"Thomas\"".to_string() }),
                file_position: FilePosition { line: 3..=3, column: 5..=25 },
            }
        ),
        AbstractSyntaxTreeNode::Variable(
            Variable {
                l_value: LValue::Identifier(Identifier { name: "tschuess".to_string() }),
                mutability: false,
                ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                define: true,
                assignable: Assignable::Integer(IntegerAST { value: "5".to_string(), ty: IntegerType::I32 }),
                file_position: FilePosition { line: 3..=3, column: 27..=43 },
            }
        ),
        AbstractSyntaxTreeNode::Variable(
            Variable {
                l_value: LValue::Identifier(Identifier { name: "mallo".to_string() }),
                mutability: false,
                ty: Some(Type::Custom(Identifier { name: String::from("*string") }, Mutability::Immutable)),
                define: true,
                assignable: Assignable::String(StaticString { value: "\"\"".to_string() }),
                file_position: FilePosition { line: 4..=4, column: 5..=19 },
            }
        ),
        AbstractSyntaxTreeNode::Variable(
            Variable {
                l_value: LValue::Identifier(Identifier { name: "value".to_string() }),
                mutability: false,
                ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                define: true,
                assignable: Assignable::Integer(IntegerAST { value: "9".to_string(), ty: IntegerType::I32 }),
                file_position: FilePosition { line: 5..=5, column: 5..=18 },
            }
        ),
        AbstractSyntaxTreeNode::Variable(
            Variable {
                l_value: LValue::Identifier(Identifier { name: "ref_value".to_string() }),
                mutability: false,
                ty: Some(Type::Custom(Identifier { name: "*i32".to_string() }, Mutability::Immutable)),
                define: true,
                assignable: Assignable::Expression(Expression {
                    lhs: None,
                    rhs: None,
                    operator: Operator::Noop,
                    prefix_arithmetic: Some(PrefixArithmetic::PointerArithmetic(PointerArithmetic::Ampersand)),
                    value: Some(Box::new(Assignable::Expression(Expression {
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
                file_position: FilePosition { line: 6..=6, column: 5..=27 },
            }
        ),
        // let pointer_arithmetic = *ref_value + 1;
        AbstractSyntaxTreeNode::Variable(
            Variable {
                l_value: LValue::Identifier(Identifier { name: "pointer_arithmetic".to_string() }),
                mutability: false,
                ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                define: true,
                assignable: Assignable::Expression(Expression {
                    lhs: Some(Box::new(Expression {
                        value: Some(Box::new(Assignable::Expression(Expression {
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
                        value: Some(Box::new(Assignable::Integer(IntegerAST { value: "1".to_string(), ty: IntegerType::I32 }))),
                        positive: true,
                        ..Default::default()
                    })),
                    operator: Operator::Add,
                    prefix_arithmetic: None,
                    value: None,
                    index_operator: None,
                    positive: true,
                }),
                file_position: FilePosition { line: 7..=7, column: 5..=44 },
            }
        ),
    ];

    assert_eq!(expected, top_level_scope.result.program);

    Ok(())
}

#[test]
fn variable_test_types() -> anyhow::Result<()> {
    // todo
    // let variables = r#"
    // let fisch = "Fische sind wirklich wirklich toll";
    // let hallo = "Thomas"; let tschuess = 5;
    // let mallo = "";
    // let michi =
    // Data {
    //     guten: "Hallo",
    //     ciau: 5,
    //     rofl: name(),
    //     mofl: name(nestedMethod("Hallo", moin("Ciao", 5)))
    // };
    // let value = 9;
    // let ref_value = &value;
    // let pointer_arithmetic = *ref_value + 1;
    // "#;
    let variables = r#"
    let fisch = "Fische sind wirklich wirklich toll";
    let hallo = "Thomas"; let tschuess = 5;
    let mallo = "";
    let value = 9;
    let ref_value = &value;
    let pointer_arithmetic = *ref_value + 1;
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(variables)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;

    let expected = vec![
        r#type::common::string(),
        r#type::common::string(),
        Type::Integer(IntegerType::I32, Mutability::Immutable),
        r#type::common::string(),
        Type::Integer(IntegerType::I32, Mutability::Immutable),
        Type::Custom(Identifier { name: "*i32".to_string() }, Mutability::Immutable),
        Type::Integer(IntegerType::I32, Mutability::Immutable),
    ];

    for (index, node) in top_level_scope.result.program.iter().enumerate() {
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

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(variables)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;
    println!("{:?}", top_level_scope);

    let expected = vec![
        Type::Float(FloatType::Float32, Mutability::Immutable),
        Type::Float(FloatType::Float32, Mutability::Immutable),
        Type::Integer(IntegerType::I32, Mutability::Immutable),
        Type::Float(FloatType::Float32, Mutability::Immutable),
        Type::Float(FloatType::Float32, Mutability::Immutable),
        Type::Float(FloatType::Float32, Mutability::Immutable),
    ];

    for (index, node) in top_level_scope.result.program.iter().enumerate() {
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

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(variables)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;

    let expected = vec![
        Type::Float(FloatType::Float32, Mutability::Immutable),
    ];

    let s = None;

    for node in &top_level_scope.result.program {
        println!("{}", node);
        match node {
            AbstractSyntaxTreeNode::Variable(v) => {
                println!("{:?}", match &v.assignable {
                    Assignable::Expression(a) => {
                        &a.prefix_arithmetic
                    },
                    _ => { &s }
                });
            },
            _ => {}
        }
    }

    for (index, node) in top_level_scope.result.program.iter().enumerate() {
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

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(variables)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;

    let expected = vec![
        Type::Integer(IntegerType::I32, Mutability::Immutable),
        Type::Custom(Identifier { name: "*i32".to_string() }, Mutability::Immutable),
        Type::Integer(IntegerType::I32, Mutability::Immutable),
        Type::Custom(Identifier { name: "*i32".to_string() }, Mutability::Immutable),
        Type::Float(FloatType::Float32, Mutability::Immutable),
        Type::Float(FloatType::Float32, Mutability::Immutable),
        Type::Float(FloatType::Float32, Mutability::Immutable),
        Type::Integer(IntegerType::I32, Mutability::Immutable),
        Type::Integer(IntegerType::I32, Mutability::Immutable),
    ];

    for (index, node) in top_level_scope.result.program.iter().enumerate() {
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

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(variables)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;

    println!("{:#?}", top_level_scope);

    let expected = vec![
        Type::Integer(IntegerType::I32, Mutability::Immutable),
        Type::Custom(Identifier { name: "*i32".to_string() }, Mutability::Immutable),
        Type::Integer(IntegerType::I32, Mutability::Immutable),
        Type::Integer(IntegerType::I32, Mutability::Immutable),
        Type::Integer(IntegerType::I32, Mutability::Immutable),
    ];

    for node in &top_level_scope.result.program {
        println!("{}", node);
    }


    for (index, node) in top_level_scope.result.program.iter().enumerate() {
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

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(variables)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;

    println!("{:#?}", top_level_scope);

    let expected = vec![
        Type::Integer(IntegerType::I32, Mutability::Immutable),
        Type::Integer(IntegerType::I64, Mutability::Immutable),
        Type::Integer(IntegerType::I16, Mutability::Immutable),
        Type::Integer(IntegerType::I8, Mutability::Immutable),
        Type::Integer(IntegerType::I64, Mutability::Immutable),
        Type::Integer(IntegerType::U8, Mutability::Immutable),
        Type::Integer(IntegerType::U16, Mutability::Immutable),
        Type::Integer(IntegerType::U32, Mutability::Immutable),
        Type::Integer(IntegerType::U64, Mutability::Immutable),
        Type::Integer(IntegerType::U64, Mutability::Immutable),
    ];

    for node in &top_level_scope.result.program {
        println!("{}", node);
    }


    for (index, node) in top_level_scope.result.program.iter().enumerate() {
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

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(variables)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;

    println!("{:#?}", top_level_scope);

    let expected = vec![
        Type::Integer(IntegerType::I32, Mutability::Immutable),
        Type::Integer(IntegerType::I64, Mutability::Immutable),
        Type::Integer(IntegerType::I16, Mutability::Immutable),
        Type::Integer(IntegerType::I8, Mutability::Immutable),
        Type::Integer(IntegerType::U8, Mutability::Immutable),
        Type::Integer(IntegerType::U16, Mutability::Immutable),
        Type::Integer(IntegerType::U32, Mutability::Immutable),
        Type::Integer(IntegerType::U64, Mutability::Immutable),
    ];

    for node in &top_level_scope.result.program {
        println!("{}", node);
    }


    for (index, node) in top_level_scope.result.program.iter().enumerate() {
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
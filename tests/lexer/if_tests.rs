use monkey_language::core::io::code_line::CodeLine;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::scanner::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::assignable::Assignable;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::assignables::equation_parser::operator::Operator;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::assignables::equation_parser::operator::Operator::Div;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::assignables::integer::IntegerAST;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::assignables::string::StaticString;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::identifier::Identifier;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::r#if::If;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::l_value::LValue;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::variable::Variable;
use monkey_language::core::scanner::parser::ASTParser;
use monkey_language::core::scanner::types::integer::Integer;
use monkey_language::core::scanner::types::r#type::{Mutability, Type};

#[test]
fn if_test() -> anyhow::Result<()> {
    let function = r#"
    if (variable) {
        let mut if_variable_one = 10;
        let if_variable_two = 2;
    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = ASTParser::from(monkey_file);
    let top_level_scope = lexer.parse()?;

    let expected = vec![
        AbstractSyntaxTreeNode::If(If {
            condition: Assignable::Identifier(Identifier { name: String::from("variable") }),
            if_stack: vec![
                AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "if_variable_one".to_string() }), mutability: true, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "10".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let mut if_variable_one = 10 ;".to_string(), actual_line_number: 3..3, virtual_line_number: 2 } }),
                AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "if_variable_two".to_string() }), mutability: false, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "2".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let if_variable_two = 2 ;".to_string(), actual_line_number: 4..4, virtual_line_number: 3 } }),
            ],
            else_stack: None,
            file_position: CodeLine { line: "if  ( variable )  {".to_string(), actual_line_number: 2..2, virtual_line_number: 1 },
        }),
    ];

    println!("{:?}", top_level_scope.ast_nodes);
    println!("{:?}", expected);

    assert_eq!(expected, top_level_scope.ast_nodes);

    let function = r#"
    if(variable){
    let mut if_variable_one = 10;
        let if_variable_two = 2;
    }
    "#;

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = ASTParser::from(monkey_file);
    let top_level_scope = lexer.parse()?;

    assert_eq!(expected, top_level_scope.ast_nodes);

    let function = r#"
    if(variable){let mut if_variable_one = 10;
        let if_variable_two = 2;
    }
    "#;


    let expected = vec![
        AbstractSyntaxTreeNode::If(If {
            condition: Assignable::Identifier(Identifier { name: String::from("variable") }),
            if_stack: vec![
                AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "if_variable_one".to_string() }), mutability: true, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "10".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let mut if_variable_one = 10 ;".to_string(), actual_line_number: 2..2, virtual_line_number: 2 } }),
                AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "if_variable_two".to_string() }), mutability: false, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "2".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let if_variable_two = 2 ;".to_string(), actual_line_number: 3..3, virtual_line_number: 3 } }),
            ],
            else_stack: None,
            file_position: CodeLine { line: "if  ( variable )  {".to_string(), actual_line_number: 2..2, virtual_line_number: 1 },
        }),
    ];

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = ASTParser::from(monkey_file);
    let top_level_scope = lexer.parse()?;

    assert_eq!(expected, top_level_scope.ast_nodes);
    Ok(())
}

#[test]
fn multiple_if_test() -> anyhow::Result<()> {
    let function = r#"
    if (variable1) {
        let if_variable_one = 10;
        let if_variable_two = 2;
    }

    if (variable2) {
        let if_variable_one = 10;
        let if_variable_two = 2;
    }


    if (variable3) {

    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = ASTParser::from(monkey_file);
    let top_level_scope = lexer.parse()?;

    let expected = vec![
        AbstractSyntaxTreeNode::If(If {
            condition: Assignable::Identifier(Identifier { name: String::from("variable1") }),
            if_stack: vec![AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "if_variable_one".to_string() }), mutability: false, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "10".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let if_variable_one = 10 ;".to_string(), actual_line_number: 3..3, virtual_line_number: 2 } }), AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "if_variable_two".to_string() }), mutability: false, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "2".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let if_variable_two = 2 ;".to_string(), actual_line_number: 4..4, virtual_line_number: 3 } })],
            else_stack: None,
            file_position: CodeLine { line: "if  ( variable1 )  {".to_string(), actual_line_number: 2..2, virtual_line_number: 1 },
        }),
        AbstractSyntaxTreeNode::If(If {
            condition: Assignable::Identifier(Identifier { name: String::from("variable2") }),
            if_stack: vec![AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "if_variable_one".to_string() }), mutability: false, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "10".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let if_variable_one = 10 ;".to_string(), actual_line_number: 8..8, virtual_line_number: 6 } }), AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "if_variable_two".to_string() }), mutability: false, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "2".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let if_variable_two = 2 ;".to_string(), actual_line_number: 9..9, virtual_line_number: 7 } })],
            else_stack: None,
            file_position: CodeLine { line: "if  ( variable2 )  {".to_string(), actual_line_number: 7..7, virtual_line_number: 5 },
        }),
        AbstractSyntaxTreeNode::If(If { condition: Assignable::Identifier(Identifier { name: String::from("variable3") }), if_stack: vec![], else_stack: None, file_position: CodeLine { line: "if  ( variable3 )  {".to_string(), actual_line_number: 13..13, virtual_line_number: 9 } }),
    ];

    assert_eq!(expected, top_level_scope.ast_nodes);
    Ok(())
}

#[test]
fn if_else_test() -> anyhow::Result<()> {
    let function = r#"if (variable) {
        let mut   if_variable_one = 10;
        let if_variable_two = 2;
    } else {
        let else_variable_one = 10;
        let mut else_variable_two = 2;
    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = ASTParser::from(monkey_file);
    let top_level_scope = lexer.parse()?;

    let expected = vec![
        AbstractSyntaxTreeNode::If(If {
            condition: Assignable::Identifier(Identifier { name: String::from("variable") }),
            if_stack: vec![
                AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "if_variable_one".to_string() }), mutability: true, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "10".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let mut if_variable_one = 10 ;".to_string(), actual_line_number: 2..2, virtual_line_number: 2 } }),
                AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "if_variable_two".to_string() }), mutability: false, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "2".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let if_variable_two = 2 ;".to_string(), actual_line_number: 3..3, virtual_line_number: 3 } }),
            ],
            else_stack: Some(vec![
                AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "else_variable_one".to_string() }), mutability: false, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "10".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let else_variable_one = 10 ;".to_string(), actual_line_number: 5..5, virtual_line_number: 6 } }),
                AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "else_variable_two".to_string() }), mutability: true, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "2".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let mut else_variable_two = 2 ;".to_string(), actual_line_number: 6..6, virtual_line_number: 7 } }),
            ]),
            file_position: CodeLine { line: "if  ( variable )  {".to_string(), actual_line_number: 1..1, virtual_line_number: 1 },
        }),
    ];

    assert_eq!(expected, top_level_scope.ast_nodes);


    let function = r#"
    if (variable) {let mut if_variable_one = 10; let if_variable_two = 2; } else {
        let else_variable_one = 10;
        let   mut else_variable_two = 2;
    }
    "#;


    let expected = vec![
        AbstractSyntaxTreeNode::If(If {
            condition: Assignable::Identifier(Identifier { name: String::from("variable") }),
            if_stack: vec![
                AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "if_variable_one".to_string() }), mutability: true, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "10".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let mut if_variable_one = 10 ;".to_string(), actual_line_number: 2..2, virtual_line_number: 2 } }),
                AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "if_variable_two".to_string() }), mutability: false, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "2".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let if_variable_two = 2 ;".to_string(), actual_line_number: 2..2, virtual_line_number: 3 } }),
            ],
            else_stack: Some(vec![
                AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "else_variable_one".to_string() }), mutability: false, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "10".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let else_variable_one = 10 ;".to_string(), actual_line_number: 3..3, virtual_line_number: 6 } }),
                AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "else_variable_two".to_string() }), mutability: true, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "2".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let mut else_variable_two = 2 ;".to_string(), actual_line_number: 4..4, virtual_line_number: 7 } }),
            ]),
            file_position: CodeLine { line: "if  ( variable )  {".to_string(), actual_line_number: 2..2, virtual_line_number: 1 },
        }),
    ];

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = ASTParser::from(monkey_file);
    let top_level_scope = lexer.parse()?;

    assert_eq!(expected, top_level_scope.ast_nodes);


    let function = r#"
    if (variable) {
        let mut if_variable_one = 10;
        let if_variable_two = 2;
    }

    else { let else_variable_one = 10; let mut else_variable_two = 2; }
    "#;

    let expected = vec![
        AbstractSyntaxTreeNode::If(If {
            condition: Assignable::Identifier(Identifier { name: String::from("variable") }),
            if_stack: vec![
                AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "if_variable_one".to_string() }), mutability: true, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "10".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let mut if_variable_one = 10 ;".to_string(), actual_line_number: 3..3, virtual_line_number: 2 } }),
                AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "if_variable_two".to_string() }), mutability: false, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "2".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let if_variable_two = 2 ;".to_string(), actual_line_number: 4..4, virtual_line_number: 3 } }),
            ],
            else_stack: Some(vec![
                AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "else_variable_one".to_string() }), mutability: false, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "10".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let else_variable_one = 10 ;".to_string(), actual_line_number: 7..7, virtual_line_number: 6 } }),
                AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "else_variable_two".to_string() }), mutability: true, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "2".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let mut else_variable_two = 2 ;".to_string(), actual_line_number: 7..7, virtual_line_number: 7 } }),
            ]),
            file_position: CodeLine { line: "if  ( variable )  {".to_string(), actual_line_number: 2..2, virtual_line_number: 1 },
        }),
    ];

    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = ASTParser::from(monkey_file);
    let top_level_scope = lexer.parse()?;

    assert_eq!(expected, top_level_scope.ast_nodes);
    Ok(())
}

#[test]
fn function_in_function_test() -> anyhow::Result<()> {
    let function = r#"
    if (hallo) {
        let if_stack_variable = 5 / 2;

        if(if_stack_variable) {
            let nested_if_stack_variable = 13;
        } else {let nested_else_stack_variable = "nice";}
    } else {
        let else_stack_variable = "hallo";
    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = ASTParser::from(monkey_file);
    let top_level_scope = lexer.parse()?;

    let expected = vec![
        AbstractSyntaxTreeNode::If(If {
            condition: Assignable::Identifier(Identifier { name: "hallo".to_string() }),
            if_stack: vec![
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier { name: "if_stack_variable".to_string() }),
                    mutability: false,
                    ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::ArithmeticEquation(
                        Expression { lhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, prefix_arithmetic: None, value: Some(Box::new(Assignable::Integer(IntegerAST { value: "5".to_string(), ty: Integer::I32 }))), index_operator: None, positive: true })), operator: Div, rhs: Some(Box::new(Expression { lhs: None, rhs: None, operator: Operator::Noop, prefix_arithmetic: None, value: Some(Box::new(Assignable::Integer(IntegerAST { value: "2".to_string(), ty: Integer::I32 }))), index_operator: None, positive: true })), positive: true, value: None, prefix_arithmetic: None, index_operator: None }
                    ),
                    code_line: CodeLine { line: "let if_stack_variable = 5 / 2 ;".to_string(), actual_line_number: 3..3, virtual_line_number: 2 },
                }),
                AbstractSyntaxTreeNode::If(If {
                    condition: Assignable::Identifier(Identifier { name: "if_stack_variable".to_string() }),
                    if_stack: vec![
                        AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "nested_if_stack_variable".to_string() }), mutability: false, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "13".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let nested_if_stack_variable = 13 ;".to_string(), actual_line_number: 6..6, virtual_line_number: 4 } })
                    ],
                    else_stack: Some(vec![
                        AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "nested_else_stack_variable".to_string() }), mutability: false, ty: Some(Type::Custom(Identifier { name: String::from("*string") }, Mutability::Immutable)), define: true, assignable: Assignable::String(StaticString { value: "\"nice\"".to_string() }), code_line: CodeLine { line: "let nested_else_stack_variable = \"nice\" ;".to_string(), actual_line_number: 7..7, virtual_line_number: 7 } })
                    ]),
                    file_position: CodeLine { line: "if  ( if_stack_variable )  {".to_string(), actual_line_number: 5..5, virtual_line_number: 3 },
                }),
            ],
            else_stack: Some(vec![
                AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "else_stack_variable".to_string() }), mutability: false, ty: Some(Type::Custom(Identifier { name: String::from("*string") }, Mutability::Immutable)), define: true, assignable: Assignable::String(StaticString { value: "\"hallo\"".to_string() }), code_line: CodeLine { line: "let else_stack_variable = \"hallo\" ;".to_string(), actual_line_number: 9..9, virtual_line_number: 11 } })
            ]),
            file_position: CodeLine { line: "if  ( hallo )  {".to_string(), actual_line_number: 2..2, virtual_line_number: 1 },
        })
    ];

    assert_eq!(expected, top_level_scope.ast_nodes);
    Ok(())
}
use monkey_language::core::io::code_line::CodeLine;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::assignable::Assignable;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::assignables::integer::IntegerAST;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::assignables::string::StaticString;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::identifier::Identifier;
use monkey_language::core::lexer::parser::Lexer;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::l_value::LValue;
use monkey_language::core::lexer::abstract_syntax_tree_nodes::method_definition::{MethodArgument, MethodDefinition};
use monkey_language::core::lexer::abstract_syntax_tree_nodes::variable::Variable;
use monkey_language::core::lexer::types::integer::Integer;
use monkey_language::core::lexer::types::r#type::{Mutability, Type};

#[test]
fn function_test() -> anyhow::Result<()> {
    let function = r#"
    fn method_name(variable: i32, variable: i32): void {
        let function_variable_one = 10;
        let function_variable_two = 2;
    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.parse()?;

    let expected = vec![
        AbstractSyntaxTreeNode::MethodDefinition(MethodDefinition {
            identifier: Identifier { name: "method_name".to_string() },
            return_type: Type::Void,
            arguments: vec![
                MethodArgument {
                    name: Identifier { name: "variable".to_string() },
                    ty: Type::Integer(Integer::I32, Mutability::Immutable),
                },
                MethodArgument {
                    name: Identifier { name: "variable".to_string() },
                    ty: Type::Integer(Integer::I32, Mutability::Immutable),
                },
            ],
            stack: vec![
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier { name: "function_variable_one".to_string() }),
                    mutability: false,
                    ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Integer(IntegerAST { value: "10".to_string(), ty: Integer::I32 }),
                    code_line: CodeLine { line: "let function_variable_one = 10 ;".to_string(), actual_line_number: 3..3, virtual_line_number: 2 },
                }),
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier { name: "function_variable_two".to_string() }),
                    mutability: false,
                    ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Integer(IntegerAST { value: "2".to_string(), ty: Integer::I32 }),
                    code_line: CodeLine { line: "let function_variable_two = 2 ;".to_string(), actual_line_number: 4..4, virtual_line_number: 3 },
                })],
            is_extern: false,
            code_line: CodeLine { line: "fn method_name ( variable :  i32 ,  variable :  i32 )  :  void {".to_string(), actual_line_number: 2..2, virtual_line_number: 1 },
        }),
    ];

    assert_eq!(expected, top_level_scope.ast_nodes);
    Ok(())
}

#[test]
fn multiple_functions_test() -> anyhow::Result<()> {
    let function = r#"
    fn f(variable1: i32, variable2: Data): void
    {
        let function_variable_one = 10;
    }
    
    fn method_name(variable1: bool, variable2: *string): void {
        let function_variable_one = 10;
        let function_variable_two = 2;
    }
    

    fn method_without_parameters( ): void {

    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.parse()?;

    let expected = vec![
        AbstractSyntaxTreeNode::MethodDefinition(MethodDefinition {
            identifier: Identifier { name: "f".to_string() },
            return_type: Type::Void,
            arguments: vec![
                MethodArgument {
                    name: Identifier { name: "variable1".to_string() },
                    ty: Type::Integer(Integer::I32, Mutability::Immutable),
                },
                MethodArgument {
                    name: Identifier { name: "variable2".to_string() },
                    ty: Type::Custom(Identifier { name: "Data".to_string() }, Mutability::Immutable),
                },
            ],
            stack: vec![AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "function_variable_one".to_string() }), mutability: false, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "10".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let function_variable_one = 10 ;".to_string(), actual_line_number: 4..4, virtual_line_number: 2 } })],
            is_extern: false,
            code_line: CodeLine { line: "fn f ( variable1 :  i32 ,  variable2 :  Data )  :  void {".to_string(), actual_line_number: 2..3, virtual_line_number: 1 },
        }),
        AbstractSyntaxTreeNode::MethodDefinition(MethodDefinition {
            identifier: Identifier {
                name: "method_name".to_string()
            },
            return_type: Type::Void,
            arguments: vec![
                MethodArgument {
                    name: Identifier { name: "variable1".to_string() },
                    ty: Type::Bool(Mutability::Immutable),
                },
                MethodArgument {
                    name: Identifier { name: "variable2".to_string() },
                    ty: Type::Custom(Identifier { name: "*string".to_string() }, Mutability::Immutable),
                },
            ],
            stack: vec![AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "function_variable_one".to_string() }), mutability: false, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "10".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let function_variable_one = 10 ;".to_string(), actual_line_number: 8..8, virtual_line_number: 5 } }), AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "function_variable_two".to_string() }), mutability: false, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "2".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let function_variable_two = 2 ;".to_string(), actual_line_number: 9..9, virtual_line_number: 6 } })],
            is_extern: false,
            code_line: CodeLine { line: "fn method_name ( variable1 :  bool ,  variable2 :  *string )  :  void {".to_string(), actual_line_number: 7..7, virtual_line_number: 4 },
        }),
        AbstractSyntaxTreeNode::MethodDefinition(MethodDefinition { identifier: Identifier { name: "method_without_parameters".to_string() }, return_type: Type::Void, arguments: vec![], stack: vec![], is_extern: false, code_line: CodeLine { line: "fn method_without_parameters (   )  :  void {".to_string(), actual_line_number: 13..13, virtual_line_number: 8 } }),
    ];

    assert_eq!(expected, top_level_scope.ast_nodes);
    Ok(())
}

#[test]
fn function_different_return_type_test() -> anyhow::Result<()> {
    let function = r#"
    fn f(variable1: i32, variable2: i32): *string
    {
        let function_variable_zero = "Hallo";
    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.parse()?;

    println!("{:#?}", top_level_scope);

    let expected = vec![
        AbstractSyntaxTreeNode::MethodDefinition(MethodDefinition {
            identifier: Identifier { name: "f".to_string() },
            return_type: Type::Custom(Identifier { name: String::from("*string") }, Mutability::Immutable),
            arguments: vec![
                MethodArgument {
                    name: Identifier { name: "variable1".to_string() },
                    ty: Type::Integer(Integer::I32, Mutability::Immutable),
                },
                MethodArgument {
                    name: Identifier { name: "variable2".to_string() },
                    ty: Type::Integer(Integer::I32, Mutability::Immutable),
                },
            ],
            stack: vec![
                AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "function_variable_zero".to_string() }), mutability: false, ty: Some(Type::Custom(Identifier { name: String::from("*string") }, Mutability::Immutable)), define: true, assignable: Assignable::String(StaticString { value: "\"Hallo\"".to_string() }), code_line: CodeLine { line: "let function_variable_zero = \"Hallo\" ;".to_string(), actual_line_number: 4..4, virtual_line_number: 2 } }),
            ],
            is_extern: false,
            code_line: CodeLine { line: "fn f ( variable1 :  i32 ,  variable2 :  i32 )  :  *string {".to_string(), actual_line_number: 2..3, virtual_line_number: 1 },
        }),
    ];

    assert_eq!(expected, top_level_scope.ast_nodes);
    Ok(())
}

#[test]
fn function_in_function_test() -> anyhow::Result<()> {
    let function = r#"
    fn f(variable1: i32, variable2: i32): void
    {
        let function_variable_zero = "Hallo";
        fn method_name(variable1: i32, variable2: i32): void {
            let function_variable_one = 10;
            let function_variable_two = 2;
        }
    }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = Lexer::from(monkey_file);
    let top_level_scope = lexer.parse()?;

    let expected = vec![
        AbstractSyntaxTreeNode::MethodDefinition(MethodDefinition {
            identifier: Identifier { name: "f".to_string() },
            return_type: Type::Void,
            arguments: vec![
                MethodArgument {
                    name: Identifier { name: "variable1".to_string() },
                    ty: Type::Integer(Integer::I32, Mutability::Immutable),
                },
                MethodArgument {
                    name: Identifier { name: "variable2".to_string() },
                    ty: Type::Integer(Integer::I32, Mutability::Immutable),
                }
            ],
            stack: vec![
                AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "function_variable_zero".to_string() }), mutability: false, ty: Some(Type::Custom(Identifier { name: String::from("*string") }, Mutability::Immutable)), define: true, assignable: Assignable::String(StaticString { value: "\"Hallo\"".to_string() }), code_line: CodeLine { line: "let function_variable_zero = \"Hallo\" ;".to_string(), actual_line_number: 4..4, virtual_line_number: 2 } }),
                AbstractSyntaxTreeNode::MethodDefinition(MethodDefinition {
                    identifier: Identifier { name: "method_name".to_string() },
                    return_type: Type::Void,
                    arguments: vec![
                        MethodArgument {
                            name: Identifier { name: "variable1".to_string() },
                            ty: Type::Integer(Integer::I32, Mutability::Immutable),
                        }, MethodArgument {
                            name: Identifier { name: "variable2".to_string() },
                            ty: Type::Integer(Integer::I32, Mutability::Immutable),
                        }
                    ],
                    stack: vec![
                        AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "function_variable_one".to_string() }), mutability: false, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "10".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let function_variable_one = 10 ;".to_string(), actual_line_number: 6..6, virtual_line_number: 4 } }),
                        AbstractSyntaxTreeNode::Variable(Variable { l_value: LValue::Identifier(Identifier { name: "function_variable_two".to_string() }), mutability: false, ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)), define: true, assignable: Assignable::Integer(IntegerAST { value: "2".to_string(), ty: Integer::I32 }), code_line: CodeLine { line: "let function_variable_two = 2 ;".to_string(), actual_line_number: 7..7, virtual_line_number: 5 } }),
                    ],
                    is_extern: false,
                    code_line: CodeLine { line: "fn method_name ( variable1 :  i32 ,  variable2 :  i32 )  :  void {".to_string(), actual_line_number: 5..5, virtual_line_number: 3 },
                }),
            ],
            is_extern: false,
            code_line: CodeLine { line: "fn f ( variable1 :  i32 ,  variable2 :  i32 )  :  void {".to_string(), actual_line_number: 2..3, virtual_line_number: 1 },
        }),
    ];

    assert_eq!(expected, top_level_scope.ast_nodes);
    Ok(())
}
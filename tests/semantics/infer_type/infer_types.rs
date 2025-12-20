use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::lexer::token_with_span::FilePosition;
use monkey_language::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use monkey_language::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use monkey_language::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use monkey_language::core::model::abstract_syntax_tree_nodes::if_::If;
use monkey_language::core::model::abstract_syntax_tree_nodes::l_value::LValue;
use monkey_language::core::model::abstract_syntax_tree_nodes::variable::Variable;
use monkey_language::core::model::types::boolean::Boolean;
use monkey_language::core::model::types::float::{FloatAST, FloatType};
use monkey_language::core::model::types::integer::{IntegerAST, IntegerType};
use monkey_language::core::model::types::mutability::Mutability;
use monkey_language::core::model::types::static_string::StaticString;
use monkey_language::core::model::types::ty::Type;
use monkey_language::core::parser::ast_parser::ASTParser;
use monkey_language::core::semantics::type_infer::type_inferer::infer_type;

#[test]
fn infer_type_test() -> anyhow::Result<()> {
    let function = r#"
        let a = 1;
        let b = 2.0;
        let c = true;
        let d = "KEKW";
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;

    let expected: Vec<AbstractSyntaxTreeNode> = vec![
        AbstractSyntaxTreeNode::Variable(Variable {
            l_value: LValue::Identifier(Identifier { name: "a".to_string() }),
            mutability: false,
            ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
            define: true,
            assignable: Assignable::Integer(IntegerAST { value: "1".to_string(), ty: IntegerType::I32 }),
            file_position: FilePosition { line: 2..=2, column: 9..=18 },
        }),
        AbstractSyntaxTreeNode::Variable(Variable {
            l_value: LValue::Identifier(Identifier { name: "b".to_string() }),
            mutability: false,
            ty: Some(Type::Float(FloatType::Float32, Mutability::Immutable)),
            define: true,
            assignable: Assignable::Float(FloatAST { value: 2.0, ty: FloatType::Float32 }),
            file_position: FilePosition { line: 3..=3, column: 9..=20 },
        }),
        AbstractSyntaxTreeNode::Variable(Variable {
            l_value: LValue::Identifier(Identifier { name: "c".to_string() }),
            mutability: false,
            ty: Some(Type::Bool(Mutability::Immutable)),
            define: true,
            assignable: Assignable::Boolean(Boolean { value: true }),
            file_position: FilePosition { line: 4..=4, column: 9..=21 },
        }),
        AbstractSyntaxTreeNode::Variable(Variable {
            l_value: LValue::Identifier(Identifier { name: "d".to_string() }),
            mutability: false,
            ty: Some(Type::Custom(Identifier { name: String::from("*string") }, Mutability::Immutable)),
            define: true,
            assignable: Assignable::String(StaticString { value: "\"KEKW\"".to_string() }),
            file_position: FilePosition { line: 5..=5, column: 9..=23 },
        }),
    ];

    assert_eq!(expected, top_level_scope.result.program);
    Ok(())
}

#[test]
fn infer_type_in_scope() -> anyhow::Result<()> {
    let function = r#"
        if (true) {
            let a = 1;
            let b = 2.0;
            let c = true;
            let d = "KEKW";
        }
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    infer_type(&mut top_level_scope.result.program)?;

    let expected: Vec<AbstractSyntaxTreeNode> = vec![
        AbstractSyntaxTreeNode::If(If {
            condition: Assignable::Boolean(Boolean { value: true }),
            if_stack: vec![
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier { name: "a".to_string() }),
                    mutability: false,
                    ty: Some(Type::Integer(IntegerType::I32, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Integer(IntegerAST { value: "1".to_string(), ty: IntegerType::I32 }),
                    file_position: FilePosition { line: 3..=3, column: 13..=22 },
                }),
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier { name: "b".to_string() }),
                    mutability: false,
                    ty: Some(Type::Float(FloatType::Float32, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Float(FloatAST { value: 2.0, ty: FloatType::Float32 }),
                    file_position: FilePosition { line: 4..=4, column: 13..=24 },
                }),
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier { name: "c".to_string() }),
                    mutability: false,
                    ty: Some(Type::Bool(Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Boolean(Boolean { value: true }),
                    file_position: FilePosition { line: 5..=5, column: 13..=25 },
                }),
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier { name: "d".to_string() }),
                    mutability: false,
                    ty: Some(Type::Custom(Identifier { name: String::from("*string") }, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::String(StaticString { value: "\"KEKW\"".to_string() }),
                    file_position: FilePosition { line: 6..=6, column: 13..=27 },
                }),
            ],
            else_stack: None,
            file_position: FilePosition { line: 2..=7, column: 9..=9 },
        })
    ];

    assert_eq!(expected, top_level_scope.result.program);
    Ok(())
}
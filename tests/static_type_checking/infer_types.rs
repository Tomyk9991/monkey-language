use monkey_language::core::io::code_line::CodeLine;
use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::scanner::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::assignable::Assignable;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::assignables::boolean::Boolean;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::assignables::float::FloatAST;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::assignables::integer::IntegerAST;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::assignables::string::StaticString;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::identifier::Identifier;
use monkey_language::core::scanner::parser::ASTParser;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::r#if::If;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::l_value::LValue;
use monkey_language::core::scanner::abstract_syntax_tree_nodes::variable::Variable;
use monkey_language::core::scanner::types::float::Float;
use monkey_language::core::scanner::types::integer::Integer;
use monkey_language::core::scanner::types::r#type::{Mutability, Type};
use monkey_language::core::semantics::type_checker::static_type_checker::static_type_check;

#[test]
fn infer_type() -> anyhow::Result<()> {
    let function = r#"
        let a = 1;
        let b = 2.0;
        let c = true;
        let d = "KEKW";
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = ASTParser::from(monkey_file);
    let top_level_scope = lexer.parse()?;

    static_type_check(&top_level_scope)?;

    let expected: Vec<AbstractSyntaxTreeNode> = vec![
        AbstractSyntaxTreeNode::Variable(Variable {
            l_value: LValue::Identifier(Identifier { name: "a".to_string() }),
            mutability: false,
            ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)),
            define: true,
            assignable: Assignable::Integer(IntegerAST { value: "1".to_string(), ty: Integer::I32 }),
            code_line: CodeLine {
                line: "let a = 1 ;".to_string(),
                actual_line_number: 2..2,
                virtual_line_number: 1,
            },
        }),
        AbstractSyntaxTreeNode::Variable(Variable {
            l_value: LValue::Identifier(Identifier { name: "b".to_string() }),
            mutability: false,
            ty: Some(Type::Float(Float::Float32, Mutability::Immutable)),
            define: true,
            assignable: Assignable::Float(FloatAST { value: 2.0, ty: Float::Float32 }),
            code_line: CodeLine {
                line: "let b = 2.0 ;".to_string(),
                actual_line_number: 3..3,
                virtual_line_number: 2,
            },
        }),
        AbstractSyntaxTreeNode::Variable(Variable {
            l_value: LValue::Identifier(Identifier { name: "c".to_string() }),
            mutability: false,
            ty: Some(Type::Bool(Mutability::Immutable)),
            define: true,
            assignable: Assignable::Boolean(Boolean { value: true }),
            code_line: CodeLine {
                line: "let c = true ;".to_string(),
                actual_line_number: 4..4,
                virtual_line_number: 3,
            },
        }),
        AbstractSyntaxTreeNode::Variable(Variable {
            l_value: LValue::Identifier(Identifier { name: "d".to_string() }),
            mutability: false,
            ty: Some(Type::Custom(Identifier { name: String::from("*string") }, Mutability::Immutable)),
            define: true,
            assignable: Assignable::String(StaticString { value: "\"KEKW\"".to_string() }),
            code_line: CodeLine {
                line: "let d = \"KEKW\" ;".to_string(),
                actual_line_number: 5..5,
                virtual_line_number: 4,
            },
        }),
    ];

    assert_eq!(expected, top_level_scope.ast_nodes);
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


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function);
    let mut lexer = ASTParser::from(monkey_file);
    let top_level_scope = lexer.parse()?;

    static_type_check(&top_level_scope)?;

    let expected: Vec<AbstractSyntaxTreeNode> = vec![
        AbstractSyntaxTreeNode::If(If {
            condition: Assignable::Boolean(Boolean { value: true }),
            if_stack: vec![
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier { name: "a".to_string() }),
                    mutability: false,
                    ty: Some(Type::Integer(Integer::I32, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Integer(IntegerAST { value: "1".to_string(), ty: Integer::I32 }),
                    code_line: CodeLine {
                        line: "let a = 1 ;".to_string(),
                        actual_line_number: 3..3,
                        virtual_line_number: 2,
                    },
                }),
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier { name: "b".to_string() }),
                    mutability: false,
                    ty: Some(Type::Float(Float::Float32, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Float(FloatAST { value: 2.0, ty: Float::Float32 }),
                    code_line: CodeLine {
                        line: "let b = 2.0 ;".to_string(),
                        actual_line_number: 4..4,
                        virtual_line_number: 3,
                    },
                }),
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier { name: "c".to_string() }),
                    mutability: false,
                    ty: Some(Type::Bool(Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::Boolean(Boolean { value: true }),
                    code_line: CodeLine {
                        line: "let c = true ;".to_string(),
                        actual_line_number: 5..5,
                        virtual_line_number: 4,
                    },
                }),
                AbstractSyntaxTreeNode::Variable(Variable {
                    l_value: LValue::Identifier(Identifier { name: "d".to_string() }),
                    mutability: false,
                    ty: Some(Type::Custom(Identifier { name: String::from("*string") }, Mutability::Immutable)),
                    define: true,
                    assignable: Assignable::String(StaticString { value: "\"KEKW\"".to_string() }),
                    code_line: CodeLine {
                        line: "let d = \"KEKW\" ;".to_string(),
                        actual_line_number: 6..6,
                        virtual_line_number: 5,
                    },
                }),
            ],
            else_stack: None,
            code_line: CodeLine { line: "if  ( true )  {".to_string(), actual_line_number: 2..2, virtual_line_number: 1 },
        })
    ];

    assert_eq!(expected, top_level_scope.ast_nodes);
    Ok(())
}
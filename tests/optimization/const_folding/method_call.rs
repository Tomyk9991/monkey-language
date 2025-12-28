use monkey_language::core::io::monkey_file::MonkeyFile;
use monkey_language::core::model::abstract_syntax_tree_node::AbstractSyntaxTreeNode;
use monkey_language::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use monkey_language::core::model::abstract_syntax_tree_nodes::variable::Variable;
use monkey_language::core::model::types::integer::IntegerAST;
use monkey_language::core::optimization::optimization_trait::OptimizationContext;
use monkey_language::core::parser::ast_parser::ASTParser;
use monkey_language::core::semantics::static_type_check::static_type_checker::static_type_check;
use monkey_language::core::semantics::type_infer::type_inferer::infer_type;

#[test]
fn method_call_fold() -> anyhow::Result<()> {
    let function = r#"
        extern fn printf(format: *string, value: i32): void;
        fn method(): i32 {
            return 5 + 3;
        }
        let a = method();
        printf("%d\n", a);
    "#;


    let monkey_file: MonkeyFile = MonkeyFile::read_from_str(function)?;
    let mut top_level_scope = ASTParser::parse(&monkey_file.tokens)?;
    let _ = infer_type(&mut top_level_scope.result.program)?;
    let mut static_type_context = static_type_check(&top_level_scope.result.program)?;

    let top_level_scope = top_level_scope.result.o1(&mut static_type_context, OptimizationContext::default());

    println!("{}", top_level_scope);
    assert!(
        matches!(
            &*top_level_scope.program.get(1).unwrap(),
            AbstractSyntaxTreeNode::Variable(Variable {
                assignable: Assignable::Integer(IntegerAST { value: ref v, .. }),
                ..
            }) if v == "8"
        )
    );

    Ok(())
}
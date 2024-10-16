use syntax_parser_generator::reader::string_reader::ByteReader;
use c_lang::lex::lexical_analyzer::CLexicalAnalyzer;
use c_lang::parsing::ast;
use c_lang::parsing::parser::CParser;

mod c_lang;

#[test]
fn test_c_ast_pipeline() {
    let c_lexical_analyzer = CLexicalAnalyzer::new();
    let c_parser = CParser::new();

    let mut source_reader = ByteReader::from_string(String::from(
        "\
        int main(int argc, int argv) { \
            int local; \
            local = 5; \
         \
            while (local = 3) { \
                if (56) { \
                    int x; \
                } \
                else { \
                    if (local = 8) \
                        local = 4; \
                } \
            } \
        } \
        "
    ));
    let expected = ast::AbstractProgram {
        global_statements: vec![Box::from(ast::GlobalStatement::FunctionDefinition {
            name: "main".to_owned(),
            retval_dtype: Box::from(ast::Dtype::Int),
            formal_args: vec![
                Box::from(ast::FormalArg {
                    name: "argc".to_owned(),
                    dtype: Box::from(ast::Dtype::Int),
                }),
                Box::from(ast::FormalArg {
                    name: "argv".to_owned(),
                    dtype: Box::from(ast::Dtype::Int),
                }),
            ],
            body: Box::from(ast::LocalStatement::Block {
                statements: vec![
                    Box::from(ast::LocalStatement::VariableDeclaration {
                        name: "local".to_owned(),
                        dtype: Box::from(ast::Dtype::Int),
                    }),
                    Box::from(ast::LocalStatement::Evaluation {
                        expression: Box::from(ast::Expression::Assignment {
                            lhs: Box::from(ast::LValue::Variable { name: "local".to_owned() }),
                            rhs: Box::new(ast::Expression::IntLiteral { value: 5 }),
                        }),
                    }),
                    Box::from(ast::LocalStatement::While {
                        condition: Box::from(ast::Expression::Assignment {
                            lhs: Box::from(ast::LValue::Variable { name: "local".to_owned() }),
                            rhs: Box::new(ast::Expression::IntLiteral { value: 3 }),
                        }),
                        body: Box::new(ast::LocalStatement::Block {
                            statements: vec![
                                Box::from(ast::LocalStatement::IfElse {
                                    condition: Box::from(ast::Expression::IntLiteral {
                                        value: 56,
                                    }),
                                    true_branch: Box::new(ast::LocalStatement::Block {
                                        statements: vec![
                                            Box::from(ast::LocalStatement::VariableDeclaration {
                                                name: "x".to_owned(),
                                                dtype: Box::from(ast::Dtype::Int),
                                            })
                                        ],
                                    }),
                                    false_branch: Box::new(ast::LocalStatement::Block {
                                        statements: vec![
                                            Box::from(ast::LocalStatement::If {
                                                condition: Box::from(ast::Expression::Assignment {
                                                    lhs: Box::from(ast::LValue::Variable {
                                                        name: "local".to_owned(),
                                                    }),
                                                    rhs: Box::new(ast::Expression::IntLiteral {
                                                        value: 8,
                                                    }),
                                                }),
                                                true_branch: Box::new(
                                                    ast::LocalStatement::Evaluation {
                                                        expression: Box::from(ast::Expression::Assignment {
                                                            lhs: Box::from(ast::LValue::Variable {
                                                                name: "local".to_owned()
                                                            }),
                                                            rhs: Box::new(
                                                                ast::Expression::IntLiteral {
                                                                    value: 4,
                                                                }
                                                            ),
                                                        }),
                                                    }
                                                ),
                                            }),
                                        ],
                                    }),
                                })
                            ]
                        }),
                    }),
                ]
            }),
        })]
    };
    assert_eq!(c_parser.parse(c_lexical_analyzer.analyze(&mut source_reader)), Some(expected));
}
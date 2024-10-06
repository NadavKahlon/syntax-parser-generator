use crate::c_lang::parsing::ast;

pub enum CParserNode {
    // Core
    AbstractProgram(ast::AbstractProgram),
    GlobalStatement(ast::GlobalStatement),
    Dtype(ast::Dtype),
    FormalArg(ast::FormalArg),
    LocalStatement(ast::LocalStatement),
    Expression(ast::Expression),
    LValue(ast::LValue),

    // Utility
    Identifier(String),
    FormalArgList(Vec<Box<ast::FormalArg>>),
    LocalStatementList(Vec<Box<ast::LocalStatement>>),
}

macro_rules! generate_take_node_fn {
    ($fn_name:ident, $node_variant:ident, $node_internal_dtype:ty) => {
        pub fn $fn_name(satellites: &mut Vec<Option<CParserNode>>, index: usize)
                         -> $node_internal_dtype
        {
            match satellites.get_mut(index) {
                Some(satellite) => match satellite.take() {
                    Some(CParserNode::$node_variant(x)) => x,
                    _ => panic!(
                        "No {} satellite found at index {}", stringify!($node_variant), index
                    ),
                },
                None => panic!(
                    "Satellite index {} out of bounds (expected {}) - satellites list size is \
                    only {}", index, stringify!($node_variant), satellites.len(),
                ),
            }
        }
    };
}

impl CParserNode {
    generate_take_node_fn!(take_abstract_program, AbstractProgram, ast::AbstractProgram);
    generate_take_node_fn!(take_global_statement, GlobalStatement, ast::GlobalStatement);
    generate_take_node_fn!(take_dtype, Dtype, ast::Dtype);
    generate_take_node_fn!(take_identifier, Identifier, String);
    generate_take_node_fn!(take_formal_arg_list, FormalArgList, Vec<Box<ast::FormalArg>>);
    generate_take_node_fn!(take_local_statement, LocalStatement, ast::LocalStatement);
    generate_take_node_fn!(take_formal_arg, FormalArg, ast::FormalArg);
    generate_take_node_fn!(take_local_statement_list, LocalStatementList, Vec<Box<ast::LocalStatement>>);
    generate_take_node_fn!(take_expression, Expression, ast::Expression);
    generate_take_node_fn!(take_lvalue, LValue, ast::LValue);
}

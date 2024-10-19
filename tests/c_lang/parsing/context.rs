use crate::c_lang::parsing::ast;
use crate::c_lang::parsing::node::CParserNode;

pub struct CParserContext;

impl CParserContext {
    pub fn identifier(&mut self, lexeme: String) -> Option<CParserNode> {
        Some(CParserNode::Identifier(lexeme))
    }

    pub fn int_literal(&mut self, lexeme: String) -> Option<CParserNode> {
        Some(CParserNode::Expression(ast::Expression::IntLiteral {
            value: lexeme.parse().ok()?,
        }))
    }

    pub fn empty_program(&mut self) -> Option<CParserNode> {
        Some(CParserNode::AbstractProgram(ast::AbstractProgram {
            global_statements: vec![],
        }))
    }

    pub fn add_global_statement(
        &mut self,
        mut satellites: Vec<Option<CParserNode>>,
    ) -> Option<CParserNode> {
        let mut program = CParserNode::take_abstract_program(&mut satellites, 0);
        program
            .global_statements
            .push(Box::new(CParserNode::take_global_statement(
                &mut satellites,
                1,
            )));
        Some(CParserNode::AbstractProgram(program))
    }

    pub fn function_definition(
        &mut self,
        mut satellites: Vec<Option<CParserNode>>,
    ) -> Option<CParserNode> {
        Some(CParserNode::GlobalStatement(
            ast::GlobalStatement::FunctionDefinition {
                name: CParserNode::take_identifier(&mut satellites, 1),
                retval_dtype: Box::from(CParserNode::take_dtype(&mut satellites, 0)),
                formal_args: CParserNode::take_formal_arg_list(&mut satellites, 3),
                body: Box::from(ast::LocalStatement::Block {
                    statements: CParserNode::take_local_statement_list(&mut satellites, 6),
                }),
            },
        ))
    }

    pub fn empty_formal_arg_list(&mut self) -> Option<CParserNode> {
        Some(CParserNode::FormalArgList(vec![]))
    }

    pub fn one_item_formal_arg_list(
        &mut self,
        mut satellites: Vec<Option<CParserNode>>,
    ) -> Option<CParserNode> {
        let formal_arg = CParserNode::take_formal_arg(&mut satellites, 0);
        Some(CParserNode::FormalArgList(vec![Box::new(formal_arg)]))
    }

    pub fn add_formal_arg(
        &mut self,
        mut satellites: Vec<Option<CParserNode>>,
    ) -> Option<CParserNode> {
        let mut formal_arg_list = CParserNode::take_formal_arg_list(&mut satellites, 0);
        let formal_arg = CParserNode::take_formal_arg(&mut satellites, 2);
        formal_arg_list.push(Box::new(formal_arg));
        Some(CParserNode::FormalArgList(formal_arg_list))
    }

    pub fn formal_arg(&mut self, mut satellites: Vec<Option<CParserNode>>) -> Option<CParserNode> {
        Some(CParserNode::FormalArg(ast::FormalArg {
            name: CParserNode::take_identifier(&mut satellites, 1),
            dtype: Box::from(CParserNode::take_dtype(&mut satellites, 0)),
        }))
    }

    pub fn empty_local_statement_list(&mut self) -> Option<CParserNode> {
        Some(CParserNode::LocalStatementList(vec![]))
    }

    pub fn add_local_statement(
        &mut self,
        mut satellites: Vec<Option<CParserNode>>,
    ) -> Option<CParserNode> {
        let mut local_statement_list = CParserNode::take_local_statement_list(&mut satellites, 0);
        let local_statement = CParserNode::take_local_statement(&mut satellites, 1);
        local_statement_list.push(Box::new(local_statement));
        Some(CParserNode::LocalStatementList(local_statement_list))
    }

    pub fn local_variable_declaration(
        &mut self,
        mut satellites: Vec<Option<CParserNode>>,
    ) -> Option<CParserNode> {
        Some(CParserNode::LocalStatement(
            ast::LocalStatement::VariableDeclaration {
                name: CParserNode::take_identifier(&mut satellites, 1),
                dtype: Box::from(CParserNode::take_dtype(&mut satellites, 0)),
            },
        ))
    }

    pub fn local_evaluation(
        &mut self,
        mut satellites: Vec<Option<CParserNode>>,
    ) -> Option<CParserNode> {
        Some(CParserNode::LocalStatement(
            ast::LocalStatement::Evaluation {
                expression: Box::from(CParserNode::take_expression(&mut satellites, 0)),
            },
        ))
    }

    pub fn block_statement(
        &mut self,
        mut satellites: Vec<Option<CParserNode>>,
    ) -> Option<CParserNode> {
        Some(CParserNode::LocalStatement(ast::LocalStatement::Block {
            statements: CParserNode::take_local_statement_list(&mut satellites, 1),
        }))
    }

    pub fn if_statement(
        &mut self,
        mut satellites: Vec<Option<CParserNode>>,
    ) -> Option<CParserNode> {
        Some(CParserNode::LocalStatement(ast::LocalStatement::If {
            condition: Box::from(CParserNode::take_expression(&mut satellites, 2)),
            true_branch: Box::new(CParserNode::take_local_statement(&mut satellites, 4)),
        }))
    }

    pub fn if_else_statement(
        &mut self,
        mut satellites: Vec<Option<CParserNode>>,
    ) -> Option<CParserNode> {
        Some(CParserNode::LocalStatement(ast::LocalStatement::IfElse {
            condition: Box::from(CParserNode::take_expression(&mut satellites, 2)),
            true_branch: Box::new(CParserNode::take_local_statement(&mut satellites, 4)),
            false_branch: Box::new(CParserNode::take_local_statement(&mut satellites, 6)),
        }))
    }

    pub fn while_statement(
        &mut self,
        mut satellites: Vec<Option<CParserNode>>,
    ) -> Option<CParserNode> {
        Some(CParserNode::LocalStatement(ast::LocalStatement::While {
            condition: Box::from(CParserNode::take_expression(&mut satellites, 2)),
            body: Box::new(CParserNode::take_local_statement(&mut satellites, 4)),
        }))
    }

    pub fn variable_expression(
        &mut self,
        mut satellites: Vec<Option<CParserNode>>,
    ) -> Option<CParserNode> {
        Some(CParserNode::Expression(ast::Expression::Variable {
            name: CParserNode::take_identifier(&mut satellites, 0),
        }))
    }

    pub fn assignment(&mut self, mut satellites: Vec<Option<CParserNode>>) -> Option<CParserNode> {
        Some(CParserNode::Expression(ast::Expression::Assignment {
            lhs: Box::from(CParserNode::take_lvalue(&mut satellites, 0)),
            rhs: Box::new(CParserNode::take_expression(&mut satellites, 2)),
        }))
    }

    pub fn identifier_lvalue(
        &mut self,
        mut satellites: Vec<Option<CParserNode>>,
    ) -> Option<CParserNode> {
        Some(CParserNode::LValue(ast::LValue::Variable {
            name: CParserNode::take_identifier(&mut satellites, 0),
        }))
    }

    pub fn int_dtype(&mut self, _satellites: Vec<Option<CParserNode>>) -> Option<CParserNode> {
        Some(CParserNode::Dtype(ast::Dtype::Int))
    }
}

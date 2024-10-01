use crate::c::lex::lexeme_types::CLexemeType;
use crate::c::parsing::ast::AbstractProgram;
use crate::c::parsing::context::CParserContext;
use crate::c::parsing::node::CParserNode;
use crate::lex::Lexeme;
use crate::parsing::translator::build::SyntaxDirectedTranslatorBuilder;
use crate::parsing::translator::sdt::SyntaxDirectedTranslator;

pub struct CParser {
    translator: SyntaxDirectedTranslator<CLexemeType, CParserContext, Option<CParserNode>>,
}

impl CParser {
    pub fn parse(&self, token_stream: impl Iterator<Item=Lexeme<CLexemeType>>)
                 -> Option<AbstractProgram>
    {
        match self.translator.translate(&mut CParserContext, token_stream)?? {
            CParserNode::AbstractProgram(ast) => Some(ast),
            _ => None,
        }
    }

    pub fn new() -> Self {
        let mut builder = SyntaxDirectedTranslatorBuilder::new();

        // Register nonterminals
        builder.new_nonterminals(vec![
            "abstract_program", "global_statement_list", "global_statement", "function_definition",
            "dtype", "formal_arg_list", "formal_arg", "formal_arg_list_nonempty",
            "local_statement_list", "local_statement", "expression", "lvalue",
        ].into_iter());
        builder.set_start_nonterminal("abstract_program");

        // Register terminals
        builder.dub_lexeme_types(vec![
            (CLexemeType::If, "if"),
            (CLexemeType::Else, "else"),
            (CLexemeType::While, "while"),
            (CLexemeType::Int, "int"),
            (CLexemeType::Identifier, "IDENTIFIER"),
            (CLexemeType::IntLiteral, "INT_LITERAL"),
            (CLexemeType::Assignment, "="),
            (CLexemeType::LeftParenthesis, "("),
            (CLexemeType::RightParenthesis, ")"),
            (CLexemeType::LeftBrace, "{"),
            (CLexemeType::RightBrace, "}"),
            (CLexemeType::Semicolon, ";"),
            (CLexemeType::Comma, ","),
        ].into_iter());

        // Register leaf satellite builders
        builder.set_default_leaf_satellite_builder(|_, _| None);
        builder.set_leaf_satellite_builder(
            "IDENTIFIER",
            CParserContext::identifier,
        );
        builder.set_leaf_satellite_builder(
            "INT_LITERAL",
            CParserContext::int_literal,
        );

        // Production rules

        builder.register_identity_rule("abstract_program", "global_statement_list");
        builder.register_empty_rule("global_statement_list", CParserContext::empty_program);
        builder.register_rule(
            "global_statement_list",
            vec!["global_statement_list", "global_statement"],
            CParserContext::add_global_statement,
        );

        builder.register_identity_rule("global_statement", "function_definition");
        builder.register_rule(
            "function_definition",
            vec![
                "dtype", "IDENTIFIER",
                "(", "formal_arg_list", ")",
                "{", "local_statement_list", "}",
            ],
            CParserContext::function_definition,
        );

        builder.register_empty_rule("formal_arg_list", CParserContext::empty_formal_arg_list);
        builder.register_identity_rule("formal_arg_list", "formal_arg_list_nonempty");
        builder.register_rule(
            "formal_arg_list_nonempty",
            vec!["formal_arg"],
            CParserContext::one_item_formal_arg_list,
        );
        builder.register_rule(
            "formal_arg_list_nonempty",
            vec!["formal_arg_list_nonempty", ",", "formal_arg"],
            CParserContext::add_formal_arg,
        );
        builder.register_rule(
            "formal_arg",
            vec!["dtype", "IDENTIFIER"],
            CParserContext::formal_arg,
        );

        builder.register_empty_rule(
            "local_statement_list",
            CParserContext::empty_local_statement_list,
        );
        builder.register_rule(
            "local_statement_list",
            vec!["local_statement_list", "local_statement"],
            CParserContext::add_local_statement,
        );

        builder.register_rule(
            "local_statement",
            vec!["dtype", "IDENTIFIER", ";"],
            CParserContext::local_variable_declaration,
        );
        builder.register_rule(
            "local_statement",
            vec!["expression", ";"],
            CParserContext::local_evaluation,
        );
        builder.register_rule(
            "local_statement",
            vec!["{", "local_statement_list", "}"],
            CParserContext::block_statement,
        );
        builder.register_rule(
            "local_statement",
            vec!["if", "(", "expression", ")", "local_statement"],
            CParserContext::if_statement,
        );
        builder.register_rule(
            "local_statement",
            vec!["if", "(", "expression", ")", "local_statement", "else", "local_statement"],
            CParserContext::if_else_statement,
        );
        builder.register_rule(
            "local_statement",
            vec!["while", "(", "expression", ")", "local_statement"],
            CParserContext::while_statement,
        );

        builder.register_identity_rule("expression", "INT_LITERAL");
        builder.register_rule(
            "expression",
            vec!["IDENTIFIER"],
            CParserContext::variable_expression,
        );
        builder.register_rule(
            "expression",
            vec!["lvalue", "=", "expression"],
            CParserContext::assignment,
        );

        builder.register_rule(
            "lvalue",
            vec!["IDENTIFIER"],
            CParserContext::identifier_lvalue,
        );

        builder.register_rule(
            "dtype",
            vec!["int"],
            CParserContext::int_dtype,
        );

        Self { translator: builder.build() }
    }
}
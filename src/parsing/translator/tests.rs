use crate::handle::auto::AutomaticallyHandled;
use crate::lex::Lexeme;
use crate::parsing::lr_parser::rules::Associativity;
use crate::parsing::translator::build::SyntaxDirectedTranslatorBuilder;
use crate::parsing::translator::sdt::SyntaxDirectedTranslator;

struct Context;

#[derive(Debug, Clone, Copy)]
enum LexemeType {
    Integer,
    Plus,
    Star,
    Minus,
    Slash,
    LeftParenthesis,
    RightParenthesis,
}

impl AutomaticallyHandled for LexemeType {
    type HandleCoreType = u8;
    fn serial(&self) -> usize { *self as usize }
}

type Satellite = Option<i32>;

impl Context {
    fn parse_integer(&mut self, content: String) -> Satellite {
        Some(content.parse::<i32>().unwrap())
    }

    fn integer_to_expr(&mut self, args: Vec<Satellite>) -> Satellite {
        args[0]
    }

    fn unwrap_parentheses(&mut self, args: Vec<Satellite>) -> Satellite {
        args[1]
    }

    fn sum(&mut self, args: Vec<Satellite>) -> Satellite {
        Some(args[0].unwrap() + args[2].unwrap())
    }

    fn subtraction(&mut self, args: Vec<Satellite>) -> Satellite {
        Some(args[0].unwrap() - args[2].unwrap())
    }

    fn multiplication(&mut self, args: Vec<Satellite>) -> Satellite {
        Some(args[0].unwrap() * args[2].unwrap())
    }

    fn division(&mut self, args: Vec<Satellite>) -> Satellite {
        Some(args[0].unwrap() / args[2].unwrap())
    }
}


type CalculatorTranslator =
SyntaxDirectedTranslator<LexemeType, Context, Satellite>;

fn build_calculator() -> CalculatorTranslator {
    let mut builder = SyntaxDirectedTranslatorBuilder::new();

    builder.new_nonterminal("expression");
    builder.set_start_nonterminal("expression");

    builder.dub_lexeme_type(LexemeType::Integer, "INTEGER");
    builder.dub_lexeme_type(LexemeType::Plus, "+");
    builder.dub_lexeme_type(LexemeType::Star, "*");
    builder.dub_lexeme_type(LexemeType::Minus, "-");
    builder.dub_lexeme_type(LexemeType::Slash, "/");
    builder.dub_lexeme_type(LexemeType::LeftParenthesis, "(");
    builder.dub_lexeme_type(LexemeType::RightParenthesis, ")");

    builder.new_binding(vec!["*", "/"], Associativity::Left, "*");
    builder.new_binding(vec!["+", "-"], Associativity::Left, "+");

    builder.set_leaf_satellite_builder("INTEGER", Context::parse_integer);
    builder.set_default_leaf_satellite_builder(|_, _| None);

    builder.register_rule(
        "expression", vec!["INTEGER"],
        Context::integer_to_expr,
    );
    builder.register_rule(
        "expression", vec!["(", "expression", ")"],
        Context::unwrap_parentheses,
    );
    builder.register_bound_rule(
        "expression", vec!["expression", "+", "expression"], "+",
        Context::sum,
    );
    builder.register_bound_rule(
        "expression", vec!["expression", "-", "expression"], "+",
        Context::subtraction,
    );
    builder.register_bound_rule(
        "expression", vec!["expression", "*", "expression"], "*",
        Context::multiplication,
    );
    builder.register_bound_rule(
        "expression", vec!["expression", "/", "expression"], "*",
        Context::division,
    );

    builder.build()
}

#[test]
fn test_successful_calculator_translator() {
    let calc = build_calculator();
    let mut c = Context;

    // 8 == 8
    assert_eq!(
        calc.translate(&mut c, vec![
            Lexeme::new(LexemeType::Integer, "8"),
        ].into_iter()),
        Some(Some(8))
    );

    // 8 + 5 == 13
    assert_eq!(
        calc.translate(&mut c, vec![
            Lexeme::new(LexemeType::Integer, "8"),
            Lexeme::new(LexemeType::Plus, "+"),
            Lexeme::new(LexemeType::Integer, "5"),
        ].into_iter()),
        Some(Some(13))
    );

    // 8 * 5 == 40
    assert_eq!(
        calc.translate(&mut c, vec![
            Lexeme::new(LexemeType::Integer, "8"),
            Lexeme::new(LexemeType::Star, "*"),
            Lexeme::new(LexemeType::Integer, "5"),
        ].into_iter()),
        Some(Some(40))
    );

    // 8 - 5 == 3
    assert_eq!(
        calc.translate(&mut c, vec![
            Lexeme::new(LexemeType::Integer, "8"),
            Lexeme::new(LexemeType::Minus, "-"),
            Lexeme::new(LexemeType::Integer, "5"),
        ].into_iter()),
        Some(Some(3))
    );

    // 8 / 5 == 1
    assert_eq!(
        calc.translate(&mut c, vec![
            Lexeme::new(LexemeType::Integer, "8"),
            Lexeme::new(LexemeType::Slash, "/"),
            Lexeme::new(LexemeType::Integer, "5"),
        ].into_iter()),
        Some(Some(1))
    );

    // (8) == 8
    assert_eq!(
        calc.translate(&mut c, vec![
            Lexeme::new(LexemeType::LeftParenthesis, "("),
            Lexeme::new(LexemeType::Integer, "8"),
            Lexeme::new(LexemeType::RightParenthesis, ")"),
        ].into_iter()),
        Some(Some(8))
    );

    // 2 + 1 + 5 == 8
    assert_eq!(
        calc.translate(&mut c, vec![
            Lexeme::new(LexemeType::Integer, "2"),
            Lexeme::new(LexemeType::Plus, "+"),
            Lexeme::new(LexemeType::Integer, "1"),
            Lexeme::new(LexemeType::Plus, "+"),
            Lexeme::new(LexemeType::Integer, "5"),
        ].into_iter()),
        Some(Some(8))
    );

    // 10 - 5 + 1 == 6
    assert_eq!(
        calc.translate(&mut c, vec![
            Lexeme::new(LexemeType::Integer, "10"),
            Lexeme::new(LexemeType::Minus, "-"),
            Lexeme::new(LexemeType::Integer, "5"),
            Lexeme::new(LexemeType::Plus, "+"),
            Lexeme::new(LexemeType::Integer, "1"),
        ].into_iter()),
        Some(Some(6))
    );

    // 10 - (5 + 1) == 4
    assert_eq!(
        calc.translate(&mut c, vec![
            Lexeme::new(LexemeType::Integer, "10"),
            Lexeme::new(LexemeType::Minus, "-"),
            Lexeme::new(LexemeType::LeftParenthesis, "("),
            Lexeme::new(LexemeType::Integer, "5"),
            Lexeme::new(LexemeType::Plus, "+"),
            Lexeme::new(LexemeType::Integer, "1"),
            Lexeme::new(LexemeType::RightParenthesis, ")"),
        ].into_iter()),
        Some(Some(4))
    );

    // 6 * 8 / 2 == 24
    assert_eq!(
        calc.translate(&mut c, vec![
            Lexeme::new(LexemeType::Integer, "6"),
            Lexeme::new(LexemeType::Star, "*"),
            Lexeme::new(LexemeType::Integer, "8"),
            Lexeme::new(LexemeType::Slash, "/"),
            Lexeme::new(LexemeType::Integer, "2"),
        ].into_iter()),
        Some(Some(24))
    );

    // 6 + 2 * 5 == 16
    assert_eq!(
        calc.translate(&mut c, vec![
            Lexeme::new(LexemeType::Integer, "6"),
            Lexeme::new(LexemeType::Plus, "+"),
            Lexeme::new(LexemeType::Integer, "2"),
            Lexeme::new(LexemeType::Star, "*"),
            Lexeme::new(LexemeType::Integer, "5"),
        ].into_iter()),
        Some(Some(16))
    );

    // 6 * 2 + 5 == 17
    assert_eq!(
        calc.translate(&mut c, vec![
            Lexeme::new(LexemeType::Integer, "6"),
            Lexeme::new(LexemeType::Star, "*"),
            Lexeme::new(LexemeType::Integer, "2"),
            Lexeme::new(LexemeType::Plus, "+"),
            Lexeme::new(LexemeType::Integer, "5"),
        ].into_iter()),
        Some(Some(17))
    );

    assert_eq!(
        calc.translate(&mut c, vec![
            Lexeme::new(LexemeType::Integer, "6"),
            Lexeme::new(LexemeType::Star, "*"),
            Lexeme::new(LexemeType::LeftParenthesis, "("),
            Lexeme::new(LexemeType::Integer, "1"),
            Lexeme::new(LexemeType::Plus, "+"),
            Lexeme::new(LexemeType::Integer, "12"),
            Lexeme::new(LexemeType::Minus, "-"),
            Lexeme::new(LexemeType::LeftParenthesis, "("),
            Lexeme::new(LexemeType::Integer, "11"),
            Lexeme::new(LexemeType::Minus, "-"),
            Lexeme::new(LexemeType::Integer, "1"),
            Lexeme::new(LexemeType::RightParenthesis, ")"),
            Lexeme::new(LexemeType::RightParenthesis, ")"),
            Lexeme::new(LexemeType::Slash, "/"),
            Lexeme::new(LexemeType::LeftParenthesis, "("),
            Lexeme::new(LexemeType::Integer, "2"),
            Lexeme::new(LexemeType::RightParenthesis, ")"),
            Lexeme::new(LexemeType::Plus, "+"),
            Lexeme::new(LexemeType::Integer, "5"),
        ].into_iter()),
        Some(Some(14))
    );
}

#[test]
fn test_failing_calculator_translator() {
    let calc = build_calculator();
    let mut c = Context;

    // 8 + +
    assert_eq!(
        calc.translate(&mut c, vec![
            Lexeme::new(LexemeType::Integer, "8"),
            Lexeme::new(LexemeType::Plus, "+"),
            Lexeme::new(LexemeType::Plus, "+"),
        ].into_iter()),
        None,
    );

    // * 5
    assert_eq!(
        calc.translate(&mut c, vec![
            Lexeme::new(LexemeType::Star, "*"),
            Lexeme::new(LexemeType::Integer, "5"),
        ].into_iter()),
        None,
    );

    // 5 * (1))
    assert_eq!(
        calc.translate(&mut c, vec![
            Lexeme::new(LexemeType::Integer, "5"),
            Lexeme::new(LexemeType::Star, "*"),
            Lexeme::new(LexemeType::LeftParenthesis, "("),
            Lexeme::new(LexemeType::Integer, "1"),
            Lexeme::new(LexemeType::RightParenthesis, ")"),
            Lexeme::new(LexemeType::RightParenthesis, ")"),
        ].into_iter()),
        None,
    );

    // 5 * (1))
    assert_eq!(
        calc.translate(&mut c, vec![].into_iter()),
        None,
    );
}

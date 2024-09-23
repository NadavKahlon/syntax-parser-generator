use crate::handle::auto::AutomaticallyHandled;
use crate::parsing::lr_parser::rules::Associativity::Left;
use crate::parsing::lr_parser::rules::GrammarSymbol::{Nonterminal, Terminal};
use crate::parsing::translator::build::SyntaxDirectedTranslatorBuilder;
use crate::parsing::translator::SyntaxDirectedTranslator;

#[derive(Clone, Copy)]
enum T {
    Integer,
    Plus,
    Star,
    Slash,
    Minus,
    LeftParenthesis,
    RightParenthesis,
}

impl AutomaticallyHandled for T {
    type HandleCoreType = u8;
    fn serial(&self) -> usize { *self as usize }
}

fn build_calculator_translator() -> SyntaxDirectedTranslator<T, Option<i32>> {
    let mut builder = SyntaxDirectedTranslatorBuilder::new();

    let bindings = vec![
        builder.register_binding(
            vec![
                T::Star.handle(),
                T::Slash.handle(),
            ],
            Left,
        ),
        builder.register_binding(
            vec![
                T::Plus.handle(),
                T::Minus.handle(),
            ],
            Left,
        ),
    ];

    let expression = builder.new_nonterminal();
    builder.set_start_nonterminal(expression);

    builder.register_rule(
        expression,
        vec![Terminal(T::Integer.handle())],
        Box::new(|nums: Vec<Option<i32>>| Some(nums[0])),
    );

    builder.register_rule(
        expression,
        vec![
            Terminal(T::LeftParenthesis.handle()),
            Nonterminal(expression),
            Terminal(T::RightParenthesis.handle()),
        ],
        Box::new(|nums: Vec<Option<i32>>| Some(nums[1])),
    );

    builder.register_bound_rule(
        expression,
        vec![Nonterminal(expression), Terminal(T::Plus.handle()), Nonterminal(expression)],
        Box::new(|nums: Vec<Option<i32>>| Some(Some(nums[0].unwrap() + nums[2].unwrap()))),
        bindings[1],
    );

    builder.register_bound_rule(
        expression,
        vec![Nonterminal(expression), Terminal(T::Minus.handle()), Nonterminal(expression)],
        Box::new(|nums: Vec<Option<i32>>| Some(Some(nums[0].unwrap() - nums[2].unwrap()))),
        bindings[1],
    );

    builder.register_bound_rule(
        expression,
        vec![Nonterminal(expression), Terminal(T::Star.handle()), Nonterminal(expression)],
        Box::new(|nums: Vec<Option<i32>>| Some(Some(nums[0].unwrap() * nums[2].unwrap()))),
        bindings[0],
    );

    builder.register_bound_rule(
        expression,
        vec![Nonterminal(expression), Terminal(T::Slash.handle()), Nonterminal(expression)],
        Box::new(|nums: Vec<Option<i32>>| Some(Some(nums[0].unwrap() / nums[2].unwrap()))),
        bindings[0],
    );

    builder.build()
}

#[test]
fn test_successful_calculator_translator() {
    let calculator_translator = build_calculator_translator();

    // 8 == 8
    assert_eq!(
            calculator_translator.translate(vec![
                (T::Integer.handle(), Some(8)),
            ].into_iter()),
            Some(Some(8)),
        );

    // 8 + 5 == 13
    assert_eq!(
            calculator_translator.translate(vec![
                (T::Integer.handle(), Some(8)),
                (T::Plus.handle(), None),
                (T::Integer.handle(), Some(5)),
            ].into_iter()),
            Some(Some(13)),
        );

    // 8 * 5 = 40
    assert_eq!(
            calculator_translator.translate(vec![
                (T::Integer.handle(), Some(8)),
                (T::Star.handle(), None),
                (T::Integer.handle(), Some(5)),
            ].into_iter()),
            Some(Some(40)),
        );

    // 8 - 5 = 3
    assert_eq!(
            calculator_translator.translate(vec![
                (T::Integer.handle(), Some(8)),
                (T::Minus.handle(), None),
                (T::Integer.handle(), Some(5)),
            ].into_iter()),
            Some(Some(3)),
        );

    // 8 / 5 = 1
    assert_eq!(
            calculator_translator.translate(vec![
                (T::Integer.handle(), Some(8)),
                (T::Slash.handle(), None),
                (T::Integer.handle(), Some(5)),
            ].into_iter()),
            Some(Some(1)),
        );

    // (8) = 8
    assert_eq!(
            calculator_translator.translate(vec![
                (T::LeftParenthesis.handle(), None),
                (T::Integer.handle(), Some(8)),
                (T::RightParenthesis.handle(), None),
            ].into_iter()),
            Some(Some(8)),
        );

    // 2 + 1 + 5 = 8
    assert_eq!(
            calculator_translator.translate(vec![
                (T::Integer.handle(), Some(2)),
                (T::Plus.handle(), None),
                (T::Integer.handle(), Some(1)),
                (T::Plus.handle(), None),
                (T::Integer.handle(), Some(5)),
            ].into_iter()),
            Some(Some(8)),
        );

    // 10 - 5 + 1 = 6
    assert_eq!(
            calculator_translator.translate(vec![
                (T::Integer.handle(), Some(10)),
                (T::Minus.handle(), None),
                (T::Integer.handle(), Some(5)),
                (T::Plus.handle(), None),
                (T::Integer.handle(), Some(1)),
            ].into_iter()),
            Some(Some(6)),
        );

    // 10 - (5 + 1) = 4
    assert_eq!(
            calculator_translator.translate(vec![
                (T::Integer.handle(), Some(10)),
                (T::Minus.handle(), None),
                (T::LeftParenthesis.handle(), None),
                (T::Integer.handle(), Some(5)),
                (T::Plus.handle(), None),
                (T::Integer.handle(), Some(1)),
                (T::RightParenthesis.handle(), None),
            ].into_iter()),
            Some(Some(4)),
        );

    // 6 * 8 / 2 = 24
    assert_eq!(
            calculator_translator.translate(vec![
                (T::Integer.handle(), Some(6)),
                (T::Star.handle(), None),
                (T::Integer.handle(), Some(8)),
                (T::Slash.handle(), None),
                (T::Integer.handle(), Some(2)),
            ].into_iter()),
            Some(Some(24)),
        );

    // 6 + 2 * 5 = 16
    assert_eq!(
            calculator_translator.translate(vec![
                (T::Integer.handle(), Some(6)),
                (T::Plus.handle(), None),
                (T::Integer.handle(), Some(2)),
                (T::Star.handle(), None),
                (T::Integer.handle(), Some(5)),
            ].into_iter()),
            Some(Some(16)),
        );

    // 6 * 2 + 5 = 17
    assert_eq!(
            calculator_translator.translate(vec![
                (T::Integer.handle(), Some(6)),
                (T::Star.handle(), None),
                (T::Integer.handle(), Some(2)),
                (T::Plus.handle(), None),
                (T::Integer.handle(), Some(5)),
            ].into_iter()),
            Some(Some(17)),
        );

    // 6 * (1 + 12 - (11 - 1)) / (2) + 5 = 14
    assert_eq!(
            calculator_translator.translate(vec![
                (T::Integer.handle(), Some(6)),
                (T::Star.handle(), None),
                (T::LeftParenthesis.handle(), None),
                (T::Integer.handle(), Some(1)),
                (T::Plus.handle(), None),
                (T::Integer.handle(), Some(12)),
                (T::Minus.handle(), None),
                (T::LeftParenthesis.handle(), None),
                (T::Integer.handle(), Some(11)),
                (T::Minus.handle(), None),
                (T::Integer.handle(), Some(1)),
                (T::RightParenthesis.handle(), None),
                (T::RightParenthesis.handle(), None),
                (T::Slash.handle(), None),
                (T::LeftParenthesis.handle(), None),
                (T::Integer.handle(), Some(2)),
                (T::RightParenthesis.handle(), None),
                (T::Plus.handle(), None),
                (T::Integer.handle(), Some(5)),
            ].into_iter()),
            Some(Some(14)),
        );
}

#[test]
fn test_failing_translator() {
    let calculator_translator = build_calculator_translator();

    // 8 + +
    assert_eq!(
        calculator_translator.translate(vec![
            (T::Integer.handle(), Some(8)),
            (T::Plus.handle(), None),
            (T::Plus.handle(), None),
        ].into_iter()),
        None
    );

    // * 5
    assert_eq!(
            calculator_translator.translate(vec![
                (T::Star.handle(), None),
                (T::Integer.handle(), Some(5)),
            ].into_iter()),
            None,
        );

    // 5 * (1))
    assert_eq!(
            calculator_translator.translate(vec![
                (T::Integer.handle(), Some(5)),
                (T::Star.handle(), None),
                (T::LeftParenthesis.handle(), None),
                (T::Integer.handle(), Some(1)),
                (T::RightParenthesis.handle(), None),
                (T::RightParenthesis.handle(), None),
            ].into_iter()),
            None,
        );

    assert_eq!(
            calculator_translator.translate(vec![].into_iter()),
            None,
        );
}

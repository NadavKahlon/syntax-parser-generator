use crate::handle::{Handle, Handled};
use crate::handle::handled_vec::HandledVec;
use crate::parsing::lr_parser::execute::LrParserDecision;
use crate::parsing::lr_parser::LrParser;
use crate::parsing::translator::build::Nonterminal;
use crate::parsing::translator::atomic_translator::AtomicTranslator;

pub mod build;
mod atomic_translator;

pub struct SyntaxDirectedTranslator<Terminal, Satellite>
where
    Terminal: Handled,
{
    lr_parser: LrParser<Terminal, Nonterminal, AtomicTranslator<Satellite>>,
    atomic_translators: HandledVec<AtomicTranslator<Satellite>>,
}

impl<Terminal, Satellite> SyntaxDirectedTranslator<Terminal, Satellite>
where
    Terminal: Handled,
{
    pub fn translate(
        &self, mut stream: impl Iterator<Item=(Handle<Terminal>, Satellite)>,
    ) -> Option<Satellite>
    {
        let mut satellite_stack: Vec<Satellite> = Vec::new();
        let mut lr_parser_execution = self.lr_parser.new_execution();
        let mut next_input_pair: Option<(Handle<Terminal>, Satellite)> = None;

        loop {
            next_input_pair = match next_input_pair {
                None => {
                    match stream.next() {
                        None => break,
                        Some(input_pair) => Some(input_pair),
                    }
                }

                Some((terminal, satellite)) => {
                    match lr_parser_execution.decide(terminal)? {
                        LrParserDecision::Shift => {
                            satellite_stack.push(satellite);
                            None
                        },
                        LrParserDecision::Reduce { size, tag } => {
                            if satellite_stack.len() < size {
                                // Satellite stack is too short for rule
                                return None;
                            }
                            let rhs_satellites = satellite_stack
                                .drain((satellite_stack.len() - size)..).collect();
                            let lhs_satellite =
                                self.atomic_translators[tag].translate(rhs_satellites);
                            satellite_stack.push(lhs_satellite);

                            // Do not reset next_input_pair as Reduce does not consume input
                            Some((terminal, satellite))
                        }
                    }
                }
            }
        }

        // We reach here on input exhaustion
        if (satellite_stack.len() == 1) && lr_parser_execution.finalize() {
            satellite_stack.pop()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::handle::auto::AutomaticallyHandled;
    use crate::parsing::lr_parser::rules::Associativity::Left;
    use crate::parsing::lr_parser::rules::Symbol::{Nonterminal, Terminal};
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
            vec![Nonterminal(expression), Terminal(T::Plus.handle()), Nonterminal(expression)],
            Some(bindings[1]),
            Box::new(|nums: Vec<Option<i32>>| Some(nums[0].unwrap() + nums[2].unwrap())),
        );

        builder.register_rule(
            expression,
            vec![Nonterminal(expression), Terminal(T::Minus.handle()), Nonterminal(expression)],
            Some(bindings[1]),
            Box::new(|nums: Vec<Option<i32>>| Some(nums[0].unwrap() - nums[2].unwrap())),
        );

        builder.register_rule(
            expression,
            vec![Nonterminal(expression), Terminal(T::Star.handle()), Nonterminal(expression)],
            Some(bindings[0]),
            Box::new(|nums: Vec<Option<i32>>| Some(nums[0].unwrap() * nums[2].unwrap())),
        );

        builder.register_rule(
            expression,
            vec![Nonterminal(expression), Terminal(T::Slash.handle()), Nonterminal(expression)],
            Some(bindings[0]),
            Box::new(|nums: Vec<Option<i32>>| Some(nums[0].unwrap() / nums[2].unwrap())),
        );

        builder.build()
    }

    #[test]
    fn test_successful_calculator_translator() {
        let calculator_translator = build_calculator_translator();

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
            Some(Some(24)),
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
}
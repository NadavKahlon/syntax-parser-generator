use crate::handle::{Handle, Handled};
use crate::parser::{LrParser, LrParserAction, LrParserState};
use crate::parser::LrParserAction::Accept;

#[derive(Debug)]
pub enum LrParserDecision<ProductionRule>
where
    ProductionRule: Handled,
{
    Shift,
    Reduce(Handle<ProductionRule>),
    Accept,
}

impl<ProductionRule> PartialEq<Self> for LrParserDecision<ProductionRule>
where
    ProductionRule: Handled,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LrParserDecision::Shift, LrParserDecision::Shift) => true,
            (LrParserDecision::Accept, LrParserDecision::Accept) => true,
            (
                LrParserDecision::Reduce(prod1),
                LrParserDecision::Reduce(prod2)
            ) => prod1 == prod2,
            (_, _) => false,
        }
    }
}

pub struct LrParserExecution<'a, Terminal, Nonterminal, ProductionRule>
where
    Nonterminal: Handled,
    Terminal: Handled,
    ProductionRule: Handled,
{
    machine: &'a LrParser<Terminal, Nonterminal, ProductionRule>,
    stack: Vec<Handle<LrParserState<Terminal, Nonterminal, ProductionRule>>>,
}

impl<'a, Terminal, Nonterminal, ProductionRule>
LrParserExecution<'a, Terminal, Nonterminal, ProductionRule>
where
    Nonterminal: Handled,
    Terminal: Handled,
    ProductionRule: Handled,
{
    pub(super) fn new(machine: &'a LrParser<Terminal, Nonterminal, ProductionRule>)
                      -> LrParserExecution<Terminal, Nonterminal, ProductionRule>
    {
        let initial_state = machine.initial_state.expect(
            "Cannot create an execution environment for an LrParser with no dedicated initial \
            state"
        );
        Self {
            machine,
            stack: vec![initial_state],
        }
    }

    pub fn decide(&mut self, terminal: Handle<Terminal>) -> Option<LrParserDecision<ProductionRule>> {
        match self.machine.states[*self.stack.last()?].action_map.get(terminal)? {
            LrParserAction::Shift(state) => {
                self.stack.push(*state);
                Some(LrParserDecision::Shift)
            }

            LrParserAction::Reduce {
                size,
                nonterminal,
                production
            } => {
                self.stack.truncate(self.stack.len() - size);
                let current_last_state = &self.machine.states[*self.stack.last()?];
                let new_state =
                    current_last_state.goto_map.get(*nonterminal)?;
                self.stack.push(*new_state);
                Some(LrParserDecision::Reduce(*production))
            }

            Accept => Some(LrParserDecision::Accept)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::handle::auto::AutomaticallyHandled;
    use crate::parser::execution::tests::Production::{Sum, EIsT, Mult, TIsF, Paren, FIsId};
    use crate::parser::LrParserAction::{Reduce, Shift};
    use super::*;

    #[derive(Copy, Clone)]
    enum Terminal { Id, Plus, Star, LeftParen, RightParen, Dollar }
    impl AutomaticallyHandled for Terminal {
        type HandleCoreType = u8;
        fn serial(&self) -> usize { *self as usize }
    }

    #[derive(Copy, Clone)]
    enum Nonterminal { E, T, F }
    impl AutomaticallyHandled for Nonterminal {
        type HandleCoreType = u16;
        fn serial(&self) -> usize { *self as usize }
    }

    #[derive(Debug, Copy, Clone)]
    enum Production {
        Sum,
        EIsT,
        Mult,
        TIsF,
        Paren,
        FIsId,
    }
    impl AutomaticallyHandled for Production {
        type HandleCoreType = u8;
        fn serial(&self) -> usize { *self as usize }
    }

    fn create_parser() -> LrParser<Terminal, Nonterminal, Production> {
        // Based on the dragon book, page 252

        let mut parser = LrParser::new();
        let states: Vec<Handle<LrParserState<Terminal, Nonterminal, Production>>>
            = (0..12).map(|_| parser.new_state()).collect();

        let reductions: Vec<LrParserAction<Terminal, Nonterminal, Production>> = vec![
            Reduce { size: 3, nonterminal: Nonterminal::E.handle(), production: Sum.handle() },
            Reduce { size: 1, nonterminal: Nonterminal::E.handle(), production: EIsT.handle() },
            Reduce { size: 3, nonterminal: Nonterminal::T.handle(), production: Mult.handle() },
            Reduce { size: 1, nonterminal: Nonterminal::T.handle(), production: TIsF.handle() },
            Reduce { size: 3, nonterminal: Nonterminal::F.handle(), production: Paren.handle() },
            Reduce { size: 1, nonterminal: Nonterminal::F.handle(), production: FIsId.handle() },
        ];

        parser.set_action(states[0], Terminal::Id.handle(), Shift(states[5]));
        parser.set_action(states[0], Terminal::LeftParen.handle(), Shift(states[4]));
        parser.set_action(states[1], Terminal::Plus.handle(), Shift(states[6]));
        parser.set_action(states[1], Terminal::Dollar.handle(), Accept);
        parser.set_action(states[2], Terminal::Plus.handle(), reductions[1]);
        parser.set_action(states[2], Terminal::Star.handle(), Shift(states[7]));
        parser.set_action(states[2], Terminal::LeftParen.handle(), reductions[1]);
        parser.set_action(states[2], Terminal::Dollar.handle(), reductions[1]);
        parser.set_action(states[3], Terminal::Plus.handle(), reductions[3]);
        parser.set_action(states[3], Terminal::Star.handle(), reductions[3]);
        parser.set_action(states[3], Terminal::LeftParen.handle(), reductions[3]);
        parser.set_action(states[3], Terminal::Dollar.handle(), reductions[3]);
        parser.set_action(states[4], Terminal::Id.handle(), Shift(states[5]));
        parser.set_action(states[4], Terminal::LeftParen.handle(), Shift(states[4]));
        parser.set_action(states[5], Terminal::Plus.handle(), reductions[5]);
        parser.set_action(states[5], Terminal::Star.handle(), reductions[5]);
        parser.set_action(states[5], Terminal::RightParen.handle(), reductions[5]);
        parser.set_action(states[5], Terminal::Dollar.handle(), reductions[5]);
        parser.set_action(states[6], Terminal::Id.handle(), Shift(states[5]));
        parser.set_action(states[6], Terminal::LeftParen.handle(), Shift(states[4]));
        parser.set_action(states[7], Terminal::Id.handle(), Shift(states[5]));
        parser.set_action(states[7], Terminal::LeftParen.handle(), Shift(states[4]));
        parser.set_action(states[8], Terminal::Plus.handle(), Shift(states[6]));
        parser.set_action(states[8], Terminal::RightParen.handle(), Shift(states[11]));
        parser.set_action(states[9], Terminal::Plus.handle(), reductions[0]);
        parser.set_action(states[9], Terminal::Star.handle(), Shift(states[7]));
        parser.set_action(states[9], Terminal::RightParen.handle(), reductions[0]);
        parser.set_action(states[9], Terminal::Dollar.handle(), reductions[0]);
        parser.set_action(states[10], Terminal::Plus.handle(), reductions[2]);
        parser.set_action(states[10], Terminal::Star.handle(), reductions[2]);
        parser.set_action(states[10], Terminal::RightParen.handle(), reductions[2]);
        parser.set_action(states[10], Terminal::Dollar.handle(), reductions[2]);
        parser.set_action(states[11], Terminal::Plus.handle(), reductions[4]);
        parser.set_action(states[11], Terminal::Star.handle(), reductions[4]);
        parser.set_action(states[11], Terminal::RightParen.handle(), reductions[4]);
        parser.set_action(states[11], Terminal::Dollar.handle(), reductions[4]);

        parser.set_goto(states[0], Nonterminal::E.handle(), states[1]);
        parser.set_goto(states[0], Nonterminal::T.handle(), states[2]);
        parser.set_goto(states[0], Nonterminal::F.handle(), states[3]);
        parser.set_goto(states[4], Nonterminal::E.handle(), states[8]);
        parser.set_goto(states[4], Nonterminal::T.handle(), states[2]);
        parser.set_goto(states[4], Nonterminal::F.handle(), states[3]);
        parser.set_goto(states[6], Nonterminal::T.handle(), states[9]);
        parser.set_goto(states[6], Nonterminal::F.handle(), states[3]);
        parser.set_goto(states[7], Nonterminal::F.handle(), states[10]);

        parser.dedicate_initial_state(states[0]);

        parser
    }

    #[test]
    fn test_expressions_parser() {
        let parser = create_parser();
        let mut execution = parser.new_execution();

        assert_eq!(
            execution.decide(Terminal::Id.handle()),
            Some(LrParserDecision::Shift)
        );
        assert_eq!(
            execution.decide(Terminal::Star.handle()),
            Some(LrParserDecision::Reduce(FIsId.handle())),
        );
        assert_eq!(
            execution.decide(Terminal::Star.handle()),
            Some(LrParserDecision::Reduce(TIsF.handle())),
        );
        assert_eq!(
            execution.decide(Terminal::Star.handle()),
            Some(LrParserDecision::Shift),
        );
        assert_eq!(
            execution.decide(Terminal::Id.handle()),
            Some(LrParserDecision::Shift),
        );
        assert_eq!(
            execution.decide(Terminal::Plus.handle()),
            Some(LrParserDecision::Reduce(FIsId.handle())),
        );
        assert_eq!(
            execution.decide(Terminal::Plus.handle()),
            Some(LrParserDecision::Reduce(Mult.handle())),
        );
        assert_eq!(
            execution.decide(Terminal::Plus.handle()),
            Some(LrParserDecision::Reduce(EIsT.handle())),
        );
        assert_eq!(
            execution.decide(Terminal::Plus.handle()),
            Some(LrParserDecision::Shift),
        );
        assert_eq!(
            execution.decide(Terminal::Id.handle()),
            Some(LrParserDecision::Shift),
        );
        assert_eq!(
            execution.decide(Terminal::Dollar.handle()),
            Some(LrParserDecision::Reduce(FIsId.handle())),
        );
        assert_eq!(
            execution.decide(Terminal::Dollar.handle()),
            Some(LrParserDecision::Reduce(TIsF.handle())),
        );
        assert_eq!(
            execution.decide(Terminal::Dollar.handle()),
            Some(LrParserDecision::Reduce(Sum.handle())),
        );
        assert_eq!(
            execution.decide(Terminal::Dollar.handle()),
            Some(LrParserDecision::Accept),
        );
        assert_eq!(
            execution.decide(Terminal::Dollar.handle()),
            Some(LrParserDecision::Accept),
        );
    }
}

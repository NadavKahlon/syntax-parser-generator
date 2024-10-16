use crate::handles::{Handle, Handled};
use crate::parsing::lr_parser::{LrParser, LrParserAction, LrParserState};

#[derive(Debug)]
pub enum LrParserDecision<Tag>
where
    Tag: Handled,
{
    Shift,
    Reduce {
        size: usize,
        tag: Handle<Tag>,
    },
}

pub enum FinalDecision<Tag>
where
    Tag: Handled,
{
    Accept,
    Reduce {
        size: usize,
        tag: Handle<Tag>,
    },
}

impl<Tag> PartialEq<Self> for LrParserDecision<Tag>
where
    Tag: Handled,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LrParserDecision::Shift, LrParserDecision::Shift) => true,
            (
                LrParserDecision::Reduce { size: size_1, tag: tag_1 },
                LrParserDecision::Reduce { size: size_2, tag: tag_2 }
            ) => (size_1 == size_2) && (tag_1 == tag_2),
            (_, _) => false,
        }
    }
}

pub struct LrParserExecution<'a, Terminal, Nonterminal, Tag>
where
    Nonterminal: Handled,
    Terminal: Handled,
    Tag: Handled,
{
    machine: &'a LrParser<Terminal, Nonterminal, Tag>,
    stack: Vec<Handle<LrParserState<Terminal, Nonterminal, Tag>>>,
    end_of_input_marker: Handle<Terminal>,
}

impl<'a, Terminal, Nonterminal, Tag>
LrParserExecution<'a, Terminal, Nonterminal, Tag>
where
    Nonterminal: Handled,
    Terminal: Handled,
    Tag: Handled,
{
    pub(super) fn new(machine: &'a LrParser<Terminal, Nonterminal, Tag>)
                      -> LrParserExecution<Terminal, Nonterminal, Tag>
    {
        let initial_state = machine.initial_state.expect(
            "Cannot create an execution environment for an LrParser with no dedicated initial \
            state"
        );
        let end_of_input_marker = machine.end_of_input_marker.expect(
            "Cannot create an execution environment for an LrParser with no dedicated end-of-\
            input terminal symbol"
        );
        Self {
            machine,
            stack: vec![initial_state],
            end_of_input_marker,
        }
    }

    pub fn decide(&mut self, terminal: Handle<Terminal>) -> Option<LrParserDecision<Tag>> {
        match self.decide_internal(terminal)? {
            LrParserInternalDecision::Continue(decision) => Some(decision),
            LrParserInternalDecision::Accept => {
                panic!(
                    "LR parser should not have decided to accept on client's input, as it should \
                    only accept on the end-of-marker terminal (mock handles) only known internally \
                    to the parser"
                )
            }
        }
    }

    pub fn finalize(&mut self) -> bool {
        loop {
            match self.decide_internal(self.end_of_input_marker) {
                None => return false,
                Some(LrParserInternalDecision::<Tag>::Accept) => return true,
                _ => {}
            }
        }
    }

    pub fn decide_final(&mut self) -> Option<FinalDecision<Tag>> {
        match self.decide_internal(self.end_of_input_marker)? {
            LrParserInternalDecision::Continue(decision) => match decision {
                LrParserDecision::Shift => panic!("End of input marker shouldn't be shifted"),
                LrParserDecision::Reduce { size, tag } => {
                    Some(FinalDecision::Reduce { size, tag })
                }
            }
            LrParserInternalDecision::Accept => Some(FinalDecision::Accept),
        }
    }

    fn decide_internal(&mut self, terminal: Handle<Terminal>)
                       -> Option<LrParserInternalDecision<Tag>>
    {
        match self.machine.states[*self.stack.last()?].action_map.get(terminal)? {
            LrParserAction::Shift(state) => {
                self.stack.push(*state);
                Some(LrParserInternalDecision::Continue(LrParserDecision::Shift))
            }

            LrParserAction::Reduce {
                size,
                nonterminal,
                tag
            } => {
                self.stack.truncate(self.stack.len() - size);
                let current_last_state = &self.machine.states[*self.stack.last()?];
                let new_state =
                    current_last_state.goto_map.get(*nonterminal)?;
                self.stack.push(*new_state);
                Some(LrParserInternalDecision::Continue(
                    LrParserDecision::Reduce { size: *size, tag: *tag }
                ))
            }

            LrParserAction::Accept => Some(LrParserInternalDecision::Accept),
        }
    }
}

pub enum LrParserInternalDecision<Tag>
where
    Tag: Handled,
{
    Continue(LrParserDecision<Tag>),
    Accept,
}

#[cfg(test)]
mod tests {
    use crate::handles::specials::AutomaticallyHandled;
    use crate::parsing::lr_parser::{LrParser, LrParserAction, LrParserState};
    use crate::parsing::lr_parser::execute::tests::Production::{EIsT, FIsId, Mult, Paren, Sum, TIsF};
    use crate::parsing::lr_parser::LrParserAction::{Accept, Reduce, Shift};
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
            Reduce { size: 3, nonterminal: Nonterminal::E.handle(), tag: Sum.handle() },
            Reduce { size: 1, nonterminal: Nonterminal::E.handle(), tag: EIsT.handle() },
            Reduce { size: 3, nonterminal: Nonterminal::T.handle(), tag: Mult.handle() },
            Reduce { size: 1, nonterminal: Nonterminal::T.handle(), tag: TIsF.handle() },
            Reduce { size: 3, nonterminal: Nonterminal::F.handle(), tag: Paren.handle() },
            Reduce { size: 1, nonterminal: Nonterminal::F.handle(), tag: FIsId.handle() },
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

        parser.set_initial_state(states[0]);
        parser.set_end_of_input_marker(Terminal::Dollar.handle());

        parser
    }

    #[test]
    fn test_successful() {
        let parser = create_parser();
        let mut execution = parser.new_execution();

        assert_eq!(
            execution.decide(Terminal::Id.handle()),
            Some(LrParserDecision::Shift)
        );
        assert_eq!(
            execution.decide(Terminal::Star.handle()),
            Some(LrParserDecision::Reduce { size: 1, tag: FIsId.handle() }),
        );
        assert_eq!(
            execution.decide(Terminal::Star.handle()),
            Some(LrParserDecision::Reduce { size: 1, tag: TIsF.handle() }),
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
            Some(LrParserDecision::Reduce { size: 1, tag: FIsId.handle() }),
        );
        assert_eq!(
            execution.decide(Terminal::Plus.handle()),
            Some(LrParserDecision::Reduce { size: 3, tag: Mult.handle() }),
        );
        assert_eq!(
            execution.decide(Terminal::Plus.handle()),
            Some(LrParserDecision::Reduce { size: 1, tag: EIsT.handle() }),
        );
        assert_eq!(
            execution.decide(Terminal::Plus.handle()),
            Some(LrParserDecision::Shift),
        );
        assert_eq!(
            execution.decide(Terminal::Id.handle()),
            Some(LrParserDecision::Shift),
        );
        assert_eq!(execution.finalize(), true);
    }

    #[test]
    fn test_failing() {
        let parser = create_parser();
        let mut execution = parser.new_execution();

        assert_eq!(
            execution.decide(Terminal::Id.handle()),
            Some(LrParserDecision::Shift)
        );
        assert_eq!(
            execution.decide(Terminal::Star.handle()),
            Some(LrParserDecision::Reduce { size: 1, tag: FIsId.handle() }),
        );
        assert_eq!(
            execution.decide(Terminal::Star.handle()),
            Some(LrParserDecision::Reduce { size: 1, tag: TIsF.handle() }),
        );
        assert_eq!(
            execution.decide(Terminal::Star.handle()),
            Some(LrParserDecision::Shift),
        );
        assert_eq!(execution.finalize(), false);
    }
}

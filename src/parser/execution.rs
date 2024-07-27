use crate::handle::{Handle, Handled};
use crate::parser::{LrParser, LrParserAction, LrParserState};
use crate::parser::LrParserAction::Accept;

enum LrParserDecision<ProductionRule>
where
    ProductionRule: Handled,
{
    Shift,
    Reduce(Handle<ProductionRule>),
    Accept,
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
        Self {
            machine,
            stack: vec![machine.initial_state],
        }
    }

    pub fn feed(&mut self, terminal: Handle<Terminal>) -> Option<LrParserDecision<ProductionRule>> {
        match self.machine.states[*self.stack.last()?].action_map.get(terminal)? {
            LrParserAction::Shift(state) => {
                self.stack.push(*state);
                Some(LrParserDecision::Shift)
            }

            LrParserAction::Reduce {
                substitution_size,
                substituted_nonterminal,
                production
            } => {
                self.stack.truncate(self.stack.len() - substitution_size);
                let current_last_state = &self.machine.states[*self.stack.last()?];
                let new_state =
                    current_last_state.goto_map.get(*substituted_nonterminal)?;
                self.stack.push(*new_state);
                Some(LrParserDecision::Reduce(*production))
            }

            Accept => Some(LrParserDecision::Accept)
        }
    }
}

// TODO add test
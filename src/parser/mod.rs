mod execution;

use crate::handle::handle_map::HandleMap;
use crate::handle::{Handle, Handled};
use crate::handle::handled_vec::HandledVec;
use crate::parser::execution::LrParserExecution;

#[derive(Clone, Copy)]
enum LrParserAction<Terminal, Nonterminal, ProductionRule>
where
    Terminal: Handled,
    Nonterminal: Handled,
    ProductionRule: Handled,
{
    Shift(Handle<LrParserState<Terminal, Nonterminal, ProductionRule>>),
    Reduce {
        size: usize,
        nonterminal: Handle<Nonterminal>,
        production: Handle<ProductionRule>,
    },
    Accept,
}

struct LrParserState<Terminal, Nonterminal, ProductionRule>
where
    Terminal: Handled,
    Nonterminal: Handled,
    ProductionRule: Handled,
{
    // Note that this current implementation does not utilize locality in space, since each
    // HandleMap is a vector by itself (and so the maps are not a successive in memory)
    action_map: HandleMap<Terminal, LrParserAction<Terminal, Nonterminal, ProductionRule>>,
    goto_map: HandleMap<Nonterminal, Handle<LrParserState<Terminal, Nonterminal, ProductionRule>>>,
}

impl<Terminal, Nonterminal, ProductionRule> LrParserState<Terminal, Nonterminal, ProductionRule>
where
    Terminal: Handled,
    Nonterminal: Handled,
    ProductionRule: Handled,
{
    fn new() -> Self {
        Self {
            action_map: HandleMap::new(),
            goto_map: HandleMap::new(),
        }
    }
}

impl<Terminal, Nonterminal, ProductionRule> Handled
for LrParserState<Terminal, Nonterminal, ProductionRule>
where
    Terminal: Handled,
    Nonterminal: Handled,
    ProductionRule: Handled,
{
    type HandleCoreType = u16;
}

struct LrParser<Terminal, Nonterminal, ProductionRule>
where
    Terminal: Handled,
    Nonterminal: Handled,
    ProductionRule: Handled,
{
    states: HandledVec<LrParserState<Terminal, Nonterminal, ProductionRule>>,
    initial_state: Option<Handle<LrParserState<Terminal, Nonterminal, ProductionRule>>>,
}

impl<Terminal, Nonterminal, ProductionRule> LrParser<Terminal, Nonterminal, ProductionRule>
where
    Terminal: Handled,
    Nonterminal: Handled,
    ProductionRule: Handled,
{
    fn new() -> Self {
        Self {
            states: HandledVec::new(),
            initial_state: None,
        }
    }

    fn new_state(&mut self) -> Handle<LrParserState<Terminal, Nonterminal, ProductionRule>> {
        self.states.insert(LrParserState::new())
    }

    fn set_action(
        &mut self, state: Handle<LrParserState<Terminal, Nonterminal, ProductionRule>>,
        terminal: Handle<Terminal>, action: LrParserAction<Terminal, Nonterminal, ProductionRule>,
    ) {
        self.states[state].action_map.insert(terminal, action);
    }

    fn set_goto(
        &mut self, state: Handle<LrParserState<Terminal, Nonterminal, ProductionRule>>,
        nonterminal: Handle<Nonterminal>,
        target: Handle<LrParserState<Terminal, Nonterminal, ProductionRule>>,
    ) {
        self.states[state].goto_map.insert(nonterminal, target);
    }

    fn new_execution(&self) -> LrParserExecution<Terminal, Nonterminal, ProductionRule> {
        LrParserExecution::new(self)
    }


    fn dedicate_initial_state(
        &mut self, state: Handle<LrParserState<Terminal, Nonterminal, ProductionRule>>
    ) {
        self.initial_state = Some(state)
    }
}
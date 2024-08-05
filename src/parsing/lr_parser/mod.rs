pub mod execute;
pub mod build;
pub mod rules;

use crate::handle::{Handle, Handled};
use crate::handle::handle_map::HandleMap;
use crate::handle::handled_vec::HandledVec;
use crate::parsing::lr_parser::execute::LrParserExecution;

#[derive(Clone, Copy)]
enum LrParserAction<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: Handled,
{
    Shift(Handle<LrParserState<Terminal, Nonterminal, Tag>>),
    Reduce {
        size: usize,
        nonterminal: Handle<Nonterminal>,
        tag: Handle<Tag>,
    },
    Accept,
}

struct LrParserState<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: Handled,
{
    // Note that this current implementation does not utilize locality in space, since each
    // HandleMap is a vector by itself (and so the maps are not a successive in memory)
    action_map: HandleMap<Terminal, LrParserAction<Terminal, Nonterminal, Tag>>,
    goto_map: HandleMap<Nonterminal, Handle<LrParserState<Terminal, Nonterminal, Tag>>>,
}

impl<Terminal, Nonterminal, Tag> LrParserState<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: Handled,
{
    fn new() -> Self {
        Self {
            action_map: HandleMap::new(),
            goto_map: HandleMap::new(),
        }
    }
}

impl<Terminal, Nonterminal, Tag> Handled
for LrParserState<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: Handled,
{
    type HandleCoreType = u16;
}

pub struct LrParser<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: Handled,
{
    states: HandledVec<LrParserState<Terminal, Nonterminal, Tag>>,
    initial_state: Option<Handle<LrParserState<Terminal, Nonterminal, Tag>>>,
    end_of_input_marker: Option<Handle<Terminal>>,
}

impl<Terminal, Nonterminal, Tag> LrParser<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: Handled,
{
    fn new() -> Self {
        Self {
            states: HandledVec::new(),
            initial_state: None,
            end_of_input_marker: None,
        }
    }

    fn new_state(&mut self) -> Handle<LrParserState<Terminal, Nonterminal, Tag>> {
        self.states.insert(LrParserState::new())
    }

    fn set_action(
        &mut self, state: Handle<LrParserState<Terminal, Nonterminal, Tag>>,
        terminal: Handle<Terminal>, action: LrParserAction<Terminal, Nonterminal, Tag>,
    ) {
        self.states[state].action_map.insert(terminal, action);
    }

    fn set_goto(
        &mut self, state: Handle<LrParserState<Terminal, Nonterminal, Tag>>,
        nonterminal: Handle<Nonterminal>,
        target: Handle<LrParserState<Terminal, Nonterminal, Tag>>,
    ) {
        self.states[state].goto_map.insert(nonterminal, target);
    }

    pub fn new_execution(&self) -> LrParserExecution<Terminal, Nonterminal, Tag> {
        LrParserExecution::new(self)
    }

    fn set_initial_state(
        &mut self, state: Handle<LrParserState<Terminal, Nonterminal, Tag>>
    ) {
        self.initial_state = Some(state)
    }

    fn set_end_of_input_marker(&mut self, terminal: Handle<Terminal>) {
        self.end_of_input_marker = Some(terminal)
    }
}
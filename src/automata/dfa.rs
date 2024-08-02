use std::collections::HashSet;
use std::fmt::Debug;
use derive_where::derive_where;
use crate::handle::handle_map::HandleMap;
use crate::handle::{Handle, Handled};
use crate::handle::handled_vec::HandledVec;


#[derive_where(Debug, PartialEq, Eq; Label)]
pub struct DfaState<Symbol, Label>
where
    Symbol: Handled,
{
    pub(super) transitions: HandleMap<Symbol, Handle<DfaState<Symbol, Label>>>,
    label: Option<Label>,
}

impl<Symbol, Label> DfaState<Symbol, Label>
where
    Symbol: Handled,
{
    fn new() -> Self {
        Self {
            transitions: HandleMap::new(),
            label: None,
        }
    }
}

impl<Symbol, Label> Handled for DfaState<Symbol, Label>
where
    Symbol: Handled,
{ type HandleCoreType = u16; }

#[derive_where(Debug; Label: Debug)]
#[derive_where(PartialEq; Label: PartialEq)]
#[derive_where(Eq; Label: Eq)]
pub struct Dfa<Symbol, Label>
where
    Symbol: Handled,
{
    pub(super) states: HandledVec<DfaState<Symbol, Label>>,  // TODO space efficienct
    pub(super) initial_state: Option<Handle<DfaState<Symbol, Label>>>,
}

impl<Symbol, Label> Dfa<Symbol, Label>
where
    Symbol: Handled,
{
    pub fn new() -> Self {
        Self {
            states: HandledVec::new(),
            initial_state: None,
        }
    }

    pub fn new_state(&mut self) -> Handle<DfaState<Symbol, Label>> {
        self.states.insert(DfaState::new())
    }

    pub(super) fn list_states(&self) -> impl Iterator<Item=Handle<DfaState<Symbol, Label>>> {
        self.states.list_handles()
    }

    pub(super) fn list_symbols(&self) -> impl Iterator<Item=Handle<Symbol>> {
        let mut symbols: HashSet<Handle<Symbol>> = HashSet::new();
        for state in &self.states {
            symbols.extend(state.transitions.keys());
        }
        symbols.into_iter()
    }

    pub fn set_initial_state(&mut self, initial_state: Handle<DfaState<Symbol, Label>>) {
        self.initial_state = Some(initial_state);
    }

    pub fn get_initial_state(&self) -> Option<Handle<DfaState<Symbol, Label>>> {
        self.initial_state
    }

    pub fn link(
        &mut self, src: Handle<DfaState<Symbol, Label>>, dst: Handle<DfaState<Symbol, Label>>,
        symbol: Handle<Symbol>,
    ) {
        self.states[src].transitions.insert(symbol, dst);
    }

    pub fn label(&mut self, state: Handle<DfaState<Symbol, Label>>, label: Option<Label>) {
        self.states[state].label = label;
    }

    pub fn get_label(&self, state: Handle<DfaState<Symbol, Label>>) -> &Option<Label> {
        &self.states[state].label
    }

    pub fn step(
        &self, src: Handle<DfaState<Symbol, Label>>, symbol: Handle<Symbol>,
    ) -> Option<Handle<DfaState<Symbol, Label>>> {
        self.states[src].transitions.get(symbol).copied()
    }

    pub fn scan(
        &self, stream: impl Iterator<Item=Handle<Symbol>>,
    ) -> Option<Handle<DfaState<Symbol, Label>>> {
        stream
            .fold(
                self.initial_state,
                |optional_state, symbol| {
                    match optional_state {
                        None => None,
                        Some(state) => self.step(state, symbol)
                    }
                },
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handle::auto::AutomaticallyHandled;

    #[derive(Clone, Copy)]
    enum Symbol {
        Symbol0,
        Symbol1,
    }
    impl AutomaticallyHandled for Symbol {
        type HandleCoreType = u8;
        fn serial(&self) -> usize { *self as usize }
    }

    fn build_test_data_1() -> (Dfa<Symbol, u32>, Vec<Handle<DfaState<Symbol, u32>>>) {
        let mut dfa = Dfa::new();
        let states = vec![dfa.new_state(), dfa.new_state()];

        dfa.link(states[0], states[0], Symbol::Symbol0.handle());
        dfa.link(states[0], states[1], Symbol::Symbol1.handle());
        dfa.link(states[1], states[0], Symbol::Symbol1.handle());
        dfa.link(states[1], states[1], Symbol::Symbol0.handle());

        dfa.set_initial_state(states[0]);

        (dfa, states)
    }

    #[test]
    fn test_scan_1() {
        let (dfa, states) = build_test_data_1();
        let stream = vec![
            Symbol::Symbol1.handle(),
            Symbol::Symbol0.handle(),
            Symbol::Symbol0.handle(),
            Symbol::Symbol1.handle(),
        ];
        assert_eq!(dfa.scan(stream.into_iter()), Some(states[0]))
    }

    #[test]
    fn test_scan_2() {
        let (dfa, states) = build_test_data_1();
        let input_string = vec![
            Symbol::Symbol1.handle(),
            Symbol::Symbol0.handle(),
            Symbol::Symbol0.handle(),
            Symbol::Symbol1.handle(),
            Symbol::Symbol1.handle(),
        ];
        assert_eq!(dfa.scan(input_string.into_iter()), Some(states[1]))
    }

    fn build_test_data_2() -> (Dfa<Symbol, u32>, Vec<Handle<DfaState<Symbol, u32>>>) {
        let mut dfa = Dfa::new();
        let states = vec![dfa.new_state(), dfa.new_state()];

        dfa.link(states[0], states[0], Symbol::Symbol0.handle());
        dfa.link(states[0], states[1], Symbol::Symbol1.handle());
        dfa.link(states[1], states[1], Symbol::Symbol0.handle());

        dfa.set_initial_state(states[0]);

        (dfa, states)
    }

    #[test]
    fn test_dead_route() {
        let (dfa, states) = build_test_data_2();
        let input_string_good = vec![
            Symbol::Symbol0.handle(),
            Symbol::Symbol0.handle(),
            Symbol::Symbol1.handle(),
        ];
        let input_string_bad = vec![
            Symbol::Symbol0.handle(),
            Symbol::Symbol0.handle(),
            Symbol::Symbol1.handle(),
            Symbol::Symbol1.handle(),
        ];
        assert_eq!(dfa.scan(input_string_good.into_iter()), Some(states[1]));
        assert_eq!(dfa.scan(input_string_bad.into_iter()), None);
    }
}


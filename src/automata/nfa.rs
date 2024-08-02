use std::collections::HashSet;
use crate::handle::{Handle, Handled};
use crate::handle::handle_bit_set::HandleBitSet;
use crate::handle::handle_map::HandleMap;
use crate::handle::handled_vec::HandledVec;

pub struct NfaState<Symbol, Label>
where
    Symbol: Handled,
{
    epsilon_transitions: HashSet<Handle<NfaState<Symbol, Label>>>,
    symbol_transitions: HandleMap<Symbol, HashSet<Handle<NfaState<Symbol, Label>>>>,
    label: Option<Label>,
}

impl<Symbol: Handled, Label> Handled for NfaState<Symbol, Label> {
    type HandleCoreType = u16;
}

impl<Symbol, Label> NfaState<Symbol, Label>
where
    Symbol: Handled,
{
    fn new() -> Self {
        Self {
            epsilon_transitions: HashSet::new(),
            symbol_transitions: HandleMap::new(),
            label: None,
        }
    }
}

pub struct Nfa<Symbol, Label>
where
    Symbol: Handled,
{
    states: HandledVec<NfaState<Symbol, Label>>,    // TODO this does not utilize locality in space
    pub(super) initial_state: Option<Handle<NfaState<Symbol, Label>>>,
}

impl<Symbol, Label> Nfa<Symbol, Label>
where
    Symbol: Handled,
{
    pub fn new() -> Self {
        Self {
            states: HandledVec::new(),
            initial_state: None,
        }
    }

    pub fn new_state(&mut self) -> Handle<NfaState<Symbol, Label>> {
        self.states.insert(NfaState::new())
    }

    pub fn set_initial_state(&mut self, initial_state: Handle<NfaState<Symbol, Label>>) {
        self.initial_state = Some(initial_state);
    }

    pub fn link(
        &mut self, src: Handle<NfaState<Symbol, Label>>, dst: Handle<NfaState<Symbol, Label>>,
        transition_label: Option<Handle<Symbol>>,
    ) {
        match transition_label {
            None => self.states[src].epsilon_transitions.insert(dst),

            Some(symbol) => {
                if !self.states[src].symbol_transitions.contains_key(symbol) {
                    self.states[src].symbol_transitions.insert(symbol, HashSet::new());
                }
                let set =
                    self.states[src].symbol_transitions.get_mut(symbol).expect(
                        "Transition set for specified symbol should just have been added"
                    );
                set.insert(dst)
            }
        };
    }

    pub fn label(&mut self, state: Handle<NfaState<Symbol, Label>>, label: Option<Label>) {
        self.states[state].label = label
    }

    pub fn get_label(&self, state: Handle<NfaState<Symbol, Label>>) -> &Option<Label> {
        &self.states[state].label
    }

    pub(super) fn epsilon_closure(
        &self, states: &HandleBitSet<NfaState<Symbol, Label>>,
    ) -> HandleBitSet<NfaState<Symbol, Label>> {
        let mut states_to_process: Vec<Handle<NfaState<Symbol, Label>>> =
            states.clone().iter().collect();
        let mut closure: HandleBitSet<NfaState<Symbol, Label>> = HandleBitSet::new();

        loop {
            match states_to_process.pop() {
                Some(state) => {
                    if closure.insert(state) {
                        states_to_process.extend(&self.states[state].epsilon_transitions)
                    }
                }
                None => break
            }
        }

        closure
    }

    pub(super) fn move_by_symbol(
        &self, states: &HandleBitSet<NfaState<Symbol, Label>>, symbol: Handle<Symbol>,
    ) -> HandleBitSet<NfaState<Symbol, Label>> {
        let mut result = HandleBitSet::new();

        for state in states {
            if let Some(destinations) =
                self.states[state].symbol_transitions.get(symbol)
            {
                result.extend(destinations);
            }
        }
        result
    }

    pub(super) fn list_symbols(&self) -> impl Iterator<Item=Handle<Symbol>> {
        let mut symbols: HashSet<Handle<Symbol>> = HashSet::new();
        for state in &self.states {
            symbols.extend(state.symbol_transitions.keys());
        }
        symbols.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::handle::auto::AutomaticallyHandled;
    use super::*;

    #[derive(Clone, Copy)]
    enum Symbol {
        Symbol0,
    }
    impl AutomaticallyHandled for Symbol {
        type HandleCoreType = u8;
        fn serial(&self) -> usize { *self as usize }
    }


    fn build_test_data() -> (Nfa<Symbol, u32>, Vec<Handle<NfaState<Symbol, u32>>>) {
        let mut nfa = Nfa::new();
        let states =
            vec![nfa.new_state(), nfa.new_state(), nfa.new_state()];

        nfa.link(states[0], states[1], None);
        nfa.link(states[0], states[1], Some(Symbol::Symbol0.handle()));
        nfa.link(states[0], states[2], Some(Symbol::Symbol0.handle()));
        nfa.link(states[2], states[0], Some(Symbol::Symbol0.handle()));
        nfa.link(states[2], states[0], None);

        (nfa, states)
    }

    #[test]
    fn test_nfa_closure() {
        let (nfa, states) = build_test_data();
        assert_eq!(
            nfa.epsilon_closure(&vec![states[0]].iter().collect()),
            vec![states[0], states[1]].iter().collect(),
        )
    }

    #[test]
    fn test_nfa_move_by_symbol() {
        let (nfa, states) = build_test_data();
        assert_eq!(
            nfa.move_by_symbol(&vec![states[0]].iter().collect(), Symbol::Symbol0.handle()),
            vec![states[1], states[2]].iter().collect()
        )
    }
}

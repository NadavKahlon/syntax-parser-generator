use std::collections::HashMap;

use crate::automata::dfa::{Dfa, DfaState};
use crate::automata::nfa::{Nfa, NfaState};
use crate::handles::{Handle, Handled};
use crate::handles::collections::HandleBitSet;

impl<Symbol, Label> Nfa<Symbol, Label>
where
    Symbol: Handled,
{
    pub fn compile_to_dfa<F, DfaLabel>(&self, label_reduction: F) -> Dfa<Symbol, DfaLabel>
    where
        F: Fn(Vec<&Label>) -> Option<DfaLabel>,
    {
        NfaToDfaCompiler::new(self, label_reduction).compile()
    }
}

struct NfaToDfaCompiler<'a, Symbol, NfaLabel, DfaLabel, F>
where
    Symbol: Handled,
    F: Fn(Vec<&NfaLabel>) -> Option<DfaLabel>,
{
    nfa: &'a Nfa<Symbol, NfaLabel>,
    dfa: Dfa<Symbol, DfaLabel>,
    dfa_states_map:
        HashMap<HandleBitSet<NfaState<Symbol, NfaLabel>>, Handle<DfaState<Symbol, DfaLabel>>>,
    unprocessed_new_states: Vec<(
        HandleBitSet<NfaState<Symbol, NfaLabel>>,
        Handle<DfaState<Symbol, DfaLabel>>,
    )>,
    label_reduction: F,
    all_symbols: Vec<Handle<Symbol>>,
    initial_nfa_state: Handle<NfaState<Symbol, NfaLabel>>,
}

impl<'a, Symbol, NfaLabel, DfaLabel, F> NfaToDfaCompiler<'a, Symbol, NfaLabel, DfaLabel, F>
where
    Symbol: Handled,
    F: Fn(Vec<&NfaLabel>) -> Option<DfaLabel>,
{
    fn new(nfa: &'a Nfa<Symbol, NfaLabel>, label_reduction: F) -> Self {
        let initial_nfa_state = nfa
            .initial_state
            .expect("Cannot compile an NFA with no initial state into a DFA");
        Self {
            nfa,
            label_reduction,
            initial_nfa_state,
            dfa: Dfa::new(),
            dfa_states_map: HashMap::new(),
            unprocessed_new_states: Vec::new(),
            all_symbols: nfa.list_symbols().collect(),
        }
    }

    fn compile(mut self) -> Dfa<Symbol, DfaLabel> {
        let initial_nfa_state_set: HandleBitSet<NfaState<Symbol, NfaLabel>> = self
            .nfa
            .epsilon_closure(&vec![self.initial_nfa_state].iter().collect());
        let initial_dfa_state = self.install_new_state(&initial_nfa_state_set);
        self.dfa.set_initial_state(initial_dfa_state);

        loop {
            match self.unprocessed_new_states.pop() {
                Some((nfa_states, dfa_state)) => {
                    self.process_new_state(nfa_states, dfa_state);
                }
                None => break,
            }
        }

        self.reduce_labels();
        self.dfa
    }

    fn install_new_state(
        &mut self,
        nfa_states: &HandleBitSet<NfaState<Symbol, NfaLabel>>,
    ) -> Handle<DfaState<Symbol, DfaLabel>> {
        let dfa_state = self.dfa.new_state();
        self.dfa_states_map.insert(nfa_states.clone(), dfa_state);
        self.unprocessed_new_states
            .push((nfa_states.clone(), dfa_state));
        dfa_state
    }

    fn process_new_state(
        &mut self,
        nfa_states: HandleBitSet<NfaState<Symbol, NfaLabel>>,
        dfa_state: Handle<DfaState<Symbol, DfaLabel>>,
    ) {
        for &symbol in &self.all_symbols.clone() {
            let target_nfa_states = self
                .nfa
                .epsilon_closure(&self.nfa.move_by_symbol(&nfa_states, symbol));

            if !target_nfa_states.is_empty() {
                let target_dfa_state = match self.dfa_states_map.get(&target_nfa_states) {
                    None => self.install_new_state(&target_nfa_states),
                    Some(&dfa_state) => dfa_state,
                };

                self.dfa.link(dfa_state, target_dfa_state, symbol);
            }
        }
    }

    fn reduce_labels(&mut self) {
        for (nfa_states, &dfa_state) in &self.dfa_states_map {
            let nfa_labels = nfa_states
                .iter()
                .flat_map(|nfa_state| self.nfa.get_label(nfa_state))
                .collect();

            let label = (self.label_reduction)(nfa_labels);
            self.dfa.label(dfa_state, label);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::handles::specials::AutomaticallyHandled;

    use super::*;

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum Symbol {
        Symbol1,
        Symbol2,
    }
    impl AutomaticallyHandled for Symbol {
        type HandleCoreType = u8;
        fn serial(&self) -> usize {
            *self as usize
        }
    }

    fn build_dfa_by_compiling_nfa() -> Dfa<Symbol, ()> {
        let mut nfa = Nfa::new();
        let states = vec![nfa.new_state(), nfa.new_state(), nfa.new_state()];

        nfa.link(states[0], states[1], Some(Symbol::Symbol1.handle()));
        nfa.link(states[1], states[0], None);
        nfa.link(states[1], states[2], Some(Symbol::Symbol2.handle()));

        nfa.set_initial_state(states[0]);
        nfa.label(states[2], Some(()));

        nfa.compile_to_dfa(|labels| if labels.is_empty() { None } else { Some(()) })
    }

    fn build_dfa_manually() -> Dfa<Symbol, ()> {
        let mut dfa = Dfa::new();

        let state_0 = dfa.new_state();
        let state_0_1 = dfa.new_state();
        let state_2 = dfa.new_state();

        dfa.link(state_0, state_0_1, Symbol::Symbol1.handle());
        dfa.link(state_0_1, state_0_1, Symbol::Symbol1.handle());
        dfa.link(state_0_1, state_2, Symbol::Symbol2.handle());

        dfa.set_initial_state(state_0);
        dfa.label(state_2, Some(()));
        dfa
    }

    #[test]
    fn test() {
        assert_eq!(build_dfa_by_compiling_nfa(), build_dfa_manually(),)
    }
}

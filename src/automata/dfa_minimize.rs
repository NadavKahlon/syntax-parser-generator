use std::collections::HashMap;
use std::hash::Hash;
use derive_where::derive_where;
use crate::automata::dfa::{Dfa, DfaState};
use crate::handle::{Handle, Handled};
use crate::handle::handle_map::HandleMap;
use crate::handle::handled_vec::HandledVec;

impl<Symbol, Label> Dfa<Symbol, Label>
where
    Symbol: Handled,
    Label: Hash + Eq + Clone,
{
    pub fn minimize(self) -> Self {
        DfaMinimizer::new(self).minimize()
    }

    fn complete_with_dead_state(&mut self) {
        let symbols: Vec<Handle<Symbol>> = self.list_symbols().collect();
        let dead_state = self.new_state();

        for &symbol in &symbols {
            self.link(dead_state, dead_state, symbol);
            for state in self.states.list_handles() {
                if self.step(state, symbol).is_none() {
                    self.link(state, dead_state, symbol);
                }
            }
        }
    }

    fn reduce_by_dead_state(self) -> Dfa<Symbol, Label> {
        let dead_state = self.locate_dead_state();

        let mut new_dfa = Dfa::new();
        let mut states_map: HandleMap<DfaState<Symbol, Label>, Handle<DfaState<Symbol, Label>>>
            = HandleMap::new();

        for state in self.states.list_handles() {
            if Some(state) != dead_state {
                let new_state = new_dfa.new_state();
                states_map.insert(state, new_state);
                new_dfa.label(new_state, self.get_label(state).clone());
            }
        }

        if let Some(origin_initial_state) = self.initial_state {
            let &initial_state = states_map.get(origin_initial_state).expect(
                "DFA cannot be reduced from its dead state if it's the initial state"
            );
            new_dfa.set_initial_state(initial_state);
        }

        for (origin_src, &src) in &states_map {
            for symbol in self.list_symbols() {
                let optional_origin_tar = self.step(origin_src, symbol);
                if let Some(origin_tar) = optional_origin_tar {
                    if Some(origin_tar) != dead_state {
                        let &tar = states_map.get(origin_tar).expect(
                            "Every non-dead state in the original DFA should be associated \
                            with some state in the new DFA"
                        );
                        new_dfa.link(src, tar, symbol);
                    }
                }
            }
        }

        new_dfa
    }

    fn locate_dead_state(&self) -> Option<Handle<DfaState<Symbol, Label>>> {
        self.list_states()
            .filter(|&state| self.is_dead_state(state))
            .next()
    }

    fn is_dead_state(&self, state: Handle<DfaState<Symbol, Label>>) -> bool {
        self.get_label(state).is_none() &&
            self.states[state].transitions.iter()
                .all(|(_, &target)| {
                    state == target
                })
    }
}

struct DfaMinimizer<Symbol, Label>
where
    Symbol: Handled,
    Label: Hash + Eq + Clone,
{
    dfa: Dfa<Symbol, Label>,
    equivalence_sets: HandledVec<EquivalenceSet<Symbol, Label>>,
    equivalence_map: HandleMap<DfaState<Symbol, Label>, Handle<EquivalenceSet<Symbol, Label>>, >,
    symbols: Vec<Handle<Symbol>>,
}

impl<Symbol, Label> DfaMinimizer<Symbol, Label>
where
    Symbol: Handled,
    Label: Hash + Eq + Clone,
{
    fn new(mut dfa: Dfa<Symbol, Label>) -> Self {
        dfa.complete_with_dead_state();

        let mut equivalence_sets = HandledVec::new();
        let mut equivalence_map = HandleMap::new();
        let mut label_map: HashMap<Option<Label>, Handle<EquivalenceSet<Symbol, Label>>> =
            HashMap::new();
        // TODO remove dependency on label-hashability: this is the only place it is used

        for state in dfa.list_states() {
            let label = dfa.get_label(state);

            let set = match label_map.get(label) {
                Some(&set) => set,
                None => {
                    let set = equivalence_sets.insert(EquivalenceSet::new());
                    label_map.insert(label.clone(), set);
                    set
                }
            };

            equivalence_sets[set].states.push(state);
            equivalence_map.insert(state, set);
        }

        let symbols = dfa.list_symbols().collect();
        Self { dfa, equivalence_sets, equivalence_map, symbols }
    }

    fn minimize(mut self) -> Dfa<Symbol, Label> {
        while self.equivalence_sets.clone().list_handles()
            .map(|set| self.split_equivalence_set(set))
            .collect::<Vec<bool>>().iter()
            .any(|x| *x)
        {}
        self.finalize_dfa()
    }

    fn split_equivalence_set(&mut self, set_handle: Handle<EquivalenceSet<Symbol, Label>>) -> bool {
        let unprocessed: Vec<Handle<DfaState<Symbol, Label>>> =
            self.equivalence_sets[set_handle].states.drain(1..).collect();
        let mut subsets: Vec<Handle<EquivalenceSet<Symbol, Label>>> = vec![set_handle];

        for state in unprocessed {
            let mut containing_subset: Option<Handle<EquivalenceSet<Symbol, Label>>> = None;

            for &subset in &subsets {
                let subset_state = self.equivalence_sets[subset].states[0];
                if self.are_states_equivalence(state, subset_state) {
                    containing_subset = Some(subset);
                    break;
                }
            }

            let containing_subset = containing_subset.unwrap_or_else(|| {
                let subset
                    = self.equivalence_sets.insert(EquivalenceSet::new());
                subsets.push(subset);
                subset
            });
            self.equivalence_sets[containing_subset].states.push(state);
        }

        for &subset_handle in &subsets {
            for &state in &self.equivalence_sets[subset_handle].states {
                self.equivalence_map.insert(state, subset_handle);
            }
        }
        subsets.len() > 1
    }


    // This should only be based on the map, as the sets themselves have been changed
    fn are_states_equivalence(
        &self, state_1: Handle<DfaState<Symbol, Label>>, state_2: Handle<DfaState<Symbol, Label>>,
    ) -> bool {
        self.symbols.iter().all(|&symbol| {
            self.get_step_equivalence(state_1, symbol) == self.get_step_equivalence(state_2, symbol)
        })
    }

    fn get_step_equivalence(
        &self, state: Handle<DfaState<Symbol, Label>>, symbol: Handle<Symbol>,
    ) -> Handle<EquivalenceSet<Symbol, Label>>
    {
        self.get_equivalence(self.dfa.step(state, symbol).expect(
            "DFA should have been completed with dead state before minimization"
        ))
    }

    fn get_equivalence(&self, state: Handle<DfaState<Symbol, Label>>)
                       -> Handle<EquivalenceSet<Symbol, Label>>
    {
        *self.equivalence_map.get(state).expect(
            "All states should be associated with an equivalence set"
        )
    }

    fn finalize_dfa(&self) -> Dfa<Symbol, Label> {
        let mut finalized_dfa = Dfa::new();

        let mut finalized_states_map = HandleMap::new();
        for set_handle in self.equivalence_sets.list_handles() {

            let new_state = finalized_dfa.new_state();
            finalized_states_map.insert(set_handle, new_state);

            let set = &self.equivalence_sets[set_handle].states;
            let label = self.dfa.get_label(set[0]);
            finalized_dfa.label(new_state, label.clone());

            if let Some(initial_state) = self.dfa.initial_state {
                if set.contains(&initial_state) {
                    finalized_dfa.set_initial_state(new_state);
                }
            }
        }

        for (set, &src) in finalized_states_map.iter() {
            for &symbol in &self.symbols {
                let origin_src = self.equivalence_sets[set].states[0];
                let origin_tar = self.dfa.step(origin_src, symbol).expect(
                    "DFA should have been completed with dead state before minimization"
                );
                let &tar_set =
                    self.equivalence_map.get(origin_tar).expect(
                        "All states should be associated with an equivalence set"
                    );
                let &tar = finalized_states_map.get(tar_set).expect(
                    "All equivalence sets should have an associated finalized-DFA state"
                );
                finalized_dfa.link(src, tar, symbol);
            }
        }

        finalized_dfa.reduce_by_dead_state()
    }
}

#[derive_where(Clone)]
struct EquivalenceSet<Symbol: Handled, Label> {
    states: Vec<Handle<DfaState<Symbol, Label>>>,
}

impl<Symbol: Handled, Label> EquivalenceSet<Symbol, Label> {
    fn new() -> Self {
        Self { states: Vec::new() }
    }
}

impl<Symbol: Handled, Label> Handled for EquivalenceSet<Symbol, Label> {
    type HandleCoreType = <DfaState<Symbol, Label> as Handled>::HandleCoreType;
}

#[cfg(test)]
mod tests {
    use crate::handle::auto::AutomaticallyHandled;
    use pretty_assertions::assert_eq;
    use super::*;

    #[derive(Clone, Copy)]
    enum Symbol {
        Symbol0,
        Symbol1,
    }
    impl AutomaticallyHandled for Symbol {
        type HandleCoreType = u8;
        fn serial(&self) -> usize { *self as usize }
    }


    fn build_original_dfa() -> Dfa<Symbol, ()> {
        let mut dfa = Dfa::new();
        let states: Vec<Handle<DfaState<Symbol, ()>>> = (0..6).map(|_| dfa.new_state()).collect();

        dfa.set_initial_state(states[0]);

        dfa.link(states[0], states[3], Symbol::Symbol0.handle());
        dfa.link(states[0], states[1], Symbol::Symbol1.handle());

        dfa.link(states[1], states[2], Symbol::Symbol0.handle());
        dfa.link(states[1], states[5], Symbol::Symbol1.handle());

        dfa.link(states[2], states[2], Symbol::Symbol0.handle());
        dfa.link(states[2], states[5], Symbol::Symbol1.handle());

        dfa.link(states[3], states[0], Symbol::Symbol0.handle());
        dfa.link(states[3], states[4], Symbol::Symbol1.handle());

        dfa.link(states[4], states[2], Symbol::Symbol0.handle());
        dfa.link(states[4], states[5], Symbol::Symbol1.handle());

        dfa.link(states[5], states[5], Symbol::Symbol0.handle());

        dfa.label(states[1], Some(()));
        dfa.label(states[2], Some(()));
        dfa.label(states[4], Some(()));

        dfa
    }

    fn build_minimized_dfa() -> Dfa<Symbol, ()> {
        let mut dfa = Dfa::new();
        let states: Vec<Handle<DfaState<Symbol, ()>>> = (0..2)
            .map(|_| dfa.new_state())
            .collect();

        dfa.set_initial_state(states[0]);

        dfa.link(states[0], states[0], Symbol::Symbol0.handle());
        dfa.link(states[0], states[1], Symbol::Symbol1.handle());

        dfa.link(states[1], states[1], Symbol::Symbol0.handle());

        dfa.label(states[1], Some(()));

        dfa
    }

    #[test]
    fn test_dfa_minimization() {
        let original_dfa = build_original_dfa();
        let minimized_dfa = build_minimized_dfa();
        // This test is too harsh, as it does not account for isomorphism DFAs
        assert_eq!(original_dfa.minimize(), minimized_dfa);
    }
}

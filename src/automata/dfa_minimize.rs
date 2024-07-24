use std::cmp::PartialEq;
use std::collections::HashMap;
use crate::automata::dfa::{DfaBuilder, DfaStateHandle};
use crate::automata::InputSymbol;
use crate::automata::labeled_dfa::{DfaLabel, LabeledDfa};

impl LabeledDfa {
    fn minimize(&self) -> Self {
        LabeledDfaMinimizer::new(self).calculate_minimized_dfa()
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
struct EquivalenceIndex(usize);

struct LabeledDfaMinimizer<'a> {
    labeled_dfa: &'a LabeledDfa,
    equivalence_map: Vec<EquivalenceIndex>,
    equivalence_sets: Vec<Vec<DfaStateHandle>>,
}

impl<'a> LabeledDfaMinimizer<'a> {
    fn new(labeled_dfa: &'a LabeledDfa) -> Self {
        let mut result = Self {
            labeled_dfa,
            equivalence_map: vec![EquivalenceIndex(0); labeled_dfa.dfa.states.len()],
            equivalence_sets: vec![],
        };
        result.build_initial_equivalence_data();
        result
    }

    fn calculate_minimized_dfa(mut self) -> LabeledDfa {
        while self
            .equivalence_sets
            .clone()
            .iter()
            .map(|equivalence_set| self.split_equivalence_set(equivalence_set))
            .collect::<Vec<bool>>()
            .iter()
            .any(|x| *x)
        {
            self.rebuild_equivalence_sets();
        }
        self.finalize_dfa()
    }

    fn build_initial_equivalence_data(&mut self) {
        let mut equivalence_index_map: HashMap<DfaLabel, EquivalenceIndex> = HashMap::new();
        for state_index in 0..self.equivalence_map.len() {
            let state_label = self.labeled_dfa.labels[state_index];
            let equivalence_index = match equivalence_index_map.get(&state_label) {
                Some(&equivalence_index) => equivalence_index,
                None => {
                    let new_equivalence_index = self.add_equivalence_set();
                    equivalence_index_map.insert(state_label, new_equivalence_index);
                    new_equivalence_index
                }
            };
            self.equivalence_map[state_index] = equivalence_index;
            self.equivalence_sets[equivalence_index.0]
                .push(DfaStateHandle { id: state_index as u16 });
        }
    }

    fn add_equivalence_set(&mut self) -> EquivalenceIndex {
        self.equivalence_sets.push(vec![]);
        EquivalenceIndex(self.equivalence_sets.len() - 1)
    }

    // TODO document that this should not alter the equivalence sets, only the equivalence map
    // (the equivalence sets were cloned, and will be rebuilt once all sets are split)
    fn split_equivalence_set(&mut self, equivalence_set: &Vec<DfaStateHandle>) -> bool {
        let mut subsets: Vec<Vec<DfaStateHandle>> = vec![vec![equivalence_set[0]]];
        for &state in &equivalence_set[1..] {
            let mut was_added = false;
            for subset in &mut subsets {
                if self.are_states_equivalent(state, subset[0]) {
                    subset.push(state);
                    was_added = true;
                    break;
                }
            }
            if !was_added {
                // New equivalence subset shall be added
                subsets.push(vec![state]);
            }
        }

        if subsets.len() > 1 {
            // First subset's equivalence index remains the same
            for new_subset in &subsets[1..] {
                let new_equivalence_index = self.add_equivalence_set();
                for &state in new_subset {
                    self.equivalence_map[state.id as usize] = new_equivalence_index;
                }
            }
            true
        } else {
            false
        }
    }

    // This should only consult the map (as it is the most updated)
    fn are_states_equivalent(&self, state_1: DfaStateHandle, state_2: DfaStateHandle) -> bool {
        (0..self.labeled_dfa.dfa.num_symbols())
            .map(|symbol_id| {
                let symbol = InputSymbol { id: symbol_id as u16 }; // Possible type confusion
                let state_1_target = self.labeled_dfa.dfa.step(state_1, symbol);
                let state_1_eq = self.equivalence_map[state_1_target.id as usize];
                let state_2_target = self.labeled_dfa.dfa.step(state_2, symbol);
                let state_2_eq = self.equivalence_map[state_2_target.id as usize];
                state_1_eq == state_2_eq
            })
            .all(|x| x)
    }

    fn rebuild_equivalence_sets(&mut self) {
        self.equivalence_sets = vec![vec![]; self.equivalence_sets.len()];
        self.equivalence_map
            .iter()
            .enumerate()
            .for_each(|(state_index, equivalence_index)| {
                let state = DfaStateHandle { id: state_index as u16 };  // Possible type confusion
                self.equivalence_sets[equivalence_index.0].push(state);
            });
    }

    fn finalize_dfa(&self) -> LabeledDfa {
        let mut dfa_builder = DfaBuilder::new(self.labeled_dfa.dfa.num_symbols());
        let dfa_states: Vec<DfaStateHandle> =
            (0..self.equivalence_sets.len())
                .map(|_| dfa_builder.new_state())
                .collect();

        Iterator::zip(dfa_states.iter(), self.equivalence_sets.iter())
            .for_each(|(&src_state, equivalence_set)| {
                let original_src_state = equivalence_set[0];
                for symbol_id in 0..self.labeled_dfa.dfa.num_symbols() {
                    let symbol = InputSymbol { id: symbol_id as u16 };
                    let original_tar_state =
                        self.labeled_dfa.dfa.step(original_src_state, symbol);
                    let tar_equivalence_index =
                        self.equivalence_map[original_tar_state.id as usize];
                    let tar_state = dfa_states[tar_equivalence_index.0];
                    dfa_builder.link(src_state, tar_state, symbol);
                }
            });

        let initial_state = dfa_states[self.get_initial_state_equivalence_index().0];
        let dfa = dfa_builder.build(initial_state).expect(
            "The DFA minimization process should have handled all possible transitions"
        );

        let mut new_labeled_dfa = LabeledDfa::new(dfa);
        for (raw_equivalence_index, &dfa_state) in dfa_states.iter().enumerate() {
            let example_state = self.equivalence_sets[raw_equivalence_index][0];
            new_labeled_dfa.label(dfa_state, self.labeled_dfa.get_label(example_state));
        }

        new_labeled_dfa
    }

    fn get_initial_state_equivalence_index(&self) -> EquivalenceIndex {
        self.equivalence_map[self.labeled_dfa.dfa.initial_state.id as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_original_dfa() -> LabeledDfa {
        let mut dfa_builder = DfaBuilder::new(2);
        let states: Vec<DfaStateHandle> = (0..6)
            .map(|_| dfa_builder.new_state())
            .collect();

        dfa_builder.link(states[0], states[3], InputSymbol { id: 0 });
        dfa_builder.link(states[0], states[1], InputSymbol { id: 1 });

        dfa_builder.link(states[1], states[2], InputSymbol { id: 0 });
        dfa_builder.link(states[1], states[5], InputSymbol { id: 1 });

        dfa_builder.link(states[2], states[2], InputSymbol { id: 0 });
        dfa_builder.link(states[2], states[5], InputSymbol { id: 1 });

        dfa_builder.link(states[3], states[0], InputSymbol { id: 0 });
        dfa_builder.link(states[3], states[4], InputSymbol { id: 1 });

        dfa_builder.link(states[4], states[2], InputSymbol { id: 0 });
        dfa_builder.link(states[4], states[5], InputSymbol { id: 1 });

        dfa_builder.link(states[5], states[5], InputSymbol { id: 0 });
        dfa_builder.link(states[5], states[5], InputSymbol { id: 1 });

        let dfa = dfa_builder.build(states[0]).unwrap();
        let mut labeled_dfa = LabeledDfa::new(dfa);

        labeled_dfa.label(states[0], DfaLabel(0));
        labeled_dfa.label(states[1], DfaLabel(1));
        labeled_dfa.label(states[2], DfaLabel(1));
        labeled_dfa.label(states[3], DfaLabel(0));
        labeled_dfa.label(states[4], DfaLabel(1));
        labeled_dfa.label(states[5], DfaLabel(0));

        labeled_dfa
    }

    fn build_minimized_dfa() -> LabeledDfa {
        let mut dfa_builder = DfaBuilder::new(2);
        let states: Vec<DfaStateHandle> = (0..3)
            .map(|_| dfa_builder.new_state())
            .collect();

        dfa_builder.link(states[0], states[0], InputSymbol { id: 0 });
        dfa_builder.link(states[0], states[1], InputSymbol { id: 1 });

        dfa_builder.link(states[1], states[1], InputSymbol { id: 0 });
        dfa_builder.link(states[1], states[2], InputSymbol { id: 1 });

        dfa_builder.link(states[2], states[2], InputSymbol { id: 0 });
        dfa_builder.link(states[2], states[2], InputSymbol { id: 1 });

        let dfa = dfa_builder.build(states[0]).unwrap();
        let mut labeled_dfa = LabeledDfa::new(dfa);

        labeled_dfa.label(states[0], DfaLabel(0));
        labeled_dfa.label(states[1], DfaLabel(1));
        labeled_dfa.label(states[2], DfaLabel(0));

        labeled_dfa
    }

    #[test]
    fn test_dfa_minimization() {
        let original_dfa = build_original_dfa();
        let minimized_dfa = build_minimized_dfa();
        // This test is too harsh, as it does not account for isomorphism DFAs
        assert_eq!(original_dfa.minimize(), minimized_dfa);
    }
}
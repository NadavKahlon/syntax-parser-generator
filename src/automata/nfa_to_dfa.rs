use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use bit_set::BitSet;
use super::nfa::{Nfa, NfaStateHandle};
use super::dfa::{Dfa, DfaBuilder, DfaStateHandle};
use super::InputSymbol;

// TODO consider relocating this
impl Nfa {
    pub fn compile_to_dfa(&self) -> Dfa {
        NfaToDfaCompiler::new(self).compile()
    }
}

struct NfaToDfaCompiler<'a> {
    nfa: &'a Nfa,
    dfa_builder: DfaBuilder,
    dfa_states_map: HashMap<NfaStatesSetHashableWrapper, DfaStateHandle>,
    unprocessed_new_states: Vec<(DfaStateHandle, HashSet<NfaStateHandle>)>,
}

impl<'a> NfaToDfaCompiler<'a> {
    fn new(nfa: &'a Nfa) -> NfaToDfaCompiler {
        NfaToDfaCompiler {
            nfa,
            dfa_builder: DfaBuilder::new(nfa.num_symbols),
            dfa_states_map: HashMap::new(),
            unprocessed_new_states: vec![],
        }
    }

    fn compile(mut self) -> Dfa {
        let initial_nfa_state_set: HashSet<NfaStateHandle> =
            self.nfa.epsilon_closure(&[self.nfa.initial_state].into());
        let initial_dfa_state = self.install_new_state(initial_nfa_state_set);

        loop {
            match self.unprocessed_new_states.pop() {
                Some((dfa_state, nfa_states_set)) => {
                    self.process_new_state(dfa_state, nfa_states_set);
                }
                None => break,
            }
        }

        self.dfa_builder
            .build(initial_dfa_state)
            .expect("NfaToDfaCompiler should handle all symbol transitions")
    }

    fn install_new_state(&mut self, nfa_states_set: HashSet<NfaStateHandle>) -> DfaStateHandle {
        let dfa_state = self.dfa_builder.new_state();
        self.dfa_states_map.insert(NfaStatesSetHashableWrapper(nfa_states_set.clone()), dfa_state);
        self.unprocessed_new_states.push((dfa_state, nfa_states_set));
        dfa_state
    }

    fn process_new_state(
        &mut self, dfa_state: DfaStateHandle, nfa_states_set: HashSet<NfaStateHandle>,
    ) {
        for id in 0_u16..self.nfa.num_symbols {
            let symbol = InputSymbol { id };
            let target_nfa_states_set =
                self.nfa.epsilon_closure(&self.nfa.move_by_symbol(&nfa_states_set, symbol));

            // TODO prettify the borrowing juggling we do here because of the wrapper
            let target_nfa_states_set_wrapper = NfaStatesSetHashableWrapper(target_nfa_states_set);
            if !self.dfa_states_map.contains_key(&target_nfa_states_set_wrapper) {
                let new_dfa_state = self.install_new_state(target_nfa_states_set_wrapper.0);
                self.dfa_builder.link(dfa_state, new_dfa_state, symbol);
            }
        }
    }
}

// Required for efficient lookup of the DFA state associated with a given set of NFA states
#[derive(PartialEq, Eq)]
struct NfaStatesSetHashableWrapper(HashSet<NfaStateHandle>);

impl Hash for NfaStatesSetHashableWrapper {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let bitset: BitSet = self.0
            .iter()
            .map(|state| state.id as usize)
            .collect();
        bitset.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use std::hash::DefaultHasher;
    use super::*;

    #[test]
    fn test_nfa_states_set_hashing() {
        let set1: HashSet<NfaStateHandle> = [
            NfaStateHandle { id: 0 },
            NfaStateHandle { id: 1 },
            NfaStateHandle { id: 2 },
            NfaStateHandle { id: 1 },
        ].into_iter().collect();
        let set2: HashSet<NfaStateHandle> = [
            NfaStateHandle { id: 2 },
            NfaStateHandle { id: 0 },
            NfaStateHandle { id: 0 },
            NfaStateHandle { id: 1 },
            NfaStateHandle { id: 2 },
        ].into_iter().collect();

        assert_eq!(set1, set2);
        assert_eq!(hash_nfa_states_set(set1), hash_nfa_states_set(set2));
    }

    fn hash_nfa_states_set(set: HashSet<NfaStateHandle>) -> u64 {
        let mut hasher = DefaultHasher::new();
        let set_wrapper = NfaStatesSetHashableWrapper(set);
        set_wrapper.hash(&mut hasher);
        hasher.finish()
    }
}
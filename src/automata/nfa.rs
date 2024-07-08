use std::collections::HashSet;
use super::InputSymbol;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct NfaStateHandle {
    id: u16,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct NfaStateLabel {
    id: u16,
}

struct NfaState {
    label: Option<NfaStateLabel>,
    epsilon_transitions: HashSet<NfaStateHandle>,
    symbol_transitions: Box<[HashSet<NfaStateHandle>]>,  // Symbols have constant size
}

impl NfaState {
    fn new(num_symbols: u16) -> NfaState {
        NfaState {
            label: None,
            epsilon_transitions: HashSet::new(),
            symbol_transitions: vec![HashSet::new(); num_symbols as usize].into_boxed_slice(),
        }
    }
}

// More optimized to have a dynamic nature - an automaton being built live
struct NfaBuilder {
    num_symbols: u16,
    states: Vec<NfaState>,
}

impl NfaBuilder {
    fn new(num_symbols: u16) -> NfaBuilder {
        NfaBuilder {
            num_symbols,
            states: Vec::new(),
        }
    }

    fn new_state(&mut self) -> NfaStateHandle {
        self.states.push(NfaState::new(self.num_symbols));
        NfaStateHandle {
            id: (self.states.len() - 1) as u16  // TODO possible type confusion vulnerability
        }
    }

    fn link(
        &mut self, src: NfaStateHandle, dst: NfaStateHandle,
        transition_label: Option<InputSymbol>,
    ) {
        match transition_label {
            Some(InputSymbol { id: symbol_id }) => {
                self.states[src.id as usize].symbol_transitions[symbol_id as usize].insert(dst);
            }
            None => {
                self.states[src.id as usize].epsilon_transitions.insert(dst);
            }
        }
    }

    fn label(&mut self, state: NfaStateHandle, label: Option<NfaStateLabel>) {
        self.states[state.id as usize].label = label
    }

    fn build(self, initial_state: NfaStateHandle) -> Nfa {
        Nfa {
            states: self.states.into_boxed_slice(),
            initial_state,
        }
    }
}

// More optimized to have a static nature - an automaton on which only analyses are performed
struct Nfa {
    states: Box<[NfaState]>,
    initial_state: NfaStateHandle,
}

impl Nfa {
    fn epsilon_closure(&self, states: &HashSet<NfaStateHandle>) -> HashSet<NfaStateHandle> {
        let mut states_to_process: Vec<NfaStateHandle> = states.clone().into_iter().collect();
        let mut closure: HashSet<NfaStateHandle> = HashSet::new();

        loop {
            match states_to_process.pop() {
                Some(state) => {
                    if closure.insert(state) {
                        states_to_process.extend(&self.states[state.id as usize].epsilon_transitions)
                    }
                }
                None => break
            }
        }

        closure
    }

    fn move_by_symbol(&self, states: &HashSet<NfaStateHandle>, symbol: InputSymbol) -> HashSet<NfaStateHandle> {
        states
            .iter()
            .map(|state| { &self.states[state.id as usize].symbol_transitions[symbol.id as usize] })
            .flat_map(HashSet::iter)
            .map(NfaStateHandle::clone)
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_data() -> (Nfa, Vec<NfaStateHandle>, Vec<InputSymbol>) {
        let num_symbols = 1;
        let num_states = 3;

        let symbols: Vec<InputSymbol> =
            (0..num_symbols)
                .map(|id| InputSymbol { id })
                .collect();
        let mut nfa_builder = NfaBuilder::new(symbols.len() as u16);
        let states: Vec<NfaStateHandle> = (0..num_states).map(|_| nfa_builder.new_state()).collect();

        nfa_builder.link(states[0], states[1], None);
        nfa_builder.link(states[0], states[1], Some(symbols[0]));
        nfa_builder.link(states[0], states[2], Some(symbols[0]));
        nfa_builder.link(states[2], states[0], Some(symbols[0]));
        nfa_builder.link(states[2], states[0], None);

        (nfa_builder.build(states[0]), states, symbols)
    }

    #[test]
    fn test_nfa_closure() {
        let (nfa, states, _) = build_test_data();
        assert_eq!(
            nfa.epsilon_closure(&[states[0]].into()),
            [states[0], states[1]].into()
        )
    }

    #[test]
    fn test_nfa_move_by_symbol() {
        let (nfa, states, symbols) = build_test_data();
        assert_eq!(
            nfa.move_by_symbol(&[states[0]].into(), symbols[0]),
            [states[1], states[2]].into()
        )
    }
}
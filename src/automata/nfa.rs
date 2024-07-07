// IMPROVE: use trait bounds on u8, u16, u32, u64, usize, to replace all occurrences of u16 here
// IMPROVE: we may wanna replace all these `as usize`

use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct AutomatonSymbol {
    id: u16,
}

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
        transition_label: Option<AutomatonSymbol>,
    ) {
        match transition_label {
            Some(AutomatonSymbol { id: symbol_id }) => {
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
}

// More optimized to have a static nature - an automaton on which only analyses are performed
struct Nfa {
    states: Box<[NfaState]>,
}

impl From<NfaBuilder> for Nfa {
    fn from(nfa_builder: NfaBuilder) -> Self {
        Nfa { states: nfa_builder.states.into_boxed_slice() }
    }
}

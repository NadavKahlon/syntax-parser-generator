use super::InputSymbol;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DfaStateHandle {
    pub(super) id: u16,
}

struct DfaBuilderState {
    transitions: Box<[Option<DfaStateHandle>]>,  // Symbols have constant size
}

impl DfaBuilderState {
    fn new(num_symbols: usize) -> DfaBuilderState {
        DfaBuilderState {
            transitions: vec![None; num_symbols].into_boxed_slice(),
        }
    }

    fn build(self) -> Option<DfaState> {  // May fail if some transition is not
        let mut new_transitions: Vec<DfaStateHandle> = Vec::with_capacity(self.transitions.len());
        for transition in self.transitions.into_iter() {
            new_transitions.push(transition.clone()?);
        };

        Some(DfaState { transitions: new_transitions.into_boxed_slice() })
    }
}

pub struct DfaBuilder {
    num_symbols: usize,
    states: Vec<DfaBuilderState>,
}

impl DfaBuilder {
    pub fn new(num_symbols: usize) -> DfaBuilder {
        DfaBuilder {
            num_symbols,  // Possible type confusion
            states: Vec::new(),
        }
    }

    pub fn new_state(&mut self) -> DfaStateHandle {
        self.states.push(DfaBuilderState::new(self.num_symbols));
        DfaStateHandle {
            id: (self.states.len() - 1) as u16  // TODO possible type confusion vulnerability
        }
    }

    pub fn link(&mut self, src: DfaStateHandle, dst: DfaStateHandle, symbol: InputSymbol) {
        self.states[src.id as usize].transitions[symbol.id as usize] = Some(dst);
    }

    pub fn build(self, initial_state: DfaStateHandle) -> Option<Dfa> {  // May fail if some transition is None
        let mut new_states: Vec<DfaState> = Vec::new();

        for state in self.states {
            new_states.push(state.build()?);
        }

        Some(Dfa::new(
            new_states.into_boxed_slice(),
            initial_state,
        ))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct DfaState {
    transitions: Box<[DfaStateHandle]>,  // Symbols have constant size
}

#[derive(Debug, PartialEq, Eq)]
pub struct Dfa {
    pub(super) states: Box<[DfaState]>,
    pub(super) initial_state: DfaStateHandle,
}

impl Dfa {
    fn new(states: Box<[DfaState]>, initial_state: DfaStateHandle) -> Dfa {
        Dfa { states, initial_state }
    }

    pub fn step(&self, state: DfaStateHandle, symbol: InputSymbol) -> DfaStateHandle {
        return self.states[state.id as usize].transitions[symbol.id as usize];
    }

    pub fn scan(&self, symbol_string: impl Iterator<Item=InputSymbol>) -> DfaStateHandle {
        let final_state = symbol_string
            .fold(self.initial_state, |state, symbol| self.step(state, symbol));
        return final_state;
    }

    pub fn num_symbols(&self) -> usize {
        self.states[self.initial_state.id as usize].transitions.len()
    }

    pub fn locate_dead_state(&self) -> Option<DfaStateHandle> {
        for (state_id, state) in self.states.iter().enumerate() {
            let state_handle = DfaStateHandle { id: state_id as u16 }; // Optional type confusion
            if state.transitions.iter().all(|&target_state_handle| {
                target_state_handle == state_handle
            }) {
                return Some(state_handle);
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_failing_dfa_builder() {
        let num_symbols = 2;
        let num_states = 2;

        let symbols: Vec<InputSymbol> =
            (0..num_symbols)
                .map(|id| InputSymbol { id })
                .collect();
        let mut dfa_builder = DfaBuilder::new(symbols.len());
        let states: Vec<DfaStateHandle> = (0..num_states).map(|_| dfa_builder.new_state()).collect();

        dfa_builder.link(states[0], states[0], symbols[1]);
        dfa_builder.link(states[0], states[1], symbols[0]);
        dfa_builder.link(states[1], states[0], symbols[0]);

        assert!(dfa_builder.build(states[0]).is_none())
    }

    fn build_test_data() -> (Dfa, Vec<DfaStateHandle>, Vec<InputSymbol>) {
        let num_symbols = 2;
        let num_states = 2;

        let symbols: Vec<InputSymbol> =
            (0..num_symbols)
                .map(|id| InputSymbol { id })
                .collect();
        let mut dfa_builder = DfaBuilder::new(symbols.len());
        let states: Vec<DfaStateHandle> = (0..num_states).map(|_| dfa_builder.new_state()).collect();

        dfa_builder.link(states[0], states[0], symbols[0]);
        dfa_builder.link(states[0], states[1], symbols[1]);
        dfa_builder.link(states[1], states[0], symbols[1]);
        dfa_builder.link(states[1], states[1], symbols[0]);

        (dfa_builder.build(states[0]).unwrap(), states, symbols)
    }

    #[test]
    fn test_scan_1() {
        let (dfa, states, symbols) = build_test_data();
        let input_string = [symbols[1], symbols[0], symbols[0], symbols[1]];
        assert_eq!(dfa.scan(input_string.into_iter()), states[0])
    }

    #[test]
    fn test_scan_2() {
        let (dfa, states, symbols) = build_test_data();
        let input_string = [symbols[1], symbols[0], symbols[0], symbols[1], symbols[1]];
        assert_eq!(dfa.scan(input_string.into_iter()), states[1])
    }

    #[test]
    fn test_existing_dead_state_state() {
        let num_symbols = 2;
        let num_states = 3;

        let symbols: Vec<InputSymbol> =
            (0..num_symbols)
                .map(|id| InputSymbol { id })
                .collect();
        let mut dfa_builder = DfaBuilder::new(symbols.len());
        let states: Vec<DfaStateHandle> = (0..num_states).map(|_| dfa_builder.new_state()).collect();

        dfa_builder.link(states[0], states[0], symbols[0]);
        dfa_builder.link(states[0], states[1], symbols[1]);
        dfa_builder.link(states[1], states[1], symbols[0]);
        dfa_builder.link(states[1], states[1], symbols[1]);
        dfa_builder.link(states[2], states[0], symbols[1]);
        dfa_builder.link(states[2], states[1], symbols[0]);

        let dfa = dfa_builder.build(states[0]).unwrap();
        assert_eq!(dfa.locate_dead_state(), Some(states[1]));
    }

    #[test]
    fn test_no_dead_state() {
        let (dfa, states, _) = build_test_data();
        assert_eq!(dfa.locate_dead_state(), None)
    }
}
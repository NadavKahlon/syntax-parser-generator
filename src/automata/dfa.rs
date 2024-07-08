use super::InputSymbol;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct DfaStateHandle {
    id: u16,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct DfaStateLabel {
    id: u16,
}

struct DfaBuilderState {
    label: Option<DfaStateLabel>,
    transitions: Box<[Option<DfaStateHandle>]>,  // Symbols have constant size
}

impl DfaBuilderState {
    fn new(num_symbols: u16) -> DfaBuilderState {
        DfaBuilderState {
            label: None,
            transitions: vec![None; num_symbols as usize].into_boxed_slice(),
        }
    }

    fn build(self) -> Option<DfaState> {  // May fail if some transition is not
        let mut new_transitions: Vec<DfaStateHandle> = Vec::with_capacity(self.transitions.len());
        for transition in self.transitions.into_iter() {
            new_transitions.push(transition.clone()?);
        };

        Some(DfaState {
            label: self.label,
            transitions: new_transitions.into_boxed_slice(),
        })
    }
}

struct DfaBuilder {
    num_symbols: u16,
    states: Vec<DfaBuilderState>,
}

impl DfaBuilder {
    fn new(num_symbols: u16) -> DfaBuilder {
        DfaBuilder {
            num_symbols,
            states: Vec::new(),
        }
    }

    fn new_state(&mut self) -> DfaStateHandle {
        self.states.push(DfaBuilderState::new(self.num_symbols));
        DfaStateHandle {
            id: (self.states.len() - 1) as u16  // TODO possible type confusion vulnerability
        }
    }

    fn link(&mut self, src: DfaStateHandle, dst: DfaStateHandle, symbol: InputSymbol) {
        self.states[src.id as usize].transitions[symbol.id as usize] = Some(dst);
    }

    fn label(&mut self, state: DfaStateHandle, label: Option<DfaStateLabel>) {
        self.states[state.id as usize].label = label
    }

    fn build(self, initial_state: DfaStateHandle) -> Option<Dfa> {  // May fail if some transition is None
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

struct DfaState {
    label: Option<DfaStateLabel>,
    transitions: Box<[DfaStateHandle]>,  // Symbols have constant size
}

struct Dfa {
    states: Box<[DfaState]>,
    initial_state: DfaStateHandle,
}

impl Dfa {
    fn new(states: Box<[DfaState]>, initial_state: DfaStateHandle) -> Dfa {
        Dfa { states, initial_state }
    }

    fn step(&self, state: DfaStateHandle, symbol: InputSymbol) -> DfaStateHandle {
        return self.states[state.id as usize].transitions[symbol.id as usize];
    }

    fn scan(&self, symbol_string: impl Iterator<Item=InputSymbol>) -> Option<DfaStateLabel> {
        let final_state = symbol_string
            .fold(self.initial_state, |state, symbol| self.step(state, symbol));
        return self.states[final_state.id as usize].label;
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
        let mut dfa_builder = DfaBuilder::new(symbols.len() as u16);
        let states: Vec<DfaStateHandle> = (0..num_states).map(|_| dfa_builder.new_state()).collect();

        dfa_builder.link(states[0], states[0], symbols[1]);
        dfa_builder.link(states[0], states[1], symbols[0]);
        dfa_builder.link(states[1], states[0], symbols[0]);

        assert!(dfa_builder.build(states[0]).is_none())
    }

    fn build_test_data() -> (Dfa, Vec<DfaStateHandle>, Vec<InputSymbol>, DfaStateLabel) {
        let num_symbols = 2;
        let num_states = 2;

        let symbols: Vec<InputSymbol> =
            (0..num_symbols)
                .map(|id| InputSymbol { id })
                .collect();
        let mut dfa_builder = DfaBuilder::new(symbols.len() as u16);
        let states: Vec<DfaStateHandle> = (0..num_states).map(|_| dfa_builder.new_state()).collect();
        let label = DfaStateLabel { id: 0 };

        dfa_builder.link(states[0], states[0], symbols[0]);
        dfa_builder.link(states[0], states[1], symbols[1]);
        dfa_builder.link(states[1], states[0], symbols[1]);
        dfa_builder.link(states[1], states[1], symbols[0]);
        dfa_builder.label(states[1], Some(label));

        (dfa_builder.build(states[0]).unwrap(), states, symbols, label)
    }

    #[test]
    fn test_scan_1() {
        let (dfa, states, symbols, label) = build_test_data();
        let input_string = [symbols[1], symbols[0], symbols[0], symbols[1]];
        assert_eq!(dfa.scan(input_string.into_iter()), None)
    }

    #[test]
    fn test_scan_2() {
        let (dfa, states, symbols, label) = build_test_data();
        let input_string = [symbols[1], symbols[0], symbols[0], symbols[1], symbols[1]];
        assert_eq!(dfa.scan(input_string.into_iter()), Some(label))
    }
}
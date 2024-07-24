use crate::automata::dfa::{Dfa, DfaStateHandle};
use crate::automata::dfa_minimize::LabeledDfaMinimizer;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DfaLabel(pub u8);

#[derive(Debug, PartialEq, Eq)]
pub struct LabeledDfa {
    pub dfa: Dfa,
    pub labels: Box<[DfaLabel]>,
}

impl LabeledDfa {
    // TODO doc that all labels are initially 0
    pub fn new(dfa: Dfa) -> Self {
        let labels = vec![DfaLabel(0); dfa.states.len()].into_boxed_slice();
        Self { dfa, labels }
    }

    pub fn label(&mut self, state: DfaStateHandle, label: DfaLabel) {
        self.labels[state.id as usize] = label;
    }

    pub fn get_label(&self, state: DfaStateHandle) -> DfaLabel {
        self.labels[state.id as usize]
    }

    pub fn minimize(&self) -> Self {
        LabeledDfaMinimizer::new(self).calculate_minimized_dfa()
    }
}

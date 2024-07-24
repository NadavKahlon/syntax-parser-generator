use crate::automata::dfa::{Dfa, DfaStateHandle};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DfaLabel(pub u8);

#[derive(Debug, PartialEq, Eq)]
pub struct LabeledDfa {
    pub(super) dfa: Dfa,
    pub labels: Box<[DfaLabel]>,
}

impl LabeledDfa {
    // TODO doc that all labels are initially 0
    pub fn new(dfa: Dfa) -> Self {
        let labels = vec![DfaLabel(0); dfa.states.len()].into_boxed_slice();
        Self { dfa, labels }
    }

    pub(super) fn label(&mut self, state: DfaStateHandle, label: DfaLabel) {
        self.labels[state.id as usize] = label;
    }

    pub fn get_label(&self, state: DfaStateHandle) -> DfaLabel {
        self.labels[state.id as usize]
    }
}

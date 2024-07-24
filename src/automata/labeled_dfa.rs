use std::fmt::{Debug, Formatter};
use crate::automata::dfa::{Dfa, DfaStateHandle};
use crate::automata::dfa_minimize::LabeledDfaMinimizer;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DfaLabel(pub u8);

#[derive(PartialEq, Eq)]
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

impl Debug for LabeledDfa {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Write header
        write!(f, "src\t|\t")?;
        for symbol_id in 0..self.dfa.num_symbols() {
            write!(f, "{:?}\t", symbol_id as u8 as char)?;
        }
        writeln!(f, "|\tlbl")?;

        // Write in-between bar
        write!(f, "---\t|\t")?;
        for _ in 0..self.dfa.num_symbols() {
            write!(f, "--\t")?;
        }
        writeln!(f, "-\t---")?;

        // Write body
        for (state_id, (state, &label))
        in self.dfa.states.iter().zip(self.labels.iter()).enumerate()
        {
            write!(f, "{:?}\t|\t", state_id)?;
            for &DfaStateHandle { id: target_state_id } in state.transitions.iter() {
                write!(f, "{:?}\t", target_state_id)?;
            }
            writeln!(f, "|\t{:?}", label)?;
        }
        todo!()
    }
}

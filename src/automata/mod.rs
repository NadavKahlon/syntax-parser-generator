// IMPROVE: add validation at various methods
// IMPROVE: change the internal representation of DfaState and NfaState to maximize space locality

pub mod nfa;
pub mod dfa;
mod nfa_to_dfa;
mod dfa_minimize;

// IMPROVE: add validation at various methods
// IMPROVE: change the internal representation of DfaState and NfaState to maximize space locality

pub mod dfa;
mod dfa_minimize;
pub mod nfa;
mod nfa_to_dfa;

// IMPROVE: use trait bounds on u8, u16, u32, u64, usize, to replace all occurrences of u16 here
// IMPROVE: we may wanna replace all the `as usize`
// IMPROVE: add validation at various methods
// IMPROVE: change the internal representation of DfaState and NfaState to maximize space locality

mod nfa;
mod dfa;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct InputSymbol {
    id: u16,
}

use std::fmt::Debug;
use std::hash::Hash;

pub trait HandleCore: Clone + Copy + Eq + PartialEq + Hash + Debug {
    fn into_index(self) -> usize;
    fn from_index(index: usize) -> Self;
}

impl HandleCore for u16 {
    fn into_index(self) -> usize {
        self as usize
    }

    fn from_index(index: usize) -> Self {
        index as Self // Possible type confusion
    }
}

impl HandleCore for u8 {
    fn into_index(self) -> usize {
        self as usize
    }

    fn from_index(index: usize) -> Self {
        index as Self // Possible type confusion
    }
}
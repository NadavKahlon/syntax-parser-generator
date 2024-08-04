use std::fmt::Debug;
use std::hash::Hash;

// TODO remove Ord traits from here, create a complete module for ordered handles
pub trait HandleCore: Clone + Copy + Eq + PartialEq + Hash + Debug + Ord + PartialOrd {
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
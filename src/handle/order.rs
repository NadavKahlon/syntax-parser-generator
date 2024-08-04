// TODO create a whole module for this, complete with explicit data structures

use std::cmp::Ordering;
use crate::handle::{Handle, Handled};

pub trait OrderlyHandled: Handled {}

impl<T> Ord for Handle<T>
where
    T: OrderlyHandled,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.core.cmp(&other.core)
    }
}

impl<T> PartialOrd for Handle<T>
where
    T: OrderlyHandled,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.core.partial_cmp(&other.core)
    }
}
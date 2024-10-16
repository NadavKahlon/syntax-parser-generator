use std::collections::HashSet;
use crate::handles::collections::handle_bit_set::HandleBitSet;
use crate::handles::specials::AutomaticallyHandled;

enum T {
    T1,
    T2,
    T3,
    T4,
}
impl AutomaticallyHandled for T {
    type HandleCoreType = u8;
    fn serial(&self) -> usize {
        match self {
            T::T1 => 0,
            T::T2 => 89,
            T::T3 => 1000,
            T::T4 => 3,
        }
    }
}

#[test]
fn test_insert() {
    let mut set = HandleBitSet::new();

    assert_eq!(set.contains(T::T1.handle()), false);

    set.insert(T::T1.handle());
    assert_eq!(set.contains(T::T1.handle()), true);

    set.insert(T::T2.handle());
    assert_eq!(set.contains(T::T2.handle()), true);
    assert_eq!(set.contains(T::T3.handle()), false);

    set.insert(T::T3.handle());
    assert_eq!(set.contains(T::T2.handle()), true);
    assert_eq!(set.contains(T::T3.handle()), true);
}

#[test]
fn test_union() {
    let set1: HandleBitSet<T> = vec![T::T1.handle(), T::T4.handle()].into_iter().collect();
    let set2: HandleBitSet<T> = vec![T::T2.handle()].into_iter().collect();

    assert_eq!(
        set1.union(&set2),
        vec![T::T4.handle(), T::T1.handle(), T::T2.handle()].into_iter().collect()
    )
}

#[test]
fn test_hash() {
    let set_hash_set: HashSet<HandleBitSet<T>> = vec![
        vec![T::T2.handle(), T::T3.handle()]
            .into_iter().collect(),
        vec![T::T3.handle(), T::T2.handle()]
            .into_iter().collect(),
        vec![T::T3.handle()]
            .into_iter().collect(),
        vec![T::T1.handle(), T::T3.handle()]
            .into_iter().collect(),
    ].into_iter().collect();
    assert_eq!(set_hash_set.len(), 3)
}

#[test]
fn test_canonicalize() {
    let set1: HandleBitSet<T> = HandleBitSet {
        bytes: vec![1, 3, 255, 0, 5, 0, 6, 0],
        phantom_data: Default::default(),
    };
    let set2: HandleBitSet<T> = HandleBitSet {
        bytes: vec![1, 3, 255, 0, 5, 0, 6, 0, 0],
        phantom_data: Default::default(),
    };
    assert_eq!(set1, set2)
}

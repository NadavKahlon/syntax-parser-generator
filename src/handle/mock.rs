use crate::handle::{Handle, Handled};

impl<T> Handle<T>
where
    T: Handled,
{
    // Create a handle to "nothing", different from all other handles in the given set
    pub fn mock(existing_handles: Vec<Handle<T>>) -> Handle<T> {
        let existing_indices: std::collections::HashSet<usize> = existing_handles
            .iter()
            .map(|handle| handle.index())
            .collect();

        let mut index = 0;
        while existing_indices.contains(&index) {
            index += 1;
        }

        index.into()
    }
}
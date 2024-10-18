/// A unique interface for accessing a sequence of input items.
///
/// This interface is used by the lexical analyzer when accessing input text, and needs to be
/// implemented for reading bytes ([`Reader<u8>`]) before the underlying input text can be parsed.
///
/// # Implementation
///
/// A [Reader] should maintain 3 conceptual pointers into the input sequence of items, dubbed
/// `head`, `cursor`, and `tail`. All should initially point to the beginning of the sequence. The
/// interface provides an API for managing these pointers, and accessing the underlying data. This
/// API is internally used by lexical analyzers.
pub trait Reader<T> {
    /// Reads the item pointed by `cursor`, and advance `cursor` to the next item.
    ///
    /// Returns [None] if `cursor` points to no valid item (for example - when the input is
    /// exhausted).
    fn read_next(&mut self) -> Option<T>;

    /// Set `head` to point to the item pointed by `cursor`.
    fn set_head(&mut self);

    /// Set `tail` to point to the item pointed by `cursor`.
    fn set_tail(&mut self);

    /// Set `cursor` to point to the item pointed by `tail`.
    fn move_cursor_to_tail(&mut self);

    /// Get an iterator over all items between `head` (inclusive) and `tail` (exclusive).
    fn get_sequence(&self) -> impl Iterator<Item=T>;

    /// Set `cursor` and `head` to point to the item pointed by `tail`.
    fn restart_from_tail(&mut self) {
        self.move_cursor_to_tail();
        self.set_head();
        self.set_tail();
    }
}

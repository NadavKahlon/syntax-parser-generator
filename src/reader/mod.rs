mod address_based;
mod string_reader;

pub trait Reader<T> {
    fn read_next(&mut self) -> Option<T>;
    fn set_head(&mut self);
    fn set_tail(&mut self);
    fn move_cursor_to_tail(&mut self);  // The next item read is the one after tail
    fn get_sequence(&self) -> impl Iterator<Item=T>;
    fn restart_from_tail(&mut self) {
        self.move_cursor_to_tail();
        self.set_head();
        self.set_tail();
    }
}

mod string_reader;

pub trait Reader<T> {
    fn read_unit(&mut self, address: usize) -> T;
    fn read_sequence(&mut self, start: usize, end: usize) -> impl Iterator<Item=T> {
        (start..end).map(|address| self.read_unit(address))
    }
}
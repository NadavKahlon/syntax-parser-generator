mod string_reader;

pub trait Reader<T> {
    fn read_unit(&self, address: usize) -> T;
    fn read_sequence(&self, start: usize, end: usize) -> impl Iterator<Item=T> {
        (start..end).map(|address| self.read_unit(address))
    }
}
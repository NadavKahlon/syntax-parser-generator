use crate::automata::InputSymbol;
use crate::reader::Reader;

pub struct StringReader {
    data: Box<[u8]>,
}

// TODO doc: only supports ASCII
impl StringReader {
    fn new(data: String) -> Self {
        StringReader { data: data.into_bytes().into_boxed_slice() }
    }
}

impl Reader<InputSymbol> for StringReader {
    fn read_unit(&mut self, address: usize) -> InputSymbol {
        InputSymbol { id: self.data[address] as u16 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_unit() {
        let mut reader = StringReader::new("Tell me why".to_string());
        assert_eq!(reader.read_unit(3), InputSymbol { id: 'l' as u16})
    }

    #[test]
    fn test_read_sequence() {
        let mut reader = StringReader::new("Tell me why".to_string());
        assert_eq!(
            reader.read_sequence(5,7).collect::<Vec<InputSymbol>>(),
            vec![
                InputSymbol { id: 'm' as u16},
                InputSymbol { id: 'e' as u16},
            ],
        )
    }
}

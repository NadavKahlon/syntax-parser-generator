pub trait SerialNumber: Clone + Copy {
    fn into_index(self) -> usize;
    fn from_index(index: usize) -> Self;
}

impl SerialNumber for u16 {
    fn into_index(self) -> usize {
        self as usize
    }

    fn from_index(index: usize) -> Self {
        index as Self // Possible type confusion
    }
}

impl SerialNumber for u8 {
    fn into_index(self) -> usize {
        self as usize
    }

    fn from_index(index: usize) -> Self {
        index as Self // Possible type confusion
    }
}
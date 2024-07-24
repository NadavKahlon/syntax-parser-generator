use crate::lexical_analyzer::Lexeme;

use crate::lexical_analyzer::LexicalAnalyzer;

use crate::reader::Reader;

pub struct LexemeIterator<'a, T, U>
where
    U: Reader<u8>,
    T: Clone,
{
    lexical_analyzer: &'a LexicalAnalyzer<T>,
    reader: &'a mut U,
}

impl<'a, T, U> LexemeIterator<'a, T, U>
where
    U: Reader<u8>,
    T: Clone,
{
    pub fn new(lexical_analyzer: &'a LexicalAnalyzer<T>, reader: &'a mut U) -> Self
    {
        Self { lexical_analyzer, reader }
    }
}

impl<'a, T, U> Iterator for LexemeIterator<'a, T, U>
where
    U: Reader<u8>,
    T: Clone,
{
    type Item = Lexeme<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lexical_analyzer.collect_next_lexeme(self.reader)
    }
}

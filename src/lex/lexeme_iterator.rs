use std::hash::Hash;
use crate::lex::Lexeme;
use crate::lex::lexical_analyzer::LexicalAnalyzer;
use crate::reader::Reader;

pub struct LexemeIterator<'a, LexemeType, ReaderStruct>
where
    ReaderStruct: Reader<u8>,
    LexemeType: Clone,
{
    lexical_analyzer: &'a LexicalAnalyzer<LexemeType>,
    reader: &'a mut ReaderStruct,
}

impl<'a, LexemeType, ReaderType> LexemeIterator<'a, LexemeType, ReaderType>
where
    ReaderType: Reader<u8>,
    LexemeType: Clone,
{
    pub fn new(lexical_analyzer: &'a LexicalAnalyzer<LexemeType>, reader: &'a mut ReaderType) -> Self
    {
        Self { lexical_analyzer, reader }
    }
}


impl<'a, LexemeType, ReaderType> Iterator for LexemeIterator<'a, LexemeType, ReaderType>
where
    ReaderType: Reader<u8>,
    LexemeType: Hash + Clone + Eq,
{
    type Item = Lexeme<LexemeType>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lexical_analyzer.collect_next_lexeme(self.reader)
    }
}

use regex::Regex;
use crate::handle::auto::AutomaticallyHandled;

pub mod lexeme_iterator;
pub mod regex;
pub mod lexical_analyzer;
#[cfg(test)]
mod tests;

pub struct LexemeDescriptor<LexemeType> {
    pattern: Regex,
    lexeme_type: LexemeType,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Lexeme<LexemeType> {
    lexeme_type: LexemeType,
    contents: String,
}

impl AutomaticallyHandled for u8 {
    type HandleCoreType = u8;
    fn serial(&self) -> usize { *self as usize }
}

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

impl<LexemeType> LexemeDescriptor<LexemeType> {
    pub fn new(pattern: Regex, lexeme_type: LexemeType) -> Self {
        LexemeDescriptor { pattern, lexeme_type }
    }

    pub fn keyword(name: &str, lexeme_type: LexemeType) -> Self {
        Self::new(Regex::constant_string(name), lexeme_type)
    }

    pub fn special_char(value: char, lexeme_type: LexemeType) -> Self {
        Self::new(Regex::single_char(value), lexeme_type)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Lexeme<LexemeType> {
    pub lexeme_type: LexemeType,
    pub contents: String,
}

impl<LexemeType> Lexeme<LexemeType> {
    pub fn new(lexeme_type: LexemeType, contents: &str) -> Self {
        Self {
            lexeme_type,
            contents: String::from(contents),
        }
    }
}

impl AutomaticallyHandled for u8 {
    type HandleCoreType = u8;
    fn serial(&self) -> usize { *self as usize }
}

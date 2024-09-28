use regex::Regex;
use crate::handle::auto::AutomaticallyHandled;

pub mod lexeme_iterator;
pub mod regex;
pub mod lexical_analyzer;
#[cfg(test)]
mod tests;

pub struct LexemeDescriptor<LexemeType> {
    lexeme_type: LexemeType,
    pattern: Regex,
}

impl<LexemeType> LexemeDescriptor<LexemeType> {
    pub fn new(lexeme_type: LexemeType, pattern: Regex) -> Self {
        LexemeDescriptor { lexeme_type, pattern }
    }

    pub fn keyword(lexeme_type: LexemeType, name: &str) -> Self {
        Self::new(lexeme_type, Regex::constant_string(name))
    }

    pub fn special_char(lexeme_type: LexemeType, value: char) -> Self {
        Self::new(lexeme_type, Regex::single_char(value))
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

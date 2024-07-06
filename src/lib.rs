use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};


pub mod regex;

pub struct LexemeDescriptor<T> {
    pub lexeme_type: T,
    pub pattern: regex::Regex,
}

pub struct Lexeme<T> {
    pub lexeme_type: T,
    pub string: String,
}

#[derive(Serialize, Deserialize)]
pub struct LexicalAnalyzer<T> {}

impl<T> LexicalAnalyzer<T> {
    pub fn analyze_string(self, source: &str) -> impl Iterator<Item=Lexeme<T>> {
        todo!()
    }
}

impl<T> LexicalAnalyzer<T>
where
    T: Serialize + Deserialize,
{
    pub fn save_to_json_file(self, filename: &str) {
        todo!()
    }
    pub fn load_from_json_file(filepath: &str) -> LexicalAnalyzer<T> {
        todo!()
    }
}